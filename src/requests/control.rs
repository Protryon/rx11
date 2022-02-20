use super::*;

pub use crate::coding::CloseDownMode;

impl X11Connection {
    pub async fn set_close_down_mode(&self, mode: CloseDownMode) -> Result<()> {
        send_request!(self, mode as u8, SetCloseDownMode {
        });

        Ok(())
    }

    pub async fn kill_client(&self, resource: impl Resource) -> Result<()> {
        send_request!(self, KillClient {
            resource: resource.x11_handle(),
        });

        Ok(())
    }

    pub async fn kill_all_temporary_clients(&self) -> Result<()> {
        send_request!(self, KillClient {
            resource: 0,
        });

        Ok(())
    }

    pub async fn nop(&self) -> Result<()> {
        send_request!(self, NoOperation {
        });

        Ok(())
    }
}