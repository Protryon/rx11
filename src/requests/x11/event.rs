use super::*;

pub use crate::coding::x11::EventMask;

#[derive(Clone, Copy, Debug)]
pub enum EventDestination<'a> {
    PointerWindow,
    InputFocus,
    Window(Window<'a>),
}

impl X11Connection {
    pub async fn send_event(&self, window: EventDestination<'_>, propagate: bool, event_mask: EventMask, event: Event<'_>) -> Result<()> {
        let (code, event) = event.to_protocol(self)?;
        send_request!(self, propagate as u8, SendEvent {
            window: match window {
                EventDestination::PointerWindow => 0,
                EventDestination::InputFocus => 1,
                EventDestination::Window(w) => w.handle,
            },
            event_mask: event_mask,
            code: code,
            event: event,
        });
        Ok(())
    }
}