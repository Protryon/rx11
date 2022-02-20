use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Pixmap {
    pub(crate) handle: u32,
}

impl X11Connection {
    pub async fn create_pixmap<D: Into<Drawable>>(&self, depth: &Depth, drawable: D, width: u16, height: u16) -> Result<Pixmap> {
        let pixmap = self.new_resource_id();
        
        send_request!(self, depth.depth, CreatePixmap {
            pixmap: pixmap,
            drawable: drawable.into().handle(),
            width: width,
            height: height,
        });

        Ok(Pixmap {
            handle: pixmap,
        })
    }

    pub async fn free_pixmap(&self, pixmap: Pixmap) -> Result<()> {
        send_request!(self, FreePixmap {
            pixmap: pixmap.handle,
        });
        Ok(())
    }

}

impl Resource for Pixmap {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(handle: u32) -> Self {
        Self { handle }
    }
}