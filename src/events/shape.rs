pub use crate::coding::shape::ShapeEventMask;
use crate::{
    coding::shape::{self, ShapeEventCode, ShapeEventData, ShapeKind},
    net::X11Connection,
    requests::{Timestamp, Window},
};
use anyhow::Result;

#[derive(Clone, Debug)]
pub enum ShapeEvent<'a> {
    Notify(NotifyEvent<'a>),
}

impl<'a> ShapeEvent<'a> {
    pub(crate) fn code(&self) -> ShapeEventCode {
        match self {
            ShapeEvent::Notify(_) => ShapeEventCode::Notify,
        }
    }

    pub(crate) async fn from_protocol(connection: &'a X11Connection, from: Vec<u8>, code: u8) -> Result<ShapeEvent<'a>> {
        let event = ShapeEventData::decode_sync(&mut &from[..], ShapeEventCode::from_repr(code)?)?;
        Ok(match event {
            ShapeEventData::Notify(e) => ShapeEvent::Notify(NotifyEvent::from_protocol(connection, e)),
        })
    }

    pub(crate) fn to_protocol(self) -> ShapeEventData {
        match self {
            ShapeEvent::Notify(e) => ShapeEventData::Notify(e.to_protocol()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NotifyEvent<'a> {
    pub shape_kind: ShapeKind,
    pub sequence_number: u16,
    pub window: Window<'a>,
    pub extents_x: i16,
    pub extents_y: i16,
    pub extents_width: u16,
    pub extents_height: u16,
    pub time: Timestamp,
    pub shaped: bool,
}

impl<'a> NotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, event: shape::NotifyEvent) -> NotifyEvent<'a> {
        Self {
            shape_kind: event.shape_kind,
            sequence_number: event.sequence_number,
            window: Window {
                handle: event.affected_window,
                connection,
            },
            extents_x: event.extents_x,
            extents_y: event.extents_y,
            extents_width: event.extents_width,
            extents_height: event.extents_height,
            time: Timestamp(event.server_time),
            shaped: event.shaped,
        }
    }

    fn to_protocol(self) -> shape::NotifyEvent {
        shape::NotifyEvent {
            shape_kind: self.shape_kind,
            sequence_number: self.sequence_number,
            affected_window: self.window.handle,
            extents_x: self.extents_x,
            extents_y: self.extents_y,
            extents_width: self.extents_width,
            extents_height: self.extents_height,
            server_time: self.time.0,
            shaped: self.shaped,
        }
    }
}
