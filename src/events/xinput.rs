
use crate::{coding::xinput2::{XIEventData, ModifierInfo, GroupInfo, self, XIEventCode, DeviceId}, connection::X11Connection, requests::{Device, Timestamp, DeviceClass, Window, DeviceType, Atom, TouchId}};
use anyhow::Result;
use fixed::types::{I16F16, I32F32};
pub use crate::coding::xinput2::{
    ChangeReason,
    KeyEventFlags,
    PointerEventFlags,
    XINotifyMode,
    XINotifyDetail,
    HierarchyMask,
    PropertyFlag,
    TouchEventFlags,
    TouchOwnershipFlags,
    BarrierFlags,
};

#[derive(Clone, Debug)]
pub enum XIEvent<'a> {
    DeviceChanged(DeviceChangedEvent<'a>),
    KeyPress(KeyEvent<'a>),
    KeyRelease(KeyEvent<'a>),
    ButtonPress(ButtonEvent<'a>),
    ButtonRelease(ButtonEvent<'a>),
    Motion(ButtonEvent<'a>),
    Enter(TransitionEvent<'a>),
    Leave(TransitionEvent<'a>),
    FocusIn(TransitionEvent<'a>),
    FocusOut(TransitionEvent<'a>),
    Hierarchy(HierarchyEvent<'a>),
    Property(PropertyEvent<'a>),
    RawKeyPress(RawKeyEvent<'a>),
    RawKeyRelease(RawKeyEvent<'a>),
    RawButtonPress(RawButtonEvent<'a>),
    RawButtonRelease(RawButtonEvent<'a>),
    RawMotion(RawButtonEvent<'a>),
    TouchBegin(TouchEvent<'a>),
    TouchUpdate(TouchEvent<'a>),
    TouchEnd(TouchEvent<'a>),
    TouchOwnership(TouchOwnershipEvent<'a>),
    RawTouchBegin(RawTouchEvent<'a>),
    RawTouchUpdate(RawTouchEvent<'a>),
    RawTouchEnd(RawTouchEvent<'a>),
    BarrierHit(BarrierEvent<'a>),
    BarrierLeave(BarrierEvent<'a>),
}

impl<'a> XIEvent<'a> {
    pub(crate) async fn from_protocol(connection: &'a X11Connection, code: u16, from: Vec<u8>) -> Result<XIEvent<'a>> {
        let xkb_event = XIEventData::decode_sync(&mut &from[..], XIEventCode::from_repr(code)?)?;
        Ok(match xkb_event {
            XIEventData::DeviceChanged(e) => XIEvent::DeviceChanged(DeviceChangedEvent::from_protocol(connection, e).await?),
            XIEventData::KeyPress(e) => XIEvent::KeyPress(KeyEvent::from_protocol(connection, e)),
            XIEventData::KeyRelease(e) => XIEvent::KeyRelease(KeyEvent::from_protocol(connection, e)),
            XIEventData::ButtonPress(e) => XIEvent::ButtonPress(ButtonEvent::from_protocol(connection, e)),
            XIEventData::ButtonRelease(e) => XIEvent::ButtonRelease(ButtonEvent::from_protocol(connection, e)),
            XIEventData::Motion(e) => XIEvent::Motion(ButtonEvent::from_protocol(connection, e)),
            XIEventData::Enter(e) => XIEvent::Enter(TransitionEvent::from_protocol(connection, e)),
            XIEventData::Leave(e) => XIEvent::Leave(TransitionEvent::from_protocol(connection, e)),
            XIEventData::FocusIn(e) => XIEvent::FocusIn(TransitionEvent::from_protocol(connection, e)),
            XIEventData::FocusOut(e) => XIEvent::FocusOut(TransitionEvent::from_protocol(connection, e)),
            XIEventData::Hierarchy(e) => XIEvent::Hierarchy(HierarchyEvent::from_protocol(connection, e)),
            XIEventData::Property(e) => XIEvent::Property(PropertyEvent::from_protocol(connection, e).await?),
            XIEventData::RawKeyPress(e) => XIEvent::RawKeyPress(RawKeyEvent::from_protocol(connection, e)),
            XIEventData::RawKeyRelease(e) => XIEvent::RawKeyRelease(RawKeyEvent::from_protocol(connection, e)),
            XIEventData::RawButtonPress(e) => XIEvent::RawButtonPress(RawButtonEvent::from_protocol(connection, e)),
            XIEventData::RawButtonRelease(e) => XIEvent::RawButtonRelease(RawButtonEvent::from_protocol(connection, e)),
            XIEventData::RawMotion(e) => XIEvent::RawMotion(RawButtonEvent::from_protocol(connection, e)),
            XIEventData::TouchBegin(e) => XIEvent::TouchBegin(TouchEvent::from_protocol(connection, e)),
            XIEventData::TouchUpdate(e) => XIEvent::TouchUpdate(TouchEvent::from_protocol(connection, e)),
            XIEventData::TouchEnd(e) => XIEvent::TouchEnd(TouchEvent::from_protocol(connection, e)),
            XIEventData::TouchOwnership(e) => XIEvent::TouchOwnership(TouchOwnershipEvent::from_protocol(connection, e)),
            XIEventData::RawTouchBegin(e) => XIEvent::RawTouchBegin(RawTouchEvent::from_protocol(connection, e)),
            XIEventData::RawTouchUpdate(e) => XIEvent::RawTouchUpdate(RawTouchEvent::from_protocol(connection, e)),
            XIEventData::RawTouchEnd(e) => XIEvent::RawTouchEnd(RawTouchEvent::from_protocol(connection, e)),
            XIEventData::BarrierHit(e) => XIEvent::BarrierHit(BarrierEvent::from_protocol(connection, e)),
            XIEventData::BarrierLeave(e) => XIEvent::BarrierLeave(BarrierEvent::from_protocol(connection, e)),
        })
    }

    pub(crate) fn to_protocol(self) -> XIEventData {
        match self {
            XIEvent::DeviceChanged(e) => XIEventData::DeviceChanged(e.to_protocol()),
            XIEvent::KeyPress(e) => XIEventData::KeyPress(e.to_protocol()),
            XIEvent::KeyRelease(e) => XIEventData::KeyRelease(e.to_protocol()),
            XIEvent::ButtonPress(e) => XIEventData::ButtonPress(e.to_protocol()),
            XIEvent::ButtonRelease(e) => XIEventData::ButtonRelease(e.to_protocol()),
            XIEvent::Motion(e) => XIEventData::Motion(e.to_protocol()),
            XIEvent::Enter(e) => XIEventData::Enter(e.to_protocol()),
            XIEvent::Leave(e) => XIEventData::Leave(e.to_protocol()),
            XIEvent::FocusIn(e) => XIEventData::FocusIn(e.to_protocol()),
            XIEvent::FocusOut(e) => XIEventData::FocusOut(e.to_protocol()),
            XIEvent::Hierarchy(e) => XIEventData::Hierarchy(e.to_protocol()),
            XIEvent::Property(e) => XIEventData::Property(e.to_protocol()),
            XIEvent::RawKeyPress(e) => XIEventData::RawKeyPress(e.to_protocol()),
            XIEvent::RawKeyRelease(e) => XIEventData::RawKeyRelease(e.to_protocol()),
            XIEvent::RawButtonPress(e) => XIEventData::RawButtonPress(e.to_protocol()),
            XIEvent::RawButtonRelease(e) => XIEventData::RawButtonRelease(e.to_protocol()),
            XIEvent::RawMotion(e) => XIEventData::RawMotion(e.to_protocol()),
            XIEvent::TouchBegin(e) => XIEventData::TouchBegin(e.to_protocol()),
            XIEvent::TouchUpdate(e) => XIEventData::TouchUpdate(e.to_protocol()),
            XIEvent::TouchEnd(e) => XIEventData::TouchEnd(e.to_protocol()),
            XIEvent::TouchOwnership(e) => XIEventData::TouchOwnership(e.to_protocol()),
            XIEvent::RawTouchBegin(e) => XIEventData::RawTouchBegin(e.to_protocol()),
            XIEvent::RawTouchUpdate(e) => XIEventData::RawTouchUpdate(e.to_protocol()),
            XIEvent::RawTouchEnd(e) => XIEventData::RawTouchEnd(e.to_protocol()),
            XIEvent::BarrierHit(e) => XIEventData::BarrierHit(e.to_protocol()),
            XIEvent::BarrierLeave(e) => XIEventData::BarrierLeave(e.to_protocol()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceChangedEvent<'a> {
    pub device: Device<'a>,
    pub time: Timestamp,
    pub source_device: Device<'a>,
    pub reason: ChangeReason,
    pub class: DeviceClass,
}

impl<'a> DeviceChangedEvent<'a> {
    async fn from_protocol(connection: &'a X11Connection, event: xinput2::DeviceChangedEvent) -> Result<DeviceChangedEvent<'a>> {
        Ok(Self {
            device: Device {
                id: event.device,
                connection,
            },
            time: Timestamp(event.time),
            source_device: Device {
                id: event.source_device,
                connection,
            },
            reason: event.reason,
            class: DeviceClass::parse(connection, event.classes).await?,
        })
    }

    fn to_protocol(self) -> xinput2::DeviceChangedEvent {
        xinput2::DeviceChangedEvent {
            device: self.device.id,
            time: self.time.0,
            num_classes: 0,
            source_device: self.source_device.id,
            reason: self.reason,
            classes: self.class.encode(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KeyEvent<'a> {
    pub device: Device<'a>,
    pub time: Timestamp,
    pub keycode: u32,
    pub root_window: Window<'a>,
    pub event_window: Window<'a>,
    pub child_window: Option<Window<'a>>,
    pub root_x: I16F16,
    pub root_y: I16F16,
    pub event_x: I16F16,
    pub event_y: I16F16,
    pub source_device: Device<'a>,
    pub flags: KeyEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Vec<u32>,
    pub valuators: Vec<u32>,
    pub axis_values: Vec<I32F32>,
}

impl<'a> KeyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, event: xinput2::KeyEvent) -> Self {
        Self {
            device: Device {
                id: event.device,
                connection,
            },
            time: Timestamp(event.time),
            keycode: event.keycode,
            root_window: Window {
                handle: event.root_window,
                connection,
            },
            event_window: Window {
                handle: event.event_window,
                connection,
            },
            child_window: match event.child_window {
                0 => None,
                handle => Some(Window { handle, connection })
            },
            root_x: event.root_x.into(),
            root_y: event.root_y.into(),
            event_x: event.event_x.into(),
            event_y: event.event_y.into(),
            source_device: Device {
                id: event.source_device,
                connection,
            },
            flags: event.flags,
            mods: event.mods,
            group: event.group,
            buttons: event.buttons,
            valuators: event.valuators,
            axis_values: event.axis_values.into_iter().map(Into::into).collect(),
        }
    }

    fn to_protocol(self) -> xinput2::KeyEvent {
        xinput2::KeyEvent {
            device: self.device.id,
            time: self.time.0,
            keycode: self.keycode,
            root_window: self.root_window.handle,
            event_window: self.event_window.handle,
            child_window: self.child_window.map(|x| x.handle).unwrap_or(0),
            root_x: self.root_x.into(),
            root_y: self.root_y.into(),
            event_x: self.event_x.into(),
            event_y: self.event_y.into(),
            buttons_len: 0,
            valuators_len: 0,
            source_device: self.source_device.id,
            flags: self.flags,
            mods: self.mods,
            group: self.group,
            buttons: self.buttons,
            valuators: self.valuators,
            axis_values: self.axis_values.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ButtonEvent<'a> {
    pub device: Device<'a>,
    pub time: Timestamp,
    pub button: u32,
    pub root_window: Window<'a>,
    pub event_window: Window<'a>,
    pub child_window: Option<Window<'a>>,
    pub root_x: I16F16,
    pub root_y: I16F16,
    pub event_x: I16F16,
    pub event_y: I16F16,
    pub source_device: Device<'a>,
    pub flags: PointerEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Vec<u32>,
    pub valuators: Vec<u32>,
    pub axis_values: Vec<I32F32>,
}

impl<'a> ButtonEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, event: xinput2::ButtonEvent) -> Self {
        Self {
            device: Device {
                id: event.device,
                connection,
            },
            time: Timestamp(event.time),
            button: event.button,
            root_window: Window {
                handle: event.root_window,
                connection,
            },
            event_window: Window {
                handle: event.event_window,
                connection,
            },
            child_window: match event.child_window {
                0 => None,
                handle => Some(Window { handle, connection })
            },
            root_x: event.root_x.into(),
            root_y: event.root_y.into(),
            event_x: event.event_x.into(),
            event_y: event.event_y.into(),
            source_device: Device {
                id: event.source_device,
                connection,
            },
            flags: event.flags,
            mods: event.mods,
            group: event.group,
            buttons: event.buttons,
            valuators: event.valuators,
            axis_values: event.axis_values.into_iter().map(Into::into).collect(),
        }
    }

    fn to_protocol(self) -> xinput2::ButtonEvent {
        xinput2::ButtonEvent {
            device: self.device.id,
            time: self.time.0,
            button: self.button,
            root_window: self.root_window.handle,
            event_window: self.event_window.handle,
            child_window: self.child_window.map(|x| x.handle).unwrap_or(0),
            root_x: self.root_x.into(),
            root_y: self.root_y.into(),
            event_x: self.event_x.into(),
            event_y: self.event_y.into(),
            buttons_len: 0,
            valuators_len: 0,
            source_device: self.source_device.id,
            flags: self.flags,
            mods: self.mods,
            group: self.group,
            buttons: self.buttons,
            valuators: self.valuators,
            axis_values: self.axis_values.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TransitionEvent<'a> {
    pub device: Device<'a>,
    pub time: Timestamp,
    pub source_device: Device<'a>,
    pub mode: XINotifyMode,
    pub detail: XINotifyDetail,
    pub root_window: Window<'a>,
    pub event_window: Window<'a>,
    pub child_window: Option<Window<'a>>,
    pub root_x: I16F16,
    pub root_y: I16F16,
    pub event_x: I16F16,
    pub event_y: I16F16,
    pub same_screen: bool,
    pub focus: bool,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Vec<u32>,
}

impl<'a> TransitionEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, event: xinput2::TransitionEvent) -> Self {
        Self {
            device: Device {
                id: event.device,
                connection,
            },
            time: Timestamp(event.time),
            source_device: Device {
                id: event.source_device,
                connection,
            },
            mode: event.mode,
            detail: event.detail,
            root_window: Window {
                handle: event.root_window,
                connection,
            },
            event_window: Window {
                handle: event.event_window,
                connection,
            },
            child_window: match event.child_window {
                0 => None,
                handle => Some(Window { handle, connection })
            },
            root_x: event.root_x.into(),
            root_y: event.root_y.into(),
            event_x: event.event_x.into(),
            event_y: event.event_y.into(),
            same_screen: event.same_screen,
            focus: event.focus,
            mods: event.mods,
            group: event.group,
            buttons: event.buttons,
        }
    }

    fn to_protocol(self) -> xinput2::TransitionEvent {
        xinput2::TransitionEvent {
            device: self.device.id,
            time: self.time.0,
            source_device: self.source_device.id,
            mode: self.mode,
            detail: self.detail,
            root_window: self.root_window.handle,
            event_window: self.event_window.handle,
            child_window: self.child_window.map(|x| x.handle).unwrap_or(0),
            root_x: self.root_x.into(),
            root_y: self.root_y.into(),
            event_x: self.event_x.into(),
            event_y: self.event_y.into(),
            buttons_len: 0,
            same_screen: self.same_screen,
            focus: self.focus,
            mods: self.mods,
            group: self.group,
            buttons: self.buttons,
        }
    }
}

#[derive(Debug, Clone)]
pub struct HierarchyInfo<'a> {
    pub device: Device<'a>,
    pub device_type: DeviceType<'a>,
    pub enabled: bool,
    pub flags: HierarchyMask,
}

#[derive(Debug, Clone)]
pub struct HierarchyEvent<'a> {
    pub device: Device<'a>,
    pub time: Timestamp,
    pub flags: HierarchyMask,
    pub infos: Vec<HierarchyInfo<'a>>,
}

impl<'a> HierarchyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, event: xinput2::HierarchyEvent) -> Self {
        Self {
            device: Device {
                id: event.device,
                connection,
            },
            time: Timestamp(event.time),
            flags: event.flags,
            infos: event.infos.into_iter().map(|info| {
                HierarchyInfo {
                    device: Device {
                        id: info.device,
                        connection,
                    },
                    device_type: DeviceType::from_attachment(info.type_, Device {
                        id: info.attachment_device,
                        connection,
                    }),
                    enabled: info.enabled,
                    flags: info.flags,
                }
            }).collect(),
        }
    }

    fn to_protocol(self) -> xinput2::HierarchyEvent {
        xinput2::HierarchyEvent {
            device: self.device.id,
            time: self.time.0,
            flags: self.flags,
            num_infos: 0,
            infos: self.infos.into_iter().map(|info| {
                xinput2::HierarchyInfo {
                    device: info.device.id,
                    attachment_device: match &info.device_type {
                        DeviceType::FloatingSlave => DeviceId::All,
                        DeviceType::SlavePointer { master } => master.id,
                        DeviceType::SlaveKeyboard { master } => master.id,
                        DeviceType::MasterPointer { paired_keyboard } => paired_keyboard.id,
                        DeviceType::MasterKeyboard { paired_pointer } => paired_pointer.id,
                    },
                    type_: match &info.device_type {
                        DeviceType::FloatingSlave => xinput2::DeviceType::FloatingSlave,
                        DeviceType::SlavePointer { .. } => xinput2::DeviceType::SlavePointer,
                        DeviceType::SlaveKeyboard { .. } => xinput2::DeviceType::SlaveKeyboard,
                        DeviceType::MasterPointer { .. } => xinput2::DeviceType::MasterPointer,
                        DeviceType::MasterKeyboard { .. } => xinput2::DeviceType::MasterKeyboard,
                    },
                    enabled: info.enabled,
                    flags: info.flags,
                }
            }).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PropertyEvent<'a> {
    pub device: Device<'a>,
    pub time: Timestamp,
    pub property: Atom,
    pub what: PropertyFlag,
}

impl<'a> PropertyEvent<'a> {
    async fn from_protocol(connection: &'a X11Connection, event: xinput2::PropertyEvent) -> Result<PropertyEvent<'a>> {
        Ok(Self {
            device: Device {
                id: event.device,
                connection,
            },
            time: Timestamp(event.time),
            property: connection.get_atom_name(event.property_atom).await?,
            what: event.what,
        })
    }

    fn to_protocol(self) -> xinput2::PropertyEvent {
        xinput2::PropertyEvent {
            device: self.device.id,
            time: self.time.0,
            property_atom: self.property.handle,
            what: self.what,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawKeyEvent<'a> {
    pub device: Device<'a>,
    pub time: Timestamp,
    pub keycode: u32,
    pub source_device: Device<'a>,
    pub flags: KeyEventFlags,
    pub valuators: Vec<u32>,
    pub axis_values: Vec<I32F32>,
    pub axis_values_raw: Vec<I32F32>,
}

impl<'a> RawKeyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, event: xinput2::RawKeyEvent) -> Self {
        let mut axis_values: Vec<I32F32> = event.combined_axis_values.into_iter().map(Into::into).collect();
        Self {
            device: Device {
                id: event.device,
                connection,
            },
            time: Timestamp(event.time),
            keycode: event.keycode,
            source_device: Device {
                id: event.source_device,
                connection,
            },
            flags: event.flags,
            valuators: event.valuators,
            axis_values: axis_values.drain(..axis_values.len() / 2).collect(),
            axis_values_raw: axis_values,
        }
    }

    fn to_protocol(mut self) -> xinput2::RawKeyEvent {
        self.axis_values.extend(self.axis_values_raw);
        xinput2::RawKeyEvent {
            device: self.device.id,
            time: self.time.0,
            keycode: self.keycode,
            source_device: self.source_device.id,
            flags: self.flags,
            valuators_len: 0,
            valuators: self.valuators,
            combined_axis_values: self.axis_values.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawButtonEvent<'a> {
    pub device: Device<'a>,
    pub time: Timestamp,
    pub button: u32,
    pub source_device: Device<'a>,
    pub flags: PointerEventFlags,
    pub valuators: Vec<u32>,
    pub axis_values: Vec<I32F32>,
    pub axis_values_raw: Vec<I32F32>,
}

impl<'a> RawButtonEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, event: xinput2::RawButtonEvent) -> Self {
        let mut axis_values: Vec<I32F32> = event.combined_axis_values.into_iter().map(Into::into).collect();
        Self {
            device: Device {
                id: event.device,
                connection,
            },
            time: Timestamp(event.time),
            button: event.button,
            source_device: Device {
                id: event.source_device,
                connection,
            },
            flags: event.flags,
            valuators: event.valuators,
            axis_values: axis_values.drain(..axis_values.len() / 2).collect(),
            axis_values_raw: axis_values,
        }
    }

    fn to_protocol(mut self) -> xinput2::RawButtonEvent {
        self.axis_values.extend(self.axis_values_raw);
        xinput2::RawButtonEvent {
            device: self.device.id,
            time: self.time.0,
            button: self.button,
            source_device: self.source_device.id,
            flags: self.flags,
            valuators_len: 0,
            valuators: self.valuators,
            combined_axis_values: self.axis_values.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TouchEvent<'a> {
    pub device: Device<'a>,
    pub time: Timestamp,
    pub touch_id: TouchId,
    pub root_window: Window<'a>,
    pub event_window: Window<'a>,
    pub child_window: Option<Window<'a>>,
    pub root_x: I16F16,
    pub root_y: I16F16,
    pub event_x: I16F16,
    pub event_y: I16F16,
    pub source_device: Device<'a>,
    pub flags: TouchEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Vec<u32>,
    pub valuators: Vec<u32>,
    pub axis_values: Vec<I32F32>,
}

impl<'a> TouchEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, event: xinput2::TouchEvent) -> Self {
        Self {
            device: Device {
                id: event.device,
                connection,
            },
            time: Timestamp(event.time),
            touch_id: TouchId(event.touch_id),
            root_window: Window {
                handle: event.root_window,
                connection,
            },
            event_window: Window {
                handle: event.event_window,
                connection,
            },
            child_window: match event.child_window {
                0 => None,
                handle => Some(Window { handle, connection })
            },
            root_x: event.root_x.into(),
            root_y: event.root_y.into(),
            event_x: event.event_x.into(),
            event_y: event.event_y.into(),
            source_device: Device {
                id: event.source_device,
                connection,
            },
            flags: event.flags,
            mods: event.mods,
            group: event.group,
            buttons: event.buttons,
            valuators: event.valuators,
            axis_values: event.axis_values.into_iter().map(Into::into).collect(),
        }
    }

    fn to_protocol(self) -> xinput2::TouchEvent {
        xinput2::TouchEvent {
            device: self.device.id,
            time: self.time.0,
            touch_id: self.touch_id.0,
            root_window: self.root_window.handle,
            event_window: self.event_window.handle,
            child_window: self.child_window.map(|x| x.handle).unwrap_or(0),
            root_x: self.root_x.into(),
            root_y: self.root_y.into(),
            event_x: self.event_x.into(),
            event_y: self.event_y.into(),
            buttons_len: 0,
            valuators_len: 0,
            source_device: self.source_device.id,
            flags: self.flags,
            mods: self.mods,
            group: self.group,
            buttons: self.buttons,
            valuators: self.valuators,
            axis_values: self.axis_values.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TouchOwnershipEvent<'a> {
    pub device: Device<'a>,
    pub time: Timestamp,
    pub touch_id: TouchId,
    pub root_window: Window<'a>,
    pub event_window: Window<'a>,
    pub child_window: Option<Window<'a>>,
    pub source_device: Device<'a>,
    pub flags: TouchOwnershipFlags,
}

impl<'a> TouchOwnershipEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, event: xinput2::TouchOwnershipEvent) -> Self {
        Self {
            device: Device {
                id: event.device,
                connection,
            },
            time: Timestamp(event.time),
            touch_id: TouchId(event.touch_id),
            root_window: Window {
                handle: event.root_window,
                connection,
            },
            event_window: Window {
                handle: event.event_window,
                connection,
            },
            child_window: match event.child_window {
                0 => None,
                handle => Some(Window { handle, connection })
            },
            source_device: Device {
                id: event.source_device,
                connection,
            },
            flags: event.flags,
        }
    }

    fn to_protocol(self) -> xinput2::TouchOwnershipEvent {
        xinput2::TouchOwnershipEvent {
            device: self.device.id,
            time: self.time.0,
            touch_id: self.touch_id.0,
            root_window: self.root_window.handle,
            event_window: self.event_window.handle,
            child_window: self.child_window.map(|x| x.handle).unwrap_or(0),
            source_device: self.source_device.id,
            flags: self.flags,
        }
    }
}

#[derive(Debug, Clone)]
pub struct RawTouchEvent<'a> {
    pub device: Device<'a>,
    pub time: Timestamp,
    pub touch_id: TouchId,
    pub source_device: Device<'a>,
    pub flags: TouchEventFlags,
    pub valuators: Vec<u32>,
    pub axis_values: Vec<I32F32>,
    pub axis_values_raw: Vec<I32F32>,
}

impl<'a> RawTouchEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, event: xinput2::RawTouchEvent) -> Self {
        let mut axis_values: Vec<I32F32> = event.combined_axis_values.into_iter().map(Into::into).collect();
        Self {
            device: Device {
                id: event.device,
                connection,
            },
            time: Timestamp(event.time),
            touch_id: TouchId(event.touch_id),
            source_device: Device {
                id: event.source_device,
                connection,
            },
            flags: event.flags,
            valuators: event.valuators,
            axis_values: axis_values.drain(..axis_values.len() / 2).collect(),
            axis_values_raw: axis_values,
        }
    }

    fn to_protocol(mut self) -> xinput2::RawTouchEvent {
        self.axis_values.extend(self.axis_values_raw);
        xinput2::RawTouchEvent {
            device: self.device.id,
            time: self.time.0,
            touch_id: self.touch_id.0,
            source_device: self.source_device.id,
            flags: self.flags,
            valuators_len: 0,
            valuators: self.valuators,
            combined_axis_values: self.axis_values.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BarrierEvent<'a> {
    pub device: Device<'a>,
    pub time: Timestamp,
    pub event_id: u32,
    pub root_window: Window<'a>,
    pub event_window: Window<'a>,
    pub barrier: u32,
    pub dtime: u32,
    pub flags: BarrierFlags,
    pub source_device: Device<'a>,
    pub root_x: I16F16,
    pub root_y: I16F16,
    pub dx: I32F32,
    pub dy: I32F32,
}

impl<'a> BarrierEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, event: xinput2::BarrierEvent) -> Self {
        Self {
            device: Device {
                id: event.device,
                connection,
            },
            time: Timestamp(event.time),
            event_id: event.event_id,
            root_window: Window {
                handle: event.root_window,
                connection,
            },
            event_window: Window {
                handle: event.event_window,
                connection,
            },
            barrier: event.barrier,
            dtime: event.dtime,
            flags: event.flags,
            source_device: Device {
                id: event.source_device,
                connection,
            },
            root_x: event.root_x.into(),
            root_y: event.root_y.into(),
            dx: event.dx.into(),
            dy: event.dy.into(),
        }
    }

    fn to_protocol(self) -> xinput2::BarrierEvent {
        xinput2::BarrierEvent {
            device: self.device.id,
            time: self.time.0,
            event_id: self.event_id,
            root_window: self.root_window.handle,
            event_window: self.event_window.handle,
            barrier: self.barrier,
            dtime: self.dtime,
            flags: self.flags,
            source_device: self.source_device.id,
            root_x: self.root_x.into(),
            root_y: self.root_y.into(),
            dx: self.dx.into(),
            dy: self.dy.into(),
        }
    }
}
