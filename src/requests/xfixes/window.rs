use crate::coding::xfixes::{ChangeSaveSetRequest, SaveSetMode, SetWindowShapeRegionRequest};

pub use crate::coding::xfixes::{SaveSetMapping, SaveSetTarget};

use super::*;

impl<'a> Window<'a> {
    pub async fn save_set_add(self, target: SaveSetTarget, map: SaveSetMapping) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::ChangeSaveSet,
            ChangeSaveSetRequest {
                mode: SaveSetMode::Insert,
                target: target,
                map: map,
                window: self.handle,
            }
        );

        Ok(())
    }

    pub async fn save_set_delete(self) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::ChangeSaveSet,
            ChangeSaveSetRequest {
                mode: SaveSetMode::Delete,
                window: self.handle,
            }
        );
        Ok(())
    }

    pub async fn set_shape_region(self, dst_shape_kind: ShapeKind, x_offset: i16, y_offset: i16, region: Region<'_>) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::SetWindowShapeRegion,
            SetWindowShapeRegionRequest {
                dst_window: self.handle,
                dst_shape_kind: dst_shape_kind as u8,
                x_offset: x_offset,
                y_offset: y_offset,
                region: region.handle,
            }
        );

        Ok(())
    }
}
