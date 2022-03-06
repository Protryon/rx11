use super::*;

impl X11Connection {

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
