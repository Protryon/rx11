use super::*;

pub use crate::coding::CreateColormapAlloc;

#[derive(Clone, Copy)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Colormap<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

impl<'a> Window<'a> {
    pub async fn create_colormap(self, visual: Visual, alloc: CreateColormapAlloc) -> Result<Colormap<'a>> {
        let colormap = self.connection.new_resource_id();
        
        send_request!(self.connection, alloc as u8, CreateColormap {
            window: self.handle,
            visual: visual.handle,
            colormap: colormap,
        });
        Ok(Colormap {
            handle: colormap,
            connection: self.connection,
        })
    }


    pub async fn list_installed_colormaps(self) -> Result<Vec<Colormap<'a>>> {
        let seq = send_request!(self.connection, ListInstalledColormaps {
            window: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, ListInstalledColormapsReply);

        Ok(reply.colormaps.into_iter().map(|handle| Colormap {
            handle,
            connection: self.connection,
        }).collect())
    }
}

impl<'a> Colormap<'a> {

    pub async fn free(self) -> Result<()> {
        send_request!(self.connection, FreeColormap {
            colormap: self.handle,
        });
        Ok(())
    }

    pub async fn copy_and_free(self) -> Result<Colormap<'a>> {
        let colormap = self.connection.new_resource_id();
        send_request!(self.connection, CopyColormapAndFree {
            src_colormap: self.handle,
            dst_colormap: colormap,
        });
        Ok(Colormap {
            handle: colormap,
            connection: self.connection,
        })
    }

    pub async fn install(self) -> Result<()> {
        send_request!(self.connection, InstallColormap {
            colormap: self.handle,
        });
        Ok(())
    }

    pub async fn uninstall(self) -> Result<()> {
        send_request!(self.connection, UninstallColormap {
            colormap: self.handle,
        });
        Ok(())
    }
}

impl<'a> Resource<'a> for Colormap<'a> {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(connection: &'a X11Connection, handle: u32) -> Self {
        Self { connection, handle }
    }
}