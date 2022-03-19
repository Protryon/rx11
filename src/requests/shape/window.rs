use super::*;
use crate::coding::shape::{RectanglesRequest, MaskRequest, CombineRequest, OffsetRequest, QueryExtentsRequest, SelectInputRequest, InputSelectedRequest, InputSelectedResponse, GetRectanglesRequest, GetRectanglesResponse};
pub use crate::coding::shape::{
    ShapeKind,
    ShapeOperation,
    QueryExtentsResponse,
    ClipSorting,
};

impl<'a> Window<'a> {
    pub async fn set_shape_rectangles(self, kind: ShapeKind, operation: ShapeOperation, x_offset: i16, y_offset: i16, rectangles: impl IntoIterator<Item=Rectangle>) -> Result<()> {
        send_request_shape!(self.connection, ShapeOpcode::Rectangles, true, RectanglesRequest {
            dst_window: self.handle,
            dst_kind: kind,
            operation: operation,
            x_offset: x_offset,
            y_offset: y_offset,
            rectangles: rectangles.into_iter().map(Into::into).collect(),
        });

        Ok(())
    }

    pub async fn set_shape_mask_pixmap(self, kind: ShapeKind, operation: ShapeOperation, x_offset: i16, y_offset: i16, pixmap: Option<Pixmap<'_>>) -> Result<()> {
        send_request_shape!(self.connection, ShapeOpcode::Mask, true, MaskRequest {
            dst_window: self.handle,
            dst_kind: kind,
            operation: operation,
            x_offset: x_offset,
            y_offset: y_offset,
            src_pixmap: pixmap.map(|x| x.handle).unwrap_or(0),
        });

        Ok(())
    }

    pub async fn copy_shape_from(self, kind: ShapeKind, operation: ShapeOperation, x_offset: i16, y_offset: i16, source: Window<'_>, source_kind: ShapeKind) -> Result<()> {
        send_request_shape!(self.connection, ShapeOpcode::Combine, true, CombineRequest {
            dst_window: self.handle,
            dst_kind: kind,
            operation: operation,
            x_offset: x_offset,
            y_offset: y_offset,
            src_window: source.handle,
            src_kind: source_kind,
        });

        Ok(())
    }

    pub async fn shape_offset(self, kind: ShapeKind, x_offset: i16, y_offset: i16) -> Result<()> {
        send_request_shape!(self.connection, ShapeOpcode::Offset, true, OffsetRequest {
            dst_window: self.handle,
            dst_kind: kind,
            x_offset: x_offset,
            y_offset: y_offset,
        });

        Ok(())
    }

    pub async fn query_extents(self) -> Result<QueryExtentsResponse> {
        let seq = send_request_shape!(self.connection, ShapeOpcode::QueryExtents, false, QueryExtentsRequest {
            dst_window: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, QueryExtentsResponse);

        Ok(reply)
    }

    pub async fn select_shape_input(self, enable: bool) -> Result<()> {
        send_request_shape!(self.connection, ShapeOpcode::SelectInput, true, SelectInputRequest {
            window: self.handle,
            enable: enable,
        });

        Ok(())
    }

    pub async fn is_shape_input_selected(self) -> Result<bool> {
        let seq = send_request_shape!(self.connection, ShapeOpcode::InputSelected, false, InputSelectedRequest {
            window: self.handle,
        });
        let (_reply, enabled) = receive_reply!(self.connection, seq, InputSelectedResponse, fetched);

        Ok(enabled != 0)
    }

    pub async fn get_shape_rectangles(self, kind: ShapeKind) -> Result<(Vec<Rectangle>, ClipSorting)> {
        let seq = send_request_shape!(self.connection, ShapeOpcode::GetRectangles, false, GetRectanglesRequest {
            window: self.handle,
            kind: kind,
        });
        let reply = receive_reply!(self.connection, seq, GetRectanglesResponse);

        Ok((
            reply.rectangles.into_iter().map(Into::into).collect(),
            reply.ordering,
        ))
    }
}