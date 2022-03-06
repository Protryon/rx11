use crate::coding::xfixes::{SaveSetMode, ChangeSaveSetRequest, SetWindowShapeRegionRequest};

pub use crate::coding::xfixes::{
    SaveSetTarget,
    SaveSetMapping,
};

use super::*;

impl<'a> Window<'a> {
    pub async fn save_set_add(&self, target: SaveSetTarget, map: SaveSetMapping) -> Result<()> {
        send_request_xfixes!(self.connection, XFOpcode::ChangeSaveSet, true, ChangeSaveSetRequest {
            mode: SaveSetMode::Insert,
            target: target,
            map: map,
            window: self.handle,
        });

        Ok(())
    }

    pub async fn save_set_delete(&self) -> Result<()> {
        send_request_xfixes!(self.connection, XFOpcode::ChangeSaveSet, true, ChangeSaveSetRequest {
            mode: SaveSetMode::Delete,
            window: self.handle,
        });
        Ok(())
    }

    //TODO: integrate `dst_shape_kind` with `shape`
    pub async fn set_shape_region(&self, dst_shape_kind: u8, x_offset: i16, y_offset: i16, region: Region<'_>) -> Result<()> {
        send_request_xfixes!(self.connection, XFOpcode::SetWindowShapeRegion, true, SetWindowShapeRegionRequest {
            dst_window: self.handle,
            dst_shape_kind: dst_shape_kind,
            x_offset: x_offset,
            y_offset: y_offset,
            region: region.handle,
        });

        Ok(())
    }
}