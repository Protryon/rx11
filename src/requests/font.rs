use super::*;

pub use crate::coding::{
    CharInfo,
    DrawDirection,
    QueryTextExtentsReply,
};

#[derive(Debug, Clone, Copy)]
pub struct Font {
    pub(crate) handle: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum Fontable {
    Font(Font),
    GContext(GContext),
}

impl From<Font> for Fontable {
    fn from(from: Font) -> Self {
        Fontable::Font(from)
    }
}

impl From<GContext> for Fontable {
    fn from(from: GContext) -> Self {
        Fontable::GContext(from)
    }
}

impl Fontable {
    pub(crate) fn handle(&self) -> u32 {
        match self {
            Fontable::Font(x) => x.handle,
            Fontable::GContext(x) => x.handle,
        }
    }
}

impl Resource for Fontable {
    fn x11_handle(&self) -> u32 {
        self.handle()
    }

    fn from_x11_handle(_handle: u32) -> Self {
        unimplemented!("cannot call from_x11_handle on Fontable");
    }
}

#[derive(Clone, Debug)]
pub struct FontQueryInfo {
    pub min_bounds: CharInfo,
    pub max_bounds: CharInfo,
    pub min_char_or_byte2: u16,
    pub max_char_or_byte2: u16,
    pub default_char: u16,
    pub draw_direction: DrawDirection,
    pub min_byte1: u8,
    pub max_byte1: u8,
    pub all_chars_exist: bool,
    pub font_ascent: i16,
    pub font_descent: i16,
    pub properties: Vec<(Atom, u32)>,
    pub charinfos: Vec<CharInfo>,
}

impl X11Connection {
    pub async fn open_font(&self, name: impl AsRef<str>) -> Result<Font> {
        let font = self.new_resource_id();
        
        send_request!(self, OpenFont {
            font: font,
            name: name.as_ref().to_string(),
        });
        Ok(Font {
            handle: font,
        })
    }

    pub async fn close_font(&self, font: Font) -> Result<()> {
        send_request!(self, CloseFont {
            font: font.handle,
        });
        Ok(())
    }

    pub async fn query_font(&self, fontable: impl Into<Fontable>) -> Result<FontQueryInfo> {
        let seq = send_request!(self, QueryFont {
            fontable: fontable.into().handle(),
        });
        let reply = receive_reply!(self, seq, QueryFontReply);

        Ok(FontQueryInfo {
            min_bounds: reply.min_bounds,
            max_bounds: reply.max_bounds,
            min_char_or_byte2: reply.min_char_or_byte2,
            max_char_or_byte2: reply.max_char_or_byte2,
            default_char: reply.default_char,
            draw_direction: reply.draw_direction,
            min_byte1: reply.min_byte1,
            max_byte1: reply.max_byte1,
            all_chars_exist: reply.all_chars_exist,
            font_ascent: reply.font_ascent,
            font_descent: reply.font_descent,
            properties: reply.fontprops.into_iter().map(|prop| (
                Atom {
                    handle: prop.name_atom,
                },
                prop.value,
            )).collect(),
            charinfos: reply.charinfos,
        })
    }
    
    pub async fn query_text_extents(&self, fontable: impl Into<Fontable>, string: impl AsRef<str>) -> Result<QueryTextExtentsReply> {
        let string = string.as_ref().to_string();
        let is_odd_length = (string.len() * 2) % 4 == 2;
        let seq = send_request!(self, is_odd_length as u8, QueryTextExtents {
            fontable: fontable.into().handle(),
            string: string,
        });
        let reply = receive_reply!(self, seq, QueryTextExtentsReply);

        Ok(reply)
    }

    //todo: get_fonts_with_info

    pub async fn set_font_path(&self, paths: impl IntoIterator<Item=impl AsRef<str>>) -> Result<()> {
        send_request!(self, SetFontPath {
            paths: paths.into_iter().map(|str| Str {
                str: str.as_ref().to_string(),
                ..Default::default()
            }).collect(),
        });
        Ok(())
    }

    pub async fn get_font_path(&self) -> Result<Vec<String>> {
        let seq = send_request!(self, GetFontPath {
        });
        let reply = receive_reply!(self, seq, GetFontPathReply);

        Ok(reply.paths.into_iter().map(|x| x.str).collect())
    }
}

impl Resource for Font {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(handle: u32) -> Self {
        Self { handle }
    }
}