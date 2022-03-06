use super::*;

pub use crate::coding::QueryBestSizeClass;

#[derive(Debug, Clone, Copy)]
pub enum Drawable<'a> {
    Window(Window<'a>),
    Pixmap(Pixmap<'a>),
    Raw(RawDrawable<'a>),
}

#[derive(Clone, Copy, derivative::Derivative)]
#[derivative(Debug)]
pub struct RawDrawable<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    #[allow(dead_code)]
    pub(crate) connection: &'a X11Connection,
}

impl<'a> From<Window<'a>> for Drawable<'a> {
    fn from(from: Window<'a>) -> Self {
        Drawable::Window(from)
    }
}

impl<'a> From<Pixmap<'a>> for Drawable<'a> {
    fn from(from: Pixmap<'a>) -> Self {
        Drawable::Pixmap(from)
    }
}

impl<'a> Drawable<'a> {
    pub(crate) fn handle(&self) -> u32 {
        match self {
            Drawable::Window(x) => x.handle,
            Drawable::Pixmap(x) => x.handle,
            Drawable::Raw(x) => x.handle,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Geometry<'a> {
    pub root_window: Window<'a>,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
}

impl X11Connection {
    pub async fn get_geometry(&self, drawable: impl Into<Drawable<'_>>) -> Result<Geometry<'_>> {
        let seq = send_request!(
            self,
            GetGeometry {
                drawable: drawable.into().handle(),
            }
        );
        let reply = receive_reply!(self, seq, GetGeometryReply);

        Ok(Geometry {
            root_window: Window {
                handle: reply.root_window,
                connection: self,
            },
            x: reply.x,
            y: reply.y,
            width: reply.width,
            height: reply.height,
            border_width: reply.border_width,
        })
    }

    pub async fn query_best_size(
        &self,
        drawable: impl Into<Drawable<'_>>,
        class: QueryBestSizeClass,
        width: u16,
        height: u16,
    ) -> Result<(u16, u16)> {
        let seq = send_request!(
            self,
            class as u8,
            QueryBestSize {
                drawable: drawable.into().handle(),
                width: width,
                height: height,
            }
        );
        let reply = receive_reply!(self, seq, QueryBestSizeReply);

        Ok((reply.width, reply.height))
    }
}

impl<'a> Resource<'a> for Drawable<'a> {
    fn x11_handle(&self) -> u32 {
        self.handle()
    }

    fn from_x11_handle(_connection: &'a X11Connection, _handle: u32) -> Self {
        unimplemented!("cannot call from_x11_handle on Drawable");
    }
}
