use super::*;

#[derive(Debug, Clone)]
pub struct ColorPlanes {
    pub pixels: Vec<Pixel>,
    pub red_mask: u32,
    pub green_mask: u32,
    pub blue_mask: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct ColorItem {
    pub pixel: Pixel,
    pub red: Option<u16>,
    pub green: Option<u16>,
    pub blue: Option<u16>,
}

pub use crate::coding::{LookupColorReply, Rgb};

impl<'a> Colormap<'a> {
    pub async fn alloc_color(self, red: u16, green: u16, blue: u16) -> Result<Pixel> {
        let reply = send_request!(
            self.connection,
            AllocColorReply,
            AllocColor {
                colormap: self.handle,
                red: red,
                green: green,
                blue: blue,
            }
        );

        Ok(Pixel(reply.pixel))
    }

    pub async fn alloc_named_color(self, name: &str) -> Result<Pixel> {
        let reply = send_request!(
            self.connection,
            AllocNamedColorReply,
            AllocNamedColor {
                colormap: self.handle,
                name: name.to_string(),
            }
        );

        Ok(Pixel(reply.pixel))
    }

    pub async fn alloc_color_cells(self, contiguous: bool, colors: u16, planes: u16) -> Result<(Vec<Pixel>, Vec<Pixel>)> {
        let reply = send_request!(self.connection, reserved contiguous as u8, AllocColorCellsReply, AllocColorCells {
            colormap: self.handle,
            colors: colors,
            planes: planes,
        })
        .into_inner();

        Ok((reply.pixels.into_iter().map(Pixel).collect(), reply.masks.into_iter().map(Pixel).collect()))
    }

    pub async fn alloc_color_planes(self, contiguous: bool, colors: u16, reds: u16, greens: u16, blues: u16) -> Result<ColorPlanes> {
        let reply = send_request!(self.connection, reserved contiguous as u8, AllocColorPlanesReply, AllocColorPlanes {
            colormap: self.handle,
            colors: colors,
            reds: reds,
            greens: greens,
            blues: blues,
        });

        Ok(ColorPlanes {
            red_mask: reply.red_mask,
            green_mask: reply.green_mask,
            blue_mask: reply.blue_mask,
            pixels: reply.into_inner().pixels.into_iter().map(Pixel).collect(),
        })
    }

    pub async fn free_colors(self, plane_mask: u32, pixels: Vec<Pixel>) -> Result<()> {
        send_request!(
            self.connection,
            FreeColors {
                colormap: self.handle,
                plane_mask: plane_mask,
                pixels: pixels.into_iter().map(|x| x.0).collect(),
            }
        );

        Ok(())
    }

    pub async fn store_colors(self, items: &[ColorItem]) -> Result<()> {
        send_request!(
            self.connection,
            StoreColors {
                colormap: self.handle,
                items: items
                    .iter()
                    .map(|item| crate::coding::ColorItem {
                        pixel: item.pixel.0,
                        red: item.red.unwrap_or_default(),
                        green: item.green.unwrap_or_default(),
                        blue: item.blue.unwrap_or_default(),
                        color_flags: {
                            let mut out = 0u8;
                            if item.red.is_some() {
                                out |= ColorFlag::Red as u8;
                            }
                            if item.green.is_some() {
                                out |= ColorFlag::Green as u8;
                            }
                            if item.blue.is_some() {
                                out |= ColorFlag::Blue as u8;
                            }
                            out
                        },
                        reserved: 0,
                    })
                    .collect(),
            }
        );

        Ok(())
    }

    pub async fn store_named_color(self, pixel: Pixel, name: &str, do_red: bool, do_green: bool, do_blue: bool) -> Result<()> {
        let flags = {
            let mut out = 0u8;
            if do_red {
                out |= ColorFlag::Red as u8;
            }
            if do_green {
                out |= ColorFlag::Green as u8;
            }
            if do_blue {
                out |= ColorFlag::Blue as u8;
            }
            out
        };
        send_request!(self.connection, reserved flags, StoreNamedColor {
            colormap: self.handle,
            pixel: pixel.0,
            name: name.to_string(),
        });

        Ok(())
    }

    pub async fn query_colors(self, pixels: &[Pixel]) -> Result<Vec<Rgb>> {
        let reply = send_request!(
            self.connection,
            QueryColorsReply,
            QueryColors {
                colormap: self.handle,
                pixels: pixels.iter().map(|x| x.0).collect(),
            }
        );

        Ok(reply.into_inner().colors)
    }

    pub async fn lookup_color(self, name: &str) -> Result<LookupColorReply> {
        let reply = send_request!(
            self.connection,
            LookupColorReply,
            LookupColor {
                colormap: self.handle,
                name: name.to_string(),
            }
        );

        Ok(reply.into_inner())
    }
}
