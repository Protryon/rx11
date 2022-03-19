use std::fmt;

use crate::coding::xinput2::{XIQueryDeviceRequest, XIQueryDeviceResponse, DeviceType as XIDeviceType, DeviceId, XISetFocusRequest, XIGetFocusRequest, XIGetFocusResponse, XIGrabDeviceResponse, XIGrabDeviceRequest, XIUngrabDeviceRequest, XIAllowEventsRequest, EventMode, XIPassiveGrabDeviceResponse, XIPassiveGrabDeviceRequest, GrabType, XIPassiveUngrabDeviceRequest, XIChangeHierarchyRequest, self, HierarchyChangeType, HierarchyChangeData, ChangeMode, XISetClientPointerRequest};
pub use crate::coding::xinput2::{
    ValuatorMode,
    ScrollType,
    ScrollFlags,
    TouchMode,
    GrabMode,
    GrabMode22,
    XIEventMask,
    GrabStatus,
    GrabModifierInfo,
};

use super::*;

#[derive(Clone, Copy)]
pub struct Device<'a> {
    pub(crate) id: DeviceId,
    pub(crate) connection: &'a X11Connection,
}

impl<'a> fmt::Debug for Device<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Device").field("id", &self.id).finish()
    }
}

impl X11Connection {
    pub fn all_xi_devices(&self) -> Device<'_> {
        Device {
            id: DeviceId::All,
            connection: self,
        }
    }

    pub fn all_xi_master_devices(&self) -> Device<'_> {
        Device {
            id: DeviceId::AllMaster,
            connection: self,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DeviceType<'a> {
    FloatingSlave,
    SlavePointer {
        master: Device<'a>,
    },
    SlaveKeyboard {
        master: Device<'a>,
    },
    MasterPointer {
        paired_keyboard: Device<'a>,
    },
    MasterKeyboard {
        paired_pointer: Device<'a>,
    },
}

impl<'a> DeviceType<'a> {
    pub(crate) fn from_attachment(type_: XIDeviceType, attachment: Device<'a>) -> Self {
        match type_ {
            XIDeviceType::MasterPointer => DeviceType::MasterPointer { paired_keyboard: attachment },
            XIDeviceType::MasterKeyboard => DeviceType::MasterKeyboard { paired_pointer: attachment },
            XIDeviceType::SlavePointer => DeviceType::SlavePointer { master: attachment },
            XIDeviceType::SlaveKeyboard => DeviceType::SlaveKeyboard { master: attachment },
            XIDeviceType::FloatingSlave => DeviceType::FloatingSlave,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DeviceData<'a> {
    pub device: Device<'a>,
    pub name: String,
    pub enabled: bool,
    pub device_type: DeviceType<'a>,
    pub class: DeviceClass,
}

#[derive(Clone, Copy, Debug)]
pub enum AllowEventMode<'a> {
    AsyncDevice,
    SyncDevice,
    ReplayDevice,
    AsyncPairedDevice,
    AsyncPair,
    SyncPair,
    AcceptTouch {
        id: TouchId,
        grab_window: Window<'a>,
    },
    RejectTouch {
        id: TouchId,
        grab_window: Window<'a>,
    },
}

#[derive(Clone, Debug)]
pub enum PassiveGrab<'a> {
    Button {
        cursor: Option<Cursor<'a>>,
        mode: GrabMode22,
        button: u32,
    },
    Keycode {
        mode: GrabMode22,
        keycode: u32,
    },
    Enter {
        mode: GrabMode22,
    },
    FocusIn {
        mode: GrabMode22,
    },
    TouchBegin,
}

#[derive(Clone, Debug)]
pub enum PassiveUngrab {
    Button {
        button: u32,
    },
    Keycode {
        keycode: u32,
    },
    Enter,
    FocusIn,
    TouchBegin,
}

impl<'a> Device<'a> {

    pub async fn query(self) -> Result<Vec<DeviceData<'a>>> {
        let seq = send_request_xinput!(self.connection, XIOpcode::XIQueryDevice, false, XIQueryDeviceRequest {
            device: self.id,
        });
        let reply = receive_reply!(self.connection, seq, XIQueryDeviceResponse);

        let mut out = vec![];
        for device in reply.infos {
            let class = DeviceClass::parse(&self.connection, device.classes).await?;

            let attachment_device = Device {
                id: device.attachment_device,
                connection: self.connection,
            };

            out.push(DeviceData {
                device: Device {
                    id: device.device,
                    connection: self.connection,
                },
                name: device.name,
                enabled: device.enabled,
                device_type: DeviceType::from_attachment(device.type_, attachment_device),
                class,
            });
        }
        Ok(out)
    }

    pub async fn set_focus(self, window: Option<Window<'_>>, time: Timestamp) -> Result<()> {
        send_request_xinput!(self.connection, XIOpcode::XISetFocus, true, XISetFocusRequest {
            device: self.id,
            window: window.map(|x| x.handle).unwrap_or(0),
            time: time.0,
        });

        Ok(())
    }

    pub async fn get_focus(self) -> Result<Window<'a>> {
        let seq = send_request_xinput!(self.connection, XIOpcode::XISetFocus, false, XIGetFocusRequest {
            device: self.id,
        });
        let reply = receive_reply!(self.connection, seq, XIGetFocusResponse);

        Ok(Window {
            handle: reply.focus_window,
            connection: self.connection,
        })
    }

    pub async fn grab(
        self,
        window: Window<'_>,
        time: Timestamp,
        cursor: Option<Cursor<'_>>,
        mode: GrabMode,
        paired_device_mode: GrabMode,
        owner_events: bool,
        mask: XIEventMask,
    ) -> Result<GrabStatus> {
        let seq = send_request_xinput!(self.connection, XIOpcode::XIGrabDevice, false, XIGrabDeviceRequest {
            window: window.handle,
            time: time.0,
            cursor: cursor.map(|x| x.handle).unwrap_or(0),
            device: self.id,
            mode: mode,
            paired_device_mode: paired_device_mode,
            owner_events: owner_events,
            mask: mask,
        });
        let reply = receive_reply!(self.connection, seq, XIGrabDeviceResponse);

        Ok(reply.status)
    }

    pub async fn ungrab(
        self,
    ) -> Result<()> {
        send_request_xinput!(self.connection, XIOpcode::XIUngrabDevice, true, XIUngrabDeviceRequest {
            device: self.id,
        });

        Ok(())
    }

    pub async fn allow_events(
        self,
        time: Timestamp,
        event_mode: AllowEventMode<'_>,
    ) -> Result<()> {
        send_request_xinput!(self.connection, XIOpcode::XIAllowEvents, true, XIAllowEventsRequest {
            time: time.0,
            device: self.id,
            event_mode: match event_mode {
                AllowEventMode::AsyncDevice => EventMode::AsyncDevice,
                AllowEventMode::SyncDevice => EventMode::SyncDevice,
                AllowEventMode::ReplayDevice => EventMode::ReplayDevice,
                AllowEventMode::AsyncPairedDevice => EventMode::AsyncPairedDevice,
                AllowEventMode::AsyncPair => EventMode::AsyncPair,
                AllowEventMode::SyncPair => EventMode::SyncPair,
                AllowEventMode::AcceptTouch { .. } => EventMode::AcceptTouch,
                AllowEventMode::RejectTouch { .. } => EventMode::RejectTouch,
            },
            touch_id: match event_mode {
                AllowEventMode::AcceptTouch { id, .. } => id.0,
                AllowEventMode::RejectTouch { id, .. } => id.0,
                _ => 0,
            },
            grab_window: match event_mode {
                AllowEventMode::AcceptTouch { grab_window, .. } => grab_window.handle,
                AllowEventMode::RejectTouch { grab_window, .. } => grab_window.handle,
                _ => 0,
            },
        });

        Ok(())
    }

    pub async fn passive_grab(
        self,
        window: Window<'_>,
        grab: PassiveGrab<'_>,
        paired_device_mode: GrabMode,
        owner_events: bool,
        mask: XIEventMask,
        modifiers: impl AsRef<[u32]>,
    ) -> Result<Vec<GrabModifierInfo>> {
        let seq = send_request_xinput!(self.connection, XIOpcode::XIPassiveGrabDevice, false, XIPassiveGrabDeviceRequest {
            grab_window: window.handle,
            time: 0,
            cursor: match &grab {
                PassiveGrab::Button { cursor, .. } => cursor.map(|x| x.handle).unwrap_or(0),
                _ => 0,
            },
            device: self.id,
            grab_type: match &grab {
                PassiveGrab::Button { .. } => GrabType::Button,
                PassiveGrab::Keycode { .. } => GrabType::Keycode,
                PassiveGrab::Enter { .. } => GrabType::Enter,
                PassiveGrab::FocusIn { .. } => GrabType::FocusIn,
                PassiveGrab::TouchBegin => GrabType::TouchBegin,
            },
            grab_mode: match &grab {
                PassiveGrab::Button { mode, .. } => *mode,
                PassiveGrab::Keycode { mode, .. } => *mode,
                PassiveGrab::Enter { mode, .. } => *mode,
                PassiveGrab::FocusIn { mode, .. } => *mode,
                PassiveGrab::TouchBegin => GrabMode22::Touch,
            },
            paired_device_mode: paired_device_mode,
            owner_events: owner_events,
            mask: mask,
            detail: match &grab {
                PassiveGrab::Button { button, .. } => *button,
                PassiveGrab::Keycode { keycode, .. } => *keycode,
                _ => 0,
            },
            modifiers: modifiers.as_ref().to_vec(),
        });
        let reply = receive_reply!(self.connection, seq, XIPassiveGrabDeviceResponse);

        Ok(reply.modifiers)
    }

    pub async fn passive_ungrab(
        self,
        window: Window<'_>,
        grab: PassiveUngrab,
        modifiers: impl AsRef<[u32]>,
    ) -> Result<()> {
        send_request_xinput!(self.connection, XIOpcode::XIPassiveUngrabDevice, true, XIPassiveUngrabDeviceRequest {
            grab_window: window.handle,
            detail: match &grab {
                PassiveUngrab::Button { button } => *button,
                PassiveUngrab::Keycode { keycode } => *keycode,
                _ => 0,
            },
            device: self.id,
            grab_type: match &grab {
                PassiveUngrab::Button { .. } => GrabType::Button,
                PassiveUngrab::Keycode { .. } => GrabType::Keycode,
                PassiveUngrab::Enter => GrabType::Enter,
                PassiveUngrab::FocusIn => GrabType::FocusIn,
                PassiveUngrab::TouchBegin => GrabType::TouchBegin,
            },
            modifiers: modifiers.as_ref().to_vec(),
        });

        Ok(())
    }

    pub async fn set_as_client_pointer(self) -> Result<()> {
        send_request_xinput!(self.connection, XIOpcode::XISetClientPointer, true, XISetClientPointerRequest {
            device: self.id,
            window: 0,
        });
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum HierarchyChange<'a> {
    AddMaster {
        send_core: bool,
        enable: bool,
        name: String,
    },
    RemoveMasterAndFloat {
        device: Device<'a>,
    },
    RemoveMasterAndAttach {
        device: Device<'a>,
        new_pointer_master: Device<'a>,
        new_keyboard_master: Device<'a>,
    },
    AttachSlave {
        device: Device<'a>,
        master: Device<'a>,
    },
    DetachSlave {
        device: Device<'a>,
    },
}

impl X11Connection {
    pub async fn xi_change_hierarchy(
        &self,
        changes: impl IntoIterator<Item=HierarchyChange<'_>>,
    ) -> Result<()> {
        send_request_xinput!(self, XIOpcode::XIChangeHierarchy, true, XIChangeHierarchyRequest {
            changes: changes.into_iter().map(|change| match change {
                HierarchyChange::AddMaster { send_core, enable, name } => xinput2::HierarchyChange {
                    type_: HierarchyChangeType::AddMaster,
                    len: 0,
                    data: HierarchyChangeData::AddMaster {
                        name_len: 0,
                        send_core,
                        enable,
                        name,
                    },
                },
                HierarchyChange::RemoveMasterAndFloat { device } => xinput2::HierarchyChange {
                    type_: HierarchyChangeType::RemoveMaster,
                    len: 0,
                    data: HierarchyChangeData::RemoveMaster {
                        device: device.id,
                        return_mode: ChangeMode::Float,
                        return_pointer_device: DeviceId::All,
                        return_keyboard_device: DeviceId::All,
                    },
                },
                HierarchyChange::RemoveMasterAndAttach { device, new_pointer_master, new_keyboard_master } => xinput2::HierarchyChange {
                    type_: HierarchyChangeType::RemoveMaster,
                    len: 0,
                    data: HierarchyChangeData::RemoveMaster {
                        device: device.id,
                        return_mode: ChangeMode::Attach,
                        return_pointer_device: new_pointer_master.id,
                        return_keyboard_device: new_keyboard_master.id,
                    },
                },
                HierarchyChange::AttachSlave { device, master } => xinput2::HierarchyChange {
                    type_: HierarchyChangeType::AttachSlave,
                    len: 0,
                    data: HierarchyChangeData::AttachSlave {
                        device: device.id,
                        master_device: master.id,
                    },
                },
                HierarchyChange::DetachSlave { device } => xinput2::HierarchyChange {
                    type_: HierarchyChangeType::DetachSlave,
                    len: 0,
                    data: HierarchyChangeData::DetachSlave {
                        device: device.id,
                    },
                },
            }).collect(),
        });

        Ok(())
    }
}