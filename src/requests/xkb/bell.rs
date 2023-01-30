use derive_builder::Builder;

use super::*;

use crate::coding::xkb::BellRequest;
pub use crate::coding::xkb::{BellClass, ID};

#[derive(Builder, Debug, Clone)]
pub struct XKBBell<'a> {
    pub class: BellClass,
    pub id: ID,
    pub percent: i8,
    #[builder(default)]
    pub force_sound: bool,
    #[builder(default)]
    pub event_only: bool,
    pub pitch: i16,
    pub duration: i16,
    pub name: Atom,
    pub window: Window<'a>,
}

impl X11Connection {
    pub async fn xkb_bell(&self, device: DeviceSpec, details: XKBBell<'_>) -> Result<()> {
        send_request_xkb!(
            self,
            XKBOpcode::Bell,
            BellRequest {
                device_spec: device.into(),
                bell_class: details.class,
                bell_id: details.id,
                percent: details.percent,
                force_sound: details.force_sound,
                event_only: details.event_only,
                pitch: details.pitch,
                duration: details.duration,
                name_atom: details.name.handle,
                window: details.window.handle,
            }
        );
        Ok(())
    }
}
