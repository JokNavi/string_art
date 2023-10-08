pub enum Color {
    Rgb(u8, u8, u8),
    Grayscale(u8),
}

impl Color {
    /// Pixels are represented by u32 RGB values:
    /// Bits 32 through 24 are unused
    /// Bits 24 through 16 represent Red value
    /// Bits 16 through 8 represent Green value
    /// Bits 8 through 0 represent Blue value
    pub fn to_u32(&self) -> u32 {
        match self {
            Color::Rgb(r, g, b) => {
                let (r, g, b) = (*r as u32, *g as u32, *b as u32);
                (r << 16) | (g << 8) | b
            }
            Color::Grayscale(v) => {
                let (r, g, b) = (*v as u32, *v as u32, *v as u32);
                (r << 16) | (g << 8) | b
            },
        }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        color.to_u32()
    }
}
