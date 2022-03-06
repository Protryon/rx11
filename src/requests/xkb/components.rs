use super::*;

use crate::coding::xkb::{ListComponentsRequest, String8};
pub use crate::coding::xkb::{
    ListComponentsResponse,
    Listing,
};

impl X11Connection {
    pub async fn xkb_list_components(
        &self,
        device: DeviceSpec,
        max_names: u16,
        keymaps_spec: impl AsRef<str>,
        keycodes_spec: impl AsRef<str>,
        types_spec: impl AsRef<str>,
        compat_map_spec: impl AsRef<str>,
        symbols_spec: impl AsRef<str>,
        geometry_spec: impl AsRef<str>,
    ) -> Result<ListComponentsResponse> {
        let seq = send_request_xkb!(self, XKBOpcode::ListComponents, false, ListComponentsRequest {
            device_spec: device.into(),
            max_names: max_names,
            keymaps_spec: String8 { string: keymaps_spec.as_ref().to_string(), len: 0 },
            keycodes_spec: String8 { string: keycodes_spec.as_ref().to_string(), len: 0 },
            types_spec: String8 { string: types_spec.as_ref().to_string(), len: 0 },
            compat_map_spec: String8 { string: compat_map_spec.as_ref().to_string(), len: 0 },
            symbols_spec: String8 { string: symbols_spec.as_ref().to_string(), len: 0 },
            geometry_spec: String8 { string: geometry_spec.as_ref().to_string(), len: 0 },
        });
        let reply = receive_reply!(self, seq, ListComponentsResponse);

        Ok(reply)
    }
}