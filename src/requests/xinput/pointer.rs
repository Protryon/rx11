use bitvec::{order::Lsb0, prelude::BitVec};

use super::*;
use crate::coding::xinput2::{Fp1616, XIChangeCursorRequest, XIQueryPointerRequest, XIQueryPointerResponse, XIWarpPointerRequest};
pub use crate::coding::xinput2::{GroupInfo, ModifierInfo};

#[derive(Clone, Debug)]
pub struct PointerData<'a> {
    pub root: Window<'a>,
    pub child: Option<Window<'a>>,
    pub root_x: I16F16,
    pub root_y: I16F16,
    pub win_x: I16F16,
    pub win_y: I16F16,
    pub same_screen: bool,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: BitVec<u32, Lsb0>,
}

impl<'a> Window<'a> {
    pub async fn query_pointer(self, device: Device<'_>) -> Result<PointerData<'a>> {
        let reply = send_request_xinput!(
            self.connection,
            XIOpcode::XIQueryPointer,
            XIQueryPointerResponse,
            XIQueryPointerRequest {
                device: device.id,
                window: self.handle,
            }
        )
        .into_inner();

        Ok(PointerData {
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
            root_x: reply.root_x.into(),
            root_y: reply.root_y.into(),
            win_x: reply.win_x.into(),
            win_y: reply.win_y.into(),
            same_screen: reply.same_screen,
            mods: reply.mods,
            group: reply.groups,
            buttons: BitVec::from_vec(reply.buttons),
        })
    }

    pub async fn change_cursor(self, device: Device<'_>, cursor: Option<Cursor<'_>>) -> Result<()> {
        send_request_xinput!(
            self.connection,
            XIOpcode::XIChangeCursor,
            XIChangeCursorRequest {
                device: device.id,
                window: self.handle,
                cursor: cursor.map(|x| x.handle).unwrap_or(0),
            }
        );
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub enum PointerSource<'a> {
    Anywhere,
    Window {
        window: Window<'a>,
        src_x: I16F16,
        src_y: I16F16,
        src_width: u16,
        src_height: u16,
    },
}

#[derive(Clone, Debug)]
pub enum PointerDestination<'a> {
    Relative { x: I16F16, y: I16F16 },
    Absolute { window: Window<'a>, x: I16F16, y: I16F16 },
}

impl<'a> Device<'a> {
    pub async fn query_pointer<'b>(self, window: Window<'b>) -> Result<PointerData<'b>> {
        window.query_pointer(self).await
    }

    pub async fn warp_pointer(self, source: PointerSource<'_>, dest: PointerDestination<'_>) -> Result<()> {
        send_request_xinput!(
            self.connection,
            XIOpcode::XIWarpPointer,
            XIWarpPointerRequest {
                device: self.id,
                src_window: match source {
                    PointerSource::Anywhere => 0,
                    PointerSource::Window {
                        window,
                        ..
                    } => window.handle,
                },
                src_x: match source {
                    PointerSource::Anywhere => Fp1616 {
                        integral: 0,
                        frac: 0
                    },
                    PointerSource::Window {
                        src_x,
                        ..
                    } => src_x.into(),
                },
                src_y: match source {
                    PointerSource::Anywhere => Fp1616 {
                        integral: 0,
                        frac: 0
                    },
                    PointerSource::Window {
                        src_y,
                        ..
                    } => src_y.into(),
                },
                src_width: match source {
                    PointerSource::Anywhere => 0,
                    PointerSource::Window {
                        src_width,
                        ..
                    } => src_width,
                },
                src_height: match source {
                    PointerSource::Anywhere => 0,
                    PointerSource::Window {
                        src_height,
                        ..
                    } => src_height,
                },
                dst_x: match dest {
                    PointerDestination::Relative {
                        x,
                        ..
                    } => x.into(),
                    PointerDestination::Absolute {
                        x,
                        ..
                    } => x.into(),
                },
                dst_y: match dest {
                    PointerDestination::Relative {
                        y,
                        ..
                    } => y.into(),
                    PointerDestination::Absolute {
                        y,
                        ..
                    } => y.into(),
                },
                dst_window: match dest {
                    PointerDestination::Relative {
                        ..
                    } => 0,
                    PointerDestination::Absolute {
                        window,
                        ..
                    } => window.handle,
                },
            }
        );
        Ok(())
    }
}
