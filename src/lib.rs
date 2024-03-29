pub mod char_brightness_lut;
pub mod char_density_lut;

#[cfg(test)]
mod tests {

    #[test]
    fn test_index() {
        let index = 255u8;
        dbg!(index.saturating_add(1));
    }
}