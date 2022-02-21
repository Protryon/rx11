use super::*;

pub use crate::coding::{
    Keybutmask,
    InputFocusRevert,
};

#[derive(Clone, Debug)]
pub struct QueryPointerResponse<'a> {
    pub same_screen: bool,
    pub root: Window<'a>,
    pub child: Option<Window<'a>>,
    pub root_x: i16,
    pub root_y: i16,
    pub win_x: i16,
    pub win_y: i16,
    pub keybutmask: Keybutmask,
}

#[derive(Clone, Copy, Debug)]
pub struct MotionEvent {
    pub time: Timestamp,
    pub x: i16,
    pub y: i16,
}

#[derive(Clone, Copy, Debug)]
pub struct TranslatedCoordinates<'a> {
    pub same_screen: bool,
    pub child: Option<Window<'a>>,
    pub dst_x: i16,
    pub dst_y: i16,
}

#[derive(Clone, Copy, Debug)]
pub enum InputFocusWindow<'a> {
    None,
    PointerRoot,
    Window(Window<'a>)
}

impl X11Connection {
    pub async fn query_pointer(&self, window: Window<'_>) -> Result<QueryPointerResponse<'_>> {
        let seq = send_request!(self, QueryPointer {
            window: window.handle,
        });
        let (reply, same_screen) = receive_reply!(self, seq, QueryPointerReply, fetched);

        Ok(QueryPointerResponse {
            same_screen: same_screen != 0,
            root: Window { handle: reply.root_window, connection: self },
            child: match reply.child_window {
                0 => None,
                handle => Some(Window { handle, connection: self }),
            },
            root_x: reply.root_x,
            root_y: reply.root_y,
            win_x: reply.win_x,
            win_y: reply.win_y,
            keybutmask: reply.keybutmask,
        })
    }

    pub async fn get_motion_events(&self, window: Window<'_>, start: Timestamp, stop: Timestamp) -> Result<Vec<MotionEvent>> {
        let seq = send_request!(self, GetMotionEvents {
            window: window.handle,
            start_time: start.0,
            stop_time: stop.0,
        });
        let reply = receive_reply!(self, seq, GetMotionEventsReply);

        Ok(reply.events.into_iter().map(|x| MotionEvent {
            time: Timestamp(x.time),
            x: x.x,
            y: x.y,
        }).collect())
    }

    pub async fn translate_coordinates(&self, src_window: Window<'_>, dst_window: Window<'_>, src_x: i16, src_y: i16) -> Result<TranslatedCoordinates<'_>> {
        let seq = send_request!(self, TranslateCoordinates {
            src_window: src_window.handle,
            dst_window: dst_window.handle,
            src_x: src_x,
            src_y: src_y,
        });
        let (reply, same_screen) = receive_reply!(self, seq, TranslateCoordinatesReply, fetched);

        Ok(TranslatedCoordinates {
            same_screen: same_screen != 0,
            child: match reply.child_window {
                0 => None,
                handle => Some(Window { handle, connection: self }),
            },
            dst_x: reply.dst_x,
            dst_y: reply.dst_y,
        })
    }

    pub async fn warp_pointer(&self, src_window: Window<'_>, dst_window: Window<'_>, src_x: i16, src_y: i16, src_width: u16, src_height: u16, dst_x: i16, dst_y: i16) -> Result<()> {
        send_request!(self, WarpPointer {
            src_window: src_window.handle,
            dst_window: dst_window.handle,
            src_x: src_x,
            src_y: src_y,
            src_width: src_width,
            src_height: src_height,
            dst_x: dst_x,
            dst_y: dst_y,
        });
        Ok(())
    }

    pub async fn set_input_focus(&self, revert_to: InputFocusRevert, window: InputFocusWindow<'_>, time: Timestamp) -> Result<()> {
        send_request!(self, revert_to as u8, SetInputFocus {
            window: match window {
                InputFocusWindow::None => 0,
                InputFocusWindow::PointerRoot => 1,
                InputFocusWindow::Window(window) => window.handle,
            },
            time: time.0,
        });
        Ok(())
    }

    pub async fn get_input_focus(&self) -> Result<(InputFocusRevert, InputFocusWindow<'_>)> {
        let seq = send_request!(self, GetInputFocus {
        });
        let (reply, revert_to) = receive_reply!(self, seq, GetInputFocusReply, fetched);

        Ok((
            InputFocusRevert::decode_sync(&mut &[revert_to][..])?,
            match reply.focus_window {
                0 => InputFocusWindow::None,
                1 => InputFocusWindow::PointerRoot,
                handle => InputFocusWindow::Window(Window { handle, connection: self }),
            }
        ))
    }
}