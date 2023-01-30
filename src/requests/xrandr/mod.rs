use super::*;
use crate::coding::xrandr::XROpcode;
use crate::coding::RequestBody;

pub const XRANDR_EXT_NAME: &str = "RANDR";
const XR_EVENT_COUNT: u8 = 2;

macro_rules! send_request_xrandr {
    ($self_:expr, $opcode:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XRANDR_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, $name { $($key: $value,)* })
        }
    };
    ($self_:expr, $opcode:expr, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XRANDR_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, $reply, $name { $($key: $value,)* })
        }
    };
    ($self_:expr, $opcode:expr, parse_reserved $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XRANDR_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, parse_reserved $reply, $name { $($key: $value,)* })
        }
    };
    ($self_:expr, $opcode:expr, stream, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XRANDR_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, stream, $reply, $name { $($key: $value,)* })
        }
    };
}

impl Into<I16F16> for crate::coding::xrandr::Fp1616 {
    fn into(self) -> I16F16 {
        I16F16::from_bits(((self.integral as i32) << 16) | self.frac as u32 as i32)
    }
}

impl From<I16F16> for crate::coding::xrandr::Fp1616 {
    fn from(from: I16F16) -> Self {
        Self {
            integral: (from.to_bits() >> 16) as i16,
            frac: from.to_bits() as u16,
        }
    }
}

mod misc;
pub use misc::*;

mod screen;
pub use screen::*;

mod output;
pub use output::*;

mod mode;
pub use mode::*;

mod crtc;
pub use crtc::*;

mod provider;
pub use provider::*;

mod monitor;
pub use monitor::*;
