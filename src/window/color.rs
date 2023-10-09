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
    fn rgb_to_u32(r: u8, g: u8, b: u8) -> u32 {
        let (r, g, b) = (r as u32, g as u32, b as u32);
        (r << 16) | (g << 8) | b
    }

    pub fn to_u32(&self) -> u32 {
        match self {
            Color::Rgb(r, g, b) => Self::rgb_to_u32(*r, *g, *b),
            Color::Grayscale(v) => Self::rgb_to_u32(*v, *v, *v),
        }
    }
}

impl From<Color> for u32 {
    fn from(color: Color) -> u32 {
        color.to_u32()
    }
}

#[cfg(test)]
mod color_tests {
    use super::*;

    #[test]
    fn to_u32_test() {
        for r in 0..255u32 {
            for g in 0..255u32 {
                for b in 0..255u32 {
                    assert_eq!(
                        Color::rgb_to_u32(r as u8, g as u8, b as u8),
                        (r << 16) | (g << 8) | b
                    );
                }
            }
        }
    }

    #[test]
    fn into_u32() {
        let color: u32 = Color::Grayscale(100).into();
        let color: u32 = Color::Rgb(100, 50, 0).into();
    }
}
