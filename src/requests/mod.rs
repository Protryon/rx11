use crate::events::Event;
use crate::net::X11Connection;
use crate::{receive_reply, send_request, send_request_ext};
use anyhow::Result;

pub mod x11;
pub use x11::*;

pub mod xkb;
pub use xkb::*;

pub mod xinput;
pub use xinput::*;

pub mod xfixes;
pub use xfixes::*;

pub mod xge;
pub use xge::*;

mod misc;
pub use misc::*;

/// Any resource inside of X11, including extensions
pub trait Resource<'a>: Sized {
    #[doc(hidden)]
    fn x11_handle(&self) -> u32;

    #[doc(hidden)]
    fn from_x11_handle(connection: &'a X11Connection, handle: u32) -> Self;
}
