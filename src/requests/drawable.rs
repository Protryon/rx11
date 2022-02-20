use super::*;

pub use crate::coding::QueryBestSizeClass;

#[derive(Debug, Clone, Copy)]
pub enum Drawable {
    Window(Window),
    Pixmap(Pixmap),
}

impl From<Window> for Drawable {
    fn from(from: Window) -> Self {
        Drawable::Window(from)
    }
}

impl From<Pixmap> for Drawable {
    fn from(from: Pixmap) -> Self {
        Drawable::Pixmap(from)
    }
}

impl Drawable {
    pub(crate) fn handle(&self) -> u32 {
        match self {
            Drawable::Window(x) => x.handle,
            Drawable::Pixmap(x) => x.handle,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Geometry {
    pub root_window: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
}

impl X11Connection {

    pub async fn get_geometry(&self, drawable: impl Into<Drawable>) -> Result<Geometry> {
        let seq = send_request!(self, GetGeometry {
            drawable: drawable.into().handle(),
        });
        let reply = receive_reply!(self, seq, GetGeometryReply);

        Ok(Geometry {
            root_window: Window { handle: reply.root_window },
            x: reply.x,
            y: reply.y,
            width: reply.width,
            height: reply.height,
            border_width: reply.border_width,
        })
    }

    pub async fn query_best_size(&self, drawable: impl Into<Drawable>, class: QueryBestSizeClass, width: u16, height: u16) -> Result<(u16, u16)> {
        let seq = send_request!(self, class as u8, QueryBestSize {
            drawable: drawable.into().handle(),
            width: width,
            height: height,
        });
        let reply = receive_reply!(self, seq, QueryBestSizeReply);

        Ok((reply.width, reply.height))
    }
}

impl Resource for Drawable {
    fn x11_handle(&self) -> u32 {
        self.handle()
    }

    fn from_x11_handle(_handle: u32) -> Self {
        unimplemented!("cannot call from_x11_handle on Drawable");
    }
}