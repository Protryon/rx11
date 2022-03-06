use super::*;

#[derive(Debug, Clone)]
pub struct GetPropertyResult {
    pub type_: Atom,
    pub bytes_after: u32,
    pub value: PropertyValue,
}

#[derive(Debug, Clone)]
pub enum PropertyValue {
    U8(Vec<u8>),
    U16(Vec<u16>),
    U32(Vec<u32>),
}

impl From<Vec<u8>> for PropertyValue {
    fn from(value: Vec<u8>) -> Self {
        PropertyValue::U8(value)
    }
}

impl From<String> for PropertyValue {
    fn from(value: String) -> Self {
        PropertyValue::U8(value.into_bytes())
    }
}

impl From<&str> for PropertyValue {
    fn from(value: &str) -> Self {
        PropertyValue::U8(value.as_bytes().to_vec())
    }
}

impl From<Vec<u16>> for PropertyValue {
    fn from(value: Vec<u16>) -> Self {
        PropertyValue::U16(value)
    }
}

impl From<Vec<u32>> for PropertyValue {
    fn from(value: Vec<u32>) -> Self {
        PropertyValue::U32(value)
    }
}

impl<'a> Window<'a> {
    pub async fn set_property_string<S: AsRef<str>>(&self, property: Atom, value: S) -> Result<()> {
        self.replace_property(property, Atom::STRING, value.as_ref()).await
    }

    pub async fn append_property<P: Into<PropertyValue>>(&self, property: Atom, type_: Atom, data: P) -> Result<()> {
        self.change_property(property, type_, ChangePropertyMode::Append, data).await
    }

    pub async fn prepend_property<P: Into<PropertyValue>>(&self, property: Atom, type_: Atom, data: P) -> Result<()> {
        self.change_property(property, type_, ChangePropertyMode::Prepend, data).await
    }

    pub async fn replace_property<P: Into<PropertyValue>>(&self, property: Atom, type_: Atom, data: P) -> Result<()> {
        self.change_property(property, type_, ChangePropertyMode::Replace, data).await
    }

    async fn change_property<P: Into<PropertyValue>>(&self, property: Atom, type_: Atom, mode: ChangePropertyMode, data: P) -> Result<()> {
        let data = data.into();
        let (format, length) = match &data {
            PropertyValue::U8(data) => (ChangePropertyFormat::L8, data.len()),
            PropertyValue::U16(data) => (ChangePropertyFormat::L16, data.len()),
            PropertyValue::U32(data) => (ChangePropertyFormat::L32, data.len()),
        };
        let raw_data = match data {
            PropertyValue::U8(data) => data,
            PropertyValue::U16(data) => data.into_iter().flat_map(|x| x.to_be_bytes()).collect(),
            PropertyValue::U32(data) => data.into_iter().flat_map(|x| x.to_be_bytes()).collect(),
        };

        send_request!(self.connection, mode as u8, ChangeProperty {
            window: self.handle,
            property: property.handle,
            type_: type_.handle,
            format: format,
            length: length as u32,
            data: raw_data,
        });

        Ok(())
    }

    pub async fn delete_property(&self, property: Atom) -> Result<()> {
        send_request!(self.connection, DeleteProperty {
            window: self.handle,
            property: property.handle,
        });

        Ok(())
    }

    pub async fn get_property_full(&self, property: Atom, type_: Option<Atom>, long_offset: u32, long_length: u32, delete: bool) -> Result<GetPropertyResult> {
        let seq = send_request!(self.connection, delete as u8, GetProperty {
            window: self.handle,
            property: property.handle,
            type_: type_.map(|x| x.handle).unwrap_or(0),
            long_offset: long_offset,
            long_length: long_length,
        });
        let (reply, format) = receive_reply!(self.connection, seq, GetPropertyReply, double_fetched);

        Ok(GetPropertyResult {
            type_: self.connection.get_atom_name(reply.type_).await?,
            bytes_after: reply.bytes_after,
            value: match format {
                0 => PropertyValue::U8(vec![]),
                8 => PropertyValue::U8(reply.value),
                16 => PropertyValue::U16(reply.value.chunks_exact(2).map(|x| u16::from_be_bytes(x.try_into().unwrap())).collect()),
                32 => PropertyValue::U32(reply.value.chunks_exact(4).map(|x| u32::from_be_bytes(x.try_into().unwrap())).collect()),
                _ => bail!("invalid format: {}", format),
            },
        })
    }

    pub async fn get_property(&self, property: Atom, type_: Option<Atom>) -> Result<PropertyValue> {
        self.get_property_full(property, type_, 0, u32::MAX, false).await.map(|x| x.value)
    }

    pub async fn list_properties(&self) -> Result<Vec<Atom>> {
        let seq = send_request!(self.connection, ListProperties {
            window: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, ListPropertiesReply);

        let mut out = vec![];
        for atom in reply.atoms {
            out.push(self.connection.get_atom_name(atom).await?);
        }
        Ok(out)
    }

    pub async fn rotate_properties(&self, properties: &[Atom], delta: i16) -> Result<()> {
        send_request!(self.connection, RotateProperties {
            window: self.handle,
            properties: properties.iter().map(|x| x.handle).collect(),
            delta: delta,
        });

        Ok(())
    }
}