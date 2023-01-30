use anyhow::Context;
use futures::StreamExt;

pub use crate::coding::{
    shape::{ShapeEventCode, ShapeOpcode},
    x11::{ErrorCode, EventCode, MajorOpcode},
    xfixes::{XFEventCode, XFOpcode},
    xinput2::{XIEventCode, XIOpcode},
    xkb::XKBOpcode,
    xrandr::{XREventCode, XROpcode},
    xrecord::ElementHeader,
    xrecord::XRecordOpcode,
};
use crate::{
    coding::{
        x11::RequestBody,
        xrecord::{
            ContextCategory, CreateContextRequest, DisableContextRequest, EnableContextRequest, EnableContextResponse, FreeContextRequest, GetContextRequest,
            GetContextResponse, RegisterClientsRequest, UnregisterClientsRequest,
        },
        Reply, RequestHeader, Response, ResponseBody,
    },
    net::X11ErrorCode,
};

use super::*;

#[derive(Clone, Copy, Debug)]
pub enum ClientSpec {
    Device,
    CurrentClients,
    FutureClients,
    AllClients,
    XID(u32),
}

impl<'a, R: Resource<'a>> From<R> for ClientSpec {
    fn from(item: R) -> Self {
        ClientSpec::XID(item.x11_handle())
    }
}

impl ClientSpec {
    fn protocol_value(&self) -> u32 {
        match self {
            ClientSpec::Device => 0,
            ClientSpec::CurrentClients => 1,
            ClientSpec::FutureClients => 2,
            ClientSpec::AllClients => 3,
            ClientSpec::XID(x) => *x,
        }
    }
}

#[derive(Clone, Copy, derivative::Derivative)]
#[derivative(Debug)]
pub struct RecordContext<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

impl<'a> Resource<'a> for RecordContext<'a> {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(connection: &'a X11Connection, handle: u32) -> Self {
        Self {
            connection,
            handle,
        }
    }
}

impl X11Connection {
    pub async fn create_record_context(
        &self,
        element_header: ElementHeader,
        client_specs: impl IntoIterator<Item = ClientSpec>,
        targets: impl IntoIterator<Item = RecordTarget>,
    ) -> Result<RecordContext<'_>> {
        let context = self.new_resource_id();

        let ranges = RecordTarget::process_targets(self, targets)?;

        send_request_xrecord!(
            self,
            XRecordOpcode::CreateContext,
            CreateContextRequest {
                context: context,
                element_header: element_header,
                client_specs: client_specs.into_iter().map(|x| x.protocol_value()).collect(),
                ranges: ranges,
            }
        );
        Ok(RecordContext {
            handle: context,
            connection: self,
        })
    }
}

#[derive(Debug, Clone)]
pub struct RecordClientInfo {
    pub raw_handle: u32,
    pub targets: Vec<RecordTarget>,
}

#[derive(Debug, Clone)]
pub struct ReplyMetadata {
    pub client: ClientSpec,
    pub intercept_server_time: Timestamp,
    pub recorded_sequence: u32,
    // only present if element-header contains 'FromServerTime'
    pub recorded_server_time: Option<Timestamp>,
}

#[derive(Debug, Clone)]
pub struct RequestMetadata {
    pub client: ClientSpec,
    pub intercept_server_time: Timestamp,
    pub recorded_sequence: u32,
    // only present if element-header contains 'FromClientTime'
    pub recorded_server_time: Option<Timestamp>,
    // only present if element-header contains 'FromClientSequence'
    pub request_sequence: Option<u32>,
}

//TODO: implement high level abstraction over raw data for request/reply
#[allow(unused_variables)]
pub trait RecordingReceiver {
    fn client_started(&mut self, client: ClientSpec, server_time: Timestamp) {}

    fn client_died(&mut self, client: ClientSpec, server_time: Timestamp, last_sequence: Option<u32>) {}

    fn request(&mut self, major_opcode: u8, minor_opcode: u8, request: RequestBody, metadata: RequestMetadata) {}

    fn reply(&mut self, reply: Reply, metadata: ReplyMetadata) {}

    fn event(&mut self, event: Event, metadata: ReplyMetadata) {}

    fn error(&mut self, error: X11ErrorCode, value: u32, sequence_number: u32, metadata: ReplyMetadata) {}
}

impl<'a> RecordContext<'a> {
    pub async fn register_clients(
        &self,
        element_header: ElementHeader,
        client_specs: impl IntoIterator<Item = ClientSpec>,
        targets: impl IntoIterator<Item = RecordTarget>,
    ) -> Result<()> {
        let ranges = RecordTarget::process_targets(&self.connection, targets)?;

        send_request_xrecord!(
            self.connection,
            XRecordOpcode::RegisterClients,
            RegisterClientsRequest {
                context: self.handle,
                element_header: element_header,
                client_specs: client_specs.into_iter().map(|x| x.protocol_value()).collect(),
                ranges: ranges,
            }
        );
        Ok(())
    }

    pub async fn unregister_clients(&self, client_specs: impl IntoIterator<Item = ClientSpec>) -> Result<()> {
        send_request_xrecord!(
            self.connection,
            XRecordOpcode::UnregisterClients,
            UnregisterClientsRequest {
                context: self.handle,
                client_specs: client_specs.into_iter().map(|x| x.protocol_value()).collect(),
            }
        );
        Ok(())
    }

    pub async fn get(&self) -> Result<Vec<RecordClientInfo>> {
        let reply = send_request_xrecord!(
            self.connection,
            XRecordOpcode::GetContext,
            GetContextResponse,
            GetContextRequest {
                context: self.handle,
            }
        )
        .into_inner();

        let mut out = vec![];
        for client in reply.intercepted_clients {
            out.push(RecordClientInfo {
                raw_handle: client.client_resource,
                targets: RecordTarget::from_targets(&self.connection, client.ranges)?,
            });
        }

        Ok(out)
    }

    pub async fn enable<R: RecordingReceiver>(&self, receiver: &mut R) -> Result<()> {
        let mut stream = send_request_xrecord!(
            self.connection,
            XRecordOpcode::EnableContext,
            stream,
            EnableContextResponse,
            EnableContextRequest {
                context: self.handle,
            }
        );
        let first = stream.next().await.transpose()?.context("missing first StartOfData packet")?;
        let category = ContextCategory::from_repr(first.reserved)?;
        if !matches!(category, ContextCategory::StartOfData) {
            bail!("invalid first record packet, expected StartOfData, got {category:?}");
        }
        while let Some(item) = stream.next().await.transpose()? {
            if item.client_swapped {
                bail!("client_swapped not supported");
            }
            let category = ContextCategory::from_repr(item.reserved)?;
            let client = if item.xid_base == 0 {
                ClientSpec::Device
            } else {
                ClientSpec::XID(item.xid_base)
            };

            match category {
                ContextCategory::FromServer => {
                    let (server_time, start) = if item.element_header.from_server_time() {
                        if item.data.len() < 4 {
                            bail!("malformed data, expected server time, but length is < 0");
                        }
                        (Some(Timestamp(u32::from_be_bytes((&item.data[..4]).try_into().unwrap()))), 4 as usize)
                    } else {
                        (None, 0)
                    };
                    let metadata = ReplyMetadata {
                        client,
                        intercept_server_time: Timestamp(item.server_time),
                        recorded_sequence: item.rec_sequence_num,
                        recorded_server_time: server_time,
                    };
                    let response = Response::decode_sync(&mut &item.data[start..])?;
                    match response.body {
                        ResponseBody::ErrorReply(reply) => {
                            receiver.error(X11ErrorCode::from_raw(self.connection, reply.code), reply.bad_value, reply.sequence_number as u32, metadata);
                        }
                        ResponseBody::Reply(reply) => {
                            receiver.reply(reply, metadata);
                        }
                        ResponseBody::Event(event) => {
                            let event = Event::from_protocol(self.connection, response.code, event).await?;
                            receiver.event(event, metadata);
                        }
                    }
                }
                ContextCategory::FromClient => {
                    let mut data = &item.data[..];
                    let server_time = if item.element_header.from_client_time() {
                        if data.len() < 4 {
                            bail!("malformed data, expected server time, but length is < 4");
                        }
                        let timestamp = Timestamp(u32::from_be_bytes((&data[..4]).try_into().unwrap()));
                        data = &data[4..];
                        Some(timestamp)
                    } else {
                        None
                    };
                    let sequence = if item.element_header.from_client_sequence() {
                        if data.len() < 4 {
                            bail!("malformed data, expected client sequence, but length is < 4");
                        }
                        // add one to get current request sequence
                        let sequence = u32::from_be_bytes((&data[..4]).try_into().unwrap()).wrapping_add(1);
                        data = &data[4..];
                        Some(sequence)
                    } else {
                        None
                    };
                    let metadata = RequestMetadata {
                        client,
                        intercept_server_time: Timestamp(item.server_time),
                        recorded_sequence: item.rec_sequence_num,
                        recorded_server_time: server_time,
                        request_sequence: sequence,
                    };
                    let header = RequestHeader::decode_sync(&mut data)?;
                    let length = if header.length == 0 {
                        header.ext_length.unwrap() as usize
                    } else {
                        header.length as usize
                    };
                    if data.len() < length {
                        bail!("request truncated {} < {}", data.len(), length);
                    }
                    let body = RequestBody::decode_sync(&mut data, header.major_opcode, header.minor_opcode, length as u32)?;
                    receiver.request(header.major_opcode, header.minor_opcode, body, metadata);
                }
                ContextCategory::ClientStarted => receiver.client_started(client, Timestamp(item.server_time)),
                ContextCategory::ClientDied => {
                    let sequence = if item.element_header.from_client_sequence() {
                        if item.data.len() < 4 {
                            bail!("malformed data, expected client sequence, but length is < 4");
                        }
                        // add one to get current request sequence
                        let sequence = u32::from_be_bytes((&item.data[..4]).try_into().unwrap()).wrapping_add(1);
                        Some(sequence)
                    } else {
                        None
                    };

                    receiver.client_died(client, Timestamp(item.server_time), sequence);
                }
                ContextCategory::StartOfData => bail!("unexpected repitition of StartOfData"),
                ContextCategory::EndOfData => break,
            }
        }
        Ok(())
    }

    pub async fn disable(&self) -> Result<()> {
        send_request_xrecord!(
            self.connection,
            XRecordOpcode::DisableContext,
            DisableContextRequest {
                context: self.handle,
            }
        );
        Ok(())
    }

    pub async fn free(&self) -> Result<()> {
        send_request_xrecord!(
            self.connection,
            XRecordOpcode::FreeContext,
            FreeContextRequest {
                context: self.handle,
            }
        );
        Ok(())
    }
}
