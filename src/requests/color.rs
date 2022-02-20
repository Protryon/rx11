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

pub use crate::coding::{ Rgb, LookupColorReply };

impl X11Connection {

    pub async fn alloc_color(&self, colormap: Colormap, red: u16, green: u16, blue: u16) -> Result<Pixel> {
        let seq = send_request!(self, AllocColor {
            colormap: colormap.handle,
            red: red,
            green: green,
            blue: blue,
        });
        let reply = receive_reply!(self, seq, AllocColorReply);

        Ok(Pixel(reply.pixel))
    }

    pub async fn alloc_named_color(&self, colormap: Colormap, name: &str) -> Result<Pixel> {
        let seq = send_request!(self, AllocNamedColor {
            colormap: colormap.handle,
            name: name.to_string(),
        });
        let reply = receive_reply!(self, seq, AllocNamedColorReply);

        Ok(Pixel(reply.pixel))
    }
    
    pub async fn alloc_color_cells(&self, colormap: Colormap, contiguous: bool, colors: u16, planes: u16) -> Result<(Vec<Pixel>, Vec<Pixel>)> {
        let seq = send_request!(self, contiguous as u8, AllocColorCells {
            colormap: colormap.handle,
            colors: colors,
            planes: planes,
        });
        let reply = receive_reply!(self, seq, AllocColorCellsReply);

        Ok((
            reply.pixels.into_iter().map(Pixel).collect(),
            reply.masks.into_iter().map(Pixel).collect()
        ))
    }

    pub async fn alloc_color_planes(&self, colormap: Colormap, contiguous: bool, colors: u16, reds: u16, greens: u16, blues: u16) -> Result<ColorPlanes> {
        let seq = send_request!(self, contiguous as u8, AllocColorPlanes {
            colormap: colormap.handle,
            colors: colors,
            reds: reds,
            greens: greens,
            blues: blues,
        });
        let reply = receive_reply!(self, seq, AllocColorPlanesReply);

        Ok(ColorPlanes {
            pixels: reply.pixels.into_iter().map(Pixel).collect(),
            red_mask: reply.red_mask,
            green_mask: reply.green_mask,
            blue_mask: reply.blue_mask,
        })
    }

    pub async fn free_colors(&self, colormap: Colormap, plane_mask: u32, pixels: Vec<Pixel>) -> Result<()> {
        send_request!(self, FreeColors {
            colormap: colormap.handle,
            plane_mask: plane_mask,
            pixels: pixels.into_iter().map(|x| x.0).collect(),
        });

        Ok(())
    }

    pub async fn store_colors(&self, colormap: Colormap, items: &[ColorItem]) -> Result<()> {
        send_request!(self, StoreColors {
            colormap: colormap.handle,
            items: items.iter().map(|item| crate::coding::ColorItem {
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
            }).collect(),
        });

        Ok(())
    }

    pub async fn store_named_color(&self, colormap: Colormap, pixel: Pixel, name: &str, do_red: bool, do_green: bool, do_blue: bool) -> Result<()> {
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
        send_request!(self, flags, StoreNamedColor {
            colormap: colormap.handle,
            pixel: pixel.0,
            name: name.to_string(),
        });

        Ok(())
    }

    pub async fn query_colors(&self, colormap: Colormap, pixels: &[Pixel]) -> Result<Vec<Rgb>> {
        let seq = send_request!(self, QueryColors {
            colormap: colormap.handle,
            pixels: pixels.iter().map(|x| x.0).collect(),
        });

        let reply = receive_reply!(self, seq, QueryColorsReply);

        Ok(reply.colors)
    }

    pub async fn lookup_color(&self, colormap: Colormap, name: &str) -> Result<LookupColorReply> {
        let seq = send_request!(self, LookupColor {
            colormap: colormap.handle,
            name: name.to_string(),
        });

        let reply = receive_reply!(self, seq, LookupColorReply);

        Ok(reply)
    }
}