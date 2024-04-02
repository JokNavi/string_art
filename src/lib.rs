pub mod pixel_density_lut;

#[cfg(test)]
mod tests {
    use rusttype::{Font, Scale};

    use crate::pixel_density_lut::PixelDensityLut;


    #[test]
    fn test_index() {
        let brightness = 50.0;
        let color = 255.0;
        let total_pixels = 100.0;
        dbg!((brightness / total_pixels) * color);
    }

    #[test]
    fn test_pixel_density_lut() {
        const CHARS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
        const FONT_BYTES: &[u8] = include_bytes!("../files/RobotoMono-Regular.ttf");
        const SCALE: u8 = 50;
        let font = Font::try_from_bytes(FONT_BYTES).unwrap();
        let scale = Scale::uniform(SCALE as f32);
        let lut = PixelDensityLut::from_str(CHARS, &font, scale);
        println!("{:?}", &lut);
        println!("{}", '\u{1FB4D}');
    }
}
