
pub use crate::coding::xfixes::BarrierDirections;
use crate::coding::xfixes::{CreatePointerBarrierRequest, DeletePointerBarrierRequest};

use super::*;

#[derive(Clone, Copy)]
#[derive(derivative::Derivative)]
#[derivative(Debug)]
pub struct Barrier<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

impl<'a> Window<'a> {
    pub async fn create_pointer_barrier(&self, x1: u16, y1: u16, x2: u16, y2: u16, directions: BarrierDirections, devices: impl IntoIterator<Item=Device<'_>>) -> Result<Barrier<'_>> {
        let barrier = Barrier {
            handle: self.connection.new_resource_id(),
            connection: self.connection,
        };
        send_request_xfixes!(self.connection, XFOpcode::CreatePointerBarrier, true, CreatePointerBarrierRequest {
            barrier: barrier.handle,
            window: self.handle,
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
            directions: directions,
            devices: devices.into_iter().map(|x| x.id.to_repr()).collect(),
        });

        Ok(barrier)
    }

}

impl<'a> Barrier<'a> {
    pub async fn destroy(&self) -> Result<()> {
        send_request_xfixes!(self.connection, XFOpcode::DeletePointerBarrier, true, DeletePointerBarrierRequest {
            barrier: self.handle,
        });

        Ok(())
    }
}
