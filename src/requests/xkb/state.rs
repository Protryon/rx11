use super::*;

use crate::coding::xkb::{GetStateRequest, LatchLockStateRequest};
pub use crate::coding::xkb::{GetStateResponse, Group, Keybutmask, ModMask};

impl Affectable for ModMask {
    const FULL: Self = Self::ALL;
}

impl X11Connection {
    pub async fn xkb_get_state(&self, device: DeviceSpec) -> Result<GetStateResponse> {
        let reply = send_request_xkb!(
            self,
            XKBOpcode::GetState,
            GetStateResponse,
            GetStateRequest {
                device_spec: device.into(),
            }
        )
        .into_inner();

        Ok(reply)
    }

    pub async fn xkb_latch_lock_state(
        &self,
        device: DeviceSpec,
        lock_mods: impl Into<Affect<ModMask>>,
        lock_group: Option<Group>,
        //todo: investigate xcb different encoding behavior
        latch_mods: impl Into<Affect<ModMask>>,
        //todo: should this be u16?
        latch_group: Option<u16>,
    ) -> Result<()> {
        let lock_mods = lock_mods.into();
        let latch_mods = latch_mods.into();
        send_request_xkb!(
            self,
            XKBOpcode::LatchLockState,
            LatchLockStateRequest {
                device_spec: device.into(),
                affect_mod_locks: lock_mods.affect,
                mod_locks: lock_mods.value,
                lock_group: lock_group.is_some(),
                group_lock: lock_group.unwrap_or(Group::One),
                affect_mod_latches: latch_mods.affect,
                mod_latches: latch_mods.value,
                latch_group: latch_group.is_some(),
                group_latch: latch_group.unwrap_or(0),
            }
        );

        Ok(())
    }
}
