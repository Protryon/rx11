use crate::coding::xinput2::{
    XIEventMasks, XIGetClientPointerRequest, XIGetClientPointerResponse, XIGetSelectedEventsRequest, XIGetSelectedEventsResponse, XISelectEventsRequest,
    XISetClientPointerRequest,
};

use super::*;

impl<'a> Window<'a> {
    pub async fn set_client_pointer(self, device: Device<'_>) -> Result<()> {
        send_request_xinput!(
            self.connection,
            XIOpcode::XISetClientPointer,
            XISetClientPointerRequest {
                device: device.id,
                window: self.handle,
            }
        );
        Ok(())
    }

    pub async fn get_client_pointer(self) -> Result<Option<Device<'a>>> {
        let reply = send_request_xinput!(
            self.connection,
            XIOpcode::XIGetClientPointer,
            XIGetClientPointerResponse,
            XIGetClientPointerRequest {
                window: self.handle,
            }
        );

        Ok(if reply.set {
            Some(Device {
                id: reply.device,
                connection: self.connection,
            })
        } else {
            None
        })
    }

    pub async fn xi_select_events(self, masks: impl IntoIterator<Item = (Device<'a>, XIEventMask)>) -> Result<()> {
        send_request_xinput!(
            self.connection,
            XIOpcode::XISelectEvents,
            XISelectEventsRequest {
                window: self.handle,
                masks: masks
                    .into_iter()
                    .map(|(device, mask)| XIEventMasks {
                        device: device.id,
                        mask_num: 0,
                        mask
                    })
                    .collect(),
            }
        );
        Ok(())
    }

    pub async fn xi_get_selected_events(self) -> Result<Vec<(Device<'a>, XIEventMask)>> {
        let reply = send_request_xinput!(
            self.connection,
            XIOpcode::XIGetSelectedEvents,
            XIGetSelectedEventsResponse,
            XIGetSelectedEventsRequest {
                window: self.handle,
            }
        )
        .into_inner();

        Ok(reply
            .masks
            .into_iter()
            .map(|mask| {
                (
                    Device {
                        id: mask.device,
                        connection: self.connection,
                    },
                    mask.mask,
                )
            })
            .collect())
    }
}
