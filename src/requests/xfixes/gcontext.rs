use crate::coding::xfixes::SetGCClipRegionRequest;

use super::*;

impl<'a> GContext<'a> {
    pub async fn set_clip_region(self, region: Region<'_>, x_origin: i16, y_origin: i16) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::SetGCClipRegion,
            SetGCClipRegionRequest {
                gcontext: self.handle,
                region: region.handle,
                x_origin: x_origin,
                y_origin: y_origin,
            }
        );

        Ok(())
    }
}
