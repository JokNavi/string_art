/// Pixles are represented by u32 RGB values:
/// Bits 32 through 24 are unused
/// Bits 24 through 16 represent Red value
/// Bits 16 through 8 represent Green value
/// Bits 8 through 0 represent Blue value
pub type Pixel = u32;
pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> Pixel {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b //shift rgb values to correct places in u32, perform bitwise or to concatonate them together
}
