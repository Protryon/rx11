use super::*;
use crate::coding::xrandr::XROpcode;
use crate::coding::RequestBody;

pub const XRANDR_EXT_NAME: &str = "RANDR";
const XR_EVENT_COUNT: u8 = 2;

macro_rules! send_request_xrandr {
    ($self_:expr, $opcode:expr, $is_void:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let raw = $name {
                $($key: $value,)*
                ..Default::default()
            };
            let mut buf_out = vec![];
            raw.encode_sync(&mut buf_out)?;
            let ext_code = $self_.0.registered_extensions.get(XRANDR_EXT_NAME).unwrap().major_opcode;
            $self_.send_request(ext_code as u8, $opcode as u8, $is_void, RequestBody::Ext(crate::coding::ExtRequest {
                data: buf_out,
            })).await?
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
