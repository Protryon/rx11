use std::{collections::BTreeMap, sync::{atomic::{AtomicU16, Ordering, AtomicU32}, Arc}, fmt};

use tokio::{io::{AsyncRead, AsyncWrite, BufReader, BufWriter, AsyncWriteExt}, sync::{mpsc, oneshot, Mutex, broadcast::{self, error::RecvError}}};
use crate::{coding::{x11::{Request, Response, ClientHandshake, ServerHandshake, ServerHandshakeBody, ServerHandshakeSuccess}, RequestBody, ResponseBody, ErrorReply, Screen, ErrorCode, xkb::XKBErrorCode}, requests::{Depth, VisualType, Visual, XKB_EXT_NAME}, events::Event};
use dashmap::{DashMap, mapref::{entry::Entry, multiple::RefMulti}};

use super::*;

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

#[derive(Clone, Copy, Debug)]
pub(crate) struct ExtInfo {
    pub major_opcode: u8,
    pub event_start: u8,
    pub error_start: u8,
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

impl X11Connection {
    pub(crate) fn get_ext_info(&self, ext_name: &str) -> Option<ExtInfo> {
        self.0.registered_extensions.get(ext_name).map(|x| *x.value())
    }

    pub(crate) fn get_ext_info_by_opcode(&self, opcode: u8) -> Option<RefMulti<'_, String, ExtInfo>> {
        self.0.registered_extensions.iter()
            .find(|entry| entry.value().major_opcode == opcode)
    }

    pub(crate) fn get_ext_info_by_event_start(&self, code: u8) -> Option<RefMulti<'_, String, ExtInfo>> {
        self.0.registered_extensions.iter()
            .find(|entry| entry.value().event_start == code)
    }
}

#[derive(Clone)]
pub struct X11Connection(pub(crate) Arc<X11ConnectionInterior>);

const PROTOCOL_MAJOR_VERSION: u16 = 11;
const PROTOCOL_MINOR_VERSION: u16 = 0;

#[derive(Debug, Clone)]
pub enum X11ErrorCode {
    X11(ErrorCode),
    XKB(XKBErrorCode),
    Unknown(u8),
}

impl X11ErrorCode {
    fn from_raw(connection: &X11Connection, code: u8) -> Self {
        if let Ok(code) = ErrorCode::from_repr(code) {
            return X11ErrorCode::X11(code);
        }
        if let Some(xkb) = connection.get_ext_info(XKB_EXT_NAME) {
            if code == xkb.error_start {
                return X11ErrorCode::XKB(XKBErrorCode::Keyboard);
            }
        }
        X11ErrorCode::Unknown(code)
    }
}

#[derive(Debug, Clone)]
pub struct X11ErrorReply {
    pub bad_value: u32,
    pub code: X11ErrorCode,
}

impl fmt::Display for X11ErrorReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x11 error: {:?} <{}>", self.code, self.bad_value)
    }
}

impl std::error::Error for X11ErrorReply {

}

pub enum X11Error {
    Error(anyhow::Error),
    X11Error(X11ErrorReply),
}

impl fmt::Debug for X11Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X11Error::Error(e) => write!(f, "{:?}", e),
            X11Error::X11Error(e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for X11Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X11Error::Error(e) => write!(f, "{}", e),
            X11Error::X11Error(e) => write!(f, "{}", e),
        }
    }
}

impl std::error::Error for X11Error {

}

impl From<anyhow::Error> for X11Error {
    fn from(from: anyhow::Error) -> Self {
        X11Error::Error(from)
    }
}

impl X11Connection {
    async fn writer_thread(mut writer: BufWriter<impl AsyncWrite + Unpin + Send + Sync>, mut in_receiver: mpsc::Receiver<RequestLen>) -> Result<()> {
        while let Some(request) = in_receiver.recv().await {
            request.request.encode_async(&mut writer, request.len).await?;
            writer.flush().await?;
        }
        Ok(())
    }

    async fn reader_thread(mut reader: BufReader<impl AsyncRead + Unpin + Send + Sync>, output: Arc<X11OutputContext>, events: broadcast::Sender<(u8, crate::coding::Event)>) -> Result<()> {
        loop {
            let response = Response::decode_async(&mut reader).await?;
            match response.body {
                ResponseBody::Event(event) => {
                    if let Err(_) = events.send((response.code, event)) {
                        error!("failed to send event");
                    }
                },
                ResponseBody::ErrorReply(error) => {
                    let entry = output.responses.entry(error.sequence_number);
                    let pending_error = match entry {
                        Entry::Vacant(vacant) => {
                            vacant.insert(ResponseValue::Present(Response {
                                code: response.code,
                                body: ResponseBody::ErrorReply(error),
                            }));
                            None
                        },
                        Entry::Occupied(mut occupied) => {
                            match &mut *occupied.get_mut() {
                                ResponseValue::InboundVoidError => {
                                    debug!("inbound x11 error: {:?} <{}>", error.code, error.bad_value);
                                    Some(error)
                                },
                                ResponseValue::Present(old_response) => {
                                    warn!("overriding response value (are we sending/receiving too fast?): {:?}", old_response);
                                    *old_response = Response {
                                        code: response.code,
                                        body: ResponseBody::ErrorReply(error),
                                    };
                                    None
                                },
                                ResponseValue::Waiting(_) => {
                                    let sender = occupied.remove();
                                    match sender {
                                        ResponseValue::Waiting(sender) => {
                                            let _ = sender.send(Response {
                                                code: response.code,
                                                body: ResponseBody::ErrorReply(error),
                                            });
                                        },
                                        _ => unimplemented!(),
                                    }
                                    None
                                },
                            }
                        },
                    };
                    if let Some(pending_error) = pending_error {
                        output.pending_errors.lock().await.push(pending_error);
                    }
                },
                ResponseBody::Reply(reply) => {
                    let entry = output.responses.entry(reply.sequence_number);
                    match entry {
                        Entry::Vacant(vacant) => {
                            vacant.insert(ResponseValue::Present(Response {
                                code: response.code,
                                body: ResponseBody::Reply(reply),
                            }));
                        },
                        Entry::Occupied(mut occupied) => {
                            match &mut *occupied.get_mut() {
                                ResponseValue::InboundVoidError => {
                                    warn!("received unexpected reply to void request: {:?}", reply);
                                },
                                ResponseValue::Present(old_response) => {
                                    warn!("overriding response value (are we sending/receiving too fast?): {:?}", old_response);
                                    *old_response = Response {
                                        code: response.code,
                                        body: ResponseBody::Reply(reply),
                                    };
                                },
                                ResponseValue::Waiting(_) => {
                                    let sender = occupied.remove();
                                    match sender {
                                        ResponseValue::Waiting(sender) => {
                                            let _ = sender.send(Response {
                                                code: response.code,
                                                body: ResponseBody::Reply(reply),
                                            });
                                        },
                                        _ => unimplemented!(),
                                    }
                                },
                            }
                        },
                    }
                },
            }
        }
    }

    pub async fn connect(host: &str, display: u16) -> Result<Self> {
        #[cfg(not(target_os = "windows"))]
        if host == "" || host == "unix" {
            if let Ok(c) = UnixConnection::connect(display).await {
                let (writer, reader) = c.into_split();
                return Self::open(reader, writer).await;
            }
        }
        let connection = TcpConnection::connect(host, display).await?;
        let (writer, reader) = connection.into_split();
        Self::open(reader, writer).await
    }

    pub async fn open(
        writer: impl AsyncWrite + Unpin + Send + Sync + 'static,
        reader: impl AsyncRead + Unpin + Send + Sync + 'static,
    ) -> Result<Self> {
        let mut writer = BufWriter::new(writer);
        let mut reader = BufReader::new(reader);
        let handshake = ClientHandshake {
            byte_order: 0x42,
            protocol_major_version: PROTOCOL_MAJOR_VERSION,
            protocol_minor_version: PROTOCOL_MINOR_VERSION,
            auth_proto_name: "".to_string(),
            auth_proto_data: "".to_string(),
            ..Default::default()
        };
        info!("sending handshake");
        let mut output = vec![];
        handshake.encode_sync(&mut output)?;
        handshake.encode_async(&mut writer).await?;
        writer.flush().await?;

        info!("awaiting handshake");
        let handshake = ServerHandshake::decode_async(&mut reader).await?;
        info!("got handshake");
        let handshake = match handshake.body {
            ServerHandshakeBody::Failed(f) => {
                bail!("failed to connect to server: {}", f.reason);
            },
            ServerHandshakeBody::AuthRequired(f) => {
                bail!("failed to connect to server, auth required: {}", f.reason);
            },
            ServerHandshakeBody::Success(packet) => {
                packet
            },
        };
        let mut depths = BTreeMap::new();
        for screen in handshake.screens.iter() {
            for depth in &screen.depths {
                depths.insert(depth.depth, Depth {
                    _internal: (),
                    depth: depth.depth,
                    visuals: depth.visuals.iter().map(|visual| VisualType {
                        visual: Visual {
                            handle: visual.visual,
                        },
                        class: visual.class,
                        bits_per_rgb_value: visual.bits_per_rgb_value,
                        colormap_entries: visual.colormap_entries,
                        red_mask: visual.red_mask,
                        green_mask: visual.green_mask,
                        blue_mask: visual.blue_mask,
                        _internal: (),
                    }).collect(),
                });
            }
        }

        let output = Arc::new(X11OutputContext {
            pending_errors: Mutex::new(vec![]),
            responses: DashMap::new(),
        });

        let (in_sender, in_receiver) = mpsc::channel::<RequestLen>(10);
        tokio::spawn(async move {
            if let Err(e) = Self::writer_thread(writer, in_receiver).await {
                error!("x11 writing failed: {:?}", e);
            }
        });

        let (events_sender, _) = broadcast::channel(64);

        let output2 = output.clone();
        let events_sender2 = events_sender.clone();
        tokio::spawn(async move {
            if let Err(e) = Self::reader_thread(reader, output2, events_sender2).await {
                error!("x11 writing failed: {:?}", e);
            }
        });

        let self_ = Self(Arc::new(X11ConnectionInterior {
            output,
            writer: in_sender,
            handshake,
            seq: AtomicU16::new(1),
            next_resource_id: AtomicU32::new(0),
            known_atoms: DashMap::new(),
            known_atoms_inverse: DashMap::new(),
            registered_extensions: DashMap::new(),
            events_sender,
            depths
        }));
        self_.register_const_atoms();

        self_.init_state().await?;

        Ok(self_)
    }

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

    pub async fn send_request(&self, major_opcode: u8, minor_opcode: u8, is_void: bool, body: RequestBody) -> Result<u16> {
        let mut data = vec![];
        body.encode_sync(&mut data, major_opcode, minor_opcode, 0)?;
        let mut length = data.len() as u32 + 4;
        let out_len = if length % 4 != 0 {
            (length + 4 - (length % 4)) / 4
        } else {
            length / 4
        };
        length -= 4;

        let request = Request {
            major_opcode,
            minor_opcode,
            length: if out_len < u16::MAX as u32 {
                out_len as u16
            } else {
                0
            },
            ext_length: if out_len >= u16::MAX as u32 {
                Some(out_len)
            } else {
                None
            },
            data,
        };
        //TODO: there is a race condition in this atomic here, seq could get out-of-order
        let seq = self.0.seq.fetch_add(1, Ordering::SeqCst);
        if is_void {
            self.0.output.responses.insert(seq, ResponseValue::InboundVoidError);
        }
        self.0.writer.send(RequestLen { request, len: length as u64 }).await
            .ok().ok_or_else(|| anyhow!("x11 connection dead"))?;
        Ok(seq)
    }

    pub async fn receive_response(&self, seq: u16) -> Result<Response> {
        enum EntryValue {
            Receiver(oneshot::Receiver<Response>),
            Value(Response),
        }

        let entry = self.0.output.responses.entry(seq);
        let value = match entry {
            Entry::Vacant(vacant) => {
                let (sender, receiver) = oneshot::channel();
                vacant.insert(ResponseValue::Waiting(sender));
                EntryValue::Receiver(receiver)
            },
            Entry::Occupied(mut occupied) => {
                match &*occupied.get() {
                    ResponseValue::InboundVoidError => {
                        let (sender, receiver) = oneshot::channel();
                        occupied.insert(ResponseValue::Waiting(sender));
                        EntryValue::Receiver(receiver)
                    },
                    ResponseValue::Present(_) => {
                        EntryValue::Value(match occupied.remove() {
                            ResponseValue::Present(response) => response,
                            _ => unreachable!(),
                        })
                    },
                    ResponseValue::Waiting(_) => {
                        warn!("overwriting old receive_response request for seq {}", seq);
                        let (sender, receiver) = oneshot::channel();
                        occupied.insert(ResponseValue::Waiting(sender));
                        EntryValue::Receiver(receiver)
                    },
                }
            },
        };
        let response = match value {
            EntryValue::Receiver(receiver) => receiver.await?,
            EntryValue::Value(value) => value,
        };
        Ok(response)
    }

    pub async fn receive_reply<T>(&self, seq: u16, decoder: fn(&mut &[u8]) -> Result<T>) -> Result<T, X11Error> {
        let response = self.receive_response(seq).await?;
        match response.body {
            ResponseBody::ErrorReply(e) => {
                Err(X11Error::X11Error(X11ErrorReply {
                    bad_value: e.bad_value,
                    code: X11ErrorCode::from_raw(self, e.code),
                }))
            },
            ResponseBody::Reply(r) => {
                decoder(&mut &r.data[..]).map_err(Into::into)
            },
            ResponseBody::Event(_) => unimplemented!(),
        }
    }

    pub async fn receive_reply_reserved<T>(&self, seq: u16, decoder: fn(&mut &[u8], u8) -> Result<T>) -> Result<T, X11Error> {
        let response = self.receive_response(seq).await?;
        match response.body {
            ResponseBody::ErrorReply(e) => {
                Err(X11Error::X11Error(X11ErrorReply {
                    bad_value: e.bad_value,
                    code: X11ErrorCode::from_raw(self, e.code),
                }))
            },
            ResponseBody::Reply(r) => {
                decoder(&mut &r.data[..], r.reserved).map_err(Into::into)
            },
            ResponseBody::Event(_) => unimplemented!(),
        }
    }

    pub fn events(&self) -> EventReceiver<'_> {
        EventReceiver {
            connection: self,
            receiver: self.0.events_sender.subscribe(),
        }
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

pub struct EventReceiver<'a> {
    connection: &'a X11Connection,
    receiver: broadcast::Receiver<(u8, crate::coding::Event)>,
}

impl<'a> EventReceiver<'a> {
    pub async fn recv(&mut self) -> Option<Result<Event<'_>>> {
        let (code, event) = loop {
            match self.receiver.recv().await {
                Ok(x) => break x,
                Err(RecvError::Lagged(_)) => (), // todo: warn here
                Err(RecvError::Closed) => return None,
            }
        };
        Some(Event::from_protocol(self.connection, code, event).await)
    }
}

#[macro_export]
macro_rules! send_request {
    ($self_:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let body = RequestBody::$name($name {
                $($key: $value,)*
                ..Default::default()
            });
            $self_.send_request(MajorOpcode::$name as u8, 0, body.is_void(), body).await?   
        }
    };
    ($self_:expr, $reserved:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let reserved = $reserved;
            let body = RequestBody::$name($name {
                $($key: $value,)*
                ..Default::default()
            });
            $self_.send_request(MajorOpcode::$name as u8, reserved, body.is_void(), body).await?
        }
    };
}

#[macro_export]
macro_rules! send_request_ext {
    ($self_:expr, $ext_code:expr, $opcode:expr, $is_void:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let raw = $name {
                $($key: $value,)*
                ..Default::default()
            };
            let mut buf_out = vec![];
            raw.encode_sync(&mut buf_out)?;
            $self_.send_request($ext_code as u8, $opcode as u8, $is_void, RequestBody::Ext(crate::coding::ExtRequest {
                data: buf_out,
            })).await?
        }
    };
}

#[macro_export]
macro_rules! receive_reply {
    ($self_:expr, $seq:expr, $reply:ident, doubled) => {
        $self_.receive_reply_reserved($seq, |x, y| $reply::decode_sync(x, y)).await?
    };
    ($self_:expr, $seq:expr, $reply:ident, fetched) => {
        $self_.receive_reply_reserved($seq, |x, y| Ok(($reply::decode_sync(x)?, y))).await?
    };
    ($self_:expr, $seq:expr, $reply:ident, double_fetched) => {
        $self_.receive_reply_reserved($seq, |x, y| Ok(($reply::decode_sync(x, y)?, y))).await?
    };
    ($self_:expr, $seq:expr, $reply:ident) => {
        $self_.receive_reply($seq, |x| $reply::decode_sync(x)).await?
    };
}
