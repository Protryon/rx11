use crate::{
    coding::{ClientHandshake, Response, ResponseBody, ServerHandshake, ServerHandshakeBody},
    connection::{TcpConnection, UnixConnection},
};

use super::*;
use dashmap::mapref::entry::Entry;
use tokio::{
    io::{AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, BufWriter},
    sync::{broadcast, mpsc, Mutex},
};

impl X11Connection {
    async fn writer_thread(mut writer: BufWriter<impl AsyncWrite + Unpin + Send + Sync>, mut in_receiver: mpsc::Receiver<RequestLen>) -> Result<()> {
        while let Some(request) = in_receiver.recv().await {
            request.request.encode_async(&mut writer, request.len).await?;
            writer.flush().await?;
        }
        Ok(())
    }

    async fn reader_thread(
        mut reader: BufReader<impl AsyncRead + Unpin + Send + Sync>,
        output: Arc<X11OutputContext>,
        events: broadcast::Sender<(u8, crate::coding::Event)>,
    ) -> Result<()> {
        loop {
            let response = Response::decode_async(&mut reader).await?;
            match response.body {
                ResponseBody::Event(event) => {
                    if let Err(_) = events.send((response.code, event)) {
                        warn!("failed to send x11 event (no listeners)");
                    }
                }
                ResponseBody::ErrorReply(error) => {
                    let entry = output.responses.entry(error.sequence_number);
                    let pending_error = match entry {
                        Entry::Vacant(_) => {
                            warn!("received unexpected error for seq {}: {:?}", error.sequence_number, error);
                            Some(error)
                        }
                        Entry::Occupied(mut occupied) => match &mut *occupied.get_mut() {
                            ResponseValue::InboundVoidError => {
                                debug!("inbound x11 error: {:?} <{}>", error.code, error.bad_value);
                                Some(error)
                            }
                            ResponseValue::Single(_) => {
                                let sender = occupied.remove();
                                match sender {
                                    ResponseValue::Single(sender) => {
                                        let _ = sender.send(Response {
                                            code: response.code,
                                            body: ResponseBody::ErrorReply(error),
                                        });
                                    }
                                    _ => unimplemented!(),
                                }
                                None
                            }
                            ResponseValue::Stream(sender) => {
                                if sender
                                    .send(Response {
                                        code: response.code,
                                        body: ResponseBody::ErrorReply(error),
                                    })
                                    .await
                                    .is_err()
                                {
                                    occupied.remove();
                                }
                                None
                            }
                        },
                    };
                    if let Some(pending_error) = pending_error {
                        output.pending_errors.lock().await.push(pending_error);
                    }
                }
                ResponseBody::Reply(reply) => {
                    let entry = output.responses.entry(reply.sequence_number);
                    match entry {
                        Entry::Vacant(_) => {
                            warn!("received unexpected reply for seq {}: {:?}", reply.sequence_number, reply);
                        }
                        Entry::Occupied(mut occupied) => match &mut *occupied.get_mut() {
                            ResponseValue::InboundVoidError => {
                                warn!("received unexpected reply to void request: {:?}", reply);
                            }
                            ResponseValue::Single(_) => {
                                let sender = occupied.remove();
                                match sender {
                                    ResponseValue::Single(sender) => {
                                        let _ = sender.send(Response {
                                            code: response.code,
                                            body: ResponseBody::Reply(reply),
                                        });
                                    }
                                    _ => unimplemented!(),
                                }
                            }
                            ResponseValue::Stream(sender) => {
                                if sender
                                    .send(Response {
                                        code: response.code,
                                        body: ResponseBody::Reply(reply),
                                    })
                                    .await
                                    .is_err()
                                {
                                    occupied.remove();
                                }
                            }
                        },
                    }
                }
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

    pub async fn open(writer: impl AsyncWrite + Unpin + Send + Sync + 'static, reader: impl AsyncRead + Unpin + Send + Sync + 'static) -> Result<Self> {
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
            }
            ServerHandshakeBody::AuthRequired(f) => {
                bail!("failed to connect to server, auth required: {}", f.reason);
            }
            ServerHandshakeBody::Success(packet) => packet,
        };

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

        let mut self_ = Self(Arc::new(X11ConnectionInterior {
            output,
            write_data: Mutex::new(WriteData {
                seq: 1,
                writer: in_sender,
            }),
            handshake,
            next_resource_id: AtomicU32::new(0),
            known_atoms: DashMap::new(),
            known_atoms_inverse: DashMap::new(),
            registered_extensions: DashMap::new(),
            events_sender,
        }));
        self_.register_const_atoms();

        self_.init_state().await;

        Ok(self_)
    }
}
