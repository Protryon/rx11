use super::*;
use crate::coding::xfixes::XFOpcode;
use crate::coding::RequestBody;

pub const XFIXES_EXT_NAME: &str = "XFIXES";
const XF_EVENT_COUNT: u8 = 2;

macro_rules! send_request_xfixes {
    ($self_:expr, $opcode:expr, $is_void:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let raw = $name {
                $($key: $value,)*
                ..Default::default()
            };
            let mut buf_out = vec![];
            raw.encode_sync(&mut buf_out)?;
            let ext_code = $self_.0.registered_extensions.get(XFIXES_EXT_NAME).unwrap().major_opcode;
            $self_.send_request(ext_code as u8, $opcode as u8, $is_void, RequestBody::Ext(crate::coding::ExtRequest {
                data: buf_out,
            })).await?
        }
    };
}

mod misc;
pub use misc::*;

mod window;
pub use window::*;

mod selection;
pub use selection::*;

mod cursor;
pub use cursor::*;

mod region;
pub use region::*;

mod gcontext;
pub use gcontext::*;

mod picture;
pub use picture::*;

mod barrier;
pub use barrier::*;
