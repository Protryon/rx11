use std::ops::BitOr;

use tokio::sync::broadcast::error::RecvError;

use super::*;
pub use crate::coding::x11::X11EventMask;
pub(crate) use crate::coding::Event as RawEvent;
use crate::{
    coding::{shape::ShapeEventMask, xfixes::XFEventMask, xinput2::XIEventMask, xkb::XKBEventMask, xrandr::XREventMask},
    events::Event,
    requests::{SHAPE_EXT_NAME, XFIXES_EXT_NAME, XINPUT_EXT_NAME, XKB_EXT_NAME, XRANDR_EXT_NAME},
};

type RawEventData = (u8, RawEvent);

pub struct EventReceiver<'a> {
    connection: &'a X11Connection,
    receiver: broadcast::Receiver<RawEventData>,
    filter: EventFilter,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct EventFilter {
    pub core_events: X11EventMask,
    pub xkb_events: XKBEventMask,
    pub xi_events: XIEventMask,
    pub xfixes_events: XFEventMask,
    pub xrandr_events: XREventMask,
    pub shape_events: ShapeEventMask,
}

impl From<X11EventMask> for EventFilter {
    fn from(from: X11EventMask) -> Self {
        EventFilter {
            core_events: from,
            ..Default::default()
        }
    }
}

impl From<XKBEventMask> for EventFilter {
    fn from(from: XKBEventMask) -> Self {
        EventFilter {
            xkb_events: from,
            ..Default::default()
        }
    }
}

impl From<XIEventMask> for EventFilter {
    fn from(from: XIEventMask) -> Self {
        EventFilter {
            xi_events: from,
            ..Default::default()
        }
    }
}

impl From<XFEventMask> for EventFilter {
    fn from(from: XFEventMask) -> Self {
        EventFilter {
            xfixes_events: from,
            ..Default::default()
        }
    }
}

impl From<XREventMask> for EventFilter {
    fn from(from: XREventMask) -> Self {
        EventFilter {
            xrandr_events: from,
            ..Default::default()
        }
    }
}

impl From<ShapeEventMask> for EventFilter {
    fn from(from: ShapeEventMask) -> Self {
        EventFilter {
            shape_events: from,
            ..Default::default()
        }
    }
}

impl BitOr for EventFilter {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            core_events: self.core_events | rhs.core_events,
            xkb_events: self.xkb_events | rhs.xkb_events,
            xi_events: self.xi_events | rhs.xi_events,
            xfixes_events: self.xfixes_events | rhs.xfixes_events,
            xrandr_events: self.xrandr_events | rhs.xrandr_events,
            shape_events: self.shape_events | rhs.shape_events,
        }
    }
}

impl EventFilter {
    pub const ALL: Self = Self {
        core_events: X11EventMask::ALL,
        xkb_events: XKBEventMask::ALL,
        xi_events: XIEventMask::ALL,
        xfixes_events: XFEventMask::ALL,
        xrandr_events: XREventMask::ALL,
        shape_events: ShapeEventMask::ALL,
    };
    pub const ZERO: Self = Self {
        core_events: X11EventMask::ZERO,
        xkb_events: XKBEventMask::ZERO,
        xi_events: XIEventMask::ZERO,
        xfixes_events: XFEventMask::ZERO,
        xrandr_events: XREventMask::ZERO,
        shape_events: ShapeEventMask::ZERO,
    };
}

impl X11EventMask {
    fn matches(&self, code: u8) -> bool {
        if code >= 64 {
            return false;
        }
        let bit = 1u64 << code;
        (self.0 & bit) != 0
    }
}

impl XKBEventMask {
    fn matches(&self, code: u8) -> bool {
        let bit = 1u16 << code;
        (self.0 & bit) != 0
    }
}

impl XIEventMask {
    fn matches(&self, code: u16) -> bool {
        let bit = 1u32 << code;
        (self.0 & bit) != 0
    }
}

impl XFEventMask {
    fn matches(&self, code: u8) -> bool {
        let bit = 1u16 << code;
        (self.0 & bit) != 0
    }
}

impl XREventMask {
    fn matches(&self, code: u8) -> bool {
        let bit = 1u16 << code;
        (self.0 & bit) != 0
    }
}

impl ShapeEventMask {
    fn matches(&self, code: u8) -> bool {
        let bit = 1u16 << code;
        (self.0 & bit) != 0
    }
}

impl<'a> EventReceiver<'a> {
    pub fn set_filter(&mut self, filter: impl Into<EventFilter>) {
        self.filter = filter.into();
    }

    async fn recv_raw(&mut self) -> Option<RawEventData> {
        loop {
            match self.receiver.recv().await {
                Err(RecvError::Lagged(_)) => (),
                Ok(x) => return Some(x),
                Err(RecvError::Closed) => return None,
            }
        }
    }

    pub async fn recv(&mut self) -> Option<Result<Event<'_>>> {
        let (code, event) = loop {
            let (code, event) = self.recv_raw().await?;

            if self.filter.core_events.matches(code) {
                break (code, event);
            }

            if let Some(xkb) = self.connection.get_ext_info(XKB_EXT_NAME) {
                if code == xkb.event_start {
                    let xkb_code = match &event {
                        RawEvent::Ext(raw) => match raw.get(0) {
                            Some(x) => *x,
                            None => continue,
                        },
                        _ => continue,
                    };
                    if self.filter.xkb_events.matches(xkb_code) {
                        break (code, event);
                    }
                    continue;
                }
            }

            if let Some(xfixes) = self.connection.get_ext_info(XFIXES_EXT_NAME) {
                if code >= xfixes.event_start && code < xfixes.event_start + xfixes.event_count {
                    if self.filter.xfixes_events.matches(code - xfixes.event_start) {
                        break (code, event);
                    }
                    continue;
                }
            }

            if let Some(xrandr) = self.connection.get_ext_info(XRANDR_EXT_NAME) {
                if code >= xrandr.event_start && code < xrandr.event_start + xrandr.event_count {
                    if self.filter.xrandr_events.matches(code - xrandr.event_start) {
                        break (code, event);
                    }
                    continue;
                }
            }

            if let Some(shape) = self.connection.get_ext_info(SHAPE_EXT_NAME) {
                if code >= shape.event_start && code < shape.event_start + shape.event_count {
                    if self.filter.shape_events.matches(code - shape.event_start) {
                        break (code, event);
                    }
                    continue;
                }
            }

            if let RawEvent::Generic(generic) = &event {
                if let Some(xinput) = self.connection.get_ext_info(XINPUT_EXT_NAME) {
                    if generic.extension_opcode == xinput.major_opcode {
                        if self.filter.xi_events.matches(generic.evtype) {
                            break (code, event);
                        }
                        continue;
                    }
                }
            }
        };

        Some(Event::from_protocol(self.connection, code, event).await)
    }
}

impl X11Connection {
    pub fn events<'a>(&'a self) -> EventReceiver<'a> {
        EventReceiver {
            connection: self,
            receiver: self.0.events_sender.subscribe(),
            filter: EventFilter::ALL,
        }
    }
}
