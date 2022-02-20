use super::*;

pub use crate::coding::VisualTypeClass;

#[derive(Debug, Clone, Copy)]
pub struct Visual {
    pub(crate) handle: u32,
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

#[derive(Debug, Clone)]
pub struct Depth {
    pub depth: u8,
    pub visuals: Vec<VisualType>,
    pub(crate) _internal: (),
}

impl Resource for Visual {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(handle: u32) -> Self {
        Self { handle }
    }
}