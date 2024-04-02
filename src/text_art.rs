use crate::pixel_density_lut::PixelDensityLut;
use image::DynamicImage;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TextArtEncoder {
    pixel_density_lut: PixelDensityLut,
}

impl TextArtEncoder {
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
        self.encode_rows(image)
            .iter_mut()
            .map(|row| {
                row.push('\n');
                row.clone()
            })
            .collect::<String>()
    }

    pub fn encode_alternating(&self, image: &DynamicImage) -> String {
        self.encode_rows(image)
            .iter_mut().step_by(2)
            .map(|row| {
                row.push('\n');
                row.clone()
            })
            .collect::<String>()
    }
}
