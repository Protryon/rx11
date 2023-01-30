// pub use crate::coding::xfixes::{
//     CursorNotifyMask,
//     GetCursorImageResponse as CursorImage,
// };

use crate::coding::xfixes::{
    CopyRegionRequest, CreateRegionFromBitmapRequest, CreateRegionFromGCRequest, CreateRegionFromWindowRequest, CreateRegionRequest, DestroyRegionRequest,
    ExpandRegionRequest, FetchRegionRequest, FetchRegionResponse, IntersectRegionRequest, InvertRegionRequest, RegionExtentsRequest, SetRegionRequest,
    SubtractRegionRequest, TranslateRegionRequest, UnionRegionRequest,
};

use super::*;

#[derive(Clone, Copy, derivative::Derivative)]
#[derivative(Debug)]
pub struct Region<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

#[derive(Clone, Debug)]
pub struct RegionRectangles {
    pub extents: Rectangle,
    pub rectangles: Vec<Rectangle>,
}

impl X11Connection {
    pub async fn create_region(&self, rectangles: impl IntoIterator<Item = Rectangle>) -> Result<Region<'_>> {
        let region = Region {
            handle: self.new_resource_id(),
            connection: self,
        };
        send_request_xfixes!(
            self,
            XFOpcode::CreateRegion,
            CreateRegionRequest {
                region: region.handle,
                rectangles: rectangles.into_iter().map(Into::into).collect(),
            }
        );

        Ok(region)
    }

    pub async fn create_region_from_bitmap(&self, pixmap: Pixmap<'_>) -> Result<Region<'_>> {
        let region = Region {
            handle: self.new_resource_id(),
            connection: self,
        };
        send_request_xfixes!(
            self,
            XFOpcode::CreateRegionFromBitmap,
            CreateRegionFromBitmapRequest {
                region: region.handle,
                pixmap: pixmap.handle,
            }
        );

        Ok(region)
    }

    pub async fn create_region_from_window(&self, window: Window<'_>) -> Result<Region<'_>> {
        let region = Region {
            handle: self.new_resource_id(),
            connection: self,
        };
        send_request_xfixes!(
            self,
            XFOpcode::CreateRegionFromWindow,
            CreateRegionFromWindowRequest {
                region: region.handle,
                window: window.handle,
            }
        );

        Ok(region)
    }

    pub async fn create_region_from_gcontext(&self, gcontext: GContext<'_>) -> Result<Region<'_>> {
        let region = Region {
            handle: self.new_resource_id(),
            connection: self,
        };
        send_request_xfixes!(
            self,
            XFOpcode::CreateRegionFromGC,
            CreateRegionFromGCRequest {
                region: region.handle,
                gcontext: gcontext.handle,
            }
        );

        Ok(region)
    }

    //TODO: depends on render
    // pub async fn create_region_from_picture(&self, picture: Picture<'_>) -> Result<Region<'_>> {
    //     let region = Region {
    //         handle: self.new_resource_id(),
    //         connection: self,
    //     };
    //     send_request_xfixes!(self, XFOpcode::CreateRegionFromPicture, CreateRegionFromPictureRequest {
    //         region: region.handle,
    //         picture: picture.handle,
    //     });

    //     Ok(region)
    // }
}

impl<'a> Region<'a> {
    pub async fn destroy(self) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::DestroyRegion,
            DestroyRegionRequest {
                region: self.handle,
            }
        );

        Ok(())
    }

    pub async fn set(self, rectangles: impl IntoIterator<Item = Rectangle>) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::SetRegion,
            SetRegionRequest {
                region: self.handle,
                rectangles: rectangles.into_iter().map(Into::into).collect(),
            }
        );

        Ok(())
    }

    pub async fn copy_to(self, target: Region<'_>) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::CopyRegion,
            CopyRegionRequest {
                src_region: self.handle,
                dst_region: target.handle,
            }
        );

        Ok(())
    }

    pub async fn union_from(self, src1: Region<'_>, src2: Region<'_>) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::UnionRegion,
            UnionRegionRequest {
                dst_region: self.handle,
                src_region1: src1.handle,
                src_region2: src2.handle,
            }
        );

        Ok(())
    }

    pub async fn union(self, other: Region<'_>) -> Result<Region<'a>> {
        let new_region = self.connection.create_region(vec![]).await?;
        new_region.union_from(self, other).await?;
        Ok(new_region)
    }

    pub async fn intersect_from(self, src1: Region<'_>, src2: Region<'_>) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::IntersectRegion,
            IntersectRegionRequest {
                dst_region: self.handle,
                src_region1: src1.handle,
                src_region2: src2.handle,
            }
        );

        Ok(())
    }

    pub async fn intersect(self, other: Region<'_>) -> Result<Region<'a>> {
        let new_region = self.connection.create_region(vec![]).await?;
        new_region.intersect_from(self, other).await?;
        Ok(new_region)
    }

    pub async fn subtract_from(self, src1: Region<'_>, src2: Region<'_>) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::SubtractRegion,
            SubtractRegionRequest {
                dst_region: self.handle,
                src_region1: src1.handle,
                src_region2: src2.handle,
            }
        );

        Ok(())
    }

    pub async fn subtract_region(self, other: Region<'_>) -> Result<Region<'a>> {
        let new_region = self.connection.create_region(vec![]).await?;
        new_region.subtract_from(self, other).await?;
        Ok(new_region)
    }

    pub async fn invert(self, target: Region<'_>, bounds: Rectangle) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::InvertRegion,
            InvertRegionRequest {
                src_region: self.handle,
                bounds: bounds.into(),
                dst_region: target.handle,
            }
        );

        Ok(())
    }

    pub async fn translate(self, dx: i16, dy: i16) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::TranslateRegion,
            TranslateRegionRequest {
                region: self.handle,
                dx: dx,
                dy: dy,
            }
        );

        Ok(())
    }

    pub async fn get_extents(self, target: Region<'_>) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::RegionExtents,
            RegionExtentsRequest {
                src_region: self.handle,
                dst_region: target.handle,
            }
        );

        Ok(())
    }

    pub async fn fetch(self) -> Result<RegionRectangles> {
        let reply = send_request_xfixes!(
            self.connection,
            XFOpcode::FetchRegion,
            FetchRegionResponse,
            FetchRegionRequest {
                region: self.handle,
            }
        )
        .into_inner();

        Ok(RegionRectangles {
            extents: reply.extents.into(),
            rectangles: reply.rectangles.into_iter().map(|x| x.into()).collect(),
        })
    }

    pub async fn expand(self, target: Region<'_>, left: u16, right: u16, top: u16, bottom: u16) -> Result<()> {
        send_request_xfixes!(
            self.connection,
            XFOpcode::ExpandRegion,
            ExpandRegionRequest {
                src_region: self.handle,
                dst_region: target.handle,
                left: left,
                right: right,
                top: top,
                bottom: bottom,
            }
        );

        Ok(())
    }
}
