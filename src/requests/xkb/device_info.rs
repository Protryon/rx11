use std::collections::BTreeMap;

use crate::coding::xkb::{GetDeviceInfoRequest, GetDeviceInfoResponse};

use super::*;

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub present: XIFeature,
    pub supported: XIFeature,
    pub unsupported: XIFeature,
    pub first_button_wanted: u8,
    pub first_button_returned: u8,
    pub total_buttons: u8,
    pub has_own_state: bool,
    pub default_keyboard_fb: ID,
    pub default_led_fb: ID,
    pub dev_type: Atom,
    pub name: String,
    pub button_actions: Vec<SymAction>,
    pub leds: Vec<DeviceLedInfo>,
}

#[derive(Clone, Debug)]
pub struct DeviceLedInfo {
    pub led_class: LedClass,
    pub led_id: ID,
    pub values: BTreeMap<u32, LedInfo>,
}

#[derive(Clone, Debug)]
pub struct LedInfo {
    pub is_phys: bool,
    pub state: bool,
    pub name: Option<Atom>,
    pub map: Option<IndicatorMap>,
}

impl X11Connection {
    /// buttons is Some(start, length) or None for all
    pub async fn xkb_get_device_info(&self, device: DeviceSpec, wanted: XIFeature, buttons: Option<(u8, u8)>, led_class: LedClass, led_id: ID) -> Result<DeviceInfo> {
        let seq = send_request_xkb!(self, XKBOpcode::GetDeviceInfo, false, GetDeviceInfoRequest {
            device_spec: device.into(),
            wanted: wanted,
            all_buttons: buttons.is_none(),
            first_button: buttons.map(|x| x.0).unwrap_or(0),
            num_buttons: buttons.map(|x| x.1).unwrap_or(0),
            led_class: led_class,
            led_id: led_id,
        });
        let reply = receive_reply!(self, seq, GetDeviceInfoResponse);

        Ok(DeviceInfo {
            present: reply.present,
            supported: reply.supported,
            unsupported: reply.unsupported,
            first_button_wanted: reply.first_button_wanted,
            first_button_returned: reply.first_button,
            total_buttons: reply.total_buttons,
            has_own_state: reply.has_own_state,
            default_keyboard_fb: reply.default_keyboard_fb,
            default_led_fb: reply.default_led_fb,
            dev_type: self.get_atom_name(reply.dev_type_atom).await?,
            name: reply.name,
            button_actions: reply.button_actions,
            leds: {
                let mut out = vec![];
                for led in reply.leds {
                    out.push(DeviceLedInfo {
                        led_class: led.led_class,
                        led_id: led.led_id,
                        values: {
                            let mut out = BTreeMap::new();
                            let mut name_atoms = led.name_atoms.into_iter();
                            let mut maps = led.maps.into_iter();

                            for bit in 0..31 {
                                let is_phys = led.phys_indicators & (1 << bit) != 0;
                                let state = led.state & (1 << bit) != 0;
                                let name = if led.names_present & (1 << bit) != 0 {
                                    Some(self.get_atom_name(name_atoms.next().ok_or_else(|| anyhow!("missing indicator name"))?).await?)
                                } else {
                                    None
                                };
                                let map = if led.maps_present & (1 << bit) != 0 {
                                    Some(maps.next().ok_or_else(|| anyhow!("missing indicator map"))?)
                                } else {
                                    None
                                };
                                if is_phys || state || name.is_some() || map.is_some() {
                                    out.insert(bit, LedInfo {
                                        is_phys,
                                        state,
                                        name,
                                        map,
                                    });
                                }
                            }
            
                            out
                        }
                    });
                }
                out
            },
        })
    }

    //todo: pub async fn xkb_set_device_info(&self, device: DeviceSpec) -> Result<()> {
}