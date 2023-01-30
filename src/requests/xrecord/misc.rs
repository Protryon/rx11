use crate::{
    coding::xrecord::{QueryVersionRequest, QueryVersionResponse},
    net::{ExtInfo, Extension},
};

use super::*;

impl X11Connection {
    pub(crate) async fn enable_xrecord(&self) -> Result<()> {
        // query_extension
        let queried = self.query_extension(XRECORD_EXT_NAME).await?;
        ensure!(queried.present, "xrecord missing on x11 server");
        self.0.registered_extensions.insert(
            XRECORD_EXT_NAME.to_string(),
            ExtInfo {
                extension: Extension::XRecord,
                major_opcode: queried.major_opcode,
                event_start: queried.first_event,
                error_start: queried.first_error,
                event_count: XR_EVENT_COUNT,
            },
        );

        // enable extension
        let reply = send_request_ext!(
            self,
            queried.major_opcode,
            XRecordOpcode::QueryVersion,
            QueryVersionResponse,
            QueryVersionRequest {
                major_version: 1,
                minor_version: 13,
            }
        );
        if reply.major_version != 1 {
            bail!("unsupported xrecord version on server: {}.{}", reply.major_version, reply.minor_version);
        }
        Ok(())
    }
}
