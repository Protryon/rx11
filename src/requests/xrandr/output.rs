use super::*;
use crate::coding::{xrandr::{GetOutputInfoRequest, GetOutputInfoResponse, ListOutputPropertiesRequest, ListOutputPropertiesResponse, ChangeOutputPropertyRequest, PropertyFormat, DeleteOutputPropertyRequest, GetOutputPropertyRequest, GetOutputPropertyResponse, AddOutputModeRequest, DeleteOutputModeRequest, QueryOutputPropertyRequest, QueryOutputPropertyResponse, ConfigureOutputPropertyRequest}, self};
pub use crate::coding::xrandr::{
    Connection,
    ChangePropertyMode,
};

#[derive(Clone, Copy)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Output<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

impl<'a> Resource<'a> for Output<'a> {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(connection: &'a X11Connection, handle: u32) -> Self {
        Self { connection, handle }
    }
}

#[derive(Clone, Debug)]
pub struct OutputInfo<'a> {
    pub status: SetConfig,
    pub time: Timestamp,
    pub crtc: Crtc<'a>,
    pub mm_width: u32,
    pub mm_height: u32,
    pub connection: Connection,
    pub subpixel_order: SubPixel,
    pub crtcs: Vec<Crtc<'a>>,
    pub modes: Vec<Mode<'a>>,
    pub clones: Vec<Output<'a>>,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct PropertyConfig {
    pub pending: bool,
    pub range: bool,
    /// cannot be sent in configure! will be ignored
    pub immutable: bool,
    pub values: Vec<i32>,
}

impl<'a> Output<'a> {
    pub async fn get_info(self, config_time: Timestamp) -> Result<OutputInfo<'a>> {
        let seq = send_request_xrandr!(self.connection, XROpcode::GetOutputInfo, false, GetOutputInfoRequest {
            output: self.handle,
            config_time: config_time.0,
        });
        let (reply, status) = receive_reply!(self.connection, seq, GetOutputInfoResponse, fetched);

        Ok(OutputInfo {
            status: SetConfig::from_repr(status)?,
            time: Timestamp(reply.time),
            crtc: Crtc { connection: self.connection, handle: reply.crtc },
            mm_width: reply.width_mm,
            mm_height: reply.height_mm,
            connection: reply.connection,
            subpixel_order: reply.subpixel_order,
            crtcs: reply.crtcs.into_iter().map(|handle| Crtc { connection: self.connection, handle }).collect(),
            modes: reply.modes.into_iter().map(|handle| Mode { connection: self.connection, handle }).collect(),
            clones: reply.clone_outputs.into_iter().map(|handle| Output { connection: self.connection, handle }).collect(),
            name: reply.name,
        })
    }

    pub async fn add_mode(self, mode: Mode<'_>) -> Result<()> {
        send_request_xrandr!(self.connection, XROpcode::AddOutputMode, true, AddOutputModeRequest {
            output: self.handle,
            mode: mode.handle,
        });
        Ok(())
    }

    pub async fn delete_mode(self, mode: Mode<'_>) -> Result<()> {
        send_request_xrandr!(self.connection, XROpcode::DeleteOutputMode, true, DeleteOutputModeRequest {
            output: self.handle,
            mode: mode.handle,
        });
        Ok(())
    }

    pub async fn list_properties(self) -> Result<Vec<Atom>> {
        let seq = send_request_xrandr!(self.connection, XROpcode::ListOutputProperties, false, ListOutputPropertiesRequest {
            output: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, ListOutputPropertiesResponse);

        self.connection.get_all_atoms(reply.property_atoms).await
    }

    pub async fn query_property(self, property: Atom) -> Result<PropertyConfig> {
        let seq = send_request_xrandr!(self.connection, XROpcode::QueryOutputProperty, false, QueryOutputPropertyRequest {
            output: self.handle,
            property_atom: property.handle,
        });
        let reply = receive_reply!(self.connection, seq, QueryOutputPropertyResponse);

        Ok(PropertyConfig {
            pending: reply.pending,
            range: reply.range,
            immutable: reply.immutable,
            values: reply.valid_values,
        })
    }

    pub async fn configure_property(self, property: Atom, config: PropertyConfig) -> Result<()> {
        send_request_xrandr!(self.connection, XROpcode::ConfigureOutputProperty, true, ConfigureOutputPropertyRequest {
            output: self.handle,
            property_atom: property.handle,
            pending: config.pending,
            range: config.range,
            values: config.values,
        });

        Ok(())
    }
    
    async fn change_property(self, mode: ChangePropertyMode, property: Atom, type_: Atom, value: impl Into<PropertyValue>) -> Result<()> {
        let value = value.into();
        send_request_xrandr!(self.connection, XROpcode::ChangeOutputProperty, true, ChangeOutputPropertyRequest {
            output: self.handle,
            mode: mode,
            format: match &value {
                PropertyValue::U8(_) => PropertyFormat::L8,
                PropertyValue::U16(_) => PropertyFormat::L16,
                PropertyValue::U32(_) => PropertyFormat::L32,
            },
            property_atom: property.handle,
            type_atom: type_.handle,
            value: match value {
                PropertyValue::U8(items_8) => coding::xrandr::PropertyValue::Items8 { items_8, num_items: 0 },
                PropertyValue::U16(items_16) => coding::xrandr::PropertyValue::Items16 { items_16, num_items: 0 },
                PropertyValue::U32(items_32) => coding::xrandr::PropertyValue::Items32 { items_32, num_items: 0 },
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
        send_request_xrandr!(self.connection, XROpcode::DeleteOutputProperty, true, DeleteOutputPropertyRequest {
            output: self.handle,
            property_atom: property.handle,
        });
        Ok(())
    }

    pub async fn get_property_full(self, property: Atom, type_: Option<Atom>, offset: u32, length: u32, delete: bool, pending: bool) -> Result<PropertyResponse> {
        let seq = send_request_xrandr!(self.connection, XROpcode::GetOutputProperty, false, GetOutputPropertyRequest {
            output: self.handle,
            delete: delete,
            pending: pending,
            property_atom: property.handle,
            type_atom: type_.map(|x| x.handle).unwrap_or(0),
            long_offset: offset,
            long_length: length,
        });
        let reply = self.connection.receive_reply_reserved(seq, |x, y| GetOutputPropertyResponse::decode_sync(x, PropertyFormat::from_repr(y)?)).await?;

        Ok(PropertyResponse {
            type_: self.connection.get_atom_name(reply.type_atom).await?,
            bytes_after: reply.bytes_after,
            value: match reply.value {
                coding::xrandr::PropertyValue::Items8 { items_8, .. } => PropertyValue::U8(items_8),
                coding::xrandr::PropertyValue::Items16 { items_16, .. } => PropertyValue::U16(items_16),
                coding::xrandr::PropertyValue::Items32 { items_32, .. } => PropertyValue::U32(items_32),
            },
        })
    }

    pub async fn get_property(self, property: Atom, type_: Option<Atom>) -> Result<PropertyValue> {
        self.get_property_full(property, type_, 0, u32::MAX, false, false).await.map(|x| x.value)
    }
    
    pub async fn get_property_string(self, property: Atom, type_: Option<Atom>) -> Result<String> {
        let response = self.get_property_full(property, type_, 0, u32::MAX, false, false).await?;
        match response.value {
            PropertyValue::U8(value) => Ok(String::from_utf8(value)?),
            _ => bail!("invalid non-string response value"),
        }
    }
}