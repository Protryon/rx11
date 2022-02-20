use anyhow::Result;
use crate::connection::X11Connection;
use crate::coding::*;
use crate::{send_request, receive_reply};

mod extensions;
pub use extensions::*;

mod window;
pub use window::*;

mod pixel;
pub use pixel::*;

mod pixmap;
pub use pixmap::*;

mod colormap;
pub use colormap::*;

mod cursor;
pub use cursor::*;

mod visual;
pub use visual::{*, Depth, VisualType};

mod color;
pub use color::*;

mod font;
pub use font::*;

mod drawable;
pub use drawable::*;

mod atom;
pub use atom::*;

mod timestamp;
pub use timestamp::*;

mod selection;
pub use selection::*;

mod properties;
pub use properties::*;

mod event;
pub use event::*;

mod grab;
pub use grab::*;

mod pointer;
pub use pointer::*;

mod gcontext;
pub use gcontext::*;

mod inputs;
pub use inputs::*;

mod keysym;
pub use keysym::*;

mod screensaver;
pub use screensaver::*;

mod access_control;
pub use access_control::*;

mod control;
pub use control::*;

/// Any resource inside of X11, including extensions
pub trait Resource: Sized {
    #[doc(hidden)]
    fn x11_handle(&self) -> u32;

    #[doc(hidden)]
    fn from_x11_handle(handle: u32) -> Self;
}