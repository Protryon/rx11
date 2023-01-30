pub use crate::coding::xrandr::{PropertyNotifyState, XREventMask};
use crate::{
    coding::xrandr::{self, Connection, NotifyCode, NotifyData, Rotation, SubPixel, XREventCode, XREventData},
    net::X11Connection,
    requests::{Atom, Crtc, Mode, Output, Provider, Timestamp, Window},
};
use anyhow::Result;

#[derive(Clone, Debug)]
pub enum XREvent<'a> {
    ScreenChangeNotify(ScreenChangeNotifyEvent<'a>),
    CrtcChange(CrtcChangeEvent<'a>),
    OutputChange(OutputChangeEvent<'a>),
    OutputProperty(OutputPropertyEvent<'a>),
    ProviderChange(ProviderChangeEvent<'a>),
    ProviderProperty(ProviderPropertyEvent<'a>),
    ResourceChange(ResourceChangeEvent<'a>),
}

impl<'a> XREvent<'a> {
    pub(crate) fn code(&self) -> XREventCode {
        match self {
            XREvent::ScreenChangeNotify(_) => XREventCode::ScreenChangeNotify,
            _ => XREventCode::Notify,
        }
    }

    pub(crate) async fn from_protocol(connection: &'a X11Connection, from: Vec<u8>, code: u8) -> Result<XREvent<'a>> {
        let event = XREventData::decode_sync(&mut &from[..], XREventCode::from_repr(code)?)?;
        Ok(match event {
            XREventData::ScreenChangeNotify(e) => XREvent::ScreenChangeNotify(ScreenChangeNotifyEvent::from_protocol(connection, e)),
            XREventData::Notify(e) => notify_from_protocol(connection, e).await?,
        })
    }

    pub(crate) fn to_protocol(self) -> XREventData {
        match self {
            XREvent::ScreenChangeNotify(e) => XREventData::ScreenChangeNotify(e.to_protocol()),
            e => XREventData::Notify(notify_to_protocol(e)),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScreenChangeNotifyEvent<'a> {
    pub rotation: Rotation,
    pub sequence_number: u16,
    pub time: Timestamp,
    pub config_time: Timestamp,
    pub root_window: Window<'a>,
    pub request_window: Window<'a>,
    pub size_id: u16,
    pub subpixel_order: SubPixel,
    pub width: u16,
    pub height: u16,
    pub width_mm: u16,
    pub height_mm: u16,
}

impl<'a> ScreenChangeNotifyEvent<'a> {
    fn from_protocol(connection: &'a X11Connection, event: xrandr::ScreenChangeNotifyEvent) -> ScreenChangeNotifyEvent<'a> {
        Self {
            rotation: event.rotation,
            sequence_number: event.sequence_number,
            time: Timestamp(event.time),
            config_time: Timestamp(event.config_time),
            root_window: Window {
                handle: event.root_window,
                connection,
            },
            request_window: Window {
                handle: event.request_window,
                connection,
            },
            size_id: event.size_id,
            subpixel_order: event.subpixel_order,
            width: event.width,
            height: event.height,
            width_mm: event.width_mm,
            height_mm: event.height_mm,
        }
    }

    fn to_protocol(self) -> xrandr::ScreenChangeNotifyEvent {
        xrandr::ScreenChangeNotifyEvent {
            rotation: self.rotation,
            sequence_number: self.sequence_number,
            time: self.time.0,
            config_time: self.config_time.0,
            root_window: self.root_window.handle,
            request_window: self.request_window.handle,
            size_id: self.size_id,
            subpixel_order: self.subpixel_order,
            width: self.width,
            height: self.height,
            width_mm: self.width_mm,
            height_mm: self.height_mm,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CrtcChangeEvent<'a> {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub window: Window<'a>,
    pub crtc: Crtc<'a>,
    pub mode: Mode<'a>,
    pub rotation: Rotation,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub struct OutputChangeEvent<'a> {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub config_time: Timestamp,
    pub window: Window<'a>,
    pub output: Output<'a>,
    pub crtc: Crtc<'a>,
    pub mode: Mode<'a>,
    pub rotation: Rotation,
    pub connection: Connection,
    pub subpixel_order: SubPixel,
}

#[derive(Debug, Clone)]
pub struct OutputPropertyEvent<'a> {
    pub sequence_number: u16,
    pub window: Window<'a>,
    pub output: Output<'a>,
    pub name: Atom,
    pub time: Timestamp,
    pub status: PropertyNotifyState,
}

#[derive(Debug, Clone)]
pub struct ProviderChangeEvent<'a> {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub window: Window<'a>,
    pub provider: Provider<'a>,
}

#[derive(Debug, Clone)]
pub struct ProviderPropertyEvent<'a> {
    pub sequence_number: u16,
    pub window: Window<'a>,
    pub provider: Provider<'a>,
    pub name: Atom,
    pub time: Timestamp,
    pub status: PropertyNotifyState,
}

#[derive(Debug, Clone)]
pub struct ResourceChangeEvent<'a> {
    pub sequence_number: u16,
    pub time: Timestamp,
    pub window: Window<'a>,
}

async fn notify_from_protocol<'a>(connection: &'a X11Connection, event: xrandr::NotifyEvent) -> Result<XREvent<'a>> {
    Ok(match event.data {
        NotifyData::CrtcChange {
            time,
            window,
            crtc,
            mode,
            rotation,
            x,
            y,
            width,
            height,
        } => XREvent::CrtcChange(CrtcChangeEvent {
            sequence_number: event.sequence_number,
            time: Timestamp(time),
            window: Window {
                handle: window,
                connection,
            },
            crtc: Crtc {
                handle: crtc,
                connection,
            },
            mode: Mode {
                handle: mode,
                connection,
            },
            rotation,
            x,
            y,
            width,
            height,
        }),
        NotifyData::OutputChange {
            time,
            config_time,
            window,
            output,
            crtc,
            mode,
            rotation,
            connection: conn,
            subpixel_order,
        } => XREvent::OutputChange(OutputChangeEvent {
            sequence_number: event.sequence_number,
            time: Timestamp(time),
            config_time: Timestamp(config_time),
            window: Window {
                handle: window,
                connection,
            },
            output: Output {
                handle: output,
                connection,
            },
            crtc: Crtc {
                handle: crtc,
                connection,
            },
            mode: Mode {
                handle: mode,
                connection,
            },
            rotation,
            connection: conn,
            subpixel_order,
        }),
        NotifyData::OutputProperty {
            window,
            output,
            name_atom,
            time,
            status,
        } => XREvent::OutputProperty(OutputPropertyEvent {
            sequence_number: event.sequence_number,
            window: Window {
                handle: window,
                connection,
            },
            output: Output {
                handle: output,
                connection,
            },
            name: connection.get_atom_name(name_atom).await?,
            time: Timestamp(time),
            status,
        }),
        NotifyData::ProviderChange {
            time,
            window,
            provider,
        } => XREvent::ProviderChange(ProviderChangeEvent {
            sequence_number: event.sequence_number,
            time: Timestamp(time),
            window: Window {
                handle: window,
                connection,
            },
            provider: Provider {
                handle: provider,
                connection,
            },
        }),
        NotifyData::ProviderProperty {
            window,
            provider,
            name_atom,
            time,
            status,
        } => XREvent::ProviderProperty(ProviderPropertyEvent {
            sequence_number: event.sequence_number,
            window: Window {
                handle: window,
                connection,
            },
            provider: Provider {
                handle: provider,
                connection,
            },
            name: connection.get_atom_name(name_atom).await?,
            time: Timestamp(time),
            status,
        }),
        NotifyData::ResourceChange {
            time,
            window,
        } => XREvent::ResourceChange(ResourceChangeEvent {
            sequence_number: event.sequence_number,
            time: Timestamp(time),
            window: Window {
                handle: window,
                connection,
            },
        }),
    })
}

fn notify_to_protocol(event: XREvent<'_>) -> xrandr::NotifyEvent {
    match event {
        XREvent::ScreenChangeNotify(_) => unimplemented!(),
        XREvent::CrtcChange(event) => xrandr::NotifyEvent {
            code: NotifyCode::CrtcChange,
            sequence_number: event.sequence_number,
            data: NotifyData::CrtcChange {
                time: event.time.0,
                window: event.window.handle,
                crtc: event.crtc.handle,
                mode: event.mode.handle,
                rotation: event.rotation,
                x: event.x,
                y: event.y,
                width: event.width,
                height: event.height,
            },
        },
        XREvent::OutputChange(event) => xrandr::NotifyEvent {
            code: NotifyCode::OutputChange,
            sequence_number: event.sequence_number,
            data: NotifyData::OutputChange {
                time: event.time.0,
                config_time: event.time.0,
                window: event.window.handle,
                output: event.output.handle,
                crtc: event.crtc.handle,
                mode: event.mode.handle,
                rotation: event.rotation,
                connection: event.connection,
                subpixel_order: event.subpixel_order,
            },
        },
        XREvent::OutputProperty(event) => xrandr::NotifyEvent {
            code: NotifyCode::OutputProperty,
            sequence_number: event.sequence_number,
            data: NotifyData::OutputProperty {
                window: event.window.handle,
                output: event.output.handle,
                name_atom: event.name.handle,
                time: event.time.0,
                status: event.status,
            },
        },
        XREvent::ProviderChange(event) => xrandr::NotifyEvent {
            code: NotifyCode::ProviderChange,
            sequence_number: event.sequence_number,
            data: NotifyData::ProviderChange {
                time: event.time.0,
                window: event.window.handle,
                provider: event.provider.handle,
            },
        },
        XREvent::ProviderProperty(event) => xrandr::NotifyEvent {
            code: NotifyCode::ProviderProperty,
            sequence_number: event.sequence_number,
            data: NotifyData::ProviderProperty {
                window: event.window.handle,
                provider: event.provider.handle,
                name_atom: event.name.handle,
                time: event.time.0,
                status: event.status,
            },
        },
        XREvent::ResourceChange(event) => xrandr::NotifyEvent {
            code: NotifyCode::ResourceChange,
            sequence_number: event.sequence_number,
            data: NotifyData::ResourceChange {
                time: event.time.0,
                window: event.window.handle,
            },
        },
    }
}
