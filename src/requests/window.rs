use derive_builder::Builder;

use super::*;

pub use crate::coding::{
    WindowClass,
    BitGravity,
    WinGravity,
    BackingStore,
    MapState,
    CirculateWindowDirection,
    ChangePropertyMode,
    StackMode,
    ConfigureWindowBitmask,
};

#[derive(Debug, Clone, Copy)]
pub struct Window {
    pub(crate) handle: u32,
}

#[derive(Default, Builder, Debug)]
#[builder(default)]
pub struct WindowParams {
    pub depth: u8,
    #[builder(setter(into, strip_option), default)]
    pub parent: Option<Window>,
    pub x: i16,
    pub y: i16,
    #[builder(default = "100")]
    pub width: u16,
    #[builder(default = "100")]
    pub height: u16,
    #[builder(default = "1")]
    pub border_width: u16,
    #[builder(default = "WindowClass::InputOutput")]
    pub window_class: WindowClass,
    pub visual: WindowVisual,
    pub attributes: WindowAttributes,
}

#[derive(Default, Builder, Debug)]
#[builder(default)]
pub struct WindowConfig {
    #[builder(setter(into, strip_option), default)]
    pub x: Option<i16>,
    #[builder(setter(into, strip_option), default)]
    pub y: Option<i16>,
    #[builder(setter(into, strip_option), default)]
    pub width: Option<u16>,
    #[builder(setter(into, strip_option), default)]
    pub height: Option<u16>,
    #[builder(setter(into, strip_option), default)]
    pub border_width: Option<u16>,
    #[builder(setter(into, strip_option), default)]
    pub sibling: Option<Window>,
    #[builder(setter(into, strip_option), default)]
    pub stack_mode: Option<StackMode>,
}

#[derive(Debug, Clone, Copy)]
pub enum BackgroundPixmap {
    None,
    ParentRelative,
    Some(Pixmap),
}

impl Default for BackgroundPixmap {
    fn default() -> Self {
        BackgroundPixmap::None
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BorderPixmap {
    CopyFromParent,
    Some(Pixmap),
}

impl Default for BorderPixmap {
    fn default() -> Self {
        BorderPixmap::CopyFromParent
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WindowColormap {
    CopyFromParent,
    Some(Colormap),
}

impl Default for WindowColormap {
    fn default() -> Self {
        WindowColormap::CopyFromParent
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WindowVisual {
    CopyFromParent,
    Some(Visual),
}

impl Default for WindowVisual {
    fn default() -> Self {
        WindowVisual::CopyFromParent
    }
}

#[derive(Builder, Default, Debug, Clone)]
#[builder(default)]
pub struct WindowAttributes {
    #[builder(default = "BackgroundPixmap::None")]
    pub background_pixmap: BackgroundPixmap,
    #[builder(setter(into, strip_option), default)]
    pub background_pixel: Option<Pixel>,
    #[builder(default = "BorderPixmap::CopyFromParent")]
    pub border_pixmap: BorderPixmap,
    #[builder(setter(into, strip_option), default)]
    pub border_pixel: Option<Pixel>,
    #[builder(default = "BitGravity::Forget")]
    pub bit_gravity: BitGravity,
    #[builder(default = "WinGravity::NorthWest")]
    pub win_gravity: WinGravity,
    #[builder(default = "BackingStore::NotUseful")]
    pub backing_store: BackingStore,
    #[builder(default = "u32::MAX")]
    pub backing_planes: u32,
    #[builder(setter(into, strip_option), default)]
    pub backing_pixel: Option<Pixel>,
    pub override_redirect: bool,
    pub save_under: bool,
    pub event_mask: EventMask,
    pub do_not_propagate_mask: EventMask,
    pub colormap: WindowColormap,
    #[builder(setter(into, strip_option), default)]
    pub cursor: Option<Cursor>,
}

impl Into<crate::coding::WindowAttributes> for WindowAttributes {
    fn into(self) -> crate::coding::WindowAttributes {
        let mut attrs = crate::coding::WindowAttributes::default();
        match self.background_pixmap {
            BackgroundPixmap::None => (),
            BackgroundPixmap::ParentRelative => {
                attrs.bitmask.set_background_pixmap();
                attrs.background_pixmap = Some(1);
            },
            BackgroundPixmap::Some(x) => {
                attrs.bitmask.set_background_pixmap();
                attrs.background_pixmap = Some(x.handle);
            },
        }
        match self.background_pixel {
            None => (),
            Some(pixel) => {
                attrs.bitmask.set_background_pixel();
                attrs.background_pixel = Some(pixel.0);
            },
        }
        match self.border_pixmap {
            BorderPixmap::CopyFromParent => (),
            BorderPixmap::Some(x) => {
                attrs.bitmask.set_border_pixmap();
                attrs.border_pixmap = Some(x.handle);
            },
        }
        match self.border_pixel {
            None => (),
            Some(pixel) => {
                attrs.bitmask.set_border_pixel();
                attrs.border_pixel = Some(pixel.0);
            },
        }
        match self.bit_gravity {
            BitGravity::Forget => (),
            gravity => {
                attrs.bitmask.set_bit_gravity();
                attrs.bit_gravity = Some(gravity);
            },
        }
        match self.win_gravity {
            WinGravity::NorthWest => (),
            gravity => {
                attrs.bitmask.set_win_gravity();
                attrs.win_gravity = Some(gravity);
            },
        }
        match self.backing_store {
            BackingStore::NotUseful => (),
            store => {
                attrs.bitmask.set_backing_store();
                attrs.backing_store = Some(store);
            },
        }
        match self.backing_planes {
            u32::MAX => (),
            backing_planes => {
                attrs.bitmask.set_backing_planes();
                attrs.backing_planes = Some(backing_planes);
            },
        }
        match self.backing_pixel {
            None => (),
            Some(pixel) => {
                attrs.bitmask.set_backing_pixel();
                attrs.backing_pixel = Some(pixel.0);
            },
        }
        match self.save_under {
            false => (),
            true => {
                attrs.bitmask.set_save_under();
                attrs.save_under = Some(true);
            },
        }
        match self.event_mask {
            EventMask::ZERO => (),
            event_mask => {
                attrs.bitmask.set_event_mask();
                attrs.event_mask = Some(event_mask);
            },
        }
        match self.do_not_propagate_mask {
            EventMask::ZERO => (),
            do_not_propagate_mask => {
                attrs.bitmask.set_do_not_propagate_mask();
                attrs.do_not_propagate_mask = Some(do_not_propagate_mask);
            },
        }
        match self.override_redirect {
            false => (),
            true => {
                attrs.bitmask.set_override_redirect();
                attrs.override_redirect = Some(true);
            },
        }
        match self.colormap {
            WindowColormap::CopyFromParent => (),
            WindowColormap::Some(colormap) => {
                attrs.bitmask.set_colormap();
                attrs.colormap = Some(colormap.handle);
            },
        }
        match self.cursor {
            None => (),
            Some(cursor) => {
                attrs.bitmask.set_cursor();
                attrs.cursor = Some(cursor.handle);
            },
        }
        attrs
    }
}


#[derive(Debug, Clone)]
pub struct FetchedWindowAttributes {
    pub backing_store: BackingStore,
    pub visual: Visual,
    pub class: WindowClass,
    pub bit_gravity: BitGravity,
    pub win_gravity: WinGravity,
    pub backing_planes: u32,
    pub backing_pixel: Pixel,
    pub save_under: bool,
    pub map_is_installed: bool,
    pub map_state: MapState,
    pub override_redirect: bool,
    pub colormap: Option<Colormap>,
    pub all_event_mask: EventMask,
    pub local_event_mask: EventMask,
    pub do_not_propagate_mask: EventMask,
}

#[derive(Debug, Clone)]
pub struct QueryTreeResult {
    pub root: Window,
    pub parent: Option<Window>,
    pub children: Vec<Window>,
}

impl X11Connection {
    pub fn screen(&self) -> &Screen {
        &self.handshake().screens[0]
    }

    pub fn root_window(&self) -> Window {
        Window {
            handle: self.screen().root_window,
        }
    }

    pub async fn create_window(&self, params: WindowParams) -> Result<Window> {
        let window = self.new_resource_id();
        
        send_request!(self, params.depth, CreateWindow {
            window: window,
            parent: params.parent.unwrap_or_else(|| self.root_window()).handle,
            x: params.x,
            y: params.y,
            width: params.width,
            height: params.height,
            border_width: params.border_width,
            class: params.window_class,
            visual_id: match params.visual {
                WindowVisual::CopyFromParent => 0,
                WindowVisual::Some(x) => x.handle,
            },
            attributes: params.attributes.into(),
        });
        Ok(Window {
            handle: window,
        })
    }

    pub async fn change_window_attributes(&self, window: Window, params: WindowAttributes) -> Result<()> {
        send_request!(self, ChangeWindowAttributes {
            window: window.handle,
            attributes: params.into(),
        });
        Ok(())
    }

    pub async fn get_window_attributes(&self, window: Window) -> Result<FetchedWindowAttributes> {
        let seq = send_request!(self, GetWindowAttributes {
            window: window.handle,
        });
        let (reply, backing_store) = receive_reply!(self, seq, GetWindowAttributesReply, fetched);

        Ok(FetchedWindowAttributes {
            backing_store: BackingStore::decode_sync(&mut &[backing_store][..])?,
            visual: Visual {
                handle: reply.visual_id,
            },
            class: reply.class,
            bit_gravity: reply.bit_gravity,
            win_gravity: reply.win_gravity,
            backing_planes: reply.backing_planes,
            backing_pixel: Pixel(reply.backing_pixel),
            save_under: reply.save_under,
            map_is_installed: reply.map_is_installed,
            map_state: reply.map_state,
            override_redirect: reply.override_redirect,
            colormap: match reply.colormap {
                0 => None,
                x => Some(Colormap {
                    handle: x,
                }),
            },
            all_event_mask: reply.all_event_mask,
            local_event_mask: reply.local_event_mask,
            do_not_propagate_mask: reply.do_not_propagate_mask,
        })
    }

    pub async fn destroy_window(&self, window: Window) -> Result<()> {
        send_request!(self, DestroyWindow {
            window: window.handle,
        });
        Ok(())
    }

    pub async fn destroy_subwindows(&self, window: Window) -> Result<()> {
        send_request!(self, DestroySubwindows {
            window: window.handle,
        });
        Ok(())
    }

    pub async fn save_set_add_window(&self, window: Window) -> Result<()> {
        send_request!(self, InsertDelete::Insert as u8, ChangeSaveSet {
            window: window.handle,
        });
        Ok(())
    }

    pub async fn save_set_delete_window(&self, window: Window) -> Result<()> {
        send_request!(self, InsertDelete::Delete as u8, ChangeSaveSet {
            window: window.handle,
        });
        Ok(())
    }

    pub async fn reparent_window(&self, window: Window, new_parent: Window, x: i16, y: i16) -> Result<()> {
        send_request!(self, ReparentWindow {
            window: window.handle,
            parent: new_parent.handle,
            x: x,
            y: y,
        });
        Ok(())
    }

    pub async fn map_window(&self, window: Window) -> Result<()> {
        send_request!(self, MapWindow {
            window: window.handle,
        });
        Ok(())
    }

    pub async fn map_subwindows(&self, window: Window) -> Result<()> {
        send_request!(self, MapSubwindows {
            window: window.handle,
        });
        Ok(())
    }

    pub async fn unmap_window(&self, window: Window) -> Result<()> {
        send_request!(self, UnmapWindow {
            window: window.handle,
        });
        Ok(())
    }

    pub async fn unmap_subwindows(&self, window: Window) -> Result<()> {
        send_request!(self, UnmapSubwindows {
            window: window.handle,
        });
        Ok(())
    }

    pub async fn configure_window(&self, window: Window, config: WindowConfig) -> Result<()> {
        let mut bitmask = ConfigureWindowBitmask::ZERO;
        if config.x.is_some() {
            bitmask.set_x();
        }
        if config.y.is_some() {
            bitmask.set_y();
        }
        if config.width.is_some() {
            bitmask.set_width();
        }
        if config.height.is_some() {
            bitmask.set_height();
        }
        if config.border_width.is_some() {
            bitmask.set_border_width();
        }
        if config.sibling.is_some() {
            bitmask.set_sibling();
        }
        if config.stack_mode.is_some() {
            bitmask.set_stack_mode();
        }
        send_request!(self, ConfigureWindow {
            window: window.handle,
            bitmask: bitmask,
            x: config.x,
            y: config.y,
            width: config.width,
            height: config.height,
            border_width: config.border_width,
            sibling: config.sibling.map(|x| x.handle),
            stack_mode: config.stack_mode,
        });

        Ok(())
    }
    
    pub async fn circulate_window(&self, window: Window, direction: CirculateWindowDirection) -> Result<()> {
        send_request!(self, direction as u8, CirculateWindow {
            window: window.handle,
        });

        Ok(())
    }

    pub async fn query_tree(&self, window: Window) -> Result<QueryTreeResult> {
        let seq = send_request!(self, QueryTree {
            window: window.handle,
        });
        let reply = receive_reply!(self, seq, QueryTreeReply);

        Ok(QueryTreeResult {
            root: Window { handle: reply.root_window },
            parent: match reply.parent_window {
                0 => None,
                handle => Some(Window { handle }),
            },
            children: reply.children_windows.into_iter().map(|handle| Window { handle }).collect(),
        })
    }
}

impl Resource for Window {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(handle: u32) -> Self {
        Self { handle }
    }
}