use crate::coding::{xrandr::{GetProvidersRequest, GetProvidersResponse, GetProviderInfoRequest, GetProviderInfoResponse, SetProviderOffloadSinkRequest, SetProviderOutputSourceRequest, ListProviderPropertiesResponse, ListProviderPropertiesRequest, QueryProviderPropertyRequest, QueryProviderPropertyResponse, ConfigureProviderPropertyRequest, ChangePropertyMode, ChangeProviderPropertyRequest, PropertyFormat, DeleteProviderPropertyRequest, GetProviderPropertyRequest, GetProviderPropertyResponse}, self};
pub use crate::coding::xrandr::{
    ProviderCapability,
};
use super::*;

#[derive(Clone, Copy)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Provider<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

impl<'a> Resource<'a> for Provider<'a> {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(connection: &'a X11Connection, handle: u32) -> Self {
        Self { connection, handle }
    }
}

impl<'a> Window<'a> {
    pub async fn get_providers(self) -> Result<(Timestamp, Vec<Provider<'a>>)> {
        let seq = send_request_xrandr!(self.connection, XROpcode::GetProviders, false, GetProvidersRequest {
            window: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, GetProvidersResponse);

        Ok((
            Timestamp(reply.time),
            reply.providers.into_iter().map(|handle| Provider { handle, connection: self.connection }).collect(),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct ProviderInfo<'a> {
    pub status: SetConfig,
    pub time: Timestamp,
    pub capabilities: ProviderCapability,
    pub crtcs: Vec<Crtc<'a>>,
    pub outputs: Vec<Output<'a>>,
    pub associated_providers: Vec<(Provider<'a>, ProviderCapability)>,
    pub name: String,
}

impl<'a> Provider<'a> {
    pub async fn get_info(self, config_time: Timestamp) -> Result<ProviderInfo<'a>> {
        let seq = send_request_xrandr!(self.connection, XROpcode::GetProviderInfo, false, GetProviderInfoRequest {
            provider: self.handle,
            config_time: config_time.0,
        });
        let (reply, status) = receive_reply!(self.connection, seq, GetProviderInfoResponse, fetched);

        Ok(ProviderInfo {
            status: SetConfig::from_repr(status)?,
            time: Timestamp(reply.time),
            capabilities: reply.capabilities,
            crtcs: reply.crtcs.into_iter().map(|handle| Crtc { connection: self.connection, handle }).collect(),
            outputs: reply.outputs.into_iter().map(|handle| Output { connection: self.connection, handle }).collect(),
            associated_providers: reply.associated_providers.into_iter().zip(reply.associated_provider_capability.into_iter())
                .map(|(handle, capability)| (Provider { connection: self.connection, handle }, capability)).collect(),
            name: reply.name,
        })
    }

    pub async fn set_offload_sink(self, sink_provider: Provider<'_>, config_time: Timestamp) -> Result<()> {
        send_request_xrandr!(self.connection, XROpcode::SetProviderOffloadSink, true, SetProviderOffloadSinkRequest {
            provider: self.handle,
            sink_provider: sink_provider.handle,
            config_time: config_time.0,
        });

        Ok(())
    }

    pub async fn set_output_source(self, source_provider: Provider<'_>, config_time: Timestamp) -> Result<()> {
        send_request_xrandr!(self.connection, XROpcode::SetProviderOutputSource, true, SetProviderOutputSourceRequest {
            provider: self.handle,
            source_provider: source_provider.handle,
            config_time: config_time.0,
        });

        Ok(())
    }

    pub async fn list_properties(self) -> Result<Vec<Atom>> {
        let seq = send_request_xrandr!(self.connection, XROpcode::ListProviderProperties, false, ListProviderPropertiesRequest {
            provider: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, ListProviderPropertiesResponse);

        self.connection.get_all_atoms(reply.property_atoms).await
    }

    pub async fn query_property(self, property: Atom) -> Result<PropertyConfig> {
        let seq = send_request_xrandr!(self.connection, XROpcode::QueryProviderProperty, false, QueryProviderPropertyRequest {
            provider: self.handle,
            property_atom: property.handle,
        });
        let reply = receive_reply!(self.connection, seq, QueryProviderPropertyResponse);

        Ok(PropertyConfig {
            pending: reply.pending,
            range: reply.range,
            immutable: reply.immutable,
            values: reply.valid_values,
        })
    }

    pub async fn configure_property(self, property: Atom, config: PropertyConfig) -> Result<()> {
        send_request_xrandr!(self.connection, XROpcode::ConfigureProviderProperty, true, ConfigureProviderPropertyRequest {
            provider: self.handle,
            property_atom: property.handle,
            pending: config.pending,
            range: config.range,
            values: config.values,
        });

        Ok(())
    }
    
    async fn change_property(self, mode: ChangePropertyMode, property: Atom, type_: Atom, value: impl Into<PropertyValue>) -> Result<()> {
        let value = value.into();
        send_request_xrandr!(self.connection, XROpcode::ChangeProviderProperty, true, ChangeProviderPropertyRequest {
            provider: self.handle,
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
        send_request_xrandr!(self.connection, XROpcode::DeleteProviderProperty, true, DeleteProviderPropertyRequest {
            provider: self.handle,
            property_atom: property.handle,
        });
        Ok(())
    }

    pub async fn get_property_full(self, property: Atom, type_: Option<Atom>, offset: u32, length: u32, delete: bool, pending: bool) -> Result<PropertyResponse> {
        let seq = send_request_xrandr!(self.connection, XROpcode::GetProviderProperty, false, GetProviderPropertyRequest {
            provider: self.handle,
            delete: delete,
            pending: pending,
            property_atom: property.handle,
            type_atom: type_.map(|x| x.handle).unwrap_or(0),
            long_offset: offset,
            long_length: length,
        });
        let reply = self.connection.receive_reply_reserved(seq, |x, y| GetProviderPropertyResponse::decode_sync(x, PropertyFormat::from_repr(y)?)).await?;

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
