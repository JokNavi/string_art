use crate::pixel_density_lut::PixelDensityLut;
use image::{GrayImage, Luma};
use rusttype::{point, Font, Scale};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TextArtStringEncoder {
    pixel_density_lut: PixelDensityLut,
}

impl TextArtStringEncoder {
    pub fn new<L: Into<PixelDensityLut>>(pixel_density_lut: L) -> Self {
        Self {
            pixel_density_lut: pixel_density_lut.into(),
        }
    }

    fn encode_rows(&self, image: impl Into<GrayImage>) -> Vec<String> {
        let image: GrayImage = image.into();
        image
            .chunks(image.width() as usize)
            .map(|byte_row| {
                byte_row
                    .iter()
                    .map(|byte| self.pixel_density_lut[*byte])
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
    }

    pub fn encode(&self, image: impl Into<GrayImage>) -> String {
        self.encode_rows(image).join("\n")
    }

    pub fn encode_alternating(&self, image: impl Into<GrayImage>) -> String {
        self.encode_rows(image)
            .iter()
            .step_by(2)
            .cloned()
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Debug, Clone)]
pub struct TextArtImageEncoder<'a> {
    font: Font<'a>,
    scale: Scale,
    string_encoder: TextArtStringEncoder,
}

impl<'a> TextArtImageEncoder<'a> {
    pub fn new(font: Font<'a>, scale: Scale, characters: &str) -> Self {
        let pixel_density_lut = PixelDensityLut::new(characters, &font, scale);
        let string_encoder = TextArtStringEncoder::new(pixel_density_lut);
        Self {
            font,
            scale,
            string_encoder,
        }
    }

    fn get_buffer(&self, image: &GrayImage) -> GrayImage {
        let glyph_width = self.scale.x.floor() as u32;
        let glyph_height = self.scale.y.floor() as u32;
        GrayImage::new(image.width() * glyph_width, image.height() * glyph_height)
    }

    fn string_to_image(&self, mut buffer: GrayImage, string: &str) -> GrayImage {
        let glyph_width = self.scale.x.floor() as u32;
        let glyph_height = self.scale.y.floor() as u32;
        for (image_y, row) in string.lines().enumerate() {
            for (image_x, char) in row.chars().enumerate() {
                let glyph = self
                    .font
                    .glyph(char)
                    .scaled(self.scale)
                    .positioned(point(image_x as f32 * glyph_width as f32, image_y as f32));
                glyph.draw(|x, y, v| {
                    if v > 0.0 {
                        buffer.put_pixel(
                            image_x as u32 * glyph_width + x,
                            image_y as u32 * glyph_height + y,
                            Luma([(v * 255.0) as u8]),
                        );
                    }
                });
            }
        }
        buffer
    }

    pub fn encode(&self, image: impl Into<GrayImage>) -> GrayImage {
        let image = image.into();
        let buffer = self.get_buffer(&image);
        let string = self.string_encoder.encode(image);
        self.string_to_image(buffer, &string)
    }

    pub fn encode_alternate(&self, image: impl Into<GrayImage>) -> GrayImage {
        let image = image.into();
        let buffer = self.get_buffer(&image);
        let string = self.string_encoder.encode_alternating(image);
        self.string_to_image(buffer, &string)
    }
}

impl Default for TextArtImageEncoder<'static> {
    fn default() -> Self {
        let scale = Scale::uniform(12.0);
        let font = Font::try_from_bytes(include_bytes!("../files/RobotoMono-Regular.ttf")).unwrap();
        Self {
            scale,
            font,
            string_encoder: TextArtStringEncoder::default(),
        }
    }
}

#[cfg(test)]
mod text_art_image_encoder_tests {
    use super::*;
    use image::io::Reader;

    #[test]
    fn test_encode() {
        let image = Reader::open("files/input/test-pattern-small.webp")
            .unwrap()
            .decode()
            .unwrap();
        let image_encoder = TextArtImageEncoder::default();
        let image = image_encoder.encode(image);
        image.save("files/output/test-pattern.jpg").unwrap();
    }
}
