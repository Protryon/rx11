use derive_builder::Builder;

use super::*;

pub use crate::coding::xkb::{
    AXNDetail, CMDetail, Control, MapPart, NKNDetail, NameDetail, StatePart, XIFeature,
};
use crate::coding::xkb::{SelectEventDetails, SelectEventsRequest, XKBEventMask};

impl Affectable for NKNDetail {
    const FULL: Self = Self::ALL;
}

impl Affectable for MapPart {
    const FULL: Self = Self::ALL;
}

impl Affectable for StatePart {
    const FULL: Self = Self::ALL;
}

impl Affectable for Control {
    const FULL: Self = Self::ALL;
}

impl Affectable for u32 {
    const FULL: Self = u32::MAX;
}

impl Affectable for NameDetail {
    const FULL: Self = Self::ALL;
}

impl Affectable for CMDetail {
    const FULL: Self = Self::ALL;
}

impl Affectable for u8 {
    const FULL: Self = u8::MAX;
}

impl Affectable for AXNDetail {
    const FULL: Self = Self::ALL;
}

impl Affectable for XIFeature {
    const FULL: Self = Self::ALL;
}

impl<T: Affectable> From<T> for Affect<T> {
    fn from(from: T) -> Self {
        Self {
            affect: T::FULL,
            value: from,
        }
    }
}

#[derive(Builder, Default, Debug, Clone)]
#[builder(default)]
pub struct XKBEvents {
    #[builder(setter(into, strip_option), default)]
    pub new_keyboard_notify: Option<Affect<NKNDetail>>,
    #[builder(setter(into, strip_option), default)]
    pub map_notify: Option<Affect<MapPart>>,
    #[builder(setter(into, strip_option), default)]
    pub state_notify: Option<Affect<StatePart>>,
    #[builder(setter(into, strip_option), default)]
    pub controls_notify: Option<Affect<Control>>,
    #[builder(setter(into, strip_option), default)]
    pub indicator_map_notify: Option<Affect<u32>>,
    #[builder(setter(into, strip_option), default)]
    pub indicator_state_notify: Option<Affect<u32>>,
    #[builder(setter(into, strip_option), default)]
    pub names_notify: Option<Affect<NameDetail>>,
    #[builder(setter(into, strip_option), default)]
    pub compat_map_notify: Option<Affect<CMDetail>>,
    #[builder(setter(into, strip_option), default)]
    pub bell_notify: Option<Affect<u8>>,
    #[builder(setter(into, strip_option), default)]
    pub action_message: Option<Affect<u8>>,
    #[builder(setter(into, strip_option), default)]
    pub access_x_notify: Option<Affect<AXNDetail>>,
    #[builder(setter(into, strip_option), default)]
    pub extension_device_notify: Option<Affect<XIFeature>>,
}

impl X11Connection {
    pub async fn xkb_select_events(&self, device: DeviceSpec, events: XKBEvents) -> Result<()> {
        let mut affect_events = XKBEventMask::ZERO;
        if events.new_keyboard_notify.is_some() {
            affect_events |= XKBEventMask::NEW_KEYBOARD_NOTIFY;
        }
        // TODO: unclear if this bit needs to be set
        if events.map_notify.is_some() {
            affect_events |= XKBEventMask::MAP_NOTIFY;
        }
        if events.state_notify.is_some() {
            affect_events |= XKBEventMask::STATE_NOTIFY;
        }
        if events.controls_notify.is_some() {
            affect_events |= XKBEventMask::CONTROLS_NOTIFY;
        }
        if events.indicator_map_notify.is_some() {
            affect_events |= XKBEventMask::INDICATOR_MAP_NOTIFY;
        }
        if events.indicator_state_notify.is_some() {
            affect_events |= XKBEventMask::INDICATOR_STATE_NOTIFY;
        }
        if events.names_notify.is_some() {
            affect_events |= XKBEventMask::NAMES_NOTIFY;
        }
        if events.compat_map_notify.is_some() {
            affect_events |= XKBEventMask::COMPAT_MAP_NOTIFY;
        }
        if events.bell_notify.is_some() {
            affect_events |= XKBEventMask::BELL_NOTIFY;
        }
        if events.action_message.is_some() {
            affect_events |= XKBEventMask::ACTION_MESSAGE;
        }
        if events.access_x_notify.is_some() {
            affect_events |= XKBEventMask::ACCESS_X_NOTIFY;
        }
        if events.extension_device_notify.is_some() {
            affect_events |= XKBEventMask::EXTENSION_DEVICE_NOTIFY;
        }
        send_request_xkb!(
            self,
            XKBOpcode::SelectEvents,
            true,
            SelectEventsRequest {
                device_spec: device.into(),
                affect_which: affect_events,
                clear: XKBEventMask::ZERO,
                select_all: XKBEventMask::ZERO,
                affect_map: events.map_notify.map(|x| x.affect).unwrap_or(MapPart::ZERO),
                map: events.map_notify.map(|x| x.value).unwrap_or(MapPart::ZERO),
                details: SelectEventDetails {
                    affect_new_keyboard: events.new_keyboard_notify.map(|x| x.affect),
                    new_keyboard_details: events.new_keyboard_notify.map(|x| x.value),
                    affect_state: events.state_notify.map(|x| x.affect),
                    state_details: events.state_notify.map(|x| x.value),
                    affect_controls: events.controls_notify.map(|x| x.affect),
                    control_details: events.controls_notify.map(|x| x.value),
                    affect_indicator_map: events.indicator_map_notify.map(|x| x.affect),
                    indicator_map_details: events.indicator_map_notify.map(|x| x.value),
                    affect_indicator_state: events.indicator_state_notify.map(|x| x.affect),
                    indicator_state_details: events.indicator_state_notify.map(|x| x.value),
                    affect_names: events.names_notify.map(|x| x.affect),
                    names_details: events.names_notify.map(|x| x.value),
                    affect_compat: events.compat_map_notify.map(|x| x.affect),
                    compat_details: events.compat_map_notify.map(|x| x.value),
                    affect_bell: events.bell_notify.map(|x| x.affect),
                    bell_details: events.bell_notify.map(|x| x.value),
                    affect_msg_details: events.action_message.map(|x| x.affect),
                    msg_details: events.action_message.map(|x| x.value),
                    affect_access_x: events.access_x_notify.map(|x| x.affect),
                    access_x_details: events.access_x_notify.map(|x| x.value),
                    affect_extension_device: events.extension_device_notify.map(|x| x.affect),
                    extension_device_details: events.extension_device_notify.map(|x| x.value),
                },
            }
        );
        Ok(())
    }
}
