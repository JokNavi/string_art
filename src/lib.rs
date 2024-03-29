pub mod char_brightness_lut;
pub mod char_pixel_density_lut;
pub mod char_pixel_density_lut_builder;

#[cfg(test)]
mod tests {

    #[test]
    fn test_index() {
        let index = 255u8;
        dbg!(index.saturating_add(1));
    }
}