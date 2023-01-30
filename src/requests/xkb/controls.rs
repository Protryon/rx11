use derive_builder::Builder;

use super::*;

pub use crate::coding::xkb::{AXOption, BoolCtrl, GetControlsResponse, MouseKeys, VMod};
use crate::coding::xkb::{GetControlsRequest, SetControlsRequest};

impl Affectable for VMod {
    const FULL: Self = Self::ALL;
}

impl Affectable for BoolCtrl {
    const FULL: Self = Self::ALL;
}

#[derive(Builder, Clone, Debug)]
pub struct SetControls {
    #[builder(setter(into))]
    pub internal_real_mods: Affect<ModMask>,
    #[builder(setter(into))]
    pub ignore_lock_real_mods: Affect<ModMask>,
    #[builder(setter(into))]
    pub internal_virtual_mods: Affect<VMod>,
    #[builder(setter(into))]
    pub ignore_lock_virtual_mods: Affect<VMod>,
    pub mouse_keys_default_button: u8,
    pub groups_wrap: u8,
    pub access_x_option: AXOption,
    #[builder(setter(into))]
    pub enabled_controls: Affect<BoolCtrl>,
    pub change_controls: Control,
    pub repeat_delay: u16,
    pub repeat_interval: u16,
    pub slow_keys_delay: u16,
    pub debounce_delay: u16,
    pub mouse_keys: MouseKeys,
    pub access_x_timeout: u16,
    pub access_x_timeout_mask: AXOption,
    pub access_x_timeout_values: AXOption,
    pub access_x_timeout_options_mask: AXOption,
    pub access_x_timeout_options_values: AXOption,
    pub per_key_repeat: [u8; 32],
}

impl X11Connection {
    pub async fn xkb_get_controls(&self, device: DeviceSpec) -> Result<GetControlsResponse> {
        let reply = send_request_xkb!(
            self,
            XKBOpcode::GetControls,
            GetControlsResponse,
            GetControlsRequest {
                device_spec: device.into(),
            }
        )
        .into_inner();

        Ok(reply)
    }

    pub async fn xkb_set_controls(&self, device: DeviceSpec, controls: SetControls) -> Result<()> {
        send_request_xkb!(
            self,
            XKBOpcode::SetControls,
            SetControlsRequest {
                device_spec: device.into(),
                affect_internal_real_mods: controls.internal_real_mods.affect,
                internal_real_mods: controls.internal_real_mods.value,
                affect_ignore_lock_real_mods: controls.ignore_lock_real_mods.affect,
                ignore_lock_real_mods: controls.ignore_lock_real_mods.value,
                affect_internal_virtual_mods: controls.internal_virtual_mods.affect,
                internal_virtual_mods: controls.internal_virtual_mods.value,
                affect_ignore_lock_virtual_mods: controls.ignore_lock_virtual_mods.affect,
                ignore_lock_virtual_mods: controls.ignore_lock_virtual_mods.value,
                mouse_keys_default_button: controls.mouse_keys_default_button,
                groups_wrap: controls.groups_wrap,
                access_x_option: controls.access_x_option,
                affect_enabled_controls: controls.enabled_controls.affect,
                enabled_controls: controls.enabled_controls.value,
                change_controls: controls.change_controls,
                repeat_delay: controls.repeat_delay,
                repeat_interval: controls.repeat_interval,
                slow_keys_delay: controls.slow_keys_delay,
                debounce_delay: controls.debounce_delay,
                mouse_keys: controls.mouse_keys,
                access_x_timeout: controls.access_x_timeout,
                access_x_timeout_mask: controls.access_x_timeout_mask,
                access_x_timeout_values: controls.access_x_timeout_values,
                access_x_timeout_options_mask: controls.access_x_timeout_options_mask,
                access_x_timeout_options_values: controls.access_x_timeout_options_values,
                per_key_repeat: controls.per_key_repeat.to_vec(),
            }
        );

        Ok(())
    }
}
