use std::fmt;

use internment::Intern;

use super::*;

#[derive(Clone, Copy)]
pub struct Atom {
    pub(crate) handle: u32,
    pub name: &'static str,
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.name)
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl X11Connection {
    pub async fn intern_atom(&self, name: &str, only_if_exists: bool) -> Result<Atom> {
        if let Some(value) = self.0.known_atoms.get(name) {
            return Ok(Atom {
                handle: *value,
                name: *value.key(),
            });
        }
        let seq = send_request!(self, only_if_exists as u8, InternAtom {
            name: name.to_string(),
        });
        let reply = receive_reply!(self, seq, InternAtomReply);

        let name = Intern::new(name.to_string()).as_ref();
        self.0.known_atoms.insert(name, reply.atom);
        self.0.known_atoms_inverse.insert(reply.atom, name);
        Ok(Atom {
            handle: reply.atom,
            name,
        })
    }

    pub(crate) async fn get_atom_name(&self, raw_atom: u32) -> Result<Atom> {
        if let Some(value) = self.0.known_atoms_inverse.get(&raw_atom) {
            return Ok(Atom {
                handle: raw_atom,
                name: *value.value(),
            });
        }
        let seq = send_request!(self, GetAtomName {
            atom: raw_atom,
        });
        let reply = receive_reply!(self, seq, GetAtomNameReply);

        let name = Intern::new(reply.name).as_ref();
        self.0.known_atoms.insert(name, raw_atom);
        self.0.known_atoms_inverse.insert(raw_atom, name);
        Ok(Atom {
            handle: raw_atom,
            name,
        })
    }

    pub(crate) fn try_get_atom_name(&self, raw_atom: u32) -> Option<Atom> {
        if let Some(value) = self.0.known_atoms_inverse.get(&raw_atom) {
            Some(Atom {
                handle: raw_atom,
                name: *value.value(),
            })
        } else {
            None
        }
    }

    pub(crate) async fn maybe_get_atom_name(&self, atom: Option<u32>) -> Result<Option<Atom>> {
        match atom {
            Some(x) => {
                match self.try_get_atom_name(x) {
                    Some(atom) => Ok(Some(atom)),
                    None => {
                        let self_ = self.clone();
                        tokio::spawn(async move {
                            Ok(Some(self_.get_atom_name(x).await?))
                        }).await?
                    },
                }
            },
            None => Ok(None),
        }
    }

    pub(crate) async fn get_all_atoms(&self, atoms: impl IntoIterator<Item=u32>) -> Result<Vec<Atom>> {
        let atoms = atoms.into_iter().map(|atom| self.maybe_get_atom_name(Some(atom))).collect::<Vec<_>>();
        futures::future::join_all(atoms).await.into_iter().flat_map(|x| x.transpose()).collect::<Result<_>>()
    }

    pub(crate) fn register_const_atoms(&self) {
        for atom in Atom::ALL_CONST_ATOMS {
            self.0.known_atoms.insert(atom.name, atom.handle);
            self.0.known_atoms_inverse.insert(atom.handle, atom.name);
        }
    }
}

impl Atom {
    pub const NULL: Atom = Atom { handle: 0, name: "" };
    pub const PRIMARY: Atom = Atom { handle: 1, name: "PRIMARY" };
    pub const SECONDARY: Atom = Atom { handle: 2, name: "SECONDARY" };
    pub const ARC: Atom = Atom { handle: 3, name: "ARC" };
    pub const ATOM: Atom = Atom { handle: 4, name: "ATOM" };
    pub const BITMAP: Atom = Atom { handle: 5, name: "BITMAP" };
    pub const CARDINAL: Atom = Atom { handle: 6, name: "CARDINAL" };
    pub const COLORMAP: Atom = Atom { handle: 7, name: "COLORMAP" };
    pub const CURSOR: Atom = Atom { handle: 8, name: "CURSOR" };
    pub const CUT_BUFFER0: Atom = Atom { handle: 9, name: "CUT_BUFFER0" };
    pub const CUT_BUFFER1: Atom = Atom { handle: 10, name: "CUT_BUFFER1" };
    pub const CUT_BUFFER2: Atom = Atom { handle: 11, name: "CUT_BUFFER2" };
    pub const CUT_BUFFER3: Atom = Atom { handle: 12, name: "CUT_BUFFER3" };
    pub const CUT_BUFFER4: Atom = Atom { handle: 13, name: "CUT_BUFFER4" };
    pub const CUT_BUFFER5: Atom = Atom { handle: 14, name: "CUT_BUFFER5" };
    pub const CUT_BUFFER6: Atom = Atom { handle: 15, name: "CUT_BUFFER6" };
    pub const CUT_BUFFER7: Atom = Atom { handle: 16, name: "CUT_BUFFER7" };
    pub const DRAWABLE: Atom = Atom { handle: 17, name: "DRAWABLE" };
    pub const FONT: Atom = Atom { handle: 18, name: "FONT" };
    pub const INTEGER: Atom = Atom { handle: 19, name: "INTEGER" };
    pub const PIXMAP: Atom = Atom { handle: 20, name: "PIXMAP" };
    pub const POINT: Atom = Atom { handle: 21, name: "POINT" };
    pub const RECTANGLE: Atom = Atom { handle: 22, name: "RECTANGLE" };
    pub const RESOURCE_MANAGER: Atom = Atom { handle: 23, name: "RESOURCE_MANAGER" };
    pub const RGB_COLOR_MAP: Atom = Atom { handle: 24, name: "RGB_COLOR_MAP" };
    pub const RGB_BEST_MAP: Atom = Atom { handle: 25, name: "RGB_BEST_MAP" };
    pub const RGB_BLUE_MAP: Atom = Atom { handle: 26, name: "RGB_BLUE_MAP" };
    pub const RGB_DEFAULT_MAP: Atom = Atom { handle: 27, name: "RGB_DEFAULT_MAP" };
    pub const RGB_GRAY_MAP: Atom = Atom { handle: 28, name: "RGB_GRAY_MAP" };
    pub const RGB_GREEN_MAP: Atom = Atom { handle: 29, name: "RGB_GREEN_MAP" };
    pub const RGB_RED_MAP: Atom = Atom { handle: 30, name: "RGB_RED_MAP" };
    pub const STRING: Atom = Atom { handle: 31, name: "STRING" };
    pub const VISUALID: Atom = Atom { handle: 32, name: "VISUALID" };
    pub const WINDOW: Atom = Atom { handle: 33, name: "WINDOW" };
    pub const WM_COMMAND: Atom = Atom { handle: 34, name: "WM_COMMAND" };
    pub const WM_HINTS: Atom = Atom { handle: 35, name: "WM_HINTS" };
    pub const WM_CLIENT_MACHINE: Atom = Atom { handle: 36, name: "WM_CLIENT_MACHINE" };
    pub const WM_ICON_NAME: Atom = Atom { handle: 37, name: "WM_ICON_NAME" };
    pub const WM_ICON_SIZE: Atom = Atom { handle: 38, name: "WM_ICON_SIZE" };
    pub const WM_NAME: Atom = Atom { handle: 39, name: "WM_NAME" };
    pub const WM_NORMAL_HINTS: Atom = Atom { handle: 40, name: "WM_NORMAL_HINTS" };
    pub const WM_SIZE_HINTS: Atom = Atom { handle: 41, name: "WM_SIZE_HINTS" };
    pub const WM_ZOOM_HINTS: Atom = Atom { handle: 42, name: "WM_ZOOM_HINTS" };
    pub const MIN_SPACE: Atom = Atom { handle: 43, name: "MIN_SPACE" };
    pub const NORM_SPACE: Atom = Atom { handle: 44, name: "NORM_SPACE" };
    pub const MAX_SPACE: Atom = Atom { handle: 45, name: "MAX_SPACE" };
    pub const END_SPACE: Atom = Atom { handle: 46, name: "END_SPACE" };
    pub const SUPERSCRIPT_X: Atom = Atom { handle: 47, name: "SUPERSCRIPT_X" };
    pub const SUPERSCRIPT_Y: Atom = Atom { handle: 48, name: "SUPERSCRIPT_Y" };
    pub const SUBSCRIPT_X: Atom = Atom { handle: 49, name: "SUBSCRIPT_X" };
    pub const SUBSCRIPT_Y: Atom = Atom { handle: 50, name: "SUBSCRIPT_Y" };
    pub const UNDERLINE_POSITION: Atom = Atom { handle: 51, name: "UNDERLINE_POSITION" };
    pub const UNDERLINE_THICKNESS: Atom = Atom { handle: 52, name: "UNDERLINE_THICKNESS" };
    pub const STRIKEOUT_ASCENT: Atom = Atom { handle: 53, name: "STRIKEOUT_ASCENT" };
    pub const STRIKEOUT_DESCENT: Atom = Atom { handle: 54, name: "STRIKEOUT_DESCENT" };
    pub const ITALIC_ANGLE: Atom = Atom { handle: 55, name: "ITALIC_ANGLE" };
    pub const X_HEIGHT: Atom = Atom { handle: 56, name: "X_HEIGHT" };
    pub const QUAD_WIDTH: Atom = Atom { handle: 57, name: "QUAD_WIDTH" };
    pub const WEIGHT: Atom = Atom { handle: 58, name: "WEIGHT" };
    pub const POINT_SIZE: Atom = Atom { handle: 59, name: "POINT_SIZE" };
    pub const RESOLUTION: Atom = Atom { handle: 60, name: "RESOLUTION" };
    pub const COPYRIGHT: Atom = Atom { handle: 61, name: "COPYRIGHT" };
    pub const NOTICE: Atom = Atom { handle: 62, name: "NOTICE" };
    pub const FONT_NAME: Atom = Atom { handle: 63, name: "FONT_NAME" };
    pub const FAMILY_NAME: Atom = Atom { handle: 64, name: "FAMILY_NAME" };
    pub const FULL_NAME: Atom = Atom { handle: 65, name: "FULL_NAME" };
    pub const CAP_HEIGHT: Atom = Atom { handle: 66, name: "CAP_HEIGHT" };
    pub const WM_CLASS: Atom = Atom { handle: 67, name: "WM_CLASS" };
    pub const WM_TRANSIENT_FOR: Atom = Atom { handle: 68, name: "WM_TRANSIENT_FOR" };

    const ALL_CONST_ATOMS: &'static [Atom] = &[
        Self::NULL,
        Self::PRIMARY,
        Self::SECONDARY,
        Self::ARC,
        Self::ATOM,
        Self::BITMAP,
        Self::CARDINAL,
        Self::COLORMAP,
        Self::CURSOR,
        Self::CUT_BUFFER0,
        Self::CUT_BUFFER1,
        Self::CUT_BUFFER2,
        Self::CUT_BUFFER3,
        Self::CUT_BUFFER4,
        Self::CUT_BUFFER5,
        Self::CUT_BUFFER6,
        Self::CUT_BUFFER7,
        Self::DRAWABLE,
        Self::FONT,
        Self::INTEGER,
        Self::PIXMAP,
        Self::POINT,
        Self::RECTANGLE,
        Self::RESOURCE_MANAGER,
        Self::RGB_COLOR_MAP,
        Self::RGB_BEST_MAP,
        Self::RGB_BLUE_MAP,
        Self::RGB_DEFAULT_MAP,
        Self::RGB_GRAY_MAP,
        Self::RGB_GREEN_MAP,
        Self::RGB_RED_MAP,
        Self::STRING,
        Self::VISUALID,
        Self::WINDOW,
        Self::WM_COMMAND,
        Self::WM_HINTS,
        Self::WM_CLIENT_MACHINE,
        Self::WM_ICON_NAME,
        Self::WM_ICON_SIZE,
        Self::WM_NAME,
        Self::WM_NORMAL_HINTS,
        Self::WM_SIZE_HINTS,
        Self::WM_ZOOM_HINTS,
        Self::MIN_SPACE,
        Self::NORM_SPACE,
        Self::MAX_SPACE,
        Self::END_SPACE,
        Self::SUPERSCRIPT_X,
        Self::SUPERSCRIPT_Y,
        Self::SUBSCRIPT_X,
        Self::SUBSCRIPT_Y,
        Self::UNDERLINE_POSITION,
        Self::UNDERLINE_THICKNESS,
        Self::STRIKEOUT_ASCENT,
        Self::STRIKEOUT_DESCENT,
        Self::ITALIC_ANGLE,
        Self::X_HEIGHT,
        Self::QUAD_WIDTH,
        Self::WEIGHT,
        Self::POINT_SIZE,
        Self::RESOLUTION,
        Self::COPYRIGHT,
        Self::NOTICE,
        Self::FONT_NAME,
        Self::FAMILY_NAME,
        Self::FULL_NAME,
        Self::CAP_HEIGHT,
        Self::WM_CLASS,
        Self::WM_TRANSIENT_FOR,
    ];
}