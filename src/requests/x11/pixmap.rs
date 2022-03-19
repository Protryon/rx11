use super::*;

#[derive(Clone, Copy)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Pixmap<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

impl X11Connection {
    pub async fn create_pixmap(&self, depth: &Depth, drawable: impl Into<Drawable<'_>>, width: u16, height: u16) -> Result<Pixmap<'_>> {
        let pixmap = self.new_resource_id();
        
        send_request!(self, depth.depth, CreatePixmap {
            pixmap: pixmap,
            drawable: drawable.into().handle(),
            width: width,
            height: height,
        });

        Ok(Pixmap {
            handle: pixmap,
            connection: self,
        })
    }
}

impl<'a> Pixmap<'a> {
    pub async fn free(self) -> Result<()> {
        send_request!(self.connection, FreePixmap {
            pixmap: self.handle,
        });
        Ok(())
    }
}

impl<'a> Resource<'a> for Pixmap<'a> {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(connection: &'a X11Connection, handle: u32) -> Self {
        Self { connection, handle }
    }
}