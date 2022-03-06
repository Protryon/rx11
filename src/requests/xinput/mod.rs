use super::*;
use crate::coding::RequestBody;
use crate::coding::xinput2::{
    XIOpcode,
};
pub use fixed::types::{I16F16, I32F32};

pub const XINPUT_EXT_NAME: &str = "XInputExtension";

macro_rules! send_request_xinput {
    ($self_:expr, $opcode:expr, $is_void:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let raw = $name {
                $($key: $value,)*
                ..Default::default()
            };
            let mut buf_out = vec![];
            raw.encode_sync(&mut buf_out)?;
            let ext_code = $self_.0.registered_extensions.get(XINPUT_EXT_NAME).unwrap().major_opcode;
            $self_.send_request(ext_code as u8, $opcode as u8, $is_void, RequestBody::Ext(crate::coding::ExtRequest {
                data: buf_out,
            })).await?
        }
    };
}

impl Into<I32F32> for crate::coding::xinput2::Fp3232 {
    fn into(self) -> I32F32 {
        I32F32::from_bits(((self.integral as i64) << 32) | self.frac as u64 as i64)
    }
}

impl Into<I16F16> for crate::coding::xinput2::Fp1616 {
    fn into(self) -> I16F16 {
        I16F16::from_bits(((self.integral as i32) << 16) | self.frac as u32 as i32)
    }
}

impl From<I32F32> for crate::coding::xinput2::Fp3232 {
    fn from(from: I32F32) -> Self {
        Self {
            integral: (from.to_bits() >> 32) as i32,
            frac: from.to_bits() as u32,
        }
    }
}

impl From<I16F16> for crate::coding::xinput2::Fp1616 {
    fn from(from: I16F16) -> Self {
        Self {
            integral: (from.to_bits() >> 16) as i16,
            frac: from.to_bits() as u16,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TouchId(pub(crate) u32);

mod misc;
pub use misc::*;

mod pointer;
pub use pointer::*;

mod window;
pub use window::*;

mod device;
pub use device::*;

mod barrier;
pub use barrier::*;

mod property;
pub use property::*;

mod class;
pub use class::*;

/*

type XIOpcode = enum u8 {
    // pointer
    // XIQueryPointer,
    // XIWarpPointer,
    // XIChangeCursor,
    // XIChangeHierarchy,

    // window
    // XISetClientPointer,
    // XIGetClientPointer,
    // XISelectEvents,
    // XIGetSelectedEvents,

    // device
    // XIQueryDevice,
    // XISetFocus,
    // XIGetFocus,
    // XIGrabDevice,
    // XIUngrabDevice,
    // XIAllowEvents,
    // XIPassiveGrabDevice,
    // XIPassiveUngrabDevice,

    // property
    // XIListProperties,
    // XIChangeProperty,
    // XIDeleteProperty,
    // XIGetProperty,

    // barrier
    XIBarrierReleasePointer,
};
*/