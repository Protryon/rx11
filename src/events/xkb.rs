pub use crate::coding::xkb::{
    AccessXNotifyEvent, ActionMessageEvent, BellClassResult, CompatMapNotifyEvent, ControlsNotifyEvent, ExtensionDeviceNotifyEvent, IndicatorMapNotifyEvent,
    IndicatorStateNotifyEvent, MapNotifyEvent, NamesNotifyEvent, NewKeyboardNotifyEvent, StateNotifyEvent,
};
use crate::{
    coding::xkb::{self, XKBEventData, XKBEventType},
    net::X11Connection,
    requests::{Atom, Timestamp, Window},
};
use anyhow::Result;

#[derive(Clone, Debug)]
pub enum XKBEvent<'a> {
    NewKeyboardNotify(NewKeyboardNotifyEvent),
    MapNotify(MapNotifyEvent),
    StateNotify(StateNotifyEvent),
    ControlsNotify(ControlsNotifyEvent),
    IndicatorStateNotify(IndicatorStateNotifyEvent),
    IndicatorMapNotify(IndicatorMapNotifyEvent),
    NamesNotify(NamesNotifyEvent),
    CompatMapNotify(CompatMapNotifyEvent),
    BellNotify(BellNotifyEvent<'a>),
    ActionMessage(ActionMessageEvent),
    AccessXNotify(AccessXNotifyEvent),
    ExtensionDeviceNotify(ExtensionDeviceNotifyEvent),
}

#[derive(Debug, Clone)]
pub struct BellNotifyEvent<'a> {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub device_id: u8,
    pub bell_class: BellClassResult,
    pub bell_id: u8,
    pub percent: u8,
    pub pitch: u16,
    pub duration: u16,
    pub name: Atom,
    pub window: Window<'a>,
    pub event_only: bool,
}

impl<'a> XKBEvent<'a> {
    pub(crate) async fn from_protocol(connection: &'a X11Connection, from: Vec<u8>) -> Result<XKBEvent<'a>> {
        let xkb_event = xkb::XKBEvent::decode_sync(&mut &from[..])?;
        Ok(match xkb_event.data {
            XKBEventData::NewKeyboardNotify(e) => XKBEvent::NewKeyboardNotify(e),
            XKBEventData::MapNotify(e) => XKBEvent::MapNotify(e),
            XKBEventData::StateNotify(e) => XKBEvent::StateNotify(e),
            XKBEventData::ControlsNotify(e) => XKBEvent::ControlsNotify(e),
            XKBEventData::IndicatorStateNotify(e) => XKBEvent::IndicatorStateNotify(e),
            XKBEventData::IndicatorMapNotify(e) => XKBEvent::IndicatorMapNotify(e),
            XKBEventData::NamesNotify(e) => XKBEvent::NamesNotify(e),
            XKBEventData::CompatMapNotify(e) => XKBEvent::CompatMapNotify(e),
            XKBEventData::BellNotify(e) => XKBEvent::BellNotify(BellNotifyEvent {
                sequence_number: e.sequence_number,
                time: Timestamp(e.time),
                device_id: e.device_id,
                bell_class: e.bell_class,
                bell_id: e.bell_id,
                percent: e.percent,
                pitch: e.pitch,
                duration: e.duration,
                name: connection.get_atom_name(e.name_atom).await?,
                window: Window {
                    handle: e.window,
                    connection,
                },
                event_only: e.event_only,
            }),
            XKBEventData::ActionMessage(e) => XKBEvent::ActionMessage(e),
            XKBEventData::AccessXNotify(e) => XKBEvent::AccessXNotify(e),
            XKBEventData::ExtensionDeviceNotify(e) => XKBEvent::ExtensionDeviceNotify(e),
        })
    }

    pub(crate) fn to_protocol(self) -> xkb::XKBEvent {
        let (code, data) = match self {
            XKBEvent::NewKeyboardNotify(e) => (XKBEventType::NewKeyboardNotify, XKBEventData::NewKeyboardNotify(e)),
            XKBEvent::MapNotify(e) => (XKBEventType::MapNotify, XKBEventData::MapNotify(e)),
            XKBEvent::StateNotify(e) => (XKBEventType::StateNotify, XKBEventData::StateNotify(e)),
            XKBEvent::ControlsNotify(e) => (XKBEventType::ControlsNotify, XKBEventData::ControlsNotify(e)),
            XKBEvent::IndicatorStateNotify(e) => (XKBEventType::IndicatorStateNotify, XKBEventData::IndicatorStateNotify(e)),
            XKBEvent::IndicatorMapNotify(e) => (XKBEventType::IndicatorMapNotify, XKBEventData::IndicatorMapNotify(e)),
            XKBEvent::NamesNotify(e) => (XKBEventType::NamesNotify, XKBEventData::NamesNotify(e)),
            XKBEvent::CompatMapNotify(e) => (XKBEventType::CompatMapNotify, XKBEventData::CompatMapNotify(e)),
            XKBEvent::BellNotify(e) => (
                XKBEventType::BellNotify,
                XKBEventData::BellNotify(xkb::BellNotifyEvent {
                    sequence_number: e.sequence_number,
                    time: e.time.0,
                    device_id: e.device_id,
                    bell_class: e.bell_class,
                    bell_id: e.bell_id,
                    percent: e.percent,
                    pitch: e.pitch,
                    duration: e.duration,
                    name_atom: e.name.handle,
                    window: e.window.handle,
                    event_only: e.event_only,
                }),
            ),
            XKBEvent::ActionMessage(e) => (XKBEventType::ActionMessage, XKBEventData::ActionMessage(e)),
            XKBEvent::AccessXNotify(e) => (XKBEventType::AccessXNotify, XKBEventData::AccessXNotify(e)),
            XKBEvent::ExtensionDeviceNotify(e) => (XKBEventType::ExtensionDeviceNotify, XKBEventData::ExtensionDeviceNotify(e)),
        };
        xkb::XKBEvent {
            code,
            data,
        }
    }
}
