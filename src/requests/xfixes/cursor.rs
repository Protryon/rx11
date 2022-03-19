use crate::coding::xfixes::{SelectCursorInputRequest, GetCursorImageRequest, SetCursorNameRequest, GetCursorNameRequest, GetCursorNameResponse, GetCursorImageAndNameResponse, GetCursorImageAndNameRequest, ChangeCursorRequest, ChangeCursorByNameRequest, HideCursorRequest, ShowCursorRequest};
pub use crate::coding::xfixes::{
    CursorNotifyMask,
    GetCursorImageResponse as CursorImage,
};

use super::*;

impl<'a> Window<'a> {
    pub async fn select_cursor_input(self, event_mask: CursorNotifyMask) -> Result<()> {
        send_request_xfixes!(self.connection, XFOpcode::SelectCursorInput, true, SelectCursorInputRequest {
            window: self.handle,
            event_mask: event_mask,
        });

        Ok(())
    }

    pub async fn hide_cursor(self) -> Result<()> {
        send_request_xfixes!(self.connection, XFOpcode::HideCursor, true, HideCursorRequest {
            window: self.handle,
        });

        Ok(())
    }

    pub async fn show_cursor(self) -> Result<()> {
        send_request_xfixes!(self.connection, XFOpcode::HideCursor, true, ShowCursorRequest {
            window: self.handle,
        });

        Ok(())
    }
}

impl X11Connection {
    pub async fn get_cursor_image(&self) -> Result<CursorImage> {
        let seq = send_request_xfixes!(self, XFOpcode::GetCursorImage, false, GetCursorImageRequest {
        });
        let reply = receive_reply!(self, seq, CursorImage);

        Ok(reply)
    }

    pub async fn get_cursor_image_and_name(&self) -> Result<(CursorImage, Option<Atom>)> {
        let seq = send_request_xfixes!(self, XFOpcode::GetCursorImageAndName, false, GetCursorImageAndNameRequest {
        });
        let reply = receive_reply!(self, seq, GetCursorImageAndNameResponse);
        let atom = {
            if reply.cursor_name_atom == 0 {
                None
            } else {
                match self.try_get_atom_name(reply.cursor_name_atom) {
                    Some(atom) => Some(atom),
                    None => {
                        self.local_intern_atom(reply.cursor_name_atom, &*reply.name);
                        self.try_get_atom_name(reply.cursor_name_atom)
                    }
                }
            }
        };

        let image = CursorImage {
            x: reply.x,
            y: reply.y,
            width: reply.width,
            height: reply.height,
            xhot: reply.xhot,
            yhot: reply.yhot,
            cursor_serial: reply.cursor_serial,
            cursor_image: reply.cursor_image,
        };

        Ok((
            image,
            atom
        ))
    }
}

impl<'a> Cursor<'a> {
    pub async fn set_name(self, name: impl AsRef<str>) -> Result<()> {
        send_request_xfixes!(self.connection, XFOpcode::SetCursorName, true, SetCursorNameRequest {
            cursor: self.handle,
            name: name.as_ref().to_string(),
        });

        Ok(())
    }

    pub async fn get_name(self) -> Result<Option<Atom>> {
        let seq = send_request_xfixes!(self.connection, XFOpcode::GetCursorName, false, GetCursorNameRequest {
            cursor: self.handle,
        });
        let reply = receive_reply!(self.connection, seq, GetCursorNameResponse);
        if reply.name_atom == 0 {
            return Ok(None);
        }
        match self.connection.try_get_atom_name(reply.name_atom) {
            Some(atom) => Ok(Some(atom)),
            None => {
                self.connection.local_intern_atom(reply.name_atom, &*reply.name);
                Ok(self.connection.try_get_atom_name(reply.name_atom))
            }
        }
    }

    pub async fn change_from(self, target: Cursor<'_>) -> Result<()> {
        send_request_xfixes!(self.connection, XFOpcode::ChangeCursor, true, ChangeCursorRequest {
            src_cursor: self.handle,
            dst_cursor: target.handle,
        });

        Ok(())
    }

    pub async fn change_from_name(self, name: impl AsRef<str>) -> Result<()> {
        send_request_xfixes!(self.connection, XFOpcode::ChangeCursorByName, true, ChangeCursorByNameRequest {
            src_cursor: self.handle,
            name: name.as_ref().to_string(),
        });

        Ok(())
    }
}