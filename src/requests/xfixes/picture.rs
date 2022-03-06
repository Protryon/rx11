// use crate::coding::xfixes::SetPictureClipRegionRequest;

// use super::*;

// TODO: pending `render` integration
// impl<'a> Picture<'a> {
//     pub async fn set_clip_region(&self, region: Region<'_>, x_origin: i16, y_origin: i16) -> Result<()> {
//         send_request_xfixes!(self.connection, XFOpcode::SetPictureClipRegion, true, SetPictureClipRegionRequest {
//             picture: self.handle,
//             region: region.handle,
//             x_origin: x_origin,
//             y_origin: y_origin,
//         });

//         Ok(())
//     }
// }
