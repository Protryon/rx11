use super::*;

pub use crate::coding::xkb::{
    Action as SymAction, ActionMessageFlag, GetCompatMapResponse, IsoLockFlag, IsoLockNoAffect,
    LockDeviceFlags, ModDef, MovePointerFlag, SAControls, SAGroup, SAMods, SAType, SetOfGroup,
    SetPointerDefaultFlag, SwitchScreenFlag, SymInterpret, SymInterpretMatch, VModsLow,
    ValuatorWhat,
};
use crate::coding::xkb::{GetCompatMapRequest, SetCompatMapRequest};

#[derive(Debug, Clone)]
pub struct CompatMapGroups {
    group_1: Option<ModDef>,
    group_2: Option<ModDef>,
    group_3: Option<ModDef>,
    group_4: Option<ModDef>,
}

impl CompatMapGroups {
    fn set(&self) -> SetOfGroup {
        let mut out = SetOfGroup::ZERO;
        if self.group_1.is_some() {
            out |= SetOfGroup::GROUP1;
        }
        if self.group_2.is_some() {
            out |= SetOfGroup::GROUP2;
        }
        if self.group_3.is_some() {
            out |= SetOfGroup::GROUP3;
        }
        if self.group_4.is_some() {
            out |= SetOfGroup::GROUP4;
        }
        out
    }

    fn maps(self) -> Vec<ModDef> {
        let mut out = vec![];
        if let Some(group) = self.group_1 {
            out.push(group);
        }
        if let Some(group) = self.group_2 {
            out.push(group);
        }
        if let Some(group) = self.group_3 {
            out.push(group);
        }
        if let Some(group) = self.group_4 {
            out.push(group);
        }
        out
    }
}

impl X11Connection {
    /// range is Some(start, length)
    pub async fn xkb_get_compat_map(
        &self,
        device: DeviceSpec,
        groups: SetOfGroup,
        range: Option<(u16, u16)>,
    ) -> Result<GetCompatMapResponse> {
        let seq = send_request_xkb!(
            self,
            XKBOpcode::GetCompatMap,
            false,
            GetCompatMapRequest {
                device_spec: device.into(),
                groups: groups,
                get_all_si: range.is_none(),
                first_si: range.map(|x| x.0).unwrap_or(0),
                num_si: range.map(|x| x.1).unwrap_or(0),
            }
        );
        let reply = receive_reply!(self, seq, GetCompatMapResponse);

        Ok(reply)
    }

    pub async fn xkb_set_compat_map(
        &self,
        device: DeviceSpec,
        groups: CompatMapGroups,
        start: u16,
        truncate: bool,
        recompute_actions: bool,
        si: Vec<SymInterpret>,
    ) -> Result<()> {
        send_request_xkb!(
            self,
            XKBOpcode::SetCompatMap,
            false,
            SetCompatMapRequest {
                device_spec: device.into(),
                recompute_actions: recompute_actions,
                truncate_si: truncate,
                groups: groups.set(),
                first_si: start,
                num_si: si.len().try_into()?,
                si: si,
                group_maps: groups.maps(),
            }
        );

        Ok(())
    }
}
