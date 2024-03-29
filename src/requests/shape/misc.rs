use crate::{
    coding::shape::{QueryVersionRequest, QueryVersionResponse},
    net::{ExtInfo, Extension},
};

use super::*;

impl X11Connection {
    pub(crate) async fn enable_shape(&self) -> Result<()> {
        // query_extension
        let queried = self.query_extension(SHAPE_EXT_NAME).await?;
        ensure!(queried.present, "shape missing on x11 server");
        self.0.registered_extensions.insert(
            SHAPE_EXT_NAME.to_string(),
            ExtInfo {
                extension: Extension::Shape,
                major_opcode: queried.major_opcode,
                event_start: queried.first_event,
                error_start: queried.first_error,
                event_count: SHAPE_EVENT_COUNT,
            },
        );

        // enable extension
        let reply = send_request_ext!(self, queried.major_opcode, ShapeOpcode::QueryVersion, QueryVersionResponse, QueryVersionRequest {});
        if reply.major_version != 1 {
            bail!("unsupported shape version on server: {}.{}", reply.major_version, reply.minor_version);
        }
        Ok(())
    }
}
