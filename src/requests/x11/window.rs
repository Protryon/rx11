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

#[derive(Clone, Copy)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Window<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

#[derive(Default, Builder, Debug)]
#[builder(default)]
pub struct WindowParams<'a> {
    pub depth: u8,
    #[builder(setter(into, strip_option), default)]
    pub parent: Option<Window<'a>>,
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
    pub attributes: WindowAttributes<'a>,
}

#[derive(Default, Builder, Debug)]
#[builder(default)]
pub struct WindowConfig<'a> {
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
    pub sibling: Option<Window<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub stack_mode: Option<StackMode>,
}

#[derive(Debug, Clone, Copy)]
pub enum BackgroundPixmap<'a> {
    None,
    ParentRelative,
    Some(Pixmap<'a>),
}

impl<'a> Default for BackgroundPixmap<'a> {
    fn default() -> Self {
        BackgroundPixmap::None
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BorderPixmap<'a> {
    CopyFromParent,
    Some(Pixmap<'a>),
}

impl<'a> Default for BorderPixmap<'a> {
    fn default() -> Self {
        BorderPixmap::CopyFromParent
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WindowColormap<'a> {
    CopyFromParent,
    Some(Colormap<'a>),
}

impl<'a> Default for WindowColormap<'a> {
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
pub struct WindowAttributes<'a> {
    #[builder(default = "BackgroundPixmap::None")]
    pub background_pixmap: BackgroundPixmap<'a>,
    #[builder(setter(into, strip_option), default)]
    pub background_pixel: Option<Pixel>,
    #[builder(default = "BorderPixmap::CopyFromParent")]
    pub border_pixmap: BorderPixmap<'a>,
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
    pub colormap: WindowColormap<'a>,
    #[builder(setter(into, strip_option), default)]
    pub cursor: Option<Cursor<'a>>,
}

impl<'a> Into<crate::coding::WindowAttributes> for WindowAttributes<'a> {
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
pub struct FetchedWindowAttributes<'a> {
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
    pub colormap: Option<Colormap<'a>>,
    pub all_event_mask: EventMask,
    pub local_event_mask: EventMask,
    pub do_not_propagate_mask: EventMask,
}

#[derive(Debug, Clone)]
pub struct QueryTreeResult<'a> {
    pub root: Window<'a>,
    pub parent: Option<Window<'a>>,
    pub children: Vec<Window<'a>>,
}

impl X11Connection {
    pub fn root_window(&self, screen: usize) -> Option<Window<'_>> {
        Some(Window {
            handle: self.0.handshake.screens.get(screen)?.root_window,
            connection: self,
        })
    }

    pub async fn create_window(&self, params: WindowParams<'_>) -> Result<Window<'_>> {
        let window = self.new_resource_id();
        
        send_request!(self, params.depth, CreateWindow {
            window: window,
            parent: params.parent.or_else(|| self.root_window(0)).ok_or_else(|| anyhow!("no screens for default parent"))?.handle,
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
            connection: self,
        })
    }
}
impl<'a> Window<'a> {

    pub async fn change_attributes(&self, params: WindowAttributes<'_>) -> Result<()> {
        let attributes = params.into();
        send_request!(self.connection, ChangeWindowAttributes {
            window: self.handle,
            attributes: attributes,
        });
        Ok(())
    }

    pub async fn get_attributes(&self) -> Result<FetchedWindowAttributes<'a>> {
        let seq = send_request!(self.connection, GetWindowAttributes {
            window: self.handle,
        });
        let (reply, backing_store) = receive_reply!(self.connection, seq, GetWindowAttributesReply, fetched);

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
                    connection: self.connection,
                }),
            },
            all_event_mask: reply.all_event_mask,
            local_event_mask: reply.local_event_mask,
            do_not_propagate_mask: reply.do_not_propagate_mask,
        })
    }

    pub async fn destroy(&self) -> Result<()> {
        send_request!(self.connection, DestroyWindow {
            window: self.handle,
        });
        Ok(())
    }

    pub async fn destroy_subwindows(&self) -> Result<()> {
        send_request!(self.connection, DestroySubwindows {
            window: self.handle,
        });
        Ok(())
    }

    pub async fn save_set_add(&self) -> Result<()> {
        send_request!(self.connection, InsertDelete::Insert as u8, ChangeSaveSet {
            window: self.handle,
        });
        Ok(())
    }

    pub async fn save_set_delete(&self) -> Result<()> {
        send_request!(self.connection, InsertDelete::Delete as u8, ChangeSaveSet {
            window: self.handle,
        });
        Ok(())
    }

    pub async fn reparent(&self, new_parent: Window<'_>, x: i16, y: i16) -> Result<()> {
        send_request!(self.connection, ReparentWindow {
            window: self.handle,
            parent: new_parent.handle,
            x: x,
            y: y,
        });
        Ok(())
    }

    pub async fn map(&self) -> Result<()> {
        send_request!(self.connection, MapWindow {
            window: self.handle,
        });
        Ok(())
    }

    pub async fn map_subwindows(&self) -> Result<()> {
        send_request!(self.connection, MapSubwindows {
            window: self.handle,
        });
        Ok(())
    }

    pub async fn unmap(&self) -> Result<()> {
        send_request!(self.connection, UnmapWindow {
            window: self.handle,
        });
        Ok(())
    }

    pub async fn unmap_subwindows(&self) -> Result<()> {
        send_request!(self.connection, UnmapSubwindows {
            window: self.handle,
        });
        Ok(())
    }

    pub async fn configure(&self, config: WindowConfig<'_>) -> Result<()> {
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
        send_request!(self.connection, ConfigureWindow {
            window: self.handle,
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
    
    pub async fn circulate(&self, direction: CirculateWindowDirection) -> Result<()> {
        send_request!(self.connection, direction as u8, CirculateWindow {
            window: self.handle,
        });

        Ok(())
    }

    pub async fn query_tree(&self) -> Result<QueryTreeResult<'_>> {
        let seq = send_request!(self.connection, QueryTree {
            window: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, QueryTreeReply);

        Ok(QueryTreeResult {
            root: Window {
                handle: reply.root_window,
                connection: self.connection,
            },
            parent: match reply.parent_window {
                0 => None,
                handle => Some(Window {
                    handle,
                    connection: self.connection,
                }),
            },
            children: reply.children_windows.into_iter().map(|handle| Window { handle, connection: self.connection }).collect(),
        })
    }
}

impl<'a> Resource<'a> for Window<'a> {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(connection: &'a X11Connection, handle: u32) -> Self {
        Self { handle, connection }
    }
}