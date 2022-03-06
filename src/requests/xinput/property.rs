use crate::coding::xinput2::{XIListPropertiesResponse, XIListPropertiesRequest, ChangePropertyMode, XIChangePropertyRequest, PropertyFormat, self, XIDeletePropertyRequest, XIGetPropertyRequest, XIGetPropertyResponse};

use super::*;

#[derive(Clone, Debug)]
pub enum PropertyValue {
    L8(Vec<u8>),
    L16(Vec<u16>),
    L32(Vec<u32>),
}

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

    async fn change_property(self, mode: ChangePropertyMode, property: Atom, type_: Atom, value: PropertyValue) -> Result<()> {
        send_request_xinput!(self.connection, XIOpcode::XIChangeProperty, true, XIChangePropertyRequest {
            device: self.id,
            mode: mode,
            format: match &value {
                PropertyValue::L8(_) => PropertyFormat::L8,
                PropertyValue::L16(_) => PropertyFormat::L16,
                PropertyValue::L32(_) => PropertyFormat::L32,
            },
            property_atom: property.handle,
            type_atom: type_.handle,
            value: match value {
                PropertyValue::L8(items_8) => xinput2::PropertyValue::Items8 { items_8, num_items: 0 },
                PropertyValue::L16(items_16) => xinput2::PropertyValue::Items16 { items_16, num_items: 0 },
                PropertyValue::L32(items_32) => xinput2::PropertyValue::Items32 { items_32, num_items: 0 },
            },
        });
        Ok(())
    }

    pub async fn replace_property(self, property: Atom, type_: Atom, value: PropertyValue) -> Result<()> {
        self.change_property(ChangePropertyMode::Replace, property, type_, value).await
    }

    pub async fn prepend_property(self, property: Atom, type_: Atom, value: PropertyValue) -> Result<()> {
        self.change_property(ChangePropertyMode::Prepend, property, type_, value).await
    }

    pub async fn append_property(self, property: Atom, type_: Atom, value: PropertyValue) -> Result<()> {
        self.change_property(ChangePropertyMode::Append, property, type_, value).await
    }

    pub async fn delete_property(self, property: Atom) -> Result<()> {
        send_request_xinput!(self.connection, XIOpcode::XIDeleteProperty, true, XIDeletePropertyRequest {
            device: self.id,
            property_atom: property.handle,
        });
        Ok(())
    }

    pub async fn get_property(self, property: Atom, type_: Option<Atom>, offset: u32, length: u32, delete: bool) -> Result<PropertyResponse> {
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
                XIGetPropertyResponse { items_8: Some(items_8), .. } => PropertyValue::L8(items_8),
                XIGetPropertyResponse { items_16: Some(items_16), .. } => PropertyValue::L16(items_16),
                XIGetPropertyResponse { items_32: Some(items_32), .. } => PropertyValue::L32(items_32),
                _ => bail!("malformed getproperty response"),
            },
        })
    }

}