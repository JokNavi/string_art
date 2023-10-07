
pub const LUT_LENGTH: usize = u8::MAX as usize + 1;

#[derive(Debug)]
pub struct CharBrightnesses {
    char_lut: [char; LUT_LENGTH],
}


impl Default for CharBrightnesses {
    fn default() -> Self {
        //const LUT: [char; LUT_LENGTH] = [];
        //Self { char_lut: LUT }
        todo!();
        
    }
}