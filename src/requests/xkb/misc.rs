use crate::{net::ExtInfo, coding::xkb::{UseExtensionRequest, UseExtensionResponse}};

use super::*;

impl X11Connection {
    pub(crate) async fn enable_xkb(&self) -> Result<()> {
        // query_extension
        let queried = self.query_extension(XKB_EXT_NAME).await?;
        ensure!(queried.present, "xkb missing on x11 server");
        self.0.registered_extensions.insert(XKB_EXT_NAME.to_string(), ExtInfo {
            major_opcode: queried.major_opcode,
            event_start: queried.first_event,
            error_start: queried.first_error,
            event_count: XKB_EVENT_COUNT,
        });

        // enable extension
        let seq = send_request_ext!(self, queried.major_opcode, XKBOpcode::UseExtension, false, UseExtensionRequest {
            wanted_major: 1,
            wanted_minor: 0,
        });
        let (reply, supported) = receive_reply!(self, seq, UseExtensionResponse, fetched);
        if supported == 0 || reply.server_major != 1 {
            bail!("unsupported xkb version on server: {}.{}", reply.server_major, reply.server_minor);
        }
        Ok(())
    }
}