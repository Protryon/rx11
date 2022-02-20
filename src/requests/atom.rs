use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Atom {
    pub(crate) handle: u32,
}

impl X11Connection {
    pub async fn intern_atom(&self, name: &str, only_if_exists: bool) -> Result<Atom> {
        let seq = send_request!(self, only_if_exists as u8, InternAtom {
            name: name.to_string(),
        });
        let reply = receive_reply!(self, seq, InternAtomReply);

        Ok(Atom {
            handle: reply.atom,
        })
    }

    pub async fn get_atom_name(&self, atom: Atom) -> Result<String> {
        let seq = send_request!(self, GetAtomName {
            atom: atom.handle,
        });
        let reply = receive_reply!(self, seq, GetAtomNameReply);

        Ok(reply.name)
    }
}

impl Atom {
    pub const PRIMARY: Atom = Atom { handle: 1 }; 
    pub const SECONDARY: Atom = Atom { handle: 2 }; 
    pub const ARC: Atom = Atom { handle: 3 }; 
    pub const ATOM: Atom = Atom { handle: 4 }; 
    pub const BITMAP: Atom = Atom { handle: 5 }; 
    pub const CARDINAL: Atom = Atom { handle: 6 }; 
    pub const COLORMAP: Atom = Atom { handle: 7 }; 
    pub const CURSOR: Atom = Atom { handle: 8 }; 
    pub const CUT_BUFFER0: Atom = Atom { handle: 9 }; 
    pub const CUT_BUFFER1: Atom = Atom { handle: 10 };
    pub const CUT_BUFFER2: Atom = Atom { handle: 11 };
    pub const CUT_BUFFER3: Atom = Atom { handle: 12 };
    pub const CUT_BUFFER4: Atom = Atom { handle: 13 };
    pub const CUT_BUFFER5: Atom = Atom { handle: 14 };
    pub const CUT_BUFFER6: Atom = Atom { handle: 15 };
    pub const CUT_BUFFER7: Atom = Atom { handle: 16 };
    pub const DRAWABLE: Atom = Atom { handle: 17 };
    pub const FONT: Atom = Atom { handle: 18 };
    pub const INTEGER: Atom = Atom { handle: 19 };
    pub const PIXMAP: Atom = Atom { handle: 20 };
    pub const POINT: Atom = Atom { handle: 21 };
    pub const RECTANGLE: Atom = Atom { handle: 22 };
    pub const RESOURCE_MANAGER: Atom = Atom { handle: 23 };
    pub const RGB_COLOR_MAP: Atom = Atom { handle: 24 };
    pub const RGB_BEST_MAP: Atom = Atom { handle: 25 };
    pub const RGB_BLUE_MAP: Atom = Atom { handle: 26 };
    pub const RGB_DEFAULT_MAP: Atom = Atom { handle: 27 };
    pub const RGB_GRAY_MAP: Atom = Atom { handle: 28 };
    pub const RGB_GREEN_MAP: Atom = Atom { handle: 29 };
    pub const RGB_RED_MAP: Atom = Atom { handle: 30 };
    pub const STRING: Atom = Atom { handle: 31 };
    pub const VISUALID: Atom = Atom { handle: 32 };
    pub const WINDOW: Atom = Atom { handle: 33 };
    pub const WM_COMMAND: Atom = Atom { handle: 34 };
    pub const WM_HINTS: Atom = Atom { handle: 35 };
    pub const WM_CLIENT_MACHINE: Atom = Atom { handle: 36 };
    pub const WM_ICON_NAME: Atom = Atom { handle: 37 };
    pub const WM_ICON_SIZE: Atom = Atom { handle: 38 };
    pub const WM_NAME: Atom = Atom { handle: 39 };
    pub const WM_NORMAL_HINTS: Atom = Atom { handle: 40 };
    pub const WM_SIZE_HINTS: Atom = Atom { handle: 41 };
    pub const WM_ZOOM_HINTS: Atom = Atom { handle: 42 };
    pub const MIN_SPACE: Atom = Atom { handle: 43 };
    pub const NORM_SPACE: Atom = Atom { handle: 44 };
    pub const MAX_SPACE: Atom = Atom { handle: 45 };
    pub const END_SPACE: Atom = Atom { handle: 46 };
    pub const SUPERSCRIPT_X: Atom = Atom { handle: 47 };
    pub const SUPERSCRIPT_Y: Atom = Atom { handle: 48 };
    pub const SUBSCRIPT_X: Atom = Atom { handle: 49 };
    pub const SUBSCRIPT_Y: Atom = Atom { handle: 50 };
    pub const UNDERLINE_POSITION: Atom = Atom { handle: 51 };
    pub const UNDERLINE_THICKNESS: Atom = Atom { handle: 52 };
    pub const STRIKEOUT_ASCENT: Atom = Atom { handle: 53 };
    pub const STRIKEOUT_DESCENT: Atom = Atom { handle: 54 };
    pub const ITALIC_ANGLE: Atom = Atom { handle: 55 };
    pub const X_HEIGHT: Atom = Atom { handle: 56 };
    pub const QUAD_WIDTH: Atom = Atom { handle: 57 };
    pub const WEIGHT: Atom = Atom { handle: 58 };
    pub const POINT_SIZE: Atom = Atom { handle: 59 };
    pub const RESOLUTION: Atom = Atom { handle: 60 };
    pub const COPYRIGHT: Atom = Atom { handle: 61 };
    pub const NOTICE: Atom = Atom { handle: 62 };
    pub const FONT_NAME: Atom = Atom { handle: 63 };
    pub const FAMILY_NAME: Atom = Atom { handle: 64 };
    pub const FULL_NAME: Atom = Atom { handle: 65 };
    pub const CAP_HEIGHT: Atom = Atom { handle: 66 };
    pub const WM_CLASS: Atom = Atom { handle: 67 };
    pub const WM_TRANSIENT_FOR: Atom = Atom { handle: 68 };
}