use super::*;

#[derive(Clone, Copy, derivative::Derivative)]
#[derivative(Debug)]
pub struct Cursor<'a> {
    pub(crate) handle: u32,
    #[derivative(Debug = "ignore")]
    pub(crate) connection: &'a X11Connection,
}

#[derive(Debug, Clone, Copy)]
pub struct Rgb16 {
    pub red: u16,
    pub green: u16,
    pub blue: u16,
}

impl X11Connection {
    pub async fn create_cursor(&self, source: Pixmap<'_>, mask: Option<Pixmap<'_>>, fore: Rgb16, back: Rgb16, x: u16, y: u16) -> Result<Cursor<'_>> {
        let cursor = self.new_resource_id();

        send_request!(
            self,
            CreateCursor {
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
            }
        );
        Ok(Cursor {
            handle: cursor,
            connection: self,
        })
    }

    pub async fn create_glyph_cursor(
        &self,
        source: Font<'_>,
        mask: Option<Font<'_>>,
        source_char: u16,
        mask_char: u16,
        fore: Rgb16,
        back: Rgb16,
    ) -> Result<Cursor<'_>> {
        let cursor = self.new_resource_id();

        send_request!(
            self,
            CreateGlyphCursor {
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
            }
        );
        Ok(Cursor {
            handle: cursor,
            connection: self,
        })
    }
}

impl<'a> Cursor<'a> {
    pub async fn free_cursor(self) -> Result<()> {
        send_request!(
            self.connection,
            FreeCursor {
                cursor: self.handle,
            }
        );
        Ok(())
    }

    pub async fn recolor_cursor(self, fore: Rgb16, back: Rgb16) -> Result<()> {
        send_request!(
            self.connection,
            RecolorCursor {
                cursor: self.handle,
                color: CursorColor {
                    fore_red: fore.red,
                    fore_green: fore.green,
                    fore_blue: fore.blue,
                    back_red: back.red,
                    back_green: back.green,
                    back_blue: back.blue,
                },
            }
        );
        Ok(())
    }
}

impl<'a> Resource<'a> for Cursor<'a> {
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
