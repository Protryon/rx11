use super::*;
pub use crate::coding::xkb::XKBEventMask;
use crate::coding::xkb::XKBOpcode;
use crate::coding::RequestBody;

pub const XKB_EXT_NAME: &str = "XKEYBOARD";
const XKB_EVENT_COUNT: u8 = 1;

macro_rules! send_request_xkb {
    ($self_:expr, $opcode:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XKB_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, $name { $($key: $value,)* })
        }
    };
    ($self_:expr, $opcode:expr, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XKB_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, $reply, $name { $($key: $value,)* })
        }
    };
    ($self_:expr, $opcode:expr, parse_reserved $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XKB_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, parse_reserved $reply, $name { $($key: $value,)* })
        }
    };
    ($self_:expr, $opcode:expr, stream, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XKB_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, stream, $reply, $name { $($key: $value,)* })
        }
    };
}

#[derive(Clone, Copy, Debug)]
pub struct Affect<T: Affectable> {
    pub affect: T,
    pub value: T,
}

pub trait Affectable: Copy + core::ops::BitOr + std::fmt::Debug {
    const FULL: Self;
}

mod misc;
pub use misc::*;

mod event;
pub use event::*;

mod device;
pub use device::*;

mod bell;
pub use bell::*;

mod state;
pub use state::*;

mod controls;
pub use controls::*;

mod map;
pub use map::*;

mod compat_map;
pub use compat_map::*;

mod indicator;
pub use indicator::*;

mod names;
pub use names::*;

mod geometry;
pub use geometry::*;

mod client_flags;
pub use client_flags::*;

mod components;
pub use components::*;

mod get_keyboard_by_name;
pub use get_keyboard_by_name::*;

mod device_info;
pub use device_info::*;

mod debug;
pub use debug::*;
