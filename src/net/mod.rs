use std::{sync::{atomic::{AtomicU16, Ordering, AtomicU32}, Arc}, fmt};

// use crate::{coding::{x11::{Request, Response, ClientHandshake, ServerHandshake, ServerHandshakeBody, ServerHandshakeSuccess}, RequestBody, ResponseBody, ErrorReply, Screen, ErrorCode, xkb::XKBErrorCode}, requests::{Depth, VisualType, Visual, XKB_EXT_NAME}, events::Event, connection::{UnixConnection, TcpConnection}};
use dashmap::DashMap;
use anyhow::Result;
use tokio::sync::{Mutex, oneshot, mpsc, broadcast};

mod errors;
pub use errors::*;

mod init;
pub use init::*;

mod ext;
pub(crate) use ext::*;

mod io;
pub(crate) use io::*;

mod event;
pub use event::*;

use crate::{requests::Screen, coding::{ErrorReply, Response, ServerHandshakeSuccess}};
pub use crate::coding::x11::{Endianness, PixmapFormat};

enum ResponseValue {
    InboundVoidError,
    Present(Response),
    Waiting(oneshot::Sender<Response>),
}

struct X11OutputContext {
    pending_errors: Mutex<Vec<ErrorReply>>,
    responses: DashMap<u16, ResponseValue>,
}

pub(crate) struct X11ConnectionInterior {
    writer: mpsc::Sender<RequestLen>,
    output: Arc<X11OutputContext>,
    seq: AtomicU16,
    next_resource_id: AtomicU32,
    pub(crate) handshake: ServerHandshakeSuccess,
    events_sender: broadcast::Sender<(u8, RawEvent)>,
    pub(crate) known_atoms: DashMap<&'static str, u32>,
    pub(crate) known_atoms_inverse: DashMap<u32, &'static str>,
    // map of ext name -> major opcode
    pub(crate) registered_extensions: DashMap<String, ExtInfo>,
}

#[derive(Clone)]
pub struct X11Connection(pub(crate) Arc<X11ConnectionInterior>);

const PROTOCOL_MAJOR_VERSION: u16 = 11;
const PROTOCOL_MINOR_VERSION: u16 = 0;

#[derive(Clone, Debug)]
pub struct HandshakeInfo<'a> {
    pub protocol_major_version: u16,
    pub protocol_minor_version: u16,
    pub release_number: u32,
    pub motion_buffer_size: u32,
    pub maximum_request_length: u16,
    pub image_byte_order: Endianness,
    pub bitmap_format_bit_order: Endianness,
    pub bitmap_format_scanline_unit: u8,
    pub bitmap_format_scanline_pad: u8,
    pub min_keycode: u8,
    pub max_keycode: u8,
    pub vendor: &'a str,
    pub pixmap_formats: &'a [PixmapFormat],
}

fn ensure_log(name: &str, result: Result<()>) {
    if let Err(e) = result {
        error!("failed to load {} extension: {:?}", name, e);
    }
}

impl X11Connection {
    async fn init_state(&mut self) {
        ensure_log("xge", self.enable_xge().await);
        ensure_log("xkb", self.enable_xkb().await);
        ensure_log("xfixes", self.enable_xfixes().await);
        ensure_log("xinput2", self.enable_xinput2().await);
        ensure_log("xrandr", self.enable_xrandr().await);
        ensure_log("shape", self.enable_shape().await);
    }

    pub(crate) fn new_resource_id(&self) -> u32 {
        let raw = self.0.next_resource_id.fetch_add(1, Ordering::SeqCst);
        (raw << self.0.handshake.resource_id_mask.trailing_zeros()) | self.0.handshake.resource_id_base
    }

    pub fn screens(&self) -> Vec<Screen<'_>> {
        let mut out = vec![];
        for screen in &self.0.handshake.screens {
            out.push(Screen::decode(self, screen.clone()));
        }
        out
    }

    pub fn handshake(&self) -> HandshakeInfo<'_> {
        let handshake = &self.0.handshake;
        HandshakeInfo {
            protocol_major_version: handshake.protocol_major_version,
            protocol_minor_version: handshake.protocol_minor_version,
            release_number: handshake.release_number,
            motion_buffer_size: handshake.motion_buffer_size,
            maximum_request_length: handshake.maximum_request_length,
            image_byte_order: handshake.image_byte_order,
            bitmap_format_bit_order: handshake.bitmap_format_bit_order,
            bitmap_format_scanline_unit: handshake.bitmap_format_scanline_unit,
            bitmap_format_scanline_pad: handshake.bitmap_format_scanline_pad,
            min_keycode: handshake.min_keycode,
            max_keycode: handshake.max_keycode,
            vendor: &*handshake.vendor,
            pixmap_formats: &handshake.pixmap_formats[..],
        }
    }

    pub async fn check_errors(&self) -> Vec<X11ErrorReply> {
        let mut errors = self.0.output.pending_errors.lock().await;
        errors.drain(..).map(|e| X11ErrorReply {
            bad_value: e.bad_value,
            code: X11ErrorCode::from_raw(self, e.code),
        }).collect()
    }

    pub async fn log_errors(&self) {
        for error in self.check_errors().await {
            error!("{}", error);
        }
    }
}
