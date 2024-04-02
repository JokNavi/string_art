pub mod pixel_density_lut;

#[cfg(test)]
mod tests {

    #[test]
    fn test_index() {
        let brightness = 50.0;
        let color = 255.0;
        let total_pixels = 100.0;
        dbg!((brightness / total_pixels) * color);
    }
}
