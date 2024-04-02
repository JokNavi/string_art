use image::{DynamicImage, GrayAlphaImage, GrayImage};
use rusttype::{Font, Scale};

use crate::pixel_density_lut::{self, PixelDensityLut};

#[derive(Debug, Clone)]
pub struct TextArtBuilder<'a> {
    font: Option<Font<'a>>,
    scale: Option<Scale>,
    chars: Option<String>,
}

impl<'a> TextArtBuilder<'a> {
    pub fn new() -> Self {
        Self {
            font: None,
            scale: None,
            chars: None,
        }
    }

    pub fn with_font(&mut self, font: Font<'a>) -> &mut Self {
        self.font = Some(font);
        self
    }

    pub fn with_scale(&mut self, scale: Scale) -> &mut Self {
        self.scale = Some(scale);
        self
    }

    pub fn with_chars(&mut self, chars: &str) -> &mut Self {
        self.chars = Some(chars.to_owned());
        self
    }

    fn pixel_density_lut(&self) -> PixelDensityLut {
        let font = self.font.clone().unwrap_or(Font::try_from_bytes(include_bytes!("../files/RobotoMono-Regular.ttf")).unwrap());
        let scale = self.scale.clone().unwrap_or(Scale::uniform(50.0));
        let chars = self.chars.clone().unwrap_or(" !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~".to_owned());
        PixelDensityLut::new(&chars, self.font.as_ref().unwrap(), self.scale.unwrap())
    }

    pub fn build_string(&mut self, image: &GrayImage) -> String {
        let pixel_density_lut = self.pixel_density_lut();
        let image_width: usize = image.width() as usize;
        let mut text_art_string = String::with_capacity(image_width * image.height() as usize);
        for (index, value) in image.iter().enumerate() {
            if (index + 1) % image_width == 0 {
                text_art_string.push('\n');
            }
        }
        todo!();
    }
}
