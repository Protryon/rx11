#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel(pub u32);

impl Pixel {
    pub const fn rgb(&self) -> (u8, u8, u8) {
        (
            (self.0 >> 24) as u8,
            (self.0 >> 16) as u8,
            (self.0 >> 8) as u8,
        )
    }

    pub const fn from_rgb(red: u8, green: u8, blue: u8) -> Self {
        Self((red as u32) << 24 | (green as u32) << 16 | (blue as u32) << 8)
    }

    pub const WHITE: Pixel = Pixel::from_rgb(255, 255, 255);
    pub const BLACK: Pixel = Pixel::from_rgb(0, 0, 0);
}
