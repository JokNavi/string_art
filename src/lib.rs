pub mod pixel_density_lut;
pub mod text_art;

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
        const SCALE: f32 = 50.0;
        let font = Font::try_from_bytes(FONT_BYTES).unwrap();
        let scale = Scale::uniform(SCALE);
        let lut = PixelDensityLut::new(CHARS, &font, scale);
        println!("{:?}", &lut);
        println!("{}", '\u{1FB4D}');
    }
}
