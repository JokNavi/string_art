pub const LUT_LENGTH: usize = u8::MAX as usize + 1;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct CharDensityLut {
    char_lut: [char; 256],
}

impl CharDensityLut {
    pub fn new() -> Self {
        todo!();
    }

    pub fn from_lut(lut: [char; 256]) -> Self {
        Self { char_lut: lut }
    }   
}