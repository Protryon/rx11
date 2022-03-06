use std::ops::BitOr;

use tokio::sync::broadcast::error::RecvError;

use super::*;
use crate::{coding::{xkb::XKBEventMask, xinput2::XIEventMask}, requests::{XINPUT_EXT_NAME, XKB_EXT_NAME}, events::Event};
pub(crate) use crate::coding::Event as RawEvent;
pub use crate::coding::x11::X11EventMask;

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

impl BitOr for EventFilter {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self {
            core_events: self.core_events | rhs.core_events,
            xkb_events: self.xkb_events | rhs.xkb_events,
            xi_events: self.xi_events | rhs.xi_events,
        }
    }
}

impl EventFilter {
    pub const ALL: Self = Self {
        core_events: X11EventMask::ALL,
        xkb_events: XKBEventMask::ALL,
        xi_events: XIEventMask::ALL,
    };
    pub const ZERO: Self = Self {
        core_events: X11EventMask::ZERO,
        xkb_events: XKBEventMask::ZERO,
        xi_events: XIEventMask::ZERO,
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

impl<'a> EventReceiver<'a> {
    pub fn set_filter(&mut self, filter: EventFilter) {
        self.filter = filter;
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
                if code == xkb.major_opcode {
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

    pub fn events(&self) -> EventReceiver<'_> {
        EventReceiver {
            connection: self,
            receiver: self.0.events_sender.subscribe(),
            filter: EventFilter::ALL
        }
    }

}