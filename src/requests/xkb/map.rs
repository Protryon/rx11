use super::*;

use crate::coding::xkb::GetMapRequest;
pub use crate::coding::xkb::{
    GetMapResponse,
};

impl X11Connection {
    pub async fn xkb_get_map(&self, device: DeviceSpec, parts: MapPart) -> Result<GetMapResponse> {
        // we don't support partial `get_map` calls due to:
        // 1. i can't see a reason to have it other than bandwidth (but this isn't the 1980s)
        // 2. it'll make this interface more complicated
        let seq = send_request_xkb!(self, XKBOpcode::GetMap, false, GetMapRequest {
            device_spec: device.into(),
            full: parts,
            partial: MapPart::ZERO,
        });
        let reply = receive_reply!(self, seq, GetMapResponse);

        Ok(reply)
    }

    // todo: 
    // pub async fn xkb_set_map(&self, device: DeviceSpec) -> Result<()>;
}