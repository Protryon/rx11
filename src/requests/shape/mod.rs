use super::*;
use crate::coding::shape::ShapeOpcode;
use crate::coding::RequestBody;

pub const SHAPE_EXT_NAME: &str = "SHAPE";
const SHAPE_EVENT_COUNT: u8 = 1;

macro_rules! send_request_shape {
    ($self_:expr, $opcode:expr, $is_void:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let raw = $name {
                $($key: $value,)*
                ..Default::default()
            };
            let mut buf_out = vec![];
            raw.encode_sync(&mut buf_out)?;
            let ext_code = $self_.0.registered_extensions.get(SHAPE_EXT_NAME).unwrap().major_opcode;
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