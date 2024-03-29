use super::*;

use crate::coding::xkb::PerClientFlagsRequest;
pub use crate::coding::xkb::{PerClientFlag, PerClientFlagsResponse};

impl Affectable for PerClientFlag {
    const FULL: Self = Self::ALL;
}

impl X11Connection {
    pub async fn xkb_per_client_flags(
        &self,
        device: DeviceSpec,
        flags: impl Into<Affect<PerClientFlag>>,
        controls_to_change: BoolCtrl,
        auto_controls: BoolCtrl,
        auto_controls_values: BoolCtrl,
    ) -> Result<PerClientFlagsResponse> {
        let flags = flags.into();
        let reply = send_request_xkb!(
            self,
            XKBOpcode::PerClientFlags,
            PerClientFlagsResponse,
            PerClientFlagsRequest {
                device_spec: device.into(),
                change: flags.affect,
                value: flags.value,
                controls_to_change: controls_to_change,
                auto_controls: auto_controls,
                auto_controls_values: auto_controls_values,
            }
        )
        .into_inner();

        Ok(reply)
    }
}
