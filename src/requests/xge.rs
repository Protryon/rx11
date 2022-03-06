
use crate::{net::ExtInfo, coding::{XgeQueryVersionRequest, XgeQueryVersionResponse, RequestBody}};

use super::*;

pub const XGE_EXT_NAME: &str = "Generic Event Extension";

impl X11Connection {
    pub(crate) async fn enable_xge(&self) -> Result<()> {
        // query_extension
        let queried = self.query_extension(XGE_EXT_NAME).await?;
        ensure!(queried.present, "xge missing on x11 server");
        self.0.registered_extensions.insert(XGE_EXT_NAME.to_string(), ExtInfo {
            major_opcode: queried.major_opcode,
            event_start: queried.first_event,
            error_start: queried.first_error,
            event_count: 0,
        });

        // enable extension
        let seq = send_request_ext!(self, queried.major_opcode, 0, false, XgeQueryVersionRequest {
            client_major_version: 1,
            client_minor_version: 0,
        });
        let reply = receive_reply!(self, seq, XgeQueryVersionResponse);
        if reply.major_version != 1 {
            bail!("unsupported xge version on server: {}.{}", reply.major_version, reply.minor_version);
        }
        Ok(())
    }
}