use crate::{
    coding::xinput2::{XIQueryVersionRequest, XIQueryVersionResponse},
    net::{ExtInfo, Extension},
};

use super::*;

impl X11Connection {
    pub(crate) async fn enable_xinput2(&self) -> Result<()> {
        // query_extension
        let queried = self.query_extension(XINPUT_EXT_NAME).await?;
        ensure!(queried.present, "xinput2 missing on x11 server");
        self.0.registered_extensions.insert(
            XINPUT_EXT_NAME.to_string(),
            ExtInfo {
                extension: Extension::XInput,
                major_opcode: queried.major_opcode,
                event_start: queried.first_event,
                error_start: queried.first_error,
                event_count: 0,
            },
        );

        // enable extension
        let reply = send_request_ext!(
            self,
            queried.major_opcode,
            XIOpcode::XIQueryVersion,
            XIQueryVersionResponse,
            XIQueryVersionRequest {
                major_version: 2,
                minor_version: 3,
            }
        );
        if reply.major_version != 2 {
            bail!("unsupported xinput version on server: {}.{}", reply.major_version, reply.minor_version);
        }
        Ok(())
    }
}
