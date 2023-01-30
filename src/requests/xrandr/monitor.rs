use crate::coding::{
    self,
    xrandr::{DeleteMonitorRequest, GetMonitorsRequest, GetMonitorsResponse, SetMonitorRequest},
};

use super::*;

#[derive(Clone, Debug)]
pub struct MonitorInfo<'a> {
    pub name: Atom,
    pub primary: bool,
    pub automatic: bool,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub width_mm: u32,
    pub height_mm: u32,
    pub outputs: Vec<Output<'a>>,
}

impl<'a> MonitorInfo<'a> {
    async fn from_proto(connection: &'a X11Connection, info: coding::xrandr::MonitorInfo) -> Result<MonitorInfo<'a>> {
        Ok(Self {
            name: connection.get_atom_name(info.name_atom).await?,
            primary: info.primary,
            automatic: info.automatic,
            x: info.x,
            y: info.y,
            width: info.width,
            height: info.height,
            width_mm: info.width_mm,
            height_mm: info.height_mm,
            outputs: info
                .outputs
                .into_iter()
                .map(|handle| Output {
                    handle,
                    connection,
                })
                .collect(),
        })
    }

    fn into_proto(self) -> coding::xrandr::MonitorInfo {
        coding::xrandr::MonitorInfo {
            name_atom: self.name.handle,
            primary: self.primary,
            automatic: self.automatic,
            num_outputs: 0,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            width_mm: self.width_mm,
            height_mm: self.height_mm,
            outputs: self.outputs.into_iter().map(|x| x.handle).collect(),
        }
    }
}

impl<'a> Window<'a> {
    async fn get_monitors_internal(self, get_active: bool) -> Result<(Timestamp, Vec<MonitorInfo<'a>>)> {
        let reply = send_request_xrandr!(
            self.connection,
            XROpcode::GetMonitors,
            GetMonitorsResponse,
            GetMonitorsRequest {
                window: self.handle,
                get_active: get_active,
            }
        )
        .into_inner();

        let mut out = vec![];
        for info in reply.monitors {
            out.push(MonitorInfo::from_proto(self.connection, info).await?);
        }

        Ok((Timestamp(reply.time), out))
    }

    pub async fn get_monitors(self) -> Result<(Timestamp, Vec<MonitorInfo<'a>>)> {
        self.get_monitors_internal(false).await
    }

    pub async fn get_monitors_active(self) -> Result<(Timestamp, Vec<MonitorInfo<'a>>)> {
        self.get_monitors_internal(true).await
    }

    pub async fn set_monitor(self, info: MonitorInfo<'_>) -> Result<()> {
        send_request_xrandr!(
            self.connection,
            XROpcode::SetMonitor,
            SetMonitorRequest {
                window: self.handle,
                info: info.into_proto(),
            }
        );

        Ok(())
    }

    pub async fn delete_monitor(self, name: Atom) -> Result<()> {
        send_request_xrandr!(
            self.connection,
            XROpcode::DeleteMonitor,
            DeleteMonitorRequest {
                window: self.handle,
                name_atom: name.handle,
            }
        );

        Ok(())
    }
}
