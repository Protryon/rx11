
use crate::{net::ExtInfo, coding::xinput2::{XIQueryVersionRequest, XIQueryVersionResponse}};

use super::*;

impl X11Connection {
    pub(crate) async fn enable_xinput2(&self) -> Result<()> {
        // query_extension
        let queried = self.query_extension(XINPUT_EXT_NAME).await?;
        ensure!(queried.present, "xinput2 missing on x11 server");
        self.0.registered_extensions.insert(XINPUT_EXT_NAME.to_string(), ExtInfo {
            major_opcode: queried.major_opcode,
            event_start: queried.first_event,
            error_start: queried.first_error,
        });

        // enable extension
        let seq = send_request_ext!(self, queried.major_opcode, XIOpcode::XIQueryVersion, false, XIQueryVersionRequest {
            major_version: 2,
            minor_version: 3,
        });
        let reply = receive_reply!(self, seq, XIQueryVersionResponse);
        if reply.major_version != 2 {
            bail!("unsupported xinput version on server: {}.{}", reply.major_version, reply.minor_version);
        }
        Ok(())
    }
}