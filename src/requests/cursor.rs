use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Cursor {
    pub(crate) handle: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb16 {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl X11Connection {
    pub async fn create_cursor(&self, source: Pixmap, mask: Option<Pixmap>, fore: Rgb16, back: Rgb16, x: u16, y: u16) -> Result<Cursor> {
        let cursor = self.new_resource_id();
        
        send_request!(self, CreateCursor {
            cursor: cursor,
            source: source.handle,
            mask: mask.map(|x| x.handle).unwrap_or(0),
            color: CursorColor {
                fore_red: fore.red,
                fore_green: fore.green,
                fore_blue: fore.blue,
                back_red: back.red,
                back_green: back.green,
                back_blue: back.blue,
            },
            x: x,
            y: y,
        });
        Ok(Cursor {
            handle: cursor,
        })
    }

    pub async fn create_glyph_cursor(&self, source: Font, mask: Option<Font>, source_char: u16, mask_char: u16, fore: Rgb16, back: Rgb16) -> Result<Cursor> {
        let cursor = self.new_resource_id();
        
        send_request!(self, CreateGlyphCursor {
            cursor: cursor,
            source_font: source.handle,
            mask_font: mask.map(|x| x.handle).unwrap_or(0),
            color: CursorColor {
                fore_red: fore.red,
                fore_green: fore.green,
                fore_blue: fore.blue,
                back_red: back.red,
                back_green: back.green,
                back_blue: back.blue,
            },
            source_char: source_char,
            mask_char: mask_char,
        });
        Ok(Cursor {
            handle: cursor,
        })
    }

    pub async fn free_cursor(&self, cursor: Cursor) -> Result<()> {
        send_request!(self, FreeCursor {
            cursor: cursor.handle,
        });
        Ok(())
    }

    pub async fn recolor_cursor(&self, cursor: Cursor, fore: Rgb16, back: Rgb16) -> Result<()> {
        send_request!(self, RecolorCursor {
            cursor: cursor.handle,
            color: CursorColor {
                fore_red: fore.red,
                fore_green: fore.green,
                fore_blue: fore.blue,
                back_red: back.red,
                back_green: back.green,
                back_blue: back.blue,
            },
        });
        Ok(())
    }

}

impl Resource for Cursor {
    fn x11_handle(&self) -> u32 {
        self.handle
    }

    fn from_x11_handle(handle: u32) -> Self {
        Self { handle }
    }
}