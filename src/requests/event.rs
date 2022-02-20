use super::*;

pub use crate::coding::EventMask;

#[derive(Clone, Copy, Debug)]
pub enum EventDestination {
    PointerWindow,
    InputFocus,
    Window(Window),
}

impl X11Connection {
    // pub async fn send_event(&self, window: EventDestination, propagate: bool, event_mask: u32, event: Event) -> Result<()> {

    //     send_request!(self, propagate as u8, SendEvent {
    //         window: match window {
    //             EventDestination::PointerWindow => 0,
    //             EventDestination::InputFocus => 1,
    //             EventDestination::Window(w) => w.handle,
    //         },
    //         event_mask: event_mask,
            
    //     });
    //     Ok(())
    // }
}