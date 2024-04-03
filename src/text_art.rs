use crate::pixel_density_lut::PixelDensityLut;
use image::{DynamicImage, GrayImage, Luma};
use rusttype::{point, Font, Scale, ScaledGlyph};

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

    fn encode_rows(&self, image: &DynamicImage) -> Vec<String> {
        let image = image.to_luma8();
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

    pub fn encode(&self, image: &DynamicImage) -> String {
        self.encode_rows(image).join("\n")
    }

    pub fn encode_alternating(&self, image: &DynamicImage) -> String {
        self.encode_rows(image)
            .iter()
            .step_by(2)
            .cloned()
            .collect::<Vec<String>>()
            .join("\n")
    }
}

pub fn string_to_image(font: Font, scale: Scale, string: &str) -> GrayImage {
    let glyphs = string
        .lines()
        .map(|row| {
            row.chars()
                .map(|char| font.glyph(char).scaled(scale))
                .collect::<Vec<ScaledGlyph>>()
        })
        .collect::<Vec<Vec<ScaledGlyph>>>();
    let image_width = glyphs.iter().fold(0.0f32, |acc, row| {
        let row_width = row
            .iter()
            .map(|glyph| {
                let h_metrics = glyph.h_metrics();
                h_metrics.advance_width + h_metrics.left_side_bearing
            })
            .sum::<f32>().ceil();
        acc.max(row_width)
    });
    let image_height = glyphs.len() as f32 * scale.y;
    let mut image = GrayImage::new(image_width as u32, image_height as u32);
    for (y, row) in glyphs.iter().enumerate() {
        let mut image_x: u32 = 0;
        for (_, glyph) in row.iter().enumerate() {
            let width = {
                let h_metrics = glyph.h_metrics();
                h_metrics.advance_width + h_metrics.left_side_bearing
            }.ceil();
            let height = glyph.scale().y.ceil();
            let image_y = y as u32 * height as u32;
            let glyph = glyph.clone().positioned(point(0.0, 0.0));
            glyph.draw(|glyph_x, glyph_y, v| {
                image.put_pixel(
                    image_x + glyph_x,
                    image_y + glyph_y,
                    Luma([(v * 255.0) as u8]),
                );
            });
            image_x += width as u32;
        }
    }
    image
}

#[cfg(test)]
mod text_art_image_encoder_tests {
    use rusttype::{Font, Scale};

    use crate::text_art::string_to_image;

    #[test]
    fn test_encode() {
        let scale = Scale::uniform(25.0);
        const FONT_BYTES: &[u8] = include_bytes!("../files/RobotoMono-Regular.ttf");
        let font = Font::try_from_bytes(FONT_BYTES).unwrap();
        let image = string_to_image(font, scale, "BBBAAACCC\nAAABBBCCC");
        let _ = image.save("files/output/test-pattern.jpg");
    }
}
