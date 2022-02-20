use super::*;

pub use crate::coding::CreateColormapAlloc;

#[derive(Debug, Clone, Copy)]
pub struct Colormap {
    pub(crate) handle: u32,
}

impl X11Connection {

    pub async fn create_colormap(&self, window: Window, visual: Visual, alloc: CreateColormapAlloc) -> Result<Colormap> {
        let colormap = self.new_resource_id();
        
        send_request!(self, alloc as u8, CreateColormap {
            window: window.handle,
            visual: visual.handle,
            colormap: colormap,
        });
        Ok(Colormap {
            handle: colormap,
        })
    }

    pub async fn free_colormap(&self, colormap: Colormap) -> Result<()> {
        send_request!(self, FreeColormap {
            colormap: colormap.handle,
        });
        Ok(())
    }

    pub async fn copy_colormap_and_free(&self, from: Colormap) -> Result<Colormap> {
        let colormap = self.new_resource_id();
        send_request!(self, CopyColormapAndFree {
            src_colormap: from.handle,
            dst_colormap: colormap,
        });
        Ok(Colormap {
            handle: colormap,
        })
    }

    pub async fn install_colormap(&self, colormap: Colormap) -> Result<()> {
        send_request!(self, InstallColormap {
            colormap: colormap.handle,
        });
        Ok(())
    }

    pub async fn uninstall_colormap(&self, colormap: Colormap) -> Result<()> {
        send_request!(self, UninstallColormap {
            colormap: colormap.handle,
        });
        Ok(())
    }

    pub async fn list_installed_colormaps(&self, window: Window) -> Result<Vec<Colormap>> {
        let seq = send_request!(self, ListInstalledColormaps {
            window: window.handle,
        });
        let reply = receive_reply!(self, seq, ListInstalledColormapsReply);

        Ok(reply.colormaps.into_iter().map(|handle| Colormap {
            handle
        }).collect())
    }
}

impl Resource for Colormap {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(handle: u32) -> Self {
        Self { handle }
    }
}