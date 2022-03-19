use crate::coding::xrandr::{GetCrtcInfoRequest, GetCrtcInfoResponse, SetCrtcConfigRequest, SetCrtcConfigResponse, GetCrtcGammaSizeRequest, GetCrtcGammaSizeResponse, GetCrtcGammaRequest, GetCrtcGammaResponse, SetCrtcGammaRequest, SetCrtcTransformRequest, Fp1616, GetCrtcTransformRequest, GetCrtcTransformResponse, GetPanningRequest, GetPanningResponse, Transform, SetPanningRequest, SetPanningResponse};
pub use crate::coding::xrandr::{
    Panning,
};

use super::*;

#[derive(Clone, Copy)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Crtc<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

impl<'a> Resource<'a> for Crtc<'a> {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(connection: &'a X11Connection, handle: u32) -> Self {
        Self { connection, handle }
    }
}

#[derive(Clone, Debug)]
pub struct CrtcInfo<'a> {
    pub status: SetConfig,
    pub time: Timestamp,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub mode: Mode<'a>,
    pub rotation: Rotation,
    pub rotations: Rotation,
    pub outputs: Vec<Output<'a>>,
    pub possible_outputs: Vec<Output<'a>>,
}

#[derive(Clone, Debug)]
pub struct SetCrtcConfig {
    pub status: SetConfig,
    pub time: Timestamp,
}

#[derive(Clone, Debug)]
pub struct CrtcTransform {
    pub pending_transform: [[I16F16; 3]; 3],
    pub has_transforms: bool,
    pub current_transform: [[I16F16; 3]; 3],
    pub pending_filter: String,
    pub current_filter: String,
    pub pending_params: Vec<I16F16>,
    pub current_params: Vec<I16F16>,
}

impl<'a> Crtc<'a> {
    pub async fn get_info(self, config_time: Timestamp) -> Result<CrtcInfo<'a>> {
        let seq = send_request_xrandr!(self.connection, XROpcode::GetCrtcInfo, false, GetCrtcInfoRequest {
            crtc: self.handle,
            config_time: config_time.0,
        });
        let (reply, status) = receive_reply!(self.connection, seq, GetCrtcInfoResponse, fetched);

        Ok(CrtcInfo {
            status: SetConfig::from_repr(status)?,
            time: Timestamp(reply.time),
            x: reply.x,
            y: reply.y,
            width: reply.width,
            height: reply.height,
            mode: Mode { connection: self.connection, handle: reply.mode },
            rotation: reply.rotation,
            rotations: reply.rotations,
            outputs: reply.outputs.into_iter().map(|handle| Output { connection: self.connection, handle }).collect(),
            possible_outputs: reply.possible_outputs.into_iter().map(|handle| Output { connection: self.connection, handle }).collect(),
        })
    }

    pub async fn set_config(self, time: Timestamp, config_time: Timestamp, x: i16, y: i16, mode: Mode<'_>, rotation: Rotation, outputs: impl IntoIterator<Item=Output<'_>>) -> Result<SetCrtcConfig> {
        let seq = send_request_xrandr!(self.connection, XROpcode::SetCrtcConfig, false, SetCrtcConfigRequest {
            crtc: self.handle,
            time: time.0,
            config_time: config_time.0,
            x: x,
            y: y,
            mode: mode.handle,
            rotation: rotation,
            outputs: outputs.into_iter().map(|x| x.handle).collect(),
        });
        let (reply, status) = receive_reply!(self.connection, seq, SetCrtcConfigResponse, fetched);

        Ok(SetCrtcConfig {
            status: SetConfig::from_repr(status)?,
            time: Timestamp(reply.time),
        })
    }

    pub async fn get_gamma_size(self) -> Result<u16> {
        let seq = send_request_xrandr!(self.connection, XROpcode::GetCrtcGammaSize, false, GetCrtcGammaSizeRequest {
            crtc: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, GetCrtcGammaSizeResponse);

        Ok(reply.size)
    }

    /// returns Vec<(red, green, blue)>
    pub async fn get_gamma(self) -> Result<Vec<(u16, u16, u16)>> {
        let seq = send_request_xrandr!(self.connection, XROpcode::GetCrtcGamma, false, GetCrtcGammaRequest {
            crtc: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, GetCrtcGammaResponse);

        Ok(reply.red.into_iter().zip(reply.green.into_iter()).zip(reply.blue.into_iter()).map(|((red, green), blue)| (red, green, blue)).collect())
    }

    pub async fn set_gamma(self, rgb: Vec<(u16, u16, u16)>) -> Result<()> {
        send_request_xrandr!(self.connection, XROpcode::SetCrtcGamma, true, SetCrtcGammaRequest {
            crtc: self.handle,
            red: rgb.iter().map(|(r, _, _)| *r).collect(),
            green: rgb.iter().map(|(_, g, _)| *g).collect(),
            blue: rgb.iter().map(|(_, _, b)| *b).collect(),
        });

        Ok(())
    }

    pub async fn set_transform(self, transform: [[I16F16; 3]; 3], name: impl AsRef<str>, params: impl Iterator<Item=I16F16>) -> Result<()> {
        send_request_xrandr!(self.connection, XROpcode::SetCrtcTransform, true, SetCrtcTransformRequest {
            crtc: self.handle,
            transformation: Transform(transform.into_iter().flat_map(|x| x.into_iter().map(|x| -> Fp1616 { x.into() })).collect()),
            filter_name: name.as_ref().to_string(),
            filter_params: params.into_iter().map(Into::into).collect(),
        });

        Ok(())
    }

    pub async fn get_transform(self) -> Result<CrtcTransform> {
        let seq = send_request_xrandr!(self.connection, XROpcode::GetCrtcTransform, false, GetCrtcTransformRequest {
            crtc: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, GetCrtcTransformResponse);
        let mut pending_transform = [[I16F16::ZERO; 3]; 3];
        let mut transform_source = reply.pending_transform.0.into_iter().map(|x| -> I16F16 { Into::into(x) });
        for row in pending_transform.iter_mut() {
            for col in row.iter_mut() {
                *col = transform_source.next().ok_or_else(|| anyhow!("missing transform"))?;
            }
        }

        let mut current_transform = [[I16F16::ZERO; 3]; 3];
        let mut transform_source = reply.current_transform.0.into_iter().map(|x| -> I16F16 { Into::into(x) });
        for row in current_transform.iter_mut() {
            for col in row.iter_mut() {
                *col = transform_source.next().ok_or_else(|| anyhow!("missing transform"))?;
            }
        }

        Ok(CrtcTransform {
            pending_transform,
            has_transforms: reply.has_transforms,
            current_transform,
            pending_filter: reply.pending_filter_name,
            current_filter: reply.current_filter_name,
            pending_params: reply.pending_params.into_iter().map(Into::into).collect(),
            current_params: reply.current_params.into_iter().map(Into::into).collect(),
        })
    }

    pub async fn get_panning(self) -> Result<Panning> {
        let seq = send_request_xrandr!(self.connection, XROpcode::GetPanning, false, GetPanningRequest {
            crtc: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, GetPanningResponse);

        Ok(reply.value)
    }

    pub async fn set_panning(self, panning: Panning) -> Result<(SetConfig, Timestamp)> {
        let seq = send_request_xrandr!(self.connection, XROpcode::SetPanning, false, SetPanningRequest {
            crtc: self.handle,
            value: panning,
        });
        let (reply, status) = receive_reply!(self.connection, seq, SetPanningResponse, fetched);

        Ok((
            SetConfig::from_repr(status)?,
            Timestamp(reply.time),
        ))
    }
}
