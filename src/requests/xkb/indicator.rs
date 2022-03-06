use super::*;

use crate::coding::xkb::{GetIndicatorStateRequest, GetIndicatorStateResponse, GetIndicatorMapRequest, SetIndicatorMapRequest, GetNamedIndicatorRequest, SetNamedIndicatorRequest, GetNamedIndicatorResponse};
pub use crate::coding::xkb::{
    IndicatorMap,
    GetIndicatorMapResponse,
    IMFlag,
    IMGroupsWhich,
    SetOfGroup,
    IMModsWhich,
    LedClass,
};

#[derive(Clone, Debug)]
pub struct NamedIndicator {
    pub indicator: Atom,
    pub found: bool,
    pub on: bool,
    pub real: bool,
    pub index: u8,
    pub map: IndicatorMap,
    pub supported: bool,
}

impl X11Connection {
    pub async fn xkb_get_indicator_state(&self, device: DeviceSpec) -> Result<u32> {
        let seq = send_request_xkb!(self, XKBOpcode::GetIndicatorState, false, GetIndicatorStateRequest {
            device_spec: device.into(),
        });
        let reply = receive_reply!(self, seq, GetIndicatorStateResponse);

        Ok(reply.state)
    }

    pub async fn xkb_get_indicator_map(&self, device: DeviceSpec, which: u32) -> Result<GetIndicatorMapResponse> {
        let seq = send_request_xkb!(self, XKBOpcode::GetIndicatorMap, false, GetIndicatorMapRequest {
            device_spec: device.into(),
            which: which,
        });
        let reply = receive_reply!(self, seq, GetIndicatorMapResponse);

        Ok(reply)
    }

    pub async fn xkb_set_indicator_map(&self, device: DeviceSpec, which: u32, maps: Vec<IndicatorMap>) -> Result<()> {
        ensure!(which.count_ones() as usize == maps.len(), "which bits does not match maps.len()");

        send_request_xkb!(self, XKBOpcode::SetIndicatorMap, true, SetIndicatorMapRequest {
            device_spec: device.into(),
            which: which,
            maps: maps,
        });

        Ok(())
    }

    pub async fn xkb_get_named_indicator(&self, device: DeviceSpec, led_class: LedClass, led_id: ID, indicator: Atom) -> Result<NamedIndicator> {
        let seq = send_request_xkb!(self, XKBOpcode::GetNamedIndicator, false, GetNamedIndicatorRequest {
            device_spec: device.into(),
            led_class: led_class,
            led_id: led_id,
            indicator_atom: indicator.handle,
        });
        let reply = receive_reply!(self, seq, GetNamedIndicatorResponse);

        Ok(NamedIndicator {
            indicator: self.get_atom_name(reply.indicator_atom).await?,
            found: reply.found,
            on: reply.on,
            real: reply.real_indicator,
            index: reply.index,
            map: reply.map,
            supported: reply.supported,
        })
    }

    pub async fn xkb_set_named_indicator(
        &self,
        device: DeviceSpec,
        led_class: LedClass,
        led_id: ID,
        indicator: Atom,
        on: Option<bool>,
        map: Option<IndicatorMap>,
    ) -> Result<()> {
        send_request_xkb!(self, XKBOpcode::SetNamedIndicator, true, SetNamedIndicatorRequest {
            device_spec: device.into(),
            led_class: led_class,
            led_id: led_id,
            indicator_atom: indicator.handle,
            set_state: on.is_some(),
            on: on.unwrap_or(false),
            set_map: map.is_some(),
            map: map.unwrap_or_default(),
        });

        Ok(())
    }
}