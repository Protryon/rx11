
use crate::coding::{self, EventCode};
use crate::connection::X11Connection;
use crate::requests::{*, Keybutmask, StackMode, ConfigureWindowBitmask};

pub use crate::coding::{
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
};

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
    UnknownCore(u8, Vec<u8>),
    Ext(u8, Vec<u8>),
}

impl<'a> Event<'a> {
    pub(crate) fn code(&self) -> u8 {
        match self {
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
            Event::UnknownCore(code, _) => *code,
            Event::Ext(code, _) => *code,
        }
    }
}

pub(crate) trait EventFns<'a> {
    type ProtocolType;
    
    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self;

    fn to_protocol(self) -> Self::ProtocolType;
}

impl<'a> Event<'a> {
    pub(crate) fn from_protocol(connection: &'a X11Connection, code: u8, from: coding::Event) -> Self {
        use coding::Event::*;
        match from {
            KeyPress(e) => Event::KeyPress(EventFns::from_protocol(connection, e)),
            KeyRelease(e) => Event::KeyRelease(EventFns::from_protocol(connection, e)),
            ButtonPress(e) => Event::ButtonPress(EventFns::from_protocol(connection, e)),
            ButtonRelease(e) => Event::ButtonRelease(EventFns::from_protocol(connection, e)),
            MotionNotify(e) => Event::MotionNotify(EventFns::from_protocol(connection, e)),
            EnterNotify(e) => Event::EnterNotify(EventFns::from_protocol(connection, e)),
            LeaveNotify(e) => Event::LeaveNotify(EventFns::from_protocol(connection, e)),
            FocusIn(e) => Event::FocusIn(EventFns::from_protocol(connection, e)),
            FocusOut(e) => Event::FocusOut(EventFns::from_protocol(connection, e)),
            KeymapNotify(e) => Event::KeymapNotify(EventFns::from_protocol(connection, e)),
            Expose(e) => Event::Expose(EventFns::from_protocol(connection, e)),
            GraphicsExposure(e) => Event::GraphicsExposure(EventFns::from_protocol(connection, e)),
            NoExposure(e) => Event::NoExposure(EventFns::from_protocol(connection, e)),
            VisibilityNotify(e) => Event::VisibilityNotify(EventFns::from_protocol(connection, e)),
            CreateNotify(e) => Event::CreateNotify(EventFns::from_protocol(connection, e)),
            DestroyNotify(e) => Event::DestroyNotify(EventFns::from_protocol(connection, e)),
            UnmapNotify(e) => Event::UnmapNotify(EventFns::from_protocol(connection, e)),
            MapNotify(e) => Event::MapNotify(EventFns::from_protocol(connection, e)),
            MapRequest(e) => Event::MapRequest(EventFns::from_protocol(connection, e)),
            ReparentNotify(e) => Event::ReparentNotify(EventFns::from_protocol(connection, e)),
            ConfigureNotify(e) => Event::ConfigureNotify(EventFns::from_protocol(connection, e)),
            ConfigureRequest(e) => Event::ConfigureRequest(EventFns::from_protocol(connection, e)),
            GravityNotify(e) => Event::GravityNotify(EventFns::from_protocol(connection, e)),
            ResizeRequest(e) => Event::ResizeRequest(EventFns::from_protocol(connection, e)),
            CirculateNotify(e) => Event::CirculateNotify(EventFns::from_protocol(connection, e)),
            CirculateRequest(e) => Event::CirculateRequest(EventFns::from_protocol(connection, e)),
            PropertyNotify(e) => Event::PropertyNotify(EventFns::from_protocol(connection, e)),
            SelectionClear(e) => Event::SelectionClear(EventFns::from_protocol(connection, e)),
            SelectionRequest(e) => Event::SelectionRequest(EventFns::from_protocol(connection, e)),
            SelectionNotify(e) => Event::SelectionNotify(EventFns::from_protocol(connection, e)),
            ColormapNotify(e) => Event::ColormapNotify(EventFns::from_protocol(connection, e)),
            ClientMessage(e) => Event::ClientMessage(EventFns::from_protocol(connection, e)),
            MappingNotify(e) => Event::MappingNotify(EventFns::from_protocol(connection, e)),
            UnknownCore(e) => Event::UnknownCore(code, e.into()),
            Ext(e) => Event::Ext(code, e.into()),
        }
    }

    pub(crate) fn to_protocol(self) -> (u8, coding::Event) {
        use coding::Event::*;
        let code = self.code();
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
            Event::UnknownCore(_, e) => UnknownCore(e.into()),
            Event::Ext(_, e) => Ext(e.into()),
        };
        (code, event)
    }
}

// impl From<coding::Event> for Event {
//     fn from(from: coding::Event) -> Self {
//     }
// }

// impl From<Event> for coding::Event {
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

impl<'a> EventFns<'a> for KeyEvent<'a> {
    type ProtocolType = coding::KeyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
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

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for ButtonEvent<'a> {
    type ProtocolType = coding::ButtonEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
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

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for MotionNotifyEvent<'a> {
    type ProtocolType = coding::MotionNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
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

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for NotifyEvent<'a> {
    type ProtocolType = coding::NotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
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

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for FocusEvent<'a> {
    type ProtocolType = coding::FocusEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            detail: from.detail,
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            mode: from.mode,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
            detail: self.detail,
            sequence_number: self.sequence_number,
            event_window: self.event_window.handle,
            mode: self.mode,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeymapNotifyEvent {
    pub keys: Vec<u8>,
}

impl<'a> EventFns<'a> for KeymapNotifyEvent {
    type ProtocolType = coding::KeymapNotifyEvent;

    fn from_protocol(_connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            keys: from.keys,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
            keys: self.keys,
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

impl<'a> EventFns<'a> for ExposeEvent<'a> {
    type ProtocolType = coding::ExposeEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
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

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for GraphicsExposureEvent<'a> {
    type ProtocolType = coding::GraphicsExposureEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
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

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for NoExposureEvent<'a> {
    type ProtocolType = coding::NoExposureEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            drawable: Drawable::Raw(RawDrawable { handle: from.drawable, connection }),
            minor_opcode: from.minor_opcode,
            major_opcode: from.major_opcode,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for VisibilityNotifyEvent<'a> {
    type ProtocolType = coding::VisibilityNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            window: Window { handle: from.window, connection },
            state: from.state,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for CreateNotifyEvent<'a> {
    type ProtocolType = coding::CreateNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
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

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for DestroyNotifyEvent<'a> {
    type ProtocolType = coding::DestroyNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for UnmapNotifyEvent<'a> {
    type ProtocolType = coding::UnmapNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
            from_configure: from.from_configure,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for MapNotifyEvent<'a> {
    type ProtocolType = coding::MapNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
            override_redirect: from.override_redirect,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for MapRequestEvent<'a> {
    type ProtocolType = coding::MapRequestEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for ReparentNotifyEvent<'a> {
    type ProtocolType = coding::ReparentNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
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

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for ConfigureNotifyEvent<'a> {
    type ProtocolType = coding::ConfigureNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
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

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for ConfigureRequestEvent<'a> {
    type ProtocolType = coding::ConfigureRequestEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
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

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for GravityNotifyEvent<'a> {
    type ProtocolType = coding::GravityNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
            x: from.x,
            y: from.y,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for ResizeRequestEvent<'a> {
    type ProtocolType = coding::ResizeRequestEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            window: Window { handle: from.window, connection },
            width: from.width,
            height: from.height,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for CirculateNotifyEvent<'a> {
    type ProtocolType = coding::CirculateNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            event_window: Window { handle: from.event_window, connection },
            window: Window { handle: from.window, connection },
            place: from.place,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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

impl<'a> EventFns<'a> for CirculateRequestEvent<'a> {
    type ProtocolType = coding::CirculateRequestEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            parent_window: Window { handle: from.parent_window, connection },
            window: Window { handle: from.window, connection },
            place: from.place,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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
    pub name: LateAtom,
    pub time: Timestamp,
    pub state: PropertyNotifyState,
}

impl<'a> EventFns<'a> for PropertyNotifyEvent<'a> {
    type ProtocolType = coding::PropertyNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            window: Window { handle: from.window, connection },
            name: connection.try_get_atom_name(from.name_atom),
            time: Timestamp(from.time),
            state: from.state,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
            sequence_number: self.sequence_number,
            window: self.window.handle,
            name_atom: self.name.handle(),
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
    pub selection: LateAtom,
}

impl<'a> EventFns<'a> for SelectionClearEvent<'a> {
    type ProtocolType = coding::SelectionClearEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            time: Timestamp(from.time),
            owner_window: Window { handle: from.owner_window, connection },
            selection: connection.try_get_atom_name(from.selection_atom),
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
            sequence_number: self.sequence_number,
            time: self.time.0,
            owner_window: self.owner_window.handle,
            selection_atom: self.selection.handle(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SelectionRequestEvent<'a> {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub owner_window: Window<'a>,
    pub requestor_window: Window<'a>,
    pub selection: LateAtom,
    pub target: LateAtom,
    pub property: Option<LateAtom>,
}

impl<'a> EventFns<'a> for SelectionRequestEvent<'a> {
    type ProtocolType = coding::SelectionRequestEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            time: Timestamp(from.time),
            owner_window: Window { handle: from.owner_window, connection },
            requestor_window: Window { handle: from.requestor_window, connection },
            selection: connection.try_get_atom_name(from.selection_atom),
            target: connection.try_get_atom_name(from.target_atom),
            property: match from.property_atom {
                0 => None,
                atom => Some(connection.try_get_atom_name(atom))
            }
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
            sequence_number: self.sequence_number,
            time: self.time.0,
            owner_window: self.owner_window.handle,
            requestor_window: self.requestor_window.handle,
            selection_atom: self.selection.handle(),
            target_atom: self.target.handle(),
            property_atom: self.property.map(|x| x.handle()).unwrap_or(0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SelectionNotifyEvent<'a> {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub requestor_window: Window<'a>,
    pub selection: LateAtom,
    pub target: LateAtom,
    pub property: Option<LateAtom>,
}

impl<'a> EventFns<'a> for SelectionNotifyEvent<'a> {
    type ProtocolType = coding::SelectionNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            time: Timestamp(from.time),
            requestor_window: Window { handle: from.requestor_window, connection },
            selection: connection.try_get_atom_name(from.selection_atom),
            target: connection.try_get_atom_name(from.target_atom),
            property: match from.property_atom {
                0 => None,
                atom => Some(connection.try_get_atom_name(atom))
            }
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
            sequence_number: self.sequence_number,
            time: self.time.0,
            requestor_window: self.requestor_window.handle,
            selection_atom: self.selection.handle(),
            target_atom: self.target.handle(),
            property_atom: self.property.map(|x| x.handle()).unwrap_or(0),
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

impl<'a> EventFns<'a> for ColormapNotifyEvent<'a> {
    type ProtocolType = coding::ColormapNotifyEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
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

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
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
    pub type_: LateAtom,
    pub data: Vec<u8>,
}

impl<'a> EventFns<'a> for ClientMessageEvent<'a> {
    type ProtocolType = coding::ClientMessageEvent;

    fn from_protocol(connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            format: from.format,
            sequence_number: from.sequence_number,
            window: Window { handle: from.window, connection },
            type_: connection.try_get_atom_name(from.type_atom),
            data: from.data,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
            format: self.format,
            sequence_number: self.sequence_number,
            window: self.window.handle,
            type_atom: self.type_.handle(),
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

impl<'a> EventFns<'a> for MappingNotifyEvent {
    type ProtocolType = coding::MappingNotifyEvent;

    fn from_protocol(_connection: &'a X11Connection, from: Self::ProtocolType) -> Self {
        Self {
            sequence_number: from.sequence_number,
            request: from.request,
            first_keycode: from.first_keycode,
            count: from.count,
        }
    }

    fn to_protocol(self) -> Self::ProtocolType {
        Self::ProtocolType {
            sequence_number: self.sequence_number,
            request: self.request,
            first_keycode: self.first_keycode,
            count: self.count,
        }
    }
}
