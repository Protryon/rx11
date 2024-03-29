use crate::{
    coding::xfixes::{QueryVersionRequest, QueryVersionResponse},
    net::{ExtInfo, Extension},
};

use super::*;

impl X11Connection {
    pub(crate) async fn enable_xfixes(&self) -> Result<()> {
        // query_extension
        let queried = self.query_extension(XFIXES_EXT_NAME).await?;
        ensure!(queried.present, "xfixes missing on x11 server");
        self.0.registered_extensions.insert(
            XFIXES_EXT_NAME.to_string(),
            ExtInfo {
                extension: Extension::XFixes,
                major_opcode: queried.major_opcode,
                event_start: queried.first_event,
                error_start: queried.first_error,
                event_count: XF_EVENT_COUNT,
            },
        );

        // enable extension
        let reply = send_request_ext!(
            self,
            queried.major_opcode,
            XFOpcode::QueryVersion,
            QueryVersionResponse,
            QueryVersionRequest {
                client_major_version: 5,
                client_minor_version: 0,
            }
        );
        if reply.major_version != 5 {
            bail!("unsupported xinput version on server: {}.{}", reply.major_version, reply.minor_version);
        }
        Ok(())
    }
}
