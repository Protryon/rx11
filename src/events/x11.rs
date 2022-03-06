
use crate::coding::GenericEvent;
use crate::events::XIEvent;
use crate::{coding, events::XKBEvent};
use crate::net::X11Connection;
use crate::requests::*;
use anyhow::Result;
use bitvec::order::Lsb0;
use bitvec::prelude::BitVec;

pub use crate::coding::x11::{
    NotifyDetail,
    NotifyFlags,
    NotifyMode,
    FocusDetail,
    FocusMode,
    VisibilityState,
    CirculatePlace,
    PropertyNotifyState,
    ColormapNotifyState,
    MappingNotifyRequest,
    ConfigureWindowBitmask,
    StackMode,
    Keybutmask,
    EventCode,
};

use super::XFEvent;

#[derive(Clone, Debug)]
pub enum Event<'a> {
    KeyPress(KeyEvent<'a>),
    KeyRelease(KeyEvent<'a>),
    ButtonPress(ButtonEvent<'a>),
    ButtonRelease(ButtonEvent<'a>),
    MotionNotify(MotionNotifyEvent<'a>),
    EnterNotify(NotifyEvent<'a>),
    LeaveNotify(NotifyEvent<'a>),
    FocusIn(FocusEvent<'a>),
    FocusOut(FocusEvent<'a>),
    KeymapNotify(KeymapNotifyEvent),
    Expose(ExposeEvent<'a>),
    GraphicsExposure(GraphicsExposureEvent<'a>),
    NoExposure(NoExposureEvent<'a>),
    VisibilityNotify(VisibilityNotifyEvent<'a>),
    CreateNotify(CreateNotifyEvent<'a>),
    DestroyNotify(DestroyNotifyEvent<'a>),
    UnmapNotify(UnmapNotifyEvent<'a>),
    MapNotify(MapNotifyEvent<'a>),
    MapRequest(MapRequestEvent<'a>),
    ReparentNotify(ReparentNotifyEvent<'a>),
    ConfigureNotify(ConfigureNotifyEvent<'a>),
    ConfigureRequest(ConfigureRequestEvent<'a>),
    GravityNotify(GravityNotifyEvent<'a>),
    ResizeRequest(ResizeRequestEvent<'a>),
    CirculateNotify(CirculateNotifyEvent<'a>),
    CirculateRequest(CirculateRequestEvent<'a>),
    PropertyNotify(PropertyNotifyEvent<'a>),
    SelectionClear(SelectionClearEvent<'a>),
    SelectionRequest(SelectionRequestEvent<'a>),
    SelectionNotify(SelectionNotifyEvent<'a>),
    ColormapNotify(ColormapNotifyEvent<'a>),
    ClientMessage(ClientMessageEvent<'a>),
    MappingNotify(MappingNotifyEvent),
    XKB(XKBEvent<'a>),
    XF(XFEvent<'a>),
    // generic event
    XI(XIEvent<'a>),
    UnknownCore(u8, Vec<u8>),
}

impl<'a> Event<'a> {
    pub(crate) fn code(&self, connection: &X11Connection) -> Result<u8> {
        Ok(match self {
            Event::KeyPress(_) => EventCode::KeyPress as u8,
            Event::KeyRelease(_) => EventCode::KeyRelease as u8,
            Event::ButtonPress(_) => EventCode::ButtonPress as u8,
            Event::ButtonRelease(_) => EventCode::ButtonRelease as u8,
            Event::MotionNotify(_) => EventCode::MotionNotify as u8,
            Event::EnterNotify(_) => EventCode::EnterNotify as u8,
            Event::LeaveNotify(_) => EventCode::LeaveNotify as u8,
            Event::FocusIn(_) => EventCode::FocusIn as u8,
            Event::FocusOut(_) => EventCode::FocusOut as u8,
            Event::KeymapNotify(_) => EventCode::KeymapNotify as u8,
            Event::Expose(_) => EventCode::Expose as u8,
            Event::GraphicsExposure(_) => EventCode::GraphicsExposure as u8,
            Event::NoExposure(_) => EventCode::NoExposure as u8,
            Event::VisibilityNotify(_) => EventCode::VisibilityNotify as u8,
            Event::CreateNotify(_) => EventCode::CreateNotify as u8,
            Event::DestroyNotify(_) => EventCode::DestroyNotify as u8,
            Event::UnmapNotify(_) => EventCode::UnmapNotify as u8,
            Event::MapNotify(_) => EventCode::MapNotify as u8,
            Event::MapRequest(_) => EventCode::MapRequest as u8,
            Event::ReparentNotify(_) => EventCode::ReparentNotify as u8,
            Event::ConfigureNotify(_) => EventCode::ConfigureNotify as u8,
            Event::ConfigureRequest(_) => EventCode::ConfigureRequest as u8,
            Event::GravityNotify(_) => EventCode::GravityNotify as u8,
            Event::ResizeRequest(_) => EventCode::ResizeRequest as u8,
            Event::CirculateNotify(_) => EventCode::CirculateNotify as u8,
            Event::CirculateRequest(_) => EventCode::CirculateRequest as u8,
            Event::PropertyNotify(_) => EventCode::PropertyNotify as u8,
            Event::SelectionClear(_) => EventCode::SelectionClear as u8,
            Event::SelectionRequest(_) => EventCode::SelectionRequest as u8,
            Event::SelectionNotify(_) => EventCode::SelectionNotify as u8,
            Event::ColormapNotify(_) => EventCode::ColormapNotify as u8,
            Event::ClientMessage(_) => EventCode::ClientMessage as u8,
            Event::MappingNotify(_) => EventCode::MappingNotify as u8,
            Event::XF(e) => e.code() as u8 + connection.get_ext_info(XFIXES_EXT_NAME).ok_or_else(|| anyhow!("missing xfixes extension while sending event"))?.event_start,
            Event::XKB(_) => connection.get_ext_info(XKB_EXT_NAME).ok_or_else(|| anyhow!("missing xkb extension while sending event"))?.event_start,
            Event::XI(_) => EventCode::Generic as u8,
            Event::UnknownCore(code, _) => *code,
        })
    }

    pub(crate) async fn from_protocol(connection: &'a X11Connection, code: u8, from: coding::x11::Event) -> Result<Event<'a>> {
        use coding::x11::Event::*;
        Ok(match from {
            KeyPress(e) => Event::KeyPress(KeyEvent::from_protocol(connection, e)),
            KeyRelease(e) => Event::KeyRelease(KeyEvent::from_protocol(connection, e)),
            ButtonPress(e) => Event::ButtonPress(ButtonEvent::from_protocol(connection, e)),
            ButtonRelease(e) => Event::ButtonRelease(ButtonEvent::from_protocol(connection, e)),
            MotionNotify(e) => Event::MotionNotify(MotionNotifyEvent::from_protocol(connection, e)),
            EnterNotify(e) => Event::EnterNotify(NotifyEvent::from_protocol(connection, e)),
            LeaveNotify(e) => Event::LeaveNotify(NotifyEvent::from_protocol(connection, e)),
            FocusIn(e) => Event::FocusIn(FocusEvent::from_protocol(connection, e)),
            FocusOut(e) => Event::FocusOut(FocusEvent::from_protocol(connection, e)),
            KeymapNotify(e) => Event::KeymapNotify(KeymapNotifyEvent::from_protocol(connection, e)),
            Expose(e) => Event::Expose(ExposeEvent::from_protocol(connection, e)),
            GraphicsExposure(e) => Event::GraphicsExposure(GraphicsExposureEvent::from_protocol(connection, e)),
            NoExposure(e) => Event::NoExposure(NoExposureEvent::from_protocol(connection, e)),
            VisibilityNotify(e) => Event::VisibilityNotify(VisibilityNotifyEvent::from_protocol(connection, e)),
            CreateNotify(e) => Event::CreateNotify(CreateNotifyEvent::from_protocol(connection, e)),
            DestroyNotify(e) => Event::DestroyNotify(DestroyNotifyEvent::from_protocol(connection, e)),
            UnmapNotify(e) => Event::UnmapNotify(UnmapNotifyEvent::from_protocol(connection, e)),
            MapNotify(e) => Event::MapNotify(MapNotifyEvent::from_protocol(connection, e)),
            MapRequest(e) => Event::MapRequest(MapRequestEvent::from_protocol(connection, e)),
            ReparentNotify(e) => Event::ReparentNotify(ReparentNotifyEvent::from_protocol(connection, e)),
            ConfigureNotify(e) => Event::ConfigureNotify(ConfigureNotifyEvent::from_protocol(connection, e)),
            ConfigureRequest(e) => Event::ConfigureRequest(ConfigureRequestEvent::from_protocol(connection, e)),
            GravityNotify(e) => Event::GravityNotify(GravityNotifyEvent::from_protocol(connection, e)),
            ResizeRequest(e) => Event::ResizeRequest(ResizeRequestEvent::from_protocol(connection, e)),
            CirculateNotify(e) => Event::CirculateNotify(CirculateNotifyEvent::from_protocol(connection, e)),
            CirculateRequest(e) => Event::CirculateRequest(CirculateRequestEvent::from_protocol(connection, e)),
            PropertyNotify(e) => Event::PropertyNotify(PropertyNotifyEvent::from_protocol(connection, e).await?),
            SelectionClear(e) => Event::SelectionClear(SelectionClearEvent::from_protocol(connection, e).await?),
            SelectionRequest(e) => Event::SelectionRequest(SelectionRequestEvent::from_protocol(connection, e).await?),
            SelectionNotify(e) => Event::SelectionNotify(SelectionNotifyEvent::from_protocol(connection, e).await?),
            ColormapNotify(e) => Event::ColormapNotify(ColormapNotifyEvent::from_protocol(connection, e)),
            ClientMessage(e) => Event::ClientMessage(ClientMessageEvent::from_protocol(connection, e).await?),
            MappingNotify(e) => Event::MappingNotify(MappingNotifyEvent::from_protocol(connection, e)),
            Generic(e) => {
                let extension = connection.get_ext_info_by_opcode(e.extension_opcode)
                    .ok_or_else(|| anyhow!("received generic event for unknown extension"))?;
                match &**extension.key() {
                    crate::requests::XINPUT_EXT_NAME => {
                        return Ok(Event::XI(XIEvent::from_protocol(connection, e.evtype, e.data).await?));
                    },
                    _ => bail!("unimplemented event for extension {}", extension.key()),
                }
            },
            UnknownCore(e) => Event::UnknownCore(code, e.into()),
            Ext(e) => {
                let extension = connection.get_ext_info_by_event_code(code)
                    .ok_or_else(|| anyhow!("received ext event for unknown extension"))?;
                match &**extension.key() {
                    crate::requests::XKB_EXT_NAME => {
                        return Ok(Event::XKB(XKBEvent::from_protocol(connection, e).await?));
                    },
                    crate::requests::XFIXES_EXT_NAME => {
                        return Ok(Event::XF(XFEvent::from_protocol(connection, e, code - extension.event_start).await?));
                    },
                    _ => bail!("unimplemented event for extension {}", extension.key()),
                }
            },
        })
    }

    pub(crate) fn to_protocol(self, connection: &X11Connection) -> Result<(u8, coding::x11::Event)> {
        use coding::x11::Event::*;
        let code = self.code(connection)?;
        let event = match self {
            Event::KeyPress(e) => KeyPress(e.to_protocol()),
            Event::KeyRelease(e) => KeyRelease(e.to_protocol()),
            Event::ButtonPress(e) => ButtonPress(e.to_protocol()),
            Event::ButtonRelease(e) => ButtonRelease(e.to_protocol()),
            Event::MotionNotify(e) => MotionNotify(e.to_protocol()),
            Event::EnterNotify(e) => EnterNotify(e.to_protocol()),
            Event::LeaveNotify(e) => LeaveNotify(e.to_protocol()),
            Event::FocusIn(e) => FocusIn(e.to_protocol()),
            Event::FocusOut(e) => FocusOut(e.to_protocol()),
            Event::KeymapNotify(e) => KeymapNotify(e.to_protocol()),
            Event::Expose(e) => Expose(e.to_protocol()),
            Event::GraphicsExposure(e) => GraphicsExposure(e.to_protocol()),
            Event::NoExposure(e) => NoExposure(e.to_protocol()),
            Event::VisibilityNotify(e) => VisibilityNotify(e.to_protocol()),
            Event::CreateNotify(e) => CreateNotify(e.to_protocol()),
            Event::DestroyNotify(e) => DestroyNotify(e.to_protocol()),
            Event::UnmapNotify(e) => UnmapNotify(e.to_protocol()),
            Event::MapNotify(e) => MapNotify(e.to_protocol()),
            Event::MapRequest(e) => MapRequest(e.to_protocol()),
            Event::ReparentNotify(e) => ReparentNotify(e.to_protocol()),
            Event::ConfigureNotify(e) => ConfigureNotify(e.to_protocol()),
            Event::ConfigureRequest(e) => ConfigureRequest(e.to_protocol()),
            Event::GravityNotify(e) => GravityNotify(e.to_protocol()),
            Event::ResizeRequest(e) => ResizeRequest(e.to_protocol()),
            Event::CirculateNotify(e) => CirculateNotify(e.to_protocol()),
            Event::CirculateRequest(e) => CirculateRequest(e.to_protocol()),
            Event::PropertyNotify(e) => PropertyNotify(e.to_protocol()),
            Event::SelectionClear(e) => SelectionClear(e.to_protocol()),
            Event::SelectionRequest(e) => SelectionRequest(e.to_protocol()),
            Event::SelectionNotify(e) => SelectionNotify(e.to_protocol()),
            Event::ColormapNotify(e) => ColormapNotify(e.to_protocol()),
            Event::ClientMessage(e) => ClientMessage(e.to_protocol()),
            Event::MappingNotify(e) => MappingNotify(e.to_protocol()),
            Event::XKB(e) => {
                let event = e.to_protocol();
                let mut data_raw = vec![];
                event.encode_sync(&mut data_raw)?;
                Ext(data_raw)
            },
            Event::XF(e) => {
                let code = e.code();
                let event = e.to_protocol();
                let mut data_raw = vec![];
                event.encode_sync(&mut data_raw, code)?;
                Ext(data_raw)
            },
            Event::XI(e) => {
                let event = e.to_protocol();
                let mut data_raw = vec![];
                event.encode_sync(&mut data_raw, event.code())?;

                Generic(GenericEvent {
                    extension_opcode: connection.get_ext_info(XINPUT_EXT_NAME).ok_or_else(|| anyhow!("missing xi2 extension when sending event"))?.major_opcode,
                    sequence_number: 0,
                    length: 0,
                    evtype: event.code() as u16,
                    data: data_raw,
                })
            },
            Event::UnknownCore(_, e) => UnknownCore(e.into()),
        };
        Ok((code, event))
    }
}

// impl From<coding::x11::Event> for Event {
//     fn from(from: coding::x11::Event) -> Self {
//     }
// }

// impl From<Event> for coding::x11::Event {
//     fn from(from: Event) -> Self {
//     }
// }

#[derive(Clone, Debug)]
pub struct KeyEvent<'a> {
    pub keycode: u8,
    pub sequence_number: u16,
    pub time: Timestamp,
    pub root_window: Window<'a>,
    pub event_window: Window<'a>,
    pub child_window: Option<Window<'a>>,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub keybutmask: Keybutmask,
    pub same_screen: bool,
}

impl<'a> KeyEvent<'a> {

    fn from_protocol(connection: &'a X11Connection, from: coding::x11::KeyEvent) -> Self {
        Self {
            keycode: from.keycode,
            sequence_number: from.sequence_number,
            time: Timestamp(from.time),
            root_window: Window { handle: from.root_window, connection },
            event_window: Window { handle: from.event_window, connection },
            child_window: match from.child_window {
                0 => None,
                handle => Some(Window { handle, connection }),
            },
            root_x: from.root_x,
            root_y: from.root_y,
            event_x: from.event_x,
            event_y: from.event_y,
            keybutmask: from.keybutmask,
            same_screen: from.same_screen,
        }
    }

    fn to_protocol(self) -> coding::x11::KeyEvent {
        coding::x11::KeyEvent {
            keycode: self.keycode,
            sequence_number: self.sequence_number,
            time: self.time.0,
            root_window: self.root_window.handle,
            event_window: self.event_window.handle,
            child_window: self.child_window.map(|x| x.handle).unwrap_or(0),
            root_x: self.root_x,
            root_y: self.root_y,
            event_x: self.event_x,
            event_y: self.event_y,
            keybutmask: self.keybutmask,
            same_screen: self.same_screen,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ButtonEvent<'a> {
    pub button: u8,
    pub sequence_number: u16,
    pub time: Timestamp,
    pub root_window: Window<'a>,
    pub event_window: Window<'a>,
    pub child_window: Option<Window<'a>>,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub keybutmask: Keybutmask,
    pub same_screen: bool,
}

impl<'a> ButtonEvent<'a> {

    fn from_protocol(connection: &'a X11Connection, from: coding::x11::ButtonEvent) -> Self {
        Self {
            button: from.button,
            sequence_number: from.sequence_number,
            time: Timestamp(from.time),
            root_window: Window { handle: from.root_window, connection },
            event_window: Window { handle: from.event_window, connection },
            child_window: match from.child_window {
                0 => None,
                handle => Some(Window { handle, connection }),
            },
            root_x: from.root_x,
            root_y: from.root_y,
            event_x: from.event_x,
            event_y: from.event_y,
            keybutmask: from.keybutmask,
            same_screen: from.same_screen,
        }
    }

    fn to_protocol(self) -> coding::x11::ButtonEvent {
        coding::x11::ButtonEvent {
            button: self.button,
            sequence_number: self.sequence_number,
            time: self.time.0,
            root_window: self.root_window.handle,
            event_window: self.event_window.handle,
            child_window: self.child_window.map(|x| x.handle).unwrap_or(0),
            root_x: self.root_x,
            root_y: self.root_y,
            event_x: self.event_x,
            event_y: self.event_y,
            keybutmask: self.keybutmask,
            same_screen: self.same_screen,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MotionNotifyEvent<'a> {
    pub is_hint: bool,
    pub sequence_number: u16,
    pub time: Timestamp,
    pub root_window: Window<'a>,
    pub event_window: Window<'a>,
    pub child_window: Option<Window<'a>>,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub keybutmask: Keybutmask,
    pub same_screen: bool,
}

impl<'a> MotionNotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::MotionNotifyEvent) -> Self {
        Self {
            is_hint: from.is_hint,
            sequence_number: from.sequence_number,
            time: Timestamp(from.time),
            root_window: Window { handle: from.root_window, connection },
            event_window: Window { handle: from.event_window, connection },
            child_window: match from.child_window {
                0 => None,
                handle => Some(Window { handle, connection }),
            },
            root_x: from.root_x,
            root_y: from.root_y,
            event_x: from.event_x,
            event_y: from.event_y,
            keybutmask: from.keybutmask,
            same_screen: from.same_screen,
        }
    }

    fn to_protocol(self) -> coding::x11::MotionNotifyEvent {
        coding::x11::MotionNotifyEvent {
            is_hint: self.is_hint,
            sequence_number: self.sequence_number,
            time: self.time.0,
            root_window: self.root_window.handle,
            event_window: self.event_window.handle,
            child_window: self.child_window.map(|x| x.handle).unwrap_or(0),
            root_x: self.root_x,
            root_y: self.root_y,
            event_x: self.event_x,
            event_y: self.event_y,
            keybutmask: self.keybutmask,
            same_screen: self.same_screen,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NotifyEvent<'a> {
    pub detail: NotifyDetail,
    pub sequence_number: u16,
    pub time: Timestamp,
    pub root_window: Window<'a>,
    pub event_window: Window<'a>,
    pub child_window: Option<Window<'a>>,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub keybutmask: Keybutmask,
    pub mode: NotifyMode,
    pub flags: NotifyFlags,
}

impl<'a> NotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::NotifyEvent) -> Self {
        Self {
            detail: from.detail,
            sequence_number: from.sequence_number,
            time: Timestamp(from.time),
            root_window: Window { handle: from.root_window, connection },
            event_window: Window { handle: from.event_window, connection },
            child_window: match from.child_window {
                0 => None,
                handle => Some(Window { handle, connection }),
            },
            root_x: from.root_x,
            root_y: from.root_y,
            event_x: from.event_x,
            event_y: from.event_y,
            keybutmask: from.keybutmask,
            mode: from.mode,
            flags: from.flags,
        }
    }

    fn to_protocol(self) -> coding::x11::NotifyEvent {
        coding::x11::NotifyEvent {
            detail: self.detail,
            sequence_number: self.sequence_number,
            time: self.time.0,
            root_window: self.root_window.handle,
            event_window: self.event_window.handle,
            child_window: self.child_window.map(|x| x.handle).unwrap_or(0),
            root_x: self.root_x,
            root_y: self.root_y,
            event_x: self.event_x,
            event_y: self.event_y,
            keybutmask: self.keybutmask,
            mode: self.mode,
            flags: self.flags,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FocusEvent<'a> {
    pub detail: FocusDetail,
    pub sequence_number: u16,
    pub event_window: Window<'a>,
    pub mode: FocusMode,
}

impl<'a> FocusEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::FocusEvent) -> Self {
        Self {
            detail: from.detail,
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            mode: from.mode,
        }
    }

    fn to_protocol(self) -> coding::x11::FocusEvent {
        coding::x11::FocusEvent {
            detail: self.detail,
            sequence_number: self.sequence_number,
            event_window: self.event_window.handle,
            mode: self.mode,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeymapNotifyEvent {
    pub keys: BitVec<u8, Lsb0>,
}

impl KeymapNotifyEvent {
    fn from_protocol(_connection: &X11Connection, from: coding::x11::KeymapNotifyEvent) -> Self {
        Self {
            keys: BitVec::from_vec(from.keys),
        }
    }

    fn to_protocol(self) -> coding::x11::KeymapNotifyEvent {
        coding::x11::KeymapNotifyEvent {
            keys: self.keys.into_vec(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExposeEvent<'a> {
    pub sequence_number: u16,
    pub window: Window<'a>,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub count: u16,
}

impl<'a> ExposeEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::ExposeEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            window: Window { handle: from.window, connection },
            x: from.x,
            y: from.y,
            width: from.width,
            height: from.height,
            count: from.count,
        }
    }

    fn to_protocol(self) -> coding::x11::ExposeEvent {
        coding::x11::ExposeEvent {
            sequence_number: self.sequence_number,
            window: self.window.handle,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            count: self.count,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GraphicsExposureEvent<'a> {
    pub sequence_number: u16,
    pub drawable: Drawable<'a>,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub minor_opcode: u16,
    pub count: u16,
    pub major_opcode: u8,
}

impl<'a> GraphicsExposureEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::GraphicsExposureEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            drawable: Drawable::Raw(RawDrawable { handle: from.drawable, connection }),
            x: from.x,
            y: from.y,
            width: from.width,
            height: from.height,
            minor_opcode: from.minor_opcode,
            count: from.count,
            major_opcode: from.major_opcode,
        }
    }

    fn to_protocol(self) -> coding::x11::GraphicsExposureEvent {
        coding::x11::GraphicsExposureEvent {
            sequence_number: self.sequence_number,
            drawable: self.drawable.handle(),
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            minor_opcode: self.minor_opcode,
            count: self.count,
            major_opcode: self.major_opcode,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NoExposureEvent<'a> {
    pub sequence_number: u16,
    pub drawable: Drawable<'a>,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}

impl<'a> NoExposureEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::NoExposureEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            drawable: Drawable::Raw(RawDrawable { handle: from.drawable, connection }),
            minor_opcode: from.minor_opcode,
            major_opcode: from.major_opcode,
        }
    }

    fn to_protocol(self) -> coding::x11::NoExposureEvent {
        coding::x11::NoExposureEvent {
            sequence_number: self.sequence_number,
            drawable: self.drawable.handle(),
            minor_opcode: self.minor_opcode,
            major_opcode: self.major_opcode,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VisibilityNotifyEvent<'a> {
    pub sequence_number: u16,
    pub window: Window<'a>,
    pub state: VisibilityState,
}

impl<'a> VisibilityNotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::VisibilityNotifyEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            window: Window { handle: from.window, connection },
            state: from.state,
        }
    }

    fn to_protocol(self) -> coding::x11::VisibilityNotifyEvent {
        coding::x11::VisibilityNotifyEvent {
            sequence_number: self.sequence_number,
            window: self.window.handle,
            state: self.state,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CreateNotifyEvent<'a> {
    pub sequence_number: u16,
    pub parent_window: Window<'a>,
    pub window: Window<'a>,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: bool,
}

impl<'a> CreateNotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::CreateNotifyEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            parent_window: Window { handle: from.parent_window, connection },
            window: Window { handle: from.window, connection },
            x: from.x,
            y: from.y,
            width: from.width,
            height: from.height,
            border_width: from.border_width,
            override_redirect: from.override_redirect,
        }
    }

    fn to_protocol(self) -> coding::x11::CreateNotifyEvent {
        coding::x11::CreateNotifyEvent {
            sequence_number: self.sequence_number,
            parent_window: self.parent_window.handle,
            window: self.window.handle,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            border_width: self.border_width,
            override_redirect: self.override_redirect,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DestroyNotifyEvent<'a> {
    pub sequence_number: u16,
    pub event_window: Window<'a>,
    pub window: Window<'a>,
}

impl<'a> DestroyNotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::DestroyNotifyEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
        }
    }

    fn to_protocol(self) -> coding::x11::DestroyNotifyEvent {
        coding::x11::DestroyNotifyEvent {
            sequence_number: self.sequence_number,
            event_window: self.event_window.handle,
            window: self.window.handle,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UnmapNotifyEvent<'a> {
    pub sequence_number: u16,
    pub event_window: Window<'a>,
    pub window: Window<'a>,
    pub from_configure: bool,
}

impl<'a> UnmapNotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::UnmapNotifyEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
            from_configure: from.from_configure,
        }
    }

    fn to_protocol(self) -> coding::x11::UnmapNotifyEvent {
        coding::x11::UnmapNotifyEvent {
            sequence_number: self.sequence_number,
            event_window: self.event_window.handle,
            window: self.window.handle,
            from_configure: self.from_configure,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MapNotifyEvent<'a> {
    pub sequence_number: u16,
    pub event_window: Window<'a>,
    pub window: Window<'a>,
    pub override_redirect: bool,
}

impl<'a> MapNotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::MapNotifyEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
            override_redirect: from.override_redirect,
        }
    }

    fn to_protocol(self) -> coding::x11::MapNotifyEvent {
        coding::x11::MapNotifyEvent {
            sequence_number: self.sequence_number,
            event_window: self.event_window.handle,
            window: self.window.handle,
            override_redirect: self.override_redirect,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MapRequestEvent<'a> {
    pub sequence_number: u16,
    pub event_window: Window<'a>,
    pub window: Window<'a>,
}

impl<'a> MapRequestEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::MapRequestEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
        }
    }

    fn to_protocol(self) -> coding::x11::MapRequestEvent {
        coding::x11::MapRequestEvent {
            sequence_number: self.sequence_number,
            event_window: self.event_window.handle,
            window: self.window.handle,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ReparentNotifyEvent<'a> {
    pub sequence_number: u16,
    pub event_window: Window<'a>,
    pub window: Window<'a>,
    pub parent_window: Window<'a>,
    pub x: i16,
    pub y: i16,
    pub override_redirect: bool,
}

impl<'a> ReparentNotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::ReparentNotifyEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
            parent_window: Window { handle: from.parent_window, connection },
            x: from.x,
            y: from.y,
            override_redirect: from.override_redirect,
        }
    }

    fn to_protocol(self) -> coding::x11::ReparentNotifyEvent {
        coding::x11::ReparentNotifyEvent {
            sequence_number: self.sequence_number,
            event_window: self.event_window.handle,
            window: self.window.handle,
            parent_window: self.parent_window.handle,
            x: self.x,
            y: self.y,
            override_redirect: self.override_redirect,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConfigureNotifyEvent<'a> {
    pub sequence_number: u16,
    pub event_window: Window<'a>,
    pub window: Window<'a>,
    pub above_sibling: Option<Window<'a>>,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: bool,
}

impl<'a> ConfigureNotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::ConfigureNotifyEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
            above_sibling: match from.above_sibling {
                0 => None,
                handle => Some(Window { handle, connection }),
            },
            x: from.x,
            y: from.y,
            width: from.width,
            height: from.height,
            border_width: from.border_width,
            override_redirect: from.override_redirect,
        }
    }

    fn to_protocol(self) -> coding::x11::ConfigureNotifyEvent {
        coding::x11::ConfigureNotifyEvent {
            sequence_number: self.sequence_number,
            event_window: self.event_window.handle,
            window: self.window.handle,
            above_sibling: self.above_sibling.map(|x| x.handle).unwrap_or(0),
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            border_width: self.border_width,
            override_redirect: self.override_redirect,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConfigureRequestEvent<'a> {
    pub stack_mode: StackMode,
    pub sequence_number: u16,
    pub parent_window: Window<'a>,
    pub window: Window<'a>,
    pub sibling: Option<Window<'a>>,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub bitmask: ConfigureWindowBitmask,
}

impl<'a> ConfigureRequestEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::ConfigureRequestEvent) -> Self {
        Self {
            stack_mode: from.stack_mode,
            sequence_number: from.sequence_number,
            parent_window: Window { handle: from.parent_window, connection },
            window: Window { handle: from.window, connection },
            sibling: match from.sibling {
                0 => None,
                handle => Some(Window { handle, connection }),
            },
            x: from.x,
            y: from.y,
            width: from.width,
            height: from.height,
            border_width: from.border_width,
            bitmask: from.bitmask,
        }
    }

    fn to_protocol(self) -> coding::x11::ConfigureRequestEvent {
        coding::x11::ConfigureRequestEvent {
            stack_mode: self.stack_mode,
            sequence_number: self.sequence_number,
            parent_window: self.parent_window.handle,
            window: self.window.handle,
            sibling: self.sibling.map(|x| x.handle).unwrap_or(0),
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            border_width: self.border_width,
            bitmask: self.bitmask,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GravityNotifyEvent<'a> {
    pub sequence_number: u16,
    pub event_window: Window<'a>,
    pub window: Window<'a>,
    pub x: i16,
    pub y: i16,
}

impl<'a> GravityNotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::GravityNotifyEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
            x: from.x,
            y: from.y,
        }
    }

    fn to_protocol(self) -> coding::x11::GravityNotifyEvent {
        coding::x11::GravityNotifyEvent {
            sequence_number: self.sequence_number,
            event_window: self.event_window.handle,
            window: self.window.handle,
            x: self.x,
            y: self.y,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResizeRequestEvent<'a> {
    pub sequence_number: u16,
    pub window: Window<'a>,
    pub width: u16,
    pub height: u16,
}

impl<'a> ResizeRequestEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::ResizeRequestEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            window: Window { handle: from.window, connection },
            width: from.width,
            height: from.height,
        }
    }

    fn to_protocol(self) -> coding::x11::ResizeRequestEvent {
        coding::x11::ResizeRequestEvent {
            sequence_number: self.sequence_number,
            window: self.window.handle,
            width: self.width,
            height: self.height,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CirculateNotifyEvent<'a> {
    pub sequence_number: u16,
    pub event_window: Window<'a>,
    pub window: Window<'a>,
    pub place: CirculatePlace,
}

impl<'a> CirculateNotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::CirculateNotifyEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
            place: from.place,
        }
    }

    fn to_protocol(self) -> coding::x11::CirculateNotifyEvent {
        coding::x11::CirculateNotifyEvent {
            sequence_number: self.sequence_number,
            event_window: self.event_window.handle,
            window: self.window.handle,
            place: self.place,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CirculateRequestEvent<'a> {
    pub sequence_number: u16,
    pub parent_window: Window<'a>,
    pub window: Window<'a>,
    pub place: CirculatePlace,
}

impl<'a> CirculateRequestEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::CirculateRequestEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            parent_window: Window { handle: from.parent_window, connection },
            window: Window { handle: from.window, connection },
            place: from.place,
        }
    }

    fn to_protocol(self) -> coding::x11::CirculateRequestEvent {
        coding::x11::CirculateRequestEvent {
            sequence_number: self.sequence_number,
            parent_window: self.parent_window.handle,
            window: self.window.handle,
            place: self.place,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PropertyNotifyEvent<'a> {
    pub sequence_number: u16,
    pub window: Window<'a>,
    pub name: Atom,
    pub time: Timestamp,
    pub state: PropertyNotifyState,
}

impl<'a> PropertyNotifyEvent<'a> {
    async fn from_protocol(connection: &'a X11Connection, from: coding::x11::PropertyNotifyEvent) -> Result<PropertyNotifyEvent<'a>> {
        Ok(Self {
            sequence_number: from.sequence_number,
            window: Window { handle: from.window, connection },
            name: connection.get_atom_name(from.name_atom).await?,
            time: Timestamp(from.time),
            state: from.state,
        })
    }

    fn to_protocol(self) -> coding::x11::PropertyNotifyEvent {
        coding::x11::PropertyNotifyEvent {
            sequence_number: self.sequence_number,
            window: self.window.handle,
            name_atom: self.name.handle,
            time: self.time.0,
            state: self.state,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SelectionClearEvent<'a> {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub owner_window: Window<'a>,
    pub selection: Atom,
}

impl<'a> SelectionClearEvent<'a> {
    async fn from_protocol(connection: &'a X11Connection, from: coding::x11::SelectionClearEvent) -> Result<SelectionClearEvent<'a>> {
        Ok(Self {
            sequence_number: from.sequence_number,
            time: Timestamp(from.time),
            owner_window: Window { handle: from.owner_window, connection },
            selection: connection.get_atom_name(from.selection_atom).await?,
        })
    }

    fn to_protocol(self) -> coding::x11::SelectionClearEvent {
        coding::x11::SelectionClearEvent {
            sequence_number: self.sequence_number,
            time: self.time.0,
            owner_window: self.owner_window.handle,
            selection_atom: self.selection.handle,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SelectionRequestEvent<'a> {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub owner_window: Window<'a>,
    pub requestor_window: Window<'a>,
    pub selection: Atom,
    pub target: Atom,
    pub property: Option<Atom>,
}

impl<'a> SelectionRequestEvent<'a> {
    async fn from_protocol(connection: &'a X11Connection, from: coding::x11::SelectionRequestEvent) -> Result<SelectionRequestEvent<'a>> {
        Ok(Self {
            sequence_number: from.sequence_number,
            time: Timestamp(from.time),
            owner_window: Window { handle: from.owner_window, connection },
            requestor_window: Window { handle: from.requestor_window, connection },
            selection: connection.get_atom_name(from.selection_atom).await?,
            target: connection.get_atom_name(from.target_atom).await?,
            property: match from.property_atom {
                0 => None,
                atom => Some(connection.get_atom_name(atom).await?)
            }
        })
    }

    fn to_protocol(self) -> coding::x11::SelectionRequestEvent {
        coding::x11::SelectionRequestEvent {
            sequence_number: self.sequence_number,
            time: self.time.0,
            owner_window: self.owner_window.handle,
            requestor_window: self.requestor_window.handle,
            selection_atom: self.selection.handle,
            target_atom: self.target.handle,
            property_atom: self.property.map(|x| x.handle).unwrap_or(0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SelectionNotifyEvent<'a> {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub requestor_window: Window<'a>,
    pub selection: Atom,
    pub target: Atom,
    pub property: Option<Atom>,
}

impl<'a> SelectionNotifyEvent<'a> {
    async fn from_protocol(connection: &'a X11Connection, from: coding::x11::SelectionNotifyEvent) -> Result<SelectionNotifyEvent<'a>> {
        Ok(Self {
            sequence_number: from.sequence_number,
            time: Timestamp(from.time),
            requestor_window: Window { handle: from.requestor_window, connection },
            selection: connection.get_atom_name(from.selection_atom).await?,
            target: connection.get_atom_name(from.target_atom).await?,
            property: match from.property_atom {
                0 => None,
                atom => Some(connection.get_atom_name(atom).await?)
            }
        })
    }

    fn to_protocol(self) -> coding::x11::SelectionNotifyEvent {
        coding::x11::SelectionNotifyEvent {
            sequence_number: self.sequence_number,
            time: self.time.0,
            requestor_window: self.requestor_window.handle,
            selection_atom: self.selection.handle,
            target_atom: self.target.handle,
            property_atom: self.property.map(|x| x.handle).unwrap_or(0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ColormapNotifyEvent<'a> {
    pub sequence_number: u16,
    pub window: Window<'a>,
    pub colormap: Option<Colormap<'a>>,
    pub is_new: bool,
    pub state: ColormapNotifyState,
}

impl<'a> ColormapNotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, from: coding::x11::ColormapNotifyEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            window: Window { handle: from.window, connection },
            colormap: match from.colormap {
                0 => None,
                handle => Some(Colormap { handle, connection }),
            },
            is_new: from.is_new,
            state: from.state,
        }
    }

    fn to_protocol(self) -> coding::x11::ColormapNotifyEvent {
        coding::x11::ColormapNotifyEvent {
            sequence_number: self.sequence_number,
            window: self.window.handle,
            colormap: self.colormap.map(|x| x.handle).unwrap_or(0),
            is_new: self.is_new,
            state: self.state,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClientMessageEvent<'a> {
    pub format: u8,
    pub sequence_number: u16,
    pub window: Window<'a>,
    pub type_: Atom,
    pub data: Vec<u8>,
}

impl<'a> ClientMessageEvent<'a> {
    async fn from_protocol(connection: &'a X11Connection, from: coding::x11::ClientMessageEvent) -> Result<ClientMessageEvent<'a>> {
        Ok(Self {
            format: from.format,
            sequence_number: from.sequence_number,
            window: Window { handle: from.window, connection },
            type_: connection.get_atom_name(from.type_atom).await?,
            data: from.data,
        })
    }

    fn to_protocol(self) -> coding::x11::ClientMessageEvent {
        coding::x11::ClientMessageEvent {
            format: self.format,
            sequence_number: self.sequence_number,
            window: self.window.handle,
            type_atom: self.type_.handle,
            data: self.data,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MappingNotifyEvent {
    pub sequence_number: u16,
    pub request: MappingNotifyRequest,
    pub first_keycode: u8,
    pub count: u8,
}

impl<'a> MappingNotifyEvent {
    fn from_protocol(_connection: &'a X11Connection, from: coding::x11::MappingNotifyEvent) -> Self {
        Self {
            sequence_number: from.sequence_number,
            request: from.request,
            first_keycode: from.first_keycode,
            count: from.count,
        }
    }

    fn to_protocol(self) -> coding::x11::MappingNotifyEvent {
        coding::x11::MappingNotifyEvent {
            sequence_number: self.sequence_number,
            request: self.request,
            first_keycode: self.first_keycode,
            count: self.count,
        }
    }
}
