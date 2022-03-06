
use anyhow::Result;
use crate::{coding::xfixes::{self, XFEventData, XFEventCode}, net::X11Connection, requests::{Timestamp, Atom, Window}};
pub use crate::coding::xfixes::{
    SelectionEventType,
    CursorNotifyType,
};

#[derive(Clone, Debug)]
pub enum XFEvent<'a> {
    SelectionNotify(SelectionNotifyEvent<'a>),
    CursorNotify(CursorNotifyEvent<'a>),
}

impl<'a> XFEvent<'a> {
    pub(crate) fn code(&self) -> XFEventCode {
        match self {
            XFEvent::SelectionNotify(_) => XFEventCode::SelectionNotify,
            XFEvent::CursorNotify(_) => XFEventCode::CursorNotify,
        }
    }

    pub(crate) async fn from_protocol(connection: &'a X11Connection, from: Vec<u8>, code: u8) -> Result<XFEvent<'a>> {
        let xkb_event = XFEventData::decode_sync(&mut &from[..], XFEventCode::from_repr(code)?)?;
        Ok(match xkb_event {
            XFEventData::SelectionNotify(e) => XFEvent::SelectionNotify(SelectionNotifyEvent::from_protocol(connection, e).await?),
            XFEventData::CursorNotify(e) => XFEvent::CursorNotify(CursorNotifyEvent::from_protocol(connection, e).await?),
        })
    }

    pub(crate) fn to_protocol(self) -> XFEventData {
        match self {
            XFEvent::SelectionNotify(e) => XFEventData::SelectionNotify(e.to_protocol()),
            XFEvent::CursorNotify(e) => XFEventData::CursorNotify(e.to_protocol()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectionNotifyEvent<'a> {
    pub subtype: SelectionEventType,
    pub sequence_number: u16,
    pub window: Window<'a>,
    pub owner_window: Window<'a>,
    pub selection: Atom,
    pub time: Timestamp,
    pub selection_time: Timestamp,
}

impl<'a> SelectionNotifyEvent<'a> {
    async fn from_protocol(connection: &'a X11Connection, event: xfixes::SelectionNotifyEvent) -> Result<SelectionNotifyEvent<'a>> {
        Ok(Self {
            subtype: event.subtype,
            sequence_number: event.sequence_number,
            window: Window {
                handle: event.window,
                connection,
            },
            owner_window: Window {
                handle: event.owner_window,
                connection,
            },
            selection: connection.get_atom_name(event.selection_atom).await?,
            time: Timestamp(event.time),
            selection_time: Timestamp(event.selection_time),
        })
    }

    fn to_protocol(self) -> xfixes::SelectionNotifyEvent {
        xfixes::SelectionNotifyEvent {
            subtype: self.subtype,
            sequence_number: self.sequence_number,
            window: self.window.handle,
            owner_window: self.owner_window.handle,
            selection_atom: self.selection.handle,
            time: self.time.0,
            selection_time: self.selection_time.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CursorNotifyEvent<'a> {
    pub subtype: CursorNotifyType,
    pub sequence_number: u16,
    pub window: Window<'a>,
    pub cursor_serial: u32,
    pub time: Timestamp,
    pub name: Option<Atom>,
}

impl<'a> CursorNotifyEvent<'a> {
    async fn from_protocol(connection: &'a X11Connection, event: xfixes::CursorNotifyEvent) -> Result<CursorNotifyEvent<'a>> {
        Ok(Self {
            subtype: event.subtype,
            sequence_number: event.sequence_number,
            window: Window {
                handle: event.window,
                connection,
            },
            cursor_serial: event.cursor_serial,
            time: Timestamp(event.time),
            name: match event.name_atom {
                0 => None,
                atom => Some(connection.get_atom_name(atom).await?),
            },
        })
    }

    fn to_protocol(self) -> xfixes::CursorNotifyEvent {
        xfixes::CursorNotifyEvent {
            subtype: self.subtype,
            sequence_number: self.sequence_number,
            window: self.window.handle,
            cursor_serial: self.cursor_serial,
            time: self.time.0,
            name_atom: self.name.map(|x| x.handle).unwrap_or(0)
        }
    }
}
