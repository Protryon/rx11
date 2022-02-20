
use crate::coding;
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
pub enum Event {
    KeyPress(KeyEvent),
    KeyRelease(KeyEvent),
    ButtonPress(ButtonEvent),
    ButtonRelease(ButtonEvent),
    MotionNotify(MotionNotifyEvent),
    EnterNotify(NotifyEvent),
    LeaveNotify(NotifyEvent),
    FocusIn(FocusEvent),
    FocusOut(FocusEvent),
    KeymapNotify(KeymapNotifyEvent),
    Expose(ExposeEvent),
    GraphicsExposure(GraphicsExposureEvent),
    NoExposure(NoExposureEvent),
    VisibilityNotify(VisibilityNotifyEvent),
    CreateNotify(CreateNotifyEvent),
    DestroyNotify(DestroyNotifyEvent),
    UnmapNotify(UnmapNotifyEvent),
    MapNotify(MapNotifyEvent),
    MapRequest(MapRequestEvent),
    ReparentNotify(ReparentNotifyEvent),
    ConfigureNotify(ConfigureNotifyEvent),
    ConfigureRequest(ConfigureRequestEvent),
    GravityNotify(GravityNotifyEvent),
    ResizeRequest(ResizeRequestEvent),
    CirculateNotify(CirculateNotifyEvent),
    CirculateRequest(CirculateRequestEvent),
    PropertyNotify(PropertyNotifyEvent),
    SelectionClear(SelectionClearEvent),
    SelectionRequest(SelectionRequestEvent),
    SelectionNotify(SelectionNotifyEvent),
    ColormapNotify(ColormapNotifyEvent),
    ClientMessage(ClientMessageEvent),
    MappingNotify(MappingNotifyEvent),
    UnknownCore(Vec<u8>),
    Ext(Vec<u8>),
}

// impl From<coding::Event> for Event {
//     fn from(from: coding::Event) -> Self {
//         use coding::Event::*;
//         match from {
//             KeyPress(e) => Event::KeyPress(e.into()),
//             KeyRelease(e) => Event::KeyRelease(e.into()),
//             ButtonPress(e) => Event::ButtonPress(e.into()),
//             ButtonRelease(e) => Event::ButtonRelease(e.into()),
//             MotionNotify(e) => Event::MotionNotify(e.into()),
//             EnterNotify(e) => Event::EnterNotify(e.into()),
//             LeaveNotify(e) => Event::LeaveNotify(e.into()),
//             FocusIn(e) => Event::FocusIn(e.into()),
//             FocusOut(e) => Event::FocusOut(e.into()),
//             KeymapNotify(e) => Event::KeymapNotify(e.into()),
//             Expose(e) => Event::Expose(e.into()),
//             GraphicsExposure(e) => Event::GraphicsExposure(e.into()),
//             NoExposure(e) => Event::NoExposure(e.into()),
//             VisibilityNotify(e) => Event::VisibilityNotify(e.into()),
//             CreateNotify(e) => Event::CreateNotify(e.into()),
//             DestroyNotify(e) => Event::DestroyNotify(e.into()),
//             UnmapNotify(e) => Event::UnmapNotify(e.into()),
//             MapNotify(e) => Event::MapNotify(e.into()),
//             MapRequest(e) => Event::MapRequest(e.into()),
//             ReparentNotify(e) => Event::ReparentNotify(e.into()),
//             ConfigureNotify(e) => Event::ConfigureNotify(e.into()),
//             ConfigureRequest(e) => Event::ConfigureRequest(e.into()),
//             GravityNotify(e) => Event::GravityNotify(e.into()),
//             ResizeRequest(e) => Event::ResizeRequest(e.into()),
//             CirculateNotify(e) => Event::CirculateNotify(e.into()),
//             CirculateRequest(e) => Event::CirculateRequest(e.into()),
//             PropertyNotify(e) => Event::PropertyNotify(e.into()),
//             SelectionClear(e) => Event::SelectionClear(e.into()),
//             SelectionRequest(e) => Event::SelectionRequest(e.into()),
//             SelectionNotify(e) => Event::SelectionNotify(e.into()),
//             ColormapNotify(e) => Event::ColormapNotify(e.into()),
//             ClientMessage(e) => Event::ClientMessage(e.into()),
//             MappingNotify(e) => Event::MappingNotify(e.into()),
//             UnknownCore(e) => Event::UnknownCore(e.into()),
//             Ext(e) => Event::Ext(e.into()),
//         }
//     }
// }

// impl From<Event> for coding::Event {
//     fn from(from: Event) -> Self {
//         use coding::Event::*;
//         match from {
//             Event::KeyPress(e) => KeyPress(e.into()),
//             Event::KeyRelease(e) => KeyRelease(e.into()),
//             Event::ButtonPress(e) => ButtonPress(e.into()),
//             Event::ButtonRelease(e) => ButtonRelease(e.into()),
//             Event::MotionNotify(e) => MotionNotify(e.into()),
//             Event::EnterNotify(e) => EnterNotify(e.into()),
//             Event::LeaveNotify(e) => LeaveNotify(e.into()),
//             Event::FocusIn(e) => FocusIn(e.into()),
//             Event::FocusOut(e) => FocusOut(e.into()),
//             Event::KeymapNotify(e) => KeymapNotify(e.into()),
//             Event::Expose(e) => Expose(e.into()),
//             Event::GraphicsExposure(e) => GraphicsExposure(e.into()),
//             Event::NoExposure(e) => NoExposure(e.into()),
//             Event::VisibilityNotify(e) => VisibilityNotify(e.into()),
//             Event::CreateNotify(e) => CreateNotify(e.into()),
//             Event::DestroyNotify(e) => DestroyNotify(e.into()),
//             Event::UnmapNotify(e) => UnmapNotify(e.into()),
//             Event::MapNotify(e) => MapNotify(e.into()),
//             Event::MapRequest(e) => MapRequest(e.into()),
//             Event::ReparentNotify(e) => ReparentNotify(e.into()),
//             Event::ConfigureNotify(e) => ConfigureNotify(e.into()),
//             Event::ConfigureRequest(e) => ConfigureRequest(e.into()),
//             Event::GravityNotify(e) => GravityNotify(e.into()),
//             Event::ResizeRequest(e) => ResizeRequest(e.into()),
//             Event::CirculateNotify(e) => CirculateNotify(e.into()),
//             Event::CirculateRequest(e) => CirculateRequest(e.into()),
//             Event::PropertyNotify(e) => PropertyNotify(e.into()),
//             Event::SelectionClear(e) => SelectionClear(e.into()),
//             Event::SelectionRequest(e) => SelectionRequest(e.into()),
//             Event::SelectionNotify(e) => SelectionNotify(e.into()),
//             Event::ColormapNotify(e) => ColormapNotify(e.into()),
//             Event::ClientMessage(e) => ClientMessage(e.into()),
//             Event::MappingNotify(e) => MappingNotify(e.into()),
//             Event::UnknownCore(e) => UnknownCore(e.into()),
//             Event::Ext(e) => Ext(e.into()),
//         }
//     }
// }

#[derive(Clone, Debug)]
pub struct KeyEvent {
    pub keycode: u8,
    pub sequence_number: u16,
    pub time: Timestamp,
    pub root_window: Window,
    pub event_window: Window,
    pub child_window: Option<Window>,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub keybutmask: Keybutmask,
    pub same_screen: bool,
}

impl From<coding::KeyEvent> for KeyEvent {
    fn from(from: coding::KeyEvent) -> Self {
        Self {
            keycode: from.keycode,
            sequence_number: from.sequence_number,
            time: Timestamp(from.time),
            root_window: Window { handle: from.root_window },
            event_window: Window { handle: from.event_window },
            child_window: match from.child_window {
                0 => None,
                handle => Some(Window { handle }),
            },
            root_x: from.root_x,
            root_y: from.root_y,
            event_x: from.event_x,
            event_y: from.event_y,
            keybutmask: from.keybutmask,
            same_screen: from.same_screen,
        }
    }
}

impl From<KeyEvent> for coding::KeyEvent {
    fn from(from: KeyEvent) -> Self {
        Self {
            keycode: from.keycode,
            sequence_number: from.sequence_number,
            time: from.time.0,
            root_window: from.root_window.handle,
            event_window: from.event_window.handle,
            child_window: from.child_window.map(|x| x.handle).unwrap_or(0),
            root_x: from.root_x,
            root_y: from.root_y,
            event_x: from.event_x,
            event_y: from.event_y,
            keybutmask: from.keybutmask,
            same_screen: from.same_screen,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ButtonEvent {
    pub button: u8,
    pub sequence_number: u16,
    pub time: Timestamp,
    pub root_window: Window,
    pub event_window: Window,
    pub child_window: Option<Window>,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub keybutmask: Keybutmask,
    pub same_screen: bool,
}

impl From<coding::ButtonEvent> for ButtonEvent {
    fn from(from: coding::ButtonEvent) -> Self {
        Self {
            button: from.button,
            sequence_number: from.sequence_number,
            time: Timestamp(from.time),
            root_window: Window { handle: from.root_window },
            event_window: Window { handle: from.event_window },
            child_window: match from.child_window {
                0 => None,
                handle => Some(Window { handle }),
            },
            root_x: from.root_x,
            root_y: from.root_y,
            event_x: from.event_x,
            event_y: from.event_y,
            keybutmask: from.keybutmask,
            same_screen: from.same_screen,
        }
    }
}

impl From<ButtonEvent> for coding::ButtonEvent {
    fn from(from: ButtonEvent) -> Self {
        Self {
            button: from.button,
            sequence_number: from.sequence_number,
            time: from.time.0,
            root_window: from.root_window.handle,
            event_window: from.event_window.handle,
            child_window: from.child_window.map(|x| x.handle).unwrap_or(0),
            root_x: from.root_x,
            root_y: from.root_y,
            event_x: from.event_x,
            event_y: from.event_y,
            keybutmask: from.keybutmask,
            same_screen: from.same_screen,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MotionNotifyEvent {
    pub is_hint: bool,
    pub sequence_number: u16,
    pub time: Timestamp,
    pub root_window: Window,
    pub event_window: Window,
    pub child_window: Option<Window>,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub keybutmask: Keybutmask,
    pub same_screen: bool,
}

#[derive(Clone, Debug)]
pub struct NotifyEvent {
    pub detail: NotifyDetail,
    pub sequence_number: u16,
    pub time: u32,
    pub root_window: Window,
    pub event_window: Window,
    pub child_window: Option<Window>,
    pub root_x: i16,
    pub root_y: i16,
    pub event_x: i16,
    pub event_y: i16,
    pub keybutmask: Keybutmask,
    pub mode: NotifyMode,
    pub flags: NotifyFlags,
}

#[derive(Clone, Debug)]
pub struct FocusEvent {
    pub detail: FocusDetail,
    pub sequence_number: u16,
    pub event_window: Window,
    pub mode: FocusMode,
}

#[derive(Clone, Debug)]
pub struct KeymapNotifyEvent {
    pub keys: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct ExposeEvent {
    pub sequence_number: u16,
    pub window: Window,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub count: u16,
}

#[derive(Clone, Debug)]
pub struct GraphicsExposureEvent {
    pub sequence_number: u16,
    pub drawable: Drawable,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub minor_opcode: u16,
    pub count: u16,
    pub major_opcode: u8,
}

#[derive(Clone, Debug)]
pub struct NoExposureEvent {
    pub sequence_number: u16,
    pub drawable: Drawable,
    pub minor_opcode: u16,
    pub major_opcode: u8,
}

#[derive(Clone, Debug)]
pub struct VisibilityNotifyEvent {
    pub sequence_number: u16,
    pub window: Window,
    pub state: VisibilityState,
}

#[derive(Clone, Debug)]
pub struct CreateNotifyEvent {
    pub sequence_number: u16,
    pub parent_window: Window,
    pub window: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: bool,
}

#[derive(Clone, Debug)]
pub struct DestroyNotifyEvent {
    pub sequence_number: u16,
    pub event_window: Window,
    pub window: Window,
}

#[derive(Clone, Debug)]
pub struct UnmapNotifyEvent {
    pub sequence_number: u16,
    pub event_window: Window,
    pub window: Window,
    pub from_configure: bool,
}

#[derive(Clone, Debug)]
pub struct MapNotifyEvent {
    pub sequence_number: u16,
    pub event_window: Window,
    pub window: Window,
    pub override_redirect: bool,
}

#[derive(Clone, Debug)]
pub struct MapRequestEvent {
    pub sequence_number: u16,
    pub event_window: Window,
    pub window: Window,
}

#[derive(Clone, Debug)]
pub struct ReparentNotifyEvent {
    pub sequence_number: u16,
    pub event_window: Window,
    pub window: Window,
    pub parent_window: Window,
    pub x: i16,
    pub y: i16,
    pub override_redirect: bool,
}

#[derive(Clone, Debug)]
pub struct ConfigureNotifyEvent {
    pub sequence_number: u16,
    pub event_window: Window,
    pub window: Window,
    pub above_sibling: Option<Window>,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub override_redirect: bool,
}

#[derive(Clone, Debug)]
pub struct ConfigureRequestEvent {
    pub stack_mode: StackMode,
    pub sequence_number: u16,
    pub parent_window: Window,
    pub window: Window,
    pub sibling: Option<Window>,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
    pub bitmask: ConfigureWindowBitmask,
}

#[derive(Clone, Debug)]
pub struct GravityNotifyEvent {
    pub sequence_number: u16,
    pub event_window: Window,
    pub window: Window,
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Debug)]
pub struct ResizeRequestEvent {
    pub sequence_number: u16,
    pub window: Window,
    pub width: u16,
    pub height: u16,
}

#[derive(Clone, Debug)]
pub struct CirculateNotifyEvent {
    pub sequence_number: u16,
    pub event_window: Window,
    pub window: Window,
    pub place: CirculatePlace,
}

#[derive(Clone, Debug)]
pub struct CirculateRequestEvent {
    pub sequence_number: u16,
    pub parent_window: Window,
    pub window: Window,
    pub place: CirculatePlace,
}

#[derive(Clone, Debug)]
pub struct PropertyNotifyEvent {
    pub sequence_number: u16,
    pub window: Window,
    pub name_atom: Atom,
    pub time: Timestamp,
    pub state: PropertyNotifyState,
}

#[derive(Clone, Debug)]
pub struct SelectionClearEvent {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub owner_window: Window,
    pub selection_atom: Atom,
}

#[derive(Clone, Debug)]
pub struct SelectionRequestEvent {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub owner_window: Window,
    pub requestor_window: Window,
    pub selection_atom: Atom,
    pub target_atom: Atom,
    pub property_atom: Option<Atom>,
}

#[derive(Clone, Debug)]
pub struct SelectionNotifyEvent {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub requestor_window: Window,
    pub selection_atom: Atom,
    pub target_atom: Atom,
    pub property_atom: Option<Atom>,
}

#[derive(Clone, Debug)]
pub struct ColormapNotifyEvent {
    pub sequence_number: u16,
    pub window: Window,
    pub colormap: Option<Colormap>,
    pub is_new: bool,
    pub state: ColormapNotifyState,
}

#[derive(Clone, Debug)]
pub struct ClientMessageEvent {
    pub format: u8,
    pub sequence_number: u16,
    pub window: Window,
    pub type_atom: Atom,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct MappingNotifyEvent {
    pub sequence_number: u16,
    pub request: MappingNotifyRequest,
    pub first_keycode: u8,
    pub count: u8,
}