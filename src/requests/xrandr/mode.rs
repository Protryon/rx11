use crate::coding::xrandr::{CreateModeRequest, CreateModeResponse, DestroyModeRequest};

use super::*;

#[derive(Clone, Copy, derivative::Derivative)]
#[derivative(Debug)]
pub struct Mode<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

impl<'a> Resource<'a> for Mode<'a> {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(connection: &'a X11Connection, handle: u32) -> Self {
        Self {
            connection,
            handle,
        }
    }
}

impl<'a> Window<'a> {
    pub async fn create_mode(self, mode_info: ModeInfo, name: impl AsRef<str>) -> Result<Mode<'a>> {
        let reply = send_request_xrandr!(
            self.connection,
            XROpcode::CreateMode,
            CreateModeResponse,
            CreateModeRequest {
                window: self.handle,
                mode_info: mode_info,
                name: name.as_ref().to_string(),
            }
        )
        .into_inner();

        Ok(Mode {
            connection: self.connection,
            handle: reply.mode,
        })
    }
}

impl<'a> Mode<'a> {
    pub async fn destroy(self) -> Result<()> {
        send_request_xrandr!(
            self.connection,
            XROpcode::DestroyMode,
            DestroyModeRequest {
                mode: self.handle,
            }
        );
        Ok(())
    }
}
