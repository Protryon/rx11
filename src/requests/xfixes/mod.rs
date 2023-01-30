use super::*;
use crate::coding::xfixes::XFOpcode;
use crate::coding::RequestBody;

pub const XFIXES_EXT_NAME: &str = "XFIXES";
const XF_EVENT_COUNT: u8 = 2;

macro_rules! send_request_xfixes {
    ($self_:expr, $opcode:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XFIXES_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, $name { $($key: $value,)* })
        }
    };
    ($self_:expr, $opcode:expr, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XFIXES_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, $reply, $name { $($key: $value,)* })
        }
    };
    ($self_:expr, $opcode:expr, parse_reserved $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XFIXES_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, parse_reserved $reply, $name { $($key: $value,)* })
        }
    };
    ($self_:expr, $opcode:expr, stream, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XFIXES_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, stream, $reply, $name { $($key: $value,)* })
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
