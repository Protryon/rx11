use crate::coding::xinput2::{XIListPropertiesResponse, XIListPropertiesRequest, ChangePropertyMode, XIChangePropertyRequest, PropertyFormat, self, XIDeletePropertyRequest, XIGetPropertyRequest, XIGetPropertyResponse};

use super::*;

#[derive(Clone, Debug)]
pub struct PropertyResponse {
    pub type_: Atom,
    pub bytes_after: u32,
    pub value: PropertyValue,
}

impl<'a> Device<'a> {

    pub async fn list_properties(self) -> Result<Vec<Atom>> {
        let seq = send_request_xinput!(self.connection, XIOpcode::XIListProperties, false, XIListPropertiesRequest {
            device: self.id,
        });
        let reply = receive_reply!(self.connection, seq, XIListPropertiesResponse);

        self.connection.get_all_atoms(reply.property_atoms).await
    }

    async fn change_property(self, mode: ChangePropertyMode, property: Atom, type_: Atom, value: impl Into<PropertyValue>) -> Result<()> {
        let value = value.into();
        send_request_xinput!(self.connection, XIOpcode::XIChangeProperty, true, XIChangePropertyRequest {
            device: self.id,
            mode: mode,
            format: match &value {
                PropertyValue::U8(_) => PropertyFormat::L8,
                PropertyValue::U16(_) => PropertyFormat::L16,
                PropertyValue::U32(_) => PropertyFormat::L32,
            },
            property_atom: property.handle,
            type_atom: type_.handle,
            value: match value {
                PropertyValue::U8(items_8) => xinput2::PropertyValue::Items8 { items_8, num_items: 0 },
                PropertyValue::U16(items_16) => xinput2::PropertyValue::Items16 { items_16, num_items: 0 },
                PropertyValue::U32(items_32) => xinput2::PropertyValue::Items32 { items_32, num_items: 0 },
            },
        });
        Ok(())
    }

    pub async fn replace_property(self, property: Atom, type_: Atom, value: impl Into<PropertyValue>) -> Result<()> {
        self.change_property(ChangePropertyMode::Replace, property, type_, value).await
    }

    pub async fn prepend_property(self, property: Atom, type_: Atom, value: impl Into<PropertyValue>) -> Result<()> {
        self.change_property(ChangePropertyMode::Prepend, property, type_, value).await
    }

    pub async fn append_property(self, property: Atom, type_: Atom, value: impl Into<PropertyValue>) -> Result<()> {
        self.change_property(ChangePropertyMode::Append, property, type_, value).await
    }

    pub async fn delete_property(self, property: Atom) -> Result<()> {
        send_request_xinput!(self.connection, XIOpcode::XIDeleteProperty, true, XIDeletePropertyRequest {
            device: self.id,
            property_atom: property.handle,
        });
        Ok(())
    }

    pub async fn get_property_full(self, property: Atom, type_: Option<Atom>, offset: u32, length: u32, delete: bool) -> Result<PropertyResponse> {
        let seq = send_request_xinput!(self.connection, XIOpcode::XIGetProperty, false, XIGetPropertyRequest {
            device: self.id,
            delete: delete,
            property_atom: property.handle,
            type_atom: type_.map(|x| x.handle).unwrap_or(0),
            offset: offset,
            len: length,
        });
        let reply = receive_reply!(self.connection, seq, XIGetPropertyResponse);

        Ok(PropertyResponse {
            type_: self.connection.get_atom_name(reply.type_atom).await?,
            bytes_after: reply.bytes_after,
            value: match reply {
                XIGetPropertyResponse { items_8: Some(items_8), .. } => PropertyValue::U8(items_8),
                XIGetPropertyResponse { items_16: Some(items_16), .. } => PropertyValue::U16(items_16),
                XIGetPropertyResponse { items_32: Some(items_32), .. } => PropertyValue::U32(items_32),
                _ => bail!("malformed getproperty response"),
            },
        })
    }

    pub async fn get_property(self, property: Atom, type_: Option<Atom>) -> Result<PropertyValue> {
        self.get_property_full(property, type_, 0, u32::MAX, false).await.map(|x| x.value)
    }
    
    pub async fn get_property_string(self, property: Atom, type_: Option<Atom>) -> Result<String> {
        let response = self.get_property_full(property, type_, 0, u32::MAX, false).await?;
        match response.value {
            PropertyValue::U8(value) => Ok(String::from_utf8(value)?),
            _ => bail!("invalid non-string response value"),
        }
    }
}