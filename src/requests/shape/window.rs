use super::*;
pub use crate::coding::shape::{ClipSorting, QueryExtentsResponse, ShapeKind, ShapeOperation};
use crate::coding::shape::{
    CombineRequest, GetRectanglesRequest, GetRectanglesResponse, InputSelectedRequest, InputSelectedResponse, MaskRequest, OffsetRequest, QueryExtentsRequest,
    RectanglesRequest, SelectInputRequest,
};

impl<'a> Window<'a> {
    pub async fn set_shape_rectangles(
        self,
        kind: ShapeKind,
        operation: ShapeOperation,
        x_offset: i16,
        y_offset: i16,
        rectangles: impl IntoIterator<Item = Rectangle>,
    ) -> Result<()> {
        send_request_shape!(
            self.connection,
            ShapeOpcode::Rectangles,
            RectanglesRequest {
                dst_window: self.handle,
                dst_kind: kind,
                operation: operation,
                x_offset: x_offset,
                y_offset: y_offset,
                rectangles: rectangles.into_iter().map(Into::into).collect(),
            }
        );

        Ok(())
    }

    pub async fn set_shape_mask_pixmap(
        self,
        kind: ShapeKind,
        operation: ShapeOperation,
        x_offset: i16,
        y_offset: i16,
        pixmap: Option<Pixmap<'_>>,
    ) -> Result<()> {
        send_request_shape!(
            self.connection,
            ShapeOpcode::Mask,
            MaskRequest {
                dst_window: self.handle,
                dst_kind: kind,
                operation: operation,
                x_offset: x_offset,
                y_offset: y_offset,
                src_pixmap: pixmap.map(|x| x.handle).unwrap_or(0),
            }
        );

        Ok(())
    }

    pub async fn copy_shape_from(
        self,
        kind: ShapeKind,
        operation: ShapeOperation,
        x_offset: i16,
        y_offset: i16,
        source: Window<'_>,
        source_kind: ShapeKind,
    ) -> Result<()> {
        send_request_shape!(
            self.connection,
            ShapeOpcode::Combine,
            CombineRequest {
                dst_window: self.handle,
                dst_kind: kind,
                operation: operation,
                x_offset: x_offset,
                y_offset: y_offset,
                src_window: source.handle,
                src_kind: source_kind,
            }
        );

        Ok(())
    }

    pub async fn shape_offset(self, kind: ShapeKind, x_offset: i16, y_offset: i16) -> Result<()> {
        send_request_shape!(
            self.connection,
            ShapeOpcode::Offset,
            OffsetRequest {
                dst_window: self.handle,
                dst_kind: kind,
                x_offset: x_offset,
                y_offset: y_offset,
            }
        );

        Ok(())
    }

    pub async fn query_extents(self) -> Result<QueryExtentsResponse> {
        let reply = send_request_shape!(
            self.connection,
            ShapeOpcode::QueryExtents,
            QueryExtentsResponse,
            QueryExtentsRequest {
                dst_window: self.handle,
            }
        )
        .into_inner();

        Ok(reply)
    }

    pub async fn select_shape_input(self, enable: bool) -> Result<()> {
        send_request_shape!(
            self.connection,
            ShapeOpcode::SelectInput,
            SelectInputRequest {
                window: self.handle,
                enable: enable,
            }
        );

        Ok(())
    }

    pub async fn is_shape_input_selected(self) -> Result<bool> {
        let reply = send_request_shape!(
            self.connection,
            ShapeOpcode::InputSelected,
            InputSelectedResponse,
            InputSelectedRequest {
                window: self.handle,
            }
        );

        Ok(reply.reserved != 0)
    }

    pub async fn get_shape_rectangles(self, kind: ShapeKind) -> Result<(Vec<Rectangle>, ClipSorting)> {
        let reply = send_request_shape!(
            self.connection,
            ShapeOpcode::GetRectangles,
            GetRectanglesResponse,
            GetRectanglesRequest {
                window: self.handle,
                kind: kind,
            }
        )
        .into_inner();

        Ok((reply.rectangles.into_iter().map(Into::into).collect(), reply.ordering))
    }
}
