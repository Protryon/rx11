use anyhow::Result;
use crate::connection::X11Connection;
use crate::events::Event;
use crate::{send_request, send_request_ext, receive_reply};

pub mod x11;
pub use x11::*;

pub mod xkb;
pub use xkb::*;

pub mod xinput;
pub use xinput::*;

pub mod xge;
pub use xge::*;

/// Any resource inside of X11, including extensions
pub trait Resource<'a>: Sized {
    #[doc(hidden)]
    fn x11_handle(&self) -> u32;

    #[doc(hidden)]
    fn from_x11_handle(connection: &'a X11Connection, handle: u32) -> Self;
}