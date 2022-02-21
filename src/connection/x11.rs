use std::{collections::BTreeMap, sync::{atomic::{AtomicU16, Ordering, AtomicU32}, Arc}, fmt};

use tokio::{io::{AsyncRead, AsyncWrite, BufReader, BufWriter, AsyncWriteExt}, sync::{mpsc, oneshot, Mutex, broadcast}};
use crate::{coding::{x11::{Request, Response, ClientHandshake, ServerHandshake, ServerHandshakeBody, ServerHandshakeSuccess}, RequestBody, ResponseBody, ErrorReply, Screen}, requests::{Depth, VisualType, Visual}, events::Event};
use dashmap::{DashMap, mapref::entry::Entry};

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

pub struct X11Connection {
    writer: mpsc::Sender<RequestLen>,
    output: Arc<X11OutputContext>,
    seq: AtomicU16,
    next_resource_id: AtomicU32,
    handshake: ServerHandshakeSuccess,
    depths: BTreeMap<u8, Depth>,
    events_sender: broadcast::Sender<(u8, crate::coding::Event)>,
    pub(crate) known_atoms: DashMap<&'static str, u32>,
    pub(crate) known_atoms_inverse: DashMap<u32, &'static str>,
}

pub type X11ConnectionOwned = Arc<X11Connection>;

const PROTOCOL_MAJOR_VERSION: u16 = 11;
const PROTOCOL_MINOR_VERSION: u16 = 0;

impl fmt::Display for ErrorReply {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x11 error: {:?} <{}>", self.code, self.bad_value)
    }
}

impl std::error::Error for ErrorReply {

}

pub enum X11Error {
    Error(anyhow::Error),
    X11Error(ErrorReply),
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
        Ok(())
    }

    pub async fn connect(host: &str, display: u16) -> Result<Arc<Self>> {
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
    ) -> Result<Arc<Self>> {
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

        let self_ = Self {
            output,
            writer: in_sender,
            handshake,
            seq: AtomicU16::new(1),
            next_resource_id: AtomicU32::new(0),
            known_atoms: DashMap::new(),
            known_atoms_inverse: DashMap::new(),
            events_sender,
            depths
        };
        self_.register_const_atoms();
        Ok(Arc::new(self_))
    }

    pub fn handshake(&self) -> &ServerHandshakeSuccess {
        &self.handshake
    }

    pub(crate) fn new_resource_id(&self) -> u32 {
        let raw = self.next_resource_id.fetch_add(1, Ordering::SeqCst);
        (raw << self.handshake.resource_id_mask.trailing_zeros()) | self.handshake.resource_id_base
    }

    pub async fn send_request(&self, major_opcode: u8, minor_opcode: u8, body: RequestBody) -> Result<u16> {
        let is_void = body.is_void();
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
        let seq = self.seq.fetch_add(1, Ordering::SeqCst);
        if is_void {
            self.output.responses.insert(seq, ResponseValue::InboundVoidError);
        }
        self.writer.send(RequestLen { request, len: length as u64 }).await
            .ok().ok_or_else(|| anyhow!("x11 connection dead"))?;
        Ok(seq)
    }

    pub async fn receive_response(&self, seq: u16) -> Result<Response> {
        enum EntryValue {
            Receiver(oneshot::Receiver<Response>),
            Value(Response),
        }

        let entry = self.output.responses.entry(seq);
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
                Err(X11Error::X11Error(e))
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
                Err(X11Error::X11Error(e))
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
            receiver: self.events_sender.subscribe(),
        }
    }

    pub fn depths(&self) -> &BTreeMap<u8, Depth> {
        &self.depths
    }

    pub fn screen(&self) -> &Screen {
        &self.handshake().screens[0]
    }

    pub async fn check_errors(&self) -> Vec<ErrorReply> {
        let mut errors = self.output.pending_errors.lock().await;
        errors.drain(..).collect()
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
    pub async fn recv(&mut self) -> Option<Event<'_>> {
        self.receiver.recv().await
            .ok()
            .map(|(code, event)| Event::from_protocol(self.connection, code, event))
    }
}

#[macro_export]
macro_rules! send_request {
    ($self_:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        $self_.send_request(MajorOpcode::$name as u8, 0, RequestBody::$name($name {
            $($key: $value,)*
            ..Default::default()
        })).await?
    };
    ($self_:expr, $reserved:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        $self_.send_request(MajorOpcode::$name as u8, $reserved, RequestBody::$name($name {
            $($key: $value,)*
            ..Default::default()
        })).await?
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

// impl Response {
//     fn seq(&self) -> Option<u16> {
//         self.body.seq()
//     }
// }

// impl ResponseBody {
//     fn seq(&self) -> Option<u16> {
//         match self {
//             ResponseBody::ErrorReply(e) => {
//                 Some(e.sequence_number)
//             },
//             ResponseBody::Reply(e) => {
//                 Some(e.sequence_number)
//             },
//             ResponseBody::Event(_) => None,
//         }
//     }
// }