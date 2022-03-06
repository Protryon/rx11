use super::*;

use crate::coding::xkb::{GetKbdByNameRequest, GetKbdByNameResponse, String8};
pub use crate::coding::xkb::{
    GBNDetail,
};

#[derive(Clone, Debug)]
pub struct Keyboard {
    pub min_keycode: u8,
    pub max_keycode: u8,
    pub loaded: bool,
    pub new_keyboard: bool,
    pub found: GBNDetail,
    pub types_map: Option<GetMapResponse>,
    pub compat_map: Option<GetCompatMapResponse>,
    pub indicator_maps: Option<GetIndicatorMapResponse>,
    pub key_names: Option<Names>,
    pub geometry: Option<GeometryData>,
}

impl X11Connection {
    pub async fn xkb_get_keyboard_by_name(
        &self,
        device: DeviceSpec,
        need: GBNDetail,
        want: GBNDetail,
        load: bool,
        keymaps_spec: impl AsRef<str>,
        keycodes_spec: impl AsRef<str>,
        types_spec: impl AsRef<str>,
        compat_map_spec: impl AsRef<str>,
        symbols_spec: impl AsRef<str>,
        geometry_spec: impl AsRef<str>,
    ) -> Result<Keyboard> {
        let seq = send_request_xkb!(self, XKBOpcode::GetKbdByName, false, GetKbdByNameRequest {
            device_spec: device.into(),
            need: need,
            want: want,
            load: load,
            keymaps_spec: String8 { string: keymaps_spec.as_ref().to_string(), len: 0 },
            keycodes_spec: String8 { string: keycodes_spec.as_ref().to_string(), len: 0 },
            types_spec: String8 { string: types_spec.as_ref().to_string(), len: 0 },
            compat_map_spec: String8 { string: compat_map_spec.as_ref().to_string(), len: 0 },
            symbols_spec: String8 { string: symbols_spec.as_ref().to_string(), len: 0 },
            geometry_spec: String8 { string: geometry_spec.as_ref().to_string(), len: 0 },
        });
        let reply = receive_reply!(self, seq, GetKbdByNameResponse);

        Ok(Keyboard {
            min_keycode: reply.min_keycode,
            max_keycode: reply.max_keycode,
            loaded: reply.loaded,
            new_keyboard: reply.new_keyboard,
            found: reply.found,
            types_map: reply.types,
            compat_map: reply.compat_map,
            indicator_maps: reply.indicator_maps,
            key_names: match reply.key_names {
                Some(names) => Some(self.xkb_parse_names(names).await?),
                None => None,
            },
            geometry: match reply.geometry {
                Some(names) => Some(self.xkb_parse_geometry(names).await?),
                None => None,
            },
        })
    }
}