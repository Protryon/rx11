use super::*;

use crate::coding::xkb::{
    GetIndicatorMapRequest, GetIndicatorStateRequest, GetIndicatorStateResponse, GetNamedIndicatorRequest, GetNamedIndicatorResponse, SetIndicatorMapRequest,
    SetNamedIndicatorRequest,
};
pub use crate::coding::xkb::{GetIndicatorMapResponse, IMFlag, IMGroupsWhich, IMModsWhich, IndicatorMap, LedClass, SetOfGroup};

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
        let reply = send_request_xkb!(
            self,
            XKBOpcode::GetIndicatorState,
            GetIndicatorStateResponse,
            GetIndicatorStateRequest {
                device_spec: device.into(),
            }
        )
        .into_inner();

        Ok(reply.state)
    }

    pub async fn xkb_get_indicator_map(&self, device: DeviceSpec, which: u32) -> Result<GetIndicatorMapResponse> {
        let reply = send_request_xkb!(
            self,
            XKBOpcode::GetIndicatorMap,
            GetIndicatorMapResponse,
            GetIndicatorMapRequest {
                device_spec: device.into(),
                which: which,
            }
        )
        .into_inner();

        Ok(reply)
    }

    pub async fn xkb_set_indicator_map(&self, device: DeviceSpec, which: u32, maps: Vec<IndicatorMap>) -> Result<()> {
        ensure!(which.count_ones() as usize == maps.len(), "which bits does not match maps.len()");

        send_request_xkb!(
            self,
            XKBOpcode::SetIndicatorMap,
            SetIndicatorMapRequest {
                device_spec: device.into(),
                which: which,
                maps: maps,
            }
        );

        Ok(())
    }

    pub async fn xkb_get_named_indicator(&self, device: DeviceSpec, led_class: LedClass, led_id: ID, indicator: Atom) -> Result<NamedIndicator> {
        let reply = send_request_xkb!(
            self,
            XKBOpcode::GetNamedIndicator,
            GetNamedIndicatorResponse,
            GetNamedIndicatorRequest {
                device_spec: device.into(),
                led_class: led_class,
                led_id: led_id,
                indicator_atom: indicator.handle,
            }
        )
        .into_inner();

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
        send_request_xkb!(
            self,
            XKBOpcode::SetNamedIndicator,
            SetNamedIndicatorRequest {
                device_spec: device.into(),
                led_class: led_class,
                led_id: led_id,
                indicator_atom: indicator.handle,
                set_state: on.is_some(),
                on: on.unwrap_or(false),
                set_map: map.is_some(),
                map: map.unwrap_or_default(),
            }
        );

        Ok(())
    }
}
