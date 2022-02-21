use derive_builder::Builder;

use super::*;

pub use crate::coding::{
    GCFunction,
    GCBitmask,
    LineStyle,
    CapStyle,
    JoinStyle,
    FillStyle,
    ArcMode,
    SubwindowMode,
    Rectangle,
    ClipSorting,
    CoordinateMode,
    Point,
    Segment,
    Arc,
    Shape,
    ImageFormat,
};

#[derive(Clone, Copy)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct GContext<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

#[derive(Default, Builder, Debug)]
#[builder(default)]
pub struct GContextParams<'a> {
    #[builder(default = "GCFunction::Copy")]
    pub function: GCFunction,
    #[builder(default = "u32::MAX")]
    pub plane_mask: u32,
    #[builder(default = "0")]
    pub foreground: u32,
    #[builder(default = "1")]
    pub background: u32,
    #[builder(default = "0")]
    pub line_width: u16,
    #[builder(default = "LineStyle::Solid")]
    pub line_style: LineStyle,
    #[builder(default = "CapStyle::Butt")]
    pub cap_style: CapStyle,
    #[builder(default = "JoinStyle::Miter")]
    pub join_style: JoinStyle,
    #[builder(default = "FillStyle::Solid")]
    pub fill_style: FillStyle,
    #[builder(default = "ArcMode::PieSlice")]
    pub arc_mode: ArcMode,
    #[builder(setter(into, strip_option), default)]
    pub tile: Option<Pixmap<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub stipple: Option<Pixmap<'a>>,
    #[builder(default = "0")]
    pub tile_stipple_x_origin: i16,
    #[builder(default = "0")]
    pub tile_stipple_y_origin: i16,
    #[builder(setter(into, strip_option), default)]
    pub font: Option<Font<'a>>,
    #[builder(default = "SubwindowMode::ClipByChildren")]
    pub subwindow_mode: SubwindowMode,
    #[builder(default = "0")]
    pub clip_x_origin: i16,
    #[builder(default = "0")]
    pub clip_y_origin: i16,
    #[builder(setter(into, strip_option), default)]
    pub clip_mask: Option<Pixmap<'a>>,
    #[builder(default = "0")]
    pub dash_offset: u16,
    #[builder(default = "4")]
    pub dashes: u8,
}

impl<'a> Into<GCAttributes> for GContextParams<'a> {
    fn into(self) -> GCAttributes {
        let mut attributes = GCAttributes::default();
        if self.function != GCFunction::Copy {
            attributes.bitmask |= GCBitmask::FUNCTION;
            attributes.func = Some(self.function);
        }
        if self.plane_mask != u32::MAX {
            attributes.bitmask |= GCBitmask::PLANE_MASK;
            attributes.plane_mask = Some(self.plane_mask);
        }
        if self.foreground != 0 {
            attributes.bitmask |= GCBitmask::FOREGROUND;
            attributes.foreground = Some(self.foreground);
        }
        if self.background != 1 {
            attributes.bitmask |= GCBitmask::BACKGROUND;
            attributes.background = Some(self.background);
        }
        if self.line_width != 0 {
            attributes.bitmask |= GCBitmask::LINE_WIDTH;
            attributes.line_width = Some(self.line_width);
        }
        if self.line_style != LineStyle::Solid {
            attributes.bitmask |= GCBitmask::LINE_STYLE;
            attributes.line_style = Some(self.line_style);
        }
        if self.cap_style != CapStyle::Butt {
            attributes.bitmask |= GCBitmask::CAP_STYLE;
            attributes.cap_style = Some(self.cap_style);
        }
        if self.join_style != JoinStyle::Miter {
            attributes.bitmask |= GCBitmask::JOIN_STYLE;
            attributes.join_style = Some(self.join_style);
        }
        if self.fill_style != FillStyle::Solid {
            attributes.bitmask |= GCBitmask::FILL_STYLE;
            attributes.fill_style = Some(self.fill_style);
        }
        if self.arc_mode != ArcMode::PieSlice {
            attributes.bitmask |= GCBitmask::ARC_MODE;
            attributes.arc_mode = Some(self.arc_mode);
        }
        if let Some(tile) = self.tile {
            attributes.bitmask |= GCBitmask::TILE;
            attributes.tile = Some(tile.handle);
        }
        if let Some(stipple) = self.stipple {
            attributes.bitmask |= GCBitmask::STIPPLE;
            attributes.stipple = Some(stipple.handle);
        }
        if self.tile_stipple_x_origin != 0 {
            attributes.bitmask |= GCBitmask::TILE_STIPPLE_X_ORIGIN;
            attributes.tile_stipple_x_origin = Some(self.tile_stipple_x_origin);
        }
        if self.tile_stipple_y_origin != 0 {
            attributes.bitmask |= GCBitmask::TILE_STIPPLE_Y_ORIGIN;
            attributes.tile_stipple_y_origin = Some(self.tile_stipple_y_origin);
        }
        if let Some(font) = self.font {
            attributes.bitmask |= GCBitmask::FONT;
            attributes.font = Some(font.handle);
        }
        if self.subwindow_mode != SubwindowMode::ClipByChildren {
            attributes.bitmask |= GCBitmask::SUBWINDOW_MODE;
            attributes.subwindow_mode = Some(self.subwindow_mode);
        }
        if self.clip_x_origin != 0 {
            attributes.bitmask |= GCBitmask::CLIP_X_ORIGIN;
            attributes.clip_x_origin = Some(self.clip_x_origin);
        }
        if self.clip_y_origin != 0 {
            attributes.bitmask |= GCBitmask::CLIP_Y_ORIGIN;
            attributes.clip_y_origin = Some(self.clip_y_origin);
        }
        if let Some(clip_mask) = self.clip_mask {
            attributes.bitmask |= GCBitmask::CLIP_MASK;
            attributes.clip_mask= Some(clip_mask.handle);
        }
        if self.dash_offset != 0 {
            attributes.bitmask |= GCBitmask::DASH_OFFSET;
            attributes.dash_offset = Some(self.dash_offset);
        }
        if self.dashes != 4 {
            attributes.bitmask |= GCBitmask::DASHES;
            attributes.dashes = Some(self.dashes);
        }
        attributes
    }
}

#[derive(Debug, Clone)]
pub struct FetchedImage {
    pub depth: u8,
    pub visual: Option<Visual>,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub enum TextItem8<'a> {
    String(i8, String),
    Font(Font<'a>),
}

#[derive(Debug, Clone)]
pub enum TextItem16<'a> {
    String(i8, String),
    Font(Font<'a>),
}

impl X11Connection {
    pub async fn create_gcontext(&self, drawable: impl Into<Drawable<'_>>, params: GContextParams<'_>) -> Result<GContext<'_>> {
        let gcontext = self.new_resource_id();
        
        send_request!(self, CreateGC {
            gcontext: gcontext,
            drawable: drawable.into().handle(),
            attributes: params.into(),
        });
        Ok(GContext {
            handle: gcontext,
            connection: self,
        })
    }
}

impl<'a> Window<'a> {
    pub async fn clear_area(&self, exposures: bool, x: i16, y: i16, width: u16, height: u16) -> Result<()> {
        send_request!(self.connection, exposures as u8, ClearArea {
            window: self.handle,
            x: x,
            y: y,
            width: width,
            height: height,
        });
        Ok(())
    }
}

impl<'a> GContext<'a> {

    pub async fn change_attributes(&self, params: GContextParams<'_>) -> Result<()> {
        send_request!(self.connection, ChangeGC {
            gcontext: self.handle,
            attributes: params.into(),
        });
        Ok(())
    }

    pub async fn copy_to(&self, dst_gcontext: GContext<'_>, bitmask: GCBitmask) -> Result<()> {
        send_request!(self.connection, CopyGC {
            src_gcontext: self.handle,
            dst_gcontext: dst_gcontext.handle,
            bitmask: bitmask,
        });
        Ok(())
    }

    pub async fn set_dashes(&self, dash_offset: u16, dashes: Vec<u8>) -> Result<()> {
        send_request!(self.connection, SetDashes {
            gcontext: self.handle,
            dash_offset: dash_offset,
            dashes: dashes,
        });
        Ok(())
    }

    pub async fn set_clip_rectangles(&self, sorting: ClipSorting, clip_x_origin: i16, clip_y_origin: i16, rectangles: Vec<Rectangle>) -> Result<()> {
        send_request!(self.connection, sorting as u8, SetClipRectangles {
            gcontext: self.handle,
            clip_x_origin: clip_x_origin,
            clip_y_origin: clip_y_origin,
            rectangles: rectangles,
        });
        Ok(())
    }

    pub async fn free(&self) -> Result<()> {
        send_request!(self.connection, FreeGC {
            gcontext: self.handle,
        });
        Ok(())
    }
    
    pub async fn copy_area(&self, src: impl Into<Drawable<'_>>, dst: impl Into<Drawable<'_>>, src_x: i16, src_y: i16, dst_x: i16, dst_y: i16, width: u16, height: u16) -> Result<()> {
        send_request!(self.connection, CopyArea {
            src_drawable: src.into().handle(),
            dst_drawable: dst.into().handle(),
            gcontext: self.handle,
            src_x: src_x,
            src_y: src_y,
            dst_x: dst_x,
            dst_y: dst_y,
            width: width,
            height: height,
        });
        Ok(())
    }
    
    pub async fn copy_plane(&self, src: impl Into<Drawable<'_>>, dst: impl Into<Drawable<'_>>, src_x: i16, src_y: i16, dst_x: i16, dst_y: i16, width: u16, height: u16, bit_plane: u32) -> Result<()> {
        send_request!(self.connection, CopyPlane {
            src_drawable: src.into().handle(),
            dst_drawable: dst.into().handle(),
            gcontext: self.handle,
            src_x: src_x,
            src_y: src_y,
            dst_x: dst_x,
            dst_y: dst_y,
            width: width,
            height: height,
            bit_plane: bit_plane,
        });
        Ok(())
    }

    pub async fn poly_point(&self, drawable: impl Into<Drawable<'_>>, coordinate_mode: CoordinateMode, points: Vec<Point>) -> Result<()> {
        send_request!(self.connection, coordinate_mode as u8, PolyPoint {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            points: points,
        });
        Ok(())
    }

    pub async fn poly_line(&self, drawable: impl Into<Drawable<'_>>, coordinate_mode: CoordinateMode, points: Vec<Point>) -> Result<()> {
        send_request!(self.connection, coordinate_mode as u8, PolyLine {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            points: points,
        });
        Ok(())
    }

    pub async fn poly_segment(&self, drawable: impl Into<Drawable<'_>>, segments: Vec<Segment>) -> Result<()> {
        send_request!(self.connection, PolySegment {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            segments: segments,
        });
        Ok(())
    }

    pub async fn poly_rectangle(&self, drawable: impl Into<Drawable<'_>>, rectangles: Vec<Rectangle>) -> Result<()> {
        send_request!(self.connection, PolyRectangle {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            rectangles: rectangles,
        });
        Ok(())
    }

    pub async fn poly_arc(&self, drawable: impl Into<Drawable<'_>>, arcs: Vec<Arc>) -> Result<()> {
        send_request!(self.connection, PolyArc {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            arcs: arcs,
        });
        Ok(())
    }

    pub async fn fill_poly(&self, drawable: impl Into<Drawable<'_>>, coordinate_mode: CoordinateMode, shape: Shape, points: Vec<Point>) -> Result<()> {
        send_request!(self.connection, FillPoly {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            shape: shape,
            coordinate_mode: coordinate_mode,
            points: points,
        });
        Ok(())
    }

    pub async fn poly_fill_rectangle(&self, drawable: impl Into<Drawable<'_>>, rectangles: Vec<Rectangle>) -> Result<()> {
        send_request!(self.connection, PolyFillRectangle {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            rectangles: rectangles,
        });
        Ok(())
    }
    
    pub async fn poly_fill_arc(&self, drawable: impl Into<Drawable<'_>>, arcs: Vec<Arc>) -> Result<()> {
        send_request!(self.connection, PolyFillArc {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            arcs: arcs,
        });
        Ok(())
    }

    pub async fn put_image(&self, drawable: impl Into<Drawable<'_>>, format: ImageFormat, width: u16, height: u16, dst_x: i16, dst_y: i16, left_pad: u8, depth: u8, data: Vec<u8>) -> Result<()> {
        send_request!(self.connection, format as u8, PutImage {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            width: width,
            height: height,
            dst_x: dst_x,
            dst_y: dst_y,
            left_pad: left_pad,
            depth: depth,
            data: data,
        });
        Ok(())
    }

    pub async fn get_image(&self, drawable: impl Into<Drawable<'_>>, format: ImageFormat, x: i16, y: i16, width: u16, height: u16, plane_mask: u32) -> Result<FetchedImage> {
        if format == ImageFormat::Bitmap {
            bail!("cannot request bitmap image from x11");
        }
        let seq = send_request!(self.connection, format as u8, GetImage {
            drawable: drawable.into().handle(),
            x: x,
            y: y,
            width: width,
            height: height,
            plane_mask: plane_mask,
        });
        let (reply, depth) = receive_reply!(self.connection, seq, GetImageReply, fetched);

        Ok(FetchedImage {
            depth,
            visual: match reply.visual {
                0 => None,
                handle => Some(Visual { handle }),
            },
            data: reply.data,
        })
    }
    
    pub async fn poly_text8(&self, drawable: impl Into<Drawable<'_>>, x: i16, y: i16, items: impl IntoIterator<Item=TextItem8<'_>>) -> Result<()> {
        send_request!(self.connection, PolyText8 {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            x: x,
            y: y,
            items: items.into_iter().map(|item| Ok(match item {
                TextItem8::String(delta, string) => crate::coding::TextItem8 {
                    string_len: match string.len() {
                        x if x < 255 => x as u8,
                        _ => bail!("strings cannot be >254 bytes long"),
                    },
                    delta: Some(delta),
                    string: Some(string),
                    ..Default::default()
                },
                TextItem8::Font(font) => {
                    crate::coding::TextItem8 {
                        string_len: 255,
                        delta: None,
                        string: None,
                        font: Some(font.handle),
                    }
                },
            })).collect::<Result<Vec<_>>>()?,
        });

        Ok(())
    }

    pub async fn poly_text16(&self, drawable: impl Into<Drawable<'_>>, x: i16, y: i16, items: impl IntoIterator<Item=TextItem16<'_>>) -> Result<()> {
        send_request!(self.connection, PolyText16 {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            x: x,
            y: y,
            items: items.into_iter().map(|item| Ok(match item {
                TextItem16::String(delta, string) => {
                    crate::coding::TextItem16 {
                        string_len: match string.len() {
                            x if x < 255 => x as u8,
                            _ => bail!("strings cannot be >254 bytes long"),
                        },
                        delta: Some(delta),
                        string: Some(string),
                        ..Default::default()
                    }
                },
                TextItem16::Font(font) => {
                    crate::coding::TextItem16 {
                        string_len: 255,
                        delta: None,
                        string: None,
                        font: Some(font.handle),
                    }
                },
            })).collect::<Result<Vec<_>>>()?,
        });

        Ok(())
    }

    pub async fn image_text8(&self, drawable: impl Into<Drawable<'_>>, x: i16, y: i16, string: String) -> Result<()> {
        send_request!(self.connection, ImageText8 {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            x: x,
            y: y,
            string: string,
        });

        Ok(())
    }

    pub async fn image_text16(&self, drawable: impl Into<Drawable<'_>>, x: i16, y: i16, string: String) -> Result<()> {
        send_request!(self.connection, ImageText16 {
            drawable: drawable.into().handle(),
            gcontext: self.handle,
            x: x,
            y: y,
            string: string,
        });

        Ok(())
    }
}

impl<'a> Resource<'a> for GContext<'a> {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(connection: &'a X11Connection, handle: u32) -> Self {
        Self { connection, handle }
    }
}