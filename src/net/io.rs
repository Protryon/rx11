use std::ops::{Deref, DerefMut};

use anyhow::Context;
use dashmap::mapref::entry::Entry;
use futures::{Stream, StreamExt};
use tokio_stream::wrappers::ReceiverStream;

use crate::coding::{Request, RequestBody, Response, ResponseBody};

use super::*;

pub(crate) struct RequestLen {
    pub request: Request,
    pub len: u64,
}

pub enum ResponseType {
    Void,
    Single(oneshot::Sender<Response>),
    Stream(mpsc::Sender<Response>),
}

impl X11Connection {
    pub async fn send_request(&self, major_opcode: u8, minor_opcode: u8, type_: ResponseType, body: RequestBody) -> Result<()> {
        let mut data = vec![];
        body.encode_sync(&mut data, major_opcode, minor_opcode, 0)?;
        let mut length = data.len() as u32 + 4;
        let out_len = if length % 4 != 0 { (length + 4 - (length % 4)) / 4 } else { length / 4 };
        length -= 4;

        let request = Request {
            major_opcode,
            minor_opcode,
            length: if out_len < u16::MAX as u32 { out_len as u16 } else { 0 },
            ext_length: if out_len >= u16::MAX as u32 { Some(out_len) } else { None },
            data,
        };
        let mut write_data = self.0.write_data.lock().await;
        let seq = write_data.seq;
        write_data.seq += 1;
        self.0.output.responses.insert(
            seq,
            match type_ {
                ResponseType::Void => ResponseValue::InboundVoidError,
                ResponseType::Single(sender) => ResponseValue::Single(sender),
                ResponseType::Stream(sender) => ResponseValue::Stream(sender),
            },
        );
        write_data
            .writer
            .send(RequestLen {
                request,
                len: length as u64,
            })
            .await
            .ok()
            .ok_or_else(|| anyhow!("x11 connection dead"))?;
        drop(write_data);

        Ok(())
    }

    pub async fn end_stream(&self, seq: u16) -> Result<()> {
        match self.0.output.responses.entry(seq) {
            Entry::Occupied(entry) => match entry.get() {
                ResponseValue::Stream(_) => {
                    entry.remove();
                }
                _ => (),
            },
            Entry::Vacant(_) => (),
        }
        Ok(())
    }

    pub async fn send_request_void(&self, major_opcode: u8, minor_opcode: u8, body: RequestBody) -> Result<()> {
        self.send_request(major_opcode, minor_opcode, ResponseType::Void, body).await
    }

    pub async fn send_request_single<T>(
        &self,
        major_opcode: u8,
        minor_opcode: u8,
        body: RequestBody,
        decoder: fn(&mut &[u8], u8) -> Result<T>,
    ) -> Result<T, X11Error> {
        let (sender, receiver) = oneshot::channel();
        self.send_request(major_opcode, minor_opcode, ResponseType::Single(sender), body).await?;
        let response = receiver.await.context("sender dropped")?;
        match response.body {
            ResponseBody::ErrorReply(e) => Err(X11Error::X11Error(X11ErrorReply {
                bad_value: e.bad_value,
                code: X11ErrorCode::from_raw(self, e.code),
            })),
            ResponseBody::Reply(r) => decoder(&mut &r.data[..], r.reserved).map_err(Into::into),
            ResponseBody::Event(_) => unimplemented!(),
        }
    }

    pub async fn send_request_stream<'a, T: Send + Sync + 'static>(
        &'a self,
        major_opcode: u8,
        minor_opcode: u8,
        body: RequestBody,
        decoder: fn(&mut &[u8], u8) -> Result<T>,
    ) -> Result<impl Stream<Item = Result<T, X11Error>> + 'a> {
        let (sender, receiver) = mpsc::channel(5);
        self.send_request(major_opcode, minor_opcode, ResponseType::Stream(sender), body).await?;
        Ok(ReceiverStream::new(receiver).map(move |response| match response.body {
            ResponseBody::ErrorReply(e) => Err(X11Error::X11Error(X11ErrorReply {
                bad_value: e.bad_value,
                code: X11ErrorCode::from_raw(self, e.code),
            })),
            ResponseBody::Reply(r) => decoder(&mut &r.data[..], r.reserved).map_err(Into::into),
            ResponseBody::Event(_) => unimplemented!(),
        }))
        //TODO: when dropped, clear seq
    }
}

pub struct ReservedWrapper<T> {
    inner: T,
    pub reserved: u8,
}

impl<T> Deref for ReservedWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for ReservedWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T> ReservedWrapper<T> {
    pub fn new(inner: T, reserved: u8) -> Self {
        Self {
            inner,
            reserved,
        }
    }

    pub fn into_inner(self) -> T {
        self.inner
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
            assert!(body.is_void());
            $self_.send_request_void(MajorOpcode::$name as u8, 0, body).await?
        }
    };
    ($self_:expr, reserved $reserved:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let reserved = $reserved;
            let body = RequestBody::$name($name {
                $($key: $value,)*
                ..Default::default()
            });
            assert!(body.is_void());
            $self_.send_request_void(MajorOpcode::$name as u8, reserved, body).await?
        }
    };
    ($self_:expr, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let body = RequestBody::$name($name {
                $($key: $value,)*
                ..Default::default()
            });
            assert!(!body.is_void());
            $self_.send_request_single(MajorOpcode::$name as u8, 0, body, |data, reply_reserved| $reply::decode_sync(data).map(|inner| $crate::net::ReservedWrapper::new(inner, reply_reserved))).await?
        }
    };
    ($self_:expr, reserved $reserved:expr, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let reserved = $reserved;
            let body = RequestBody::$name($name {
                $($key: $value,)*
                ..Default::default()
            });
            assert!(!body.is_void());
            $self_.send_request_single(MajorOpcode::$name as u8, reserved, body, |data, reply_reserved| $reply::decode_sync(data).map(|inner| $crate::net::ReservedWrapper::new(inner, reply_reserved))).await?
        }
    };
    ($self_:expr, parse_reserved $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let body = RequestBody::$name($name {
                $($key: $value,)*
                ..Default::default()
            });
            assert!(!body.is_void());
            $self_.send_request_single(MajorOpcode::$name as u8, 0, body, |data, reply_reserved| $reply::decode_sync(data, reply_reserved).map(|inner| $crate::net::ReservedWrapper::new(inner, reply_reserved))).await?
        }
    };
    ($self_:expr, reserved $reserved:expr, parse_reserved $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let reserved = $reserved;
            let body = RequestBody::$name($name {
                $($key: $value,)*
                ..Default::default()
            });
            assert!(!body.is_void());
            $self_.send_request_single(MajorOpcode::$name as u8, reserved, body, |data, reply_reserved| $reply::decode_sync(data, reply_reserved).map(|inner| $crate::net::ReservedWrapper::new(inner, reply_reserved))).await?
        }
    };
    ($self_:expr, stream, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let body = RequestBody::$name($name {
                $($key: $value,)*
                ..Default::default()
            });
            assert!(!body.is_void());
            $self_.send_request_stream(MajorOpcode::$name as u8, 0, body, |data, reply_reserved| $reply::decode_sync(data).map(|inner| $crate::net::ReservedWrapper::new(inner, reply_reserved))).await?
        }
    };
    ($self_:expr, stream, reserved $reserved:expr, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let reserved = $reserved;
            let body = RequestBody::$name($name {
                $($key: $value,)*
                ..Default::default()
            });
            assert!(!body.is_void());
            $self_.send_request_stream(MajorOpcode::$name as u8, reserved, body, |data, reply_reserved| $reply::decode_sync(data).map(|inner| ReservedWrapper::new(inner, reply_reserved))).await?
        }
    };
}

#[macro_export]
macro_rules! encode_request_ext {
    ($name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let body = $name {
                $($key: $value,)*
                ..Default::default()
            };
            let mut buf_out = vec![];
            body.encode_sync(&mut buf_out)?;
            let body = RequestBody::Ext($crate::coding::ExtRequest {
                data: buf_out,
            });
            body
        }
    };
}

#[macro_export]
macro_rules! send_request_ext {
    ($self_:expr, $ext_code:expr, $opcode:expr, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let body = $name {
                $($key: $value,)*
                ..Default::default()
            };
            let mut buf_out = vec![];
            body.encode_sync(&mut buf_out)?;
            let body = RequestBody::Ext($crate::coding::ExtRequest {
                data: buf_out,
            });
            $self_.send_request_void($ext_code as u8, $opcode as u8, body).await?
        }
    };
    ($self_:expr, $ext_code:expr, $opcode:expr, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let body = $name {
                $($key: $value,)*
                ..Default::default()
            };
            let mut buf_out = vec![];
            body.encode_sync(&mut buf_out)?;
            let body = RequestBody::Ext($crate::coding::ExtRequest {
                data: buf_out,
            });
            $self_.send_request_single($ext_code as u8, $opcode as u8, body, |data, reply_reserved| $reply::decode_sync(data).map(|inner| $crate::net::ReservedWrapper::new(inner, reply_reserved))).await?
        }
    };
    ($self_:expr, $ext_code:expr, $opcode:expr, parse_reserved $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let body = $name {
                $($key: $value,)*
                ..Default::default()
            };
            let mut buf_out = vec![];
            body.encode_sync(&mut buf_out)?;
            let body = RequestBody::Ext($crate::coding::ExtRequest {
                data: buf_out,
            });
            $self_.send_request_single($ext_code as u8, $opcode as u8, body, |data, reply_reserved| $reply::decode_sync(data, reply_reserved).map(|inner| $crate::net::ReservedWrapper::new(inner, reply_reserved))).await?
        }
    };
    ($self_:expr, $ext_code:expr, $opcode:expr, stream, $reply:ident, $name:ident { $($key:ident: $value:expr,)* }) => {
        {
            let body = $name {
                $($key: $value,)*
                ..Default::default()
            };
            let mut buf_out = vec![];
            body.encode_sync(&mut buf_out)?;
            let body = RequestBody::Ext($crate::coding::ExtRequest {
                data: buf_out,
            });
            $self_.send_request_stream($ext_code as u8, $opcode as u8, body, |data, reply_reserved| $reply::decode_sync(data).map(|inner| $crate::net::ReservedWrapper::new(inner, reply_reserved))).await?
        }
    };
}
