use crate::coding::xrandr::{
    GetOutputPrimaryRequest, GetOutputPrimaryResponse, GetScreenInfoRequest, GetScreenInfoResponse, GetScreenResourcesRequest, GetScreenResourcesResponse,
    GetScreenSizeRangeRequest, SelectInputRequest, SetOutputPrimaryRequest, SetScreenConfigRequest, SetScreenConfigResponse, SetScreenSizeRequest,
};
pub use crate::coding::xrandr::{GetScreenSizeRangeResponse, ModeFlag, ModeInfo, RefreshRates, Rotation, ScreenSize, SetConfig, SubPixel, XREventMask};

use super::*;

#[derive(Clone, Debug)]
pub struct SetScreenConfig<'a> {
    pub status: SetConfig,
    pub new_time: Timestamp,
    pub config_time: Timestamp,
    pub root: Window<'a>,
    pub subpixel_order: SubPixel,
}

#[derive(Clone, Debug)]
pub struct ScreenInfo<'a> {
    pub rotations: Rotation,
    pub root: Window<'a>,
    pub time: Timestamp,
    pub config_time: Timestamp,
    pub size_id: u16,
    pub rotation: Rotation,
    pub rate: u16,
    pub screen_sizes: Vec<ScreenSize>,
    pub rates: Vec<RefreshRates>,
}

#[derive(Clone, Debug)]
pub struct ScreenResources<'a> {
    pub time: Timestamp,
    pub config_time: Timestamp,
    pub crtcs: Vec<Crtc<'a>>,
    pub outputs: Vec<Output<'a>>,
    /// (name, mode info)
    pub modes: Vec<(String, ModeInfo)>,
}

impl<'a> Window<'a> {
    pub async fn xrandr_select_input(self, mask: XREventMask) -> Result<()> {
        send_request_xrandr!(
            self.connection,
            XROpcode::SelectInput,
            SelectInputRequest {
                window: self.handle,
                event_mask: mask,
            }
        );

        Ok(())
    }

    pub async fn set_screen_config(self, time: Timestamp, config_time: Timestamp, size_id: u16, rotation: Rotation, rate: u16) -> Result<SetScreenConfig<'a>> {
        let reply = send_request_xrandr!(
            self.connection,
            XROpcode::SetScreenConfig,
            SetScreenConfigResponse,
            SetScreenConfigRequest {
                window: self.handle,
                time: time.0,
                config_time: config_time.0,
                size_id: size_id,
                rotation: rotation,
                rate: rate,
            }
        );
        let status = reply.reserved;
        let reply = reply.into_inner();

        Ok(SetScreenConfig {
            status: SetConfig::from_repr(status)?,
            new_time: Timestamp(reply.new_time),
            config_time: Timestamp(reply.config_time),
            root: Window {
                handle: reply.root_window,
                connection: self.connection,
            },
            subpixel_order: reply.subpixel_order,
        })
    }

    pub async fn get_screen_info(self) -> Result<ScreenInfo<'a>> {
        let reply = send_request_xrandr!(
            self.connection,
            XROpcode::GetScreenInfo,
            GetScreenInfoResponse,
            GetScreenInfoRequest {
                window: self.handle,
            }
        );
        let rotations = reply.reserved;
        let reply = reply.into_inner();

        Ok(ScreenInfo {
            rotations: Rotation(rotations),
            root: Window {
                handle: reply.root_window,
                connection: self.connection,
            },
            time: Timestamp(reply.time),
            config_time: Timestamp(reply.config_time),
            size_id: reply.size_id,
            rotation: reply.rotation,
            rate: reply.rate,
            screen_sizes: reply.sizes,
            rates: reply.rates,
        })
    }

    pub async fn get_screen_size_range(self) -> Result<GetScreenSizeRangeResponse> {
        let reply = send_request_xrandr!(
            self.connection,
            XROpcode::GetScreenSizeRange,
            GetScreenSizeRangeResponse,
            GetScreenSizeRangeRequest {
                window: self.handle,
            }
        )
        .into_inner();

        Ok(reply)
    }

    pub async fn set_screen_size(self) -> Result<()> {
        send_request_xrandr!(
            self.connection,
            XROpcode::SetScreenSize,
            SetScreenSizeRequest {
                window: self.handle,
            }
        );

        Ok(())
    }

    async fn interior_get_screen_resources(self, opcode: XROpcode) -> Result<ScreenResources<'a>> {
        let reply = send_request_xrandr!(
            self.connection,
            opcode,
            GetScreenResourcesResponse,
            GetScreenResourcesRequest {
                window: self.handle,
            }
        )
        .into_inner();

        let mut modes = vec![];
        let mut names_index = 0;
        for mode in reply.modes {
            let end = names_index + mode.name_len as usize;
            if end > reply.names.len() {
                break;
            }
            modes.push((reply.names[names_index..end].to_string(), mode));
            names_index = end;
        }

        Ok(ScreenResources {
            time: Timestamp(reply.time),
            config_time: Timestamp(reply.config_time),
            crtcs: reply
                .crtcs
                .into_iter()
                .map(|handle| Crtc {
                    connection: self.connection,
                    handle,
                })
                .collect(),
            outputs: reply
                .outputs
                .into_iter()
                .map(|handle| Output {
                    connection: self.connection,
                    handle,
                })
                .collect(),
            modes,
        })
    }

    pub async fn get_screen_resources(self) -> Result<ScreenResources<'a>> {
        self.interior_get_screen_resources(XROpcode::GetScreenResources).await
    }

    pub async fn get_screen_resources_current(self) -> Result<ScreenResources<'a>> {
        self.interior_get_screen_resources(XROpcode::GetScreenResourcesCurrent).await
    }

    pub async fn set_output_primary(self, output: Output<'_>) -> Result<()> {
        send_request_xrandr!(
            self.connection,
            XROpcode::SetOutputPrimary,
            SetOutputPrimaryRequest {
                window: self.handle,
                output: output.handle,
            }
        );

        Ok(())
    }

    pub async fn get_output_primary(self) -> Result<Output<'a>> {
        let reply = send_request_xrandr!(
            self.connection,
            XROpcode::GetOutputPrimary,
            GetOutputPrimaryResponse,
            GetOutputPrimaryRequest {
                window: self.handle,
            }
        )
        .into_inner();

        Ok(Output {
            connection: self.connection,
            handle: reply.output,
        })
    }
}
