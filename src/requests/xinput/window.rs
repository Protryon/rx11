use crate::coding::xinput2::{XISetClientPointerRequest, XIGetClientPointerRequest, XIGetClientPointerResponse, XISelectEventsRequest, XIEventMasks, XIGetSelectedEventsRequest, XIGetSelectedEventsResponse};

use super::*;

impl<'a> Window<'a> {

    pub async fn set_client_pointer(&self, device: Device<'_>) -> Result<()> {
        send_request_xinput!(self.connection, XIOpcode::XISetClientPointer, true, XISetClientPointerRequest {
            device: device.id,
            window: self.handle,
        });
        Ok(())
    }

    pub async fn get_client_pointer(&self) -> Result<Option<Device<'_>>> {
        let seq = send_request_xinput!(self.connection, XIOpcode::XIGetClientPointer, false, XIGetClientPointerRequest {
            window: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, XIGetClientPointerResponse);

        Ok(if reply.set {
            Some(Device {
                id: reply.device,
                connection: self.connection,
            })
        } else {
            None
        })
    }

    pub async fn xi_select_events(&self, masks: impl IntoIterator<Item=(Device<'_>, XIEventMask)>) -> Result<()> {
        send_request_xinput!(self.connection, XIOpcode::XISelectEvents, true, XISelectEventsRequest {
            window: self.handle,
            masks: masks.into_iter().map(|(device, mask)| XIEventMasks { device: device.id, mask_num: 0, masks: vec![mask] }).collect(),
        });
        Ok(())
    }

    pub async fn xi_get_selected_events(&self) -> Result<Vec<(Device<'_>, XIEventMask)>> {
        let seq = send_request_xinput!(self.connection, XIOpcode::XIGetSelectedEvents, false, XIGetSelectedEventsRequest {
            window: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, XIGetSelectedEventsResponse);

        Ok(reply.masks.into_iter().map(|mask| (
            Device {
                id: mask.device,
                connection: self.connection,
            },
            mask.masks.get(0).copied().unwrap_or(XIEventMask::ZERO),
        )).collect())
    }
}