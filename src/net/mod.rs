use std::{collections::BTreeMap, sync::{atomic::{AtomicU16, Ordering, AtomicU32}, Arc}, fmt};

use tokio::{io::{AsyncRead, AsyncWrite, BufReader, BufWriter, AsyncWriteExt}, sync::{mpsc, oneshot, Mutex, broadcast::{self, error::RecvError}}};
use crate::{coding::{x11::{Request, Response, ClientHandshake, ServerHandshake, ServerHandshakeBody, ServerHandshakeSuccess}, RequestBody, ResponseBody, ErrorReply, Screen, ErrorCode, xkb::XKBErrorCode}, requests::{Depth, VisualType, Visual, XKB_EXT_NAME}, events::Event, connection::{UnixConnection, TcpConnection}};
use dashmap::{DashMap, mapref::{entry::Entry, multiple::RefMulti}};
use anyhow::Result;

mod errors;
pub use errors::*;

mod init;
pub use init::*;

mod ext;
pub(crate) use ext::*;

mod io;
pub use io::*;

mod event;
pub use event::*;

struct RequestLen {
    request: Request,
    len: u64,
}

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
    handshake: ServerHandshakeSuccess,
    depths: BTreeMap<u8, Depth>,
    events_sender: broadcast::Sender<(u8, crate::coding::Event)>,
    pub(crate) known_atoms: DashMap<&'static str, u32>,
    pub(crate) known_atoms_inverse: DashMap<u32, &'static str>,
    // map of ext name -> major opcode
    pub(crate) registered_extensions: DashMap<String, ExtInfo>,
}

#[derive(Clone)]
pub struct X11Connection(pub(crate) Arc<X11ConnectionInterior>);

const PROTOCOL_MAJOR_VERSION: u16 = 11;
const PROTOCOL_MINOR_VERSION: u16 = 0;

impl X11Connection {
    async fn init_state(&self) -> Result<()> {
        self.enable_xge().await?;
        self.enable_xkb().await?;
        self.enable_xinput2().await?;
        Ok(())
    }

    pub fn handshake(&self) -> &ServerHandshakeSuccess {
        &self.0.handshake
    }

    pub(crate) fn new_resource_id(&self) -> u32 {
        let raw = self.0.next_resource_id.fetch_add(1, Ordering::SeqCst);
        (raw << self.0.handshake.resource_id_mask.trailing_zeros()) | self.0.handshake.resource_id_base
    }

    pub fn depths(&self) -> &BTreeMap<u8, Depth> {
        &self.0.depths
    }

    pub fn screen(&self) -> &Screen {
        &self.handshake().screens[0]
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
