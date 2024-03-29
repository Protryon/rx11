use super::*;

pub use crate::coding::x11::{InputFocusRevert, Keybutmask};

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
    Window(Window<'a>),
}

impl<'a> Window<'a> {
    pub async fn legacy_query_pointer(self) -> Result<QueryPointerResponse<'a>> {
        let reply = send_request!(
            self.connection,
            QueryPointerReply,
            QueryPointer {
                window: self.handle,
            }
        );
        let same_screen = reply.reserved;
        let reply = reply.into_inner();

        Ok(QueryPointerResponse {
            same_screen: same_screen != 0,
            root: Window {
                handle: reply.root_window,
                connection: self.connection,
            },
            child: match reply.child_window {
                0 => None,
                handle => Some(Window {
                    handle,
                    connection: self.connection,
                }),
            },
            root_x: reply.root_x,
            root_y: reply.root_y,
            win_x: reply.win_x,
            win_y: reply.win_y,
            keybutmask: reply.keybutmask,
        })
    }

    pub async fn legacy_get_motion_events(self, start: Timestamp, stop: Timestamp) -> Result<Vec<MotionEvent>> {
        let reply = send_request!(
            self.connection,
            GetMotionEventsReply,
            GetMotionEvents {
                window: self.handle,
                start_time: start.0,
                stop_time: stop.0,
            }
        );

        Ok(reply
            .into_inner()
            .events
            .into_iter()
            .map(|x| MotionEvent {
                time: Timestamp(x.time),
                x: x.x,
                y: x.y,
            })
            .collect())
    }
}

#[derive(Clone, Debug)]
pub enum LegacyPointerSource<'a> {
    Anywhere,
    Window {
        window: Window<'a>,
        src_x: i16,
        src_y: i16,
        src_width: u16,
        src_height: u16,
    },
}

#[derive(Clone, Debug)]
pub enum LegacyPointerDestination<'a> {
    Relative { x: i16, y: i16 },
    Absolute { window: Window<'a>, x: i16, y: i16 },
}

impl X11Connection {
    pub async fn legacy_translate_coordinates(
        &self,
        src_window: Window<'_>,
        dst_window: Window<'_>,
        src_x: i16,
        src_y: i16,
    ) -> Result<TranslatedCoordinates<'_>> {
        let reply = send_request!(
            self,
            TranslateCoordinatesReply,
            TranslateCoordinates {
                src_window: src_window.handle,
                dst_window: dst_window.handle,
                src_x: src_x,
                src_y: src_y,
            }
        );
        let same_screen = reply.reserved;
        let reply = reply.into_inner();

        Ok(TranslatedCoordinates {
            same_screen: same_screen != 0,
            child: match reply.child_window {
                0 => None,
                handle => Some(Window {
                    handle,
                    connection: self,
                }),
            },
            dst_x: reply.dst_x,
            dst_y: reply.dst_y,
        })
    }

    pub async fn legacy_warp_pointer(&self, source: LegacyPointerSource<'_>, dest: LegacyPointerDestination<'_>) -> Result<()> {
        send_request!(
            self,
            WarpPointer {
                src_window: match source {
                    LegacyPointerSource::Anywhere => 0,
                    LegacyPointerSource::Window {
                        window,
                        ..
                    } => window.handle,
                },
                src_x: match source {
                    LegacyPointerSource::Anywhere => 0,
                    LegacyPointerSource::Window {
                        src_x,
                        ..
                    } => src_x,
                },
                src_y: match source {
                    LegacyPointerSource::Anywhere => 0,
                    LegacyPointerSource::Window {
                        src_y,
                        ..
                    } => src_y,
                },
                src_width: match source {
                    LegacyPointerSource::Anywhere => 0,
                    LegacyPointerSource::Window {
                        src_width,
                        ..
                    } => src_width,
                },
                src_height: match source {
                    LegacyPointerSource::Anywhere => 0,
                    LegacyPointerSource::Window {
                        src_height,
                        ..
                    } => src_height,
                },
                dst_x: match dest {
                    LegacyPointerDestination::Relative {
                        x,
                        ..
                    } => x,
                    LegacyPointerDestination::Absolute {
                        x,
                        ..
                    } => x,
                },
                dst_y: match dest {
                    LegacyPointerDestination::Relative {
                        y,
                        ..
                    } => y,
                    LegacyPointerDestination::Absolute {
                        y,
                        ..
                    } => y,
                },
                dst_window: match dest {
                    LegacyPointerDestination::Relative {
                        ..
                    } => 0,
                    LegacyPointerDestination::Absolute {
                        window,
                        ..
                    } => window.handle,
                },
            }
        );
        Ok(())
    }

    pub async fn legacy_set_input_focus(&self, revert_to: InputFocusRevert, window: InputFocusWindow<'_>, time: Timestamp) -> Result<()> {
        send_request!(self, reserved revert_to as u8, SetInputFocus {
            window: match window {
                InputFocusWindow::None => 0,
                InputFocusWindow::PointerRoot => 1,
                InputFocusWindow::Window(window) => window.handle,
            },
            time: time.0,
        });
        Ok(())
    }

    pub async fn legacy_get_input_focus(&self) -> Result<(InputFocusRevert, InputFocusWindow<'_>)> {
        let reply = send_request!(self, GetInputFocusReply, GetInputFocus {});
        let revert_to = reply.reserved;
        let reply = reply.into_inner();

        Ok((
            InputFocusRevert::decode_sync(&mut &[revert_to][..])?,
            match reply.focus_window {
                0 => InputFocusWindow::None,
                1 => InputFocusWindow::PointerRoot,
                handle => InputFocusWindow::Window(Window {
                    handle,
                    connection: self,
                }),
            },
        ))
    }
}
