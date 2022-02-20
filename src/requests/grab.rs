use super::*;

pub use crate::coding::{
    PointerMode,
    PointerEventMask,
    GrabStatus,
    Keymask,
    AllowEventsMode,
};

#[derive(Clone, Debug)]
pub struct GrabPointerParams {
    pub grab_window: Window,
    pub owner_events: bool,
    pub event_mask: PointerEventMask,
    pub pointer_mode: PointerMode,
    pub keyboard_mode: PointerMode,
    pub confine_to: Option<Window>,
    pub cursor: Option<Cursor>,
    pub time: Timestamp,
}

#[derive(Clone, Debug)]
pub struct GrabButtonParams {
    pub grab_window: Window,
    pub owner_events: bool,
    pub event_mask: PointerEventMask,
    pub pointer_mode: PointerMode,
    pub keyboard_mode: PointerMode,
    pub confine_to: Option<Window>,
    pub cursor: Option<Cursor>,
    pub button: Option<u8>,
    pub keymask: Keymask,
}

#[derive(Clone, Debug)]
pub struct GrabKeyboardParams {
    pub grab_window: Window,
    pub owner_events: bool,
    pub time: Timestamp,
    pub pointer_mode: PointerMode,
    pub keyboard_mode: PointerMode,
}

#[derive(Clone, Debug)]
pub struct GrabKeyParams {
    pub grab_window: Window,
    pub owner_events: bool,
    pub keymask: Keymask,
    pub keycode: Option<u8>,
    pub pointer_mode: PointerMode,
    pub keyboard_mode: PointerMode,
}

impl X11Connection {
    pub async fn grab_pointer(&self, params: GrabPointerParams) -> Result<GrabStatus> {
        let seq = send_request!(self, params.owner_events as u8, GrabPointer {
            grab_window: params.grab_window.handle,
            event_mask: params.event_mask,
            pointer_mode: params.pointer_mode,
            keyboard_mode: params.keyboard_mode,
            confine_to_window: params.confine_to.map(|x| x.handle).unwrap_or(0),
            cursor: params.cursor.map(|x| x.handle).unwrap_or(0),
            time: params.time.0,
        });
        let (_, status) = receive_reply!(self, seq, GrabPointerReply, fetched);
        Ok(GrabStatus::decode_sync(&mut &[status][..])?)
    }

    pub async fn ungrab_pointer(&self, time: Timestamp) -> Result<()> {
        send_request!(self, UngrabPointer {
            time: time.0,
        });
        Ok(())
    }

    pub async fn grab_button(&self, params: GrabButtonParams) -> Result<()> {
        send_request!(self, params.owner_events as u8, GrabButton {
            grab_window: params.grab_window.handle,
            event_mask: params.event_mask,
            pointer_mode: params.pointer_mode,
            keyboard_mode: params.keyboard_mode,
            confine_to_window: params.confine_to.map(|x| x.handle).unwrap_or(0),
            cursor: params.cursor.map(|x| x.handle).unwrap_or(0),
            button: params.button.unwrap_or(0),
            keymask: params.keymask,
        });
        Ok(())
    }

    pub async fn ungrab_button(&self, window: Window, keymask: Keymask, button: Option<u8>) -> Result<()> {
        send_request!(self, button.unwrap_or(0), UngrabButton {
            grab_window: window.handle,
            keymask: keymask,
        });
        Ok(())
    }

    pub async fn change_active_pointer_grab(&self, cursor: Option<Cursor>, time: Timestamp, event_mask: PointerEventMask) -> Result<()> {
        send_request!(self, ChangeActivePointerGrab {
            cursor: cursor.map(|x| x.handle).unwrap_or(0),
            time: time.0,
            event_mask: event_mask,
        });
        Ok(())
    }

    pub async fn grab_keyboard(&self, params: GrabKeyboardParams) -> Result<GrabStatus> {
        let seq = send_request!(self, params.owner_events as u8, GrabKeyboard {
            grab_window: params.grab_window.handle,
            time: params.time.0,
            pointer_mode: params.pointer_mode,
            keyboard_mode: params.keyboard_mode,
        });
        let (_, status) = receive_reply!(self, seq, GrabPointerReply, fetched);
        Ok(GrabStatus::decode_sync(&mut &[status][..])?)
    }

    pub async fn ungrab_keyboard(&self, time: Timestamp) -> Result<()> {
        send_request!(self, UngrabKeyboard {
            time: time.0,
        });
        Ok(())
    }

    pub async fn grab_key(&self, params: GrabKeyParams) -> Result<()> {
        send_request!(self, params.owner_events as u8, GrabKey {
            grab_window: params.grab_window.handle,
            keymask: params.keymask,
            keycode: params.keycode.unwrap_or(0),
            pointer_mode: params.pointer_mode,
            keyboard_mode: params.keyboard_mode,
        });
        Ok(())
    }

    pub async fn ungrab_key(&self, grab_window: Window, keymask: Keymask, keycode: Option<u8>) -> Result<()> {
        send_request!(self, keycode.unwrap_or(0), UngrabKey {
            grab_window: grab_window.handle,
            keymask: keymask,
        });
        Ok(())
    }

    pub async fn allow_events(&self, mode: AllowEventsMode, time: Timestamp) -> Result<()> {
        send_request!(self, mode as u8, AllowEvents {
            time: time.0,
        });
        Ok(())
    }
    
    pub async fn grab_server(&self) -> Result<()> {
        send_request!(self, GrabServer {
        });
        Ok(())
    }

    pub async fn ungrab_server(&self) -> Result<()> {
        send_request!(self, UngrabServer {
        });
        Ok(())
    }
}