use super::*;
use crate::coding::xrecord::XRecordOpcode;
use crate::coding::RequestBody;

pub const XRECORD_EXT_NAME: &str = "RECORD";
const XR_EVENT_COUNT: u8 = 0;

macro_rules! send_request_xrecord {
    ($self_:expr, $opcode:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XRECORD_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, $name { $($key: $value,)* })
        }
    };
    ($self_:expr, $opcode:expr, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XRECORD_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, $reply, $name { $($key: $value,)* })
        }
    };
    ($self_:expr, $opcode:expr, parse_reserved $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XRECORD_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, parse_reserved $reply, $name { $($key: $value,)* })
        }
    };
    ($self_:expr, $opcode:expr, stream, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let ext_code = $self_.0.registered_extensions.get(XRECORD_EXT_NAME).unwrap().major_opcode;
            send_request_ext!($self_, ext_code, $opcode, stream, $reply, $name { $($key: $value,)* })
        }
    };
}

mod misc;
pub use misc::*;

mod context;
pub use context::*;

mod target;
pub use target::*;
