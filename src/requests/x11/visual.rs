use intmap::IntMap;

use super::*;

pub use crate::coding::VisualTypeClass;

#[derive(Debug, Clone, Copy)]
pub struct Visual {
    pub(crate) handle: u32,
}

impl<'a> Resource<'a> for Visual {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(_connection: &'a X11Connection, handle: u32) -> Self {
        Self { handle }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct VisualType {
    pub visual: Visual,
    pub class: VisualTypeClass,
    pub bits_per_rgb_value: u8,
    pub colormap_entries: u16,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
    pub(crate) _internal: (),
}

impl From<crate::coding::VisualType> for VisualType {
    fn from(visual: crate::coding::VisualType) -> Self {
        Self {
            visual: Visual {
                handle: visual.visual,
            },
            class: visual.class,
            bits_per_rgb_value: visual.bits_per_rgb_value,
            colormap_entries: visual.colormap_entries,
            red_mask: visual.red_mask,
            green_mask: visual.green_mask,
            blue_mask: visual.blue_mask,
            _internal: (),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Depth {
    pub depth: u8,
    pub visuals: Vec<VisualType>,
    pub(crate) _internal: (),
}

impl From<crate::coding::Depth> for Depth {
    fn from(depth: crate::coding::Depth) -> Self {
        Self {
            _internal: (),
            depth: depth.depth,
            visuals: depth.visuals.into_iter().map(|visual| visual.into()).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Screen<'a> {
    pub root: Window<'a>,
    pub default_colormap: Colormap<'a>,
    pub white_pixel: Pixel,
    pub black_pixel: Pixel,
    pub current_input_masks: EventMask,
    pub width_in_pixels: u16,
    pub height_in_pixels: u16,
    pub width_in_mm: u16,
    pub height_in_mm: u16,
    pub min_installed_maps: u16,
    pub max_installed_maps: u16,
    pub root_visual: Visual,
    pub backing_stores: BackingStore,
    pub save_unders: bool,
    pub root_depth: u8,
    pub depths: IntMap<Depth>,
    pub(crate) _internal: (),
}

impl<'a> Screen<'a> {
    pub(crate) fn decode(connection: &'a X11Connection, screen: crate::coding::Screen) -> Self {
        Self {
            root: Window { handle: screen.root_window, connection },
            default_colormap: Colormap { handle: screen.default_colormap, connection },
            white_pixel: Pixel(screen.white_pixel),
            black_pixel: Pixel(screen.black_pixel),
            current_input_masks: screen.current_input_event_mask,
            width_in_pixels: screen.width_in_pixels,
            height_in_pixels: screen.height_in_pixels,
            width_in_mm: screen.width_in_mm,
            height_in_mm: screen.height_in_mm,
            min_installed_maps: screen.min_installed_maps,
            max_installed_maps: screen.max_installed_maps,
            root_visual: Visual { handle: screen.root_visual },
            backing_stores: screen.backing_store,
            save_unders: screen.save_under,
            root_depth: screen.root_depth,
            depths: screen.depths.into_iter().map(|x| x.into()).map(|x: Depth| (x.depth as u64, x)).collect(),
            _internal: (),
        }
    }
}
