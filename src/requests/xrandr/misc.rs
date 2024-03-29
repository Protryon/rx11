use crate::{
    coding::xrandr::{QueryVersionRequest, QueryVersionResponse},
    net::{ExtInfo, Extension},
};

use super::*;

impl X11Connection {
    pub(crate) async fn enable_xrandr(&self) -> Result<()> {
        // query_extension
        let queried = self.query_extension(XRANDR_EXT_NAME).await?;
        ensure!(queried.present, "xrandr missing on x11 server");
        self.0.registered_extensions.insert(
            XRANDR_EXT_NAME.to_string(),
            ExtInfo {
                extension: Extension::XRandr,
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
            XROpcode::QueryVersion,
            QueryVersionResponse,
            QueryVersionRequest {
                major_version: 1,
                minor_version: 5,
            }
        );
        if reply.major_version != 1 {
            bail!("unsupported xrandr version on server: {}.{}", reply.major_version, reply.minor_version);
        }
        Ok(())
    }
}
