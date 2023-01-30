use futures::StreamExt;

use super::*;

pub use crate::coding::{CharInfo, DrawDirection, QueryTextExtentsReply};

#[derive(Clone, Copy, derivative::Derivative)]
#[derivative(Debug)]
pub struct Font<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

#[derive(Debug, Clone, Copy)]
pub enum Fontable<'a> {
    Font(Font<'a>),
    GContext(GContext<'a>),
}

impl<'a> From<Font<'a>> for Fontable<'a> {
    fn from(from: Font<'a>) -> Self {
        Fontable::Font(from)
    }
}

impl<'a> From<GContext<'a>> for Fontable<'a> {
    fn from(from: GContext<'a>) -> Self {
        Fontable::GContext(from)
    }
}

impl<'a> Fontable<'a> {
    pub(crate) fn handle(self) -> u32 {
        match self {
            Fontable::Font(x) => x.handle,
            Fontable::GContext(x) => x.handle,
        }
    }

    pub(crate) fn connection(self) -> &'a X11Connection {
        match self {
            Fontable::Font(x) => x.connection,
            Fontable::GContext(x) => x.connection,
        }
    }
}

impl<'a> Resource<'a> for Fontable<'a> {
    fn x11_handle(&self) -> u32 {
        self.handle()
    }

    fn from_x11_handle(_connection: &'a X11Connection, _handle: u32) -> Self {
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
    pub async fn open_font(&self, name: impl AsRef<str>) -> Result<Font<'_>> {
        let font = self.new_resource_id();

        send_request!(
            self,
            OpenFont {
                font: font,
                name: name.as_ref().to_string(),
            }
        );
        Ok(Font {
            handle: font,
            connection: self,
        })
    }

    pub async fn set_font_path(&self, paths: impl IntoIterator<Item = impl AsRef<str>>) -> Result<()> {
        send_request!(
            self,
            SetFontPath {
                paths: paths
                    .into_iter()
                    .map(|str| Str {
                        str: str.as_ref().to_string(),
                        ..Default::default()
                    })
                    .collect(),
            }
        );
        Ok(())
    }

    pub async fn get_font_path(&self) -> Result<Vec<String>> {
        let reply = send_request!(self, GetFontPathReply, GetFontPath {});

        Ok(reply.into_inner().paths.into_iter().map(|x| x.str).collect())
    }

    pub async fn list_fonts(&self, max_count: u16, pattern: &str) -> Result<Vec<String>> {
        let reply = send_request!(
            self,
            ListFontsReply,
            ListFonts {
                max_names: max_count,
                pattern: pattern.to_string(),
            }
        );

        Ok(reply.into_inner().names.into_iter().map(|x| x.str).collect())
    }

    pub async fn list_fonts_with_info(&self, max_count: u16, pattern: &str) -> Result<Vec<(String, FontQueryInfo)>> {
        let mut reply = send_request!(
            self,
            stream,
            ListFontsWithInfoReply,
            ListFontsWithInfo {
                max_names: max_count,
                pattern: pattern.to_string(),
            }
        );

        let mut raw_out = vec![];
        while let Some(item) = reply.next().await.transpose()? {
            if item.reserved == 0 {
                drop(reply);
                break;
            }
            raw_out.push(item.into_inner());
        }
        let mut out = Vec::with_capacity(raw_out.len());
        for item in raw_out {
            let mut properties = vec![];
            for property in item.fontprops {
                properties.push((self.get_atom_name(property.name_atom).await?, property.value));
            }
            out.push((
                item.name,
                FontQueryInfo {
                    min_bounds: item.min_bounds,
                    max_bounds: item.max_bounds,
                    min_char_or_byte2: item.min_char_or_byte2,
                    max_char_or_byte2: item.max_char_or_byte2,
                    default_char: item.default_char,
                    draw_direction: item.draw_direction,
                    min_byte1: item.min_byte1,
                    max_byte1: item.max_byte1,
                    all_chars_exist: item.all_chars_exist,
                    font_ascent: item.font_ascent,
                    font_descent: item.font_descent,
                    properties,
                    charinfos: vec![],
                },
            ));
        }

        Ok(out)
    }
}

impl<'a> Font<'a> {
    pub async fn close(self) -> Result<()> {
        send_request!(
            self.connection,
            CloseFont {
                font: self.handle,
            }
        );
        Ok(())
    }

    pub async fn query(self) -> Result<FontQueryInfo> {
        Fontable::Font(self).query_font().await
    }

    pub async fn query_text_extents(self, string: impl AsRef<str>) -> Result<QueryTextExtentsReply> {
        Fontable::Font(self).query_text_extents(string).await
    }
}

impl<'a> Fontable<'a> {
    pub async fn query_font(self) -> Result<FontQueryInfo> {
        let reply = send_request!(
            self.connection(),
            QueryFontReply,
            QueryFont {
                fontable: self.handle(),
            }
        )
        .into_inner();

        let mut properties = vec![];
        for property in reply.fontprops {
            properties.push((self.connection().get_atom_name(property.name_atom).await?, property.value));
        }

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
            properties,
            charinfos: reply.charinfos,
        })
    }

    pub async fn query_text_extents(self, string: impl AsRef<str>) -> Result<QueryTextExtentsReply> {
        let string = string.as_ref().to_string();
        let is_odd_length = (string.len() * 2) % 4 == 2;
        let reply = send_request!(self.connection(), reserved is_odd_length as u8, QueryTextExtentsReply, QueryTextExtents {
            fontable: self.handle(),
            string: string,
        });

        Ok(reply.into_inner())
    }
}

impl<'a> Resource<'a> for Font<'a> {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(connection: &'a X11Connection, handle: u32) -> Self {
        Self {
            connection,
            handle,
        }
    }
}
