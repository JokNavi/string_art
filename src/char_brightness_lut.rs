pub const LUT_LENGTH: usize = u8::MAX as usize + 1;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct CharBrightnessLut {
    char_lut: [char; 256],
}

