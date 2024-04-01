use rusttype::{point, Font, Rect, Scale, ScaledGlyph};
use std::ops::Index;

pub const LUT_LENGTH: usize = u8::MAX as usize + 1;

pub enum CharPixelDensityError {
    NoGlyphForChar,
    TooLargeGlyph,
    NoBoundingBoxForGlyph,
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct CharPixelDensityLut {
    char_lut: [char; LUT_LENGTH],
}

impl CharPixelDensityLut {
    pub fn new(chars: &[char], font: &Font, scale: Scale) -> Self {
        todo!();
    }

    pub fn from_lut(lut: [char; LUT_LENGTH]) -> Self {
        Self { char_lut: lut }
    }

    fn average_pixel_density(glyph: ScaledGlyph) -> Result<u8, CharPixelDensityError> {
        let point = point(0.0, 0.0);
        let dimensions = glyph
        .exact_bounding_box()
        .map(|rect| GlyphDimensions::from_bounding_box(&rect))
        .ok_or(CharPixelDensityError::NoBoundingBoxForGlyph)?;
        Ok(0)
    }

    fn create_lut(char_pixel_density_pairs: &[(char, u8)]) -> [char; LUT_LENGTH] {
        let mut lut = ['\x00'; LUT_LENGTH];
        let mut error_offsets = [u8::MAX; LUT_LENGTH];

        for (char, pixel_density) in char_pixel_density_pairs {
            lut[*pixel_density as usize] = *char;
            error_offsets[*pixel_density as usize] = 0u8;

            let mut i = 0usize;
            while i < *pixel_density as usize {
                let new_offset = pixel_density - i as u8;
                if error_offsets[i] < new_offset {
                    break;
                }
                error_offsets[i] = new_offset;
                lut[i] = *char;

                i += 1;
            }

            let mut i = (pixel_density + 1) as usize;
            while i < LUT_LENGTH {
                let new_offset = i as u8 - pixel_density;
                if error_offsets[i] < new_offset {
                    break;
                }
                error_offsets[i] = new_offset;
                lut[i] = *char;
                i += 1;
            }
        }
        lut
    }
}

impl Index<u8> for CharPixelDensityLut {
    type Output = char;

    fn index(&self, index: u8) -> &Self::Output {
        &self.char_lut[index as usize]
    }
}

struct GlyphDimensions {
    width: usize,
    height: usize,
}

impl GlyphDimensions {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn from_bounding_box(bounding_box: &Rect<f32>) -> Self {
        let width = bounding_box.width().ceil();
        let height = bounding_box.height().ceil();
        let width = width.abs() as usize;
        let height = height.abs() as usize;
        GlyphDimensions::new(width, height)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
