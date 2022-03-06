use crate::coding::xinput2::{XIBarrierReleasePointerRequest, BarrierReleasePointerInfo};

use super::*;

impl X11Connection {

    pub async fn barrier_release_pointer(&self, events: impl IntoIterator<Item=(Device<'_>, Barrier<'_>, BarrierEventId)>) -> Result<()> {
        send_request_xinput!(self, XIOpcode::XIBarrierReleasePointer, true, XIBarrierReleasePointerRequest {
            barriers: events.into_iter().map(|(device, barrier, event_id)| BarrierReleasePointerInfo {
                device: device.id,
                barrier: barrier.handle,
                event_id: event_id.0,
            }).collect(),
        });

        Ok(())
    }
}