use rusttype::{point, Font, Scale, ScaledGlyph};
use std::ops::Index;

pub const LUT_LENGTH: usize = u8::MAX as usize + 1;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct PixelDensityLut {
    char_lut: [char; LUT_LENGTH],
}

impl PixelDensityLut {
    pub fn new(chars: &str, font: &Font, scale: Scale) -> Self {
        let mut char_pixel_density_pairs = vec![(' ', u8::MIN); chars.len()];
        for (i, char) in chars.char_indices() {
            char_pixel_density_pairs[i] = (
                char,
                Self::average_pixel_density(&font.glyph(char).scaled(scale)),
            );
        }
        let pixel_density_lut = Self::create_lut(&char_pixel_density_pairs);
        Self::from_lut(pixel_density_lut)
    }

    pub fn from_chars(chars: &[char], font: &Font, scale: Scale) -> Self {
        let mut char_pixel_density_pairs = vec![('\x00', u8::MIN); chars.len()];
        for (i, char) in chars.iter().enumerate() {
            char_pixel_density_pairs[i] = (
                *char,
                Self::average_pixel_density(&font.glyph(*char).scaled(scale)),
            );
        }
        let pixel_density_lut = Self::create_lut(&char_pixel_density_pairs);
        Self::from_lut(pixel_density_lut)
    }    

    pub fn from_lut(lut: [char; LUT_LENGTH]) -> Self {
        Self { char_lut: lut }
    }

    fn average_pixel_density(glyph: &ScaledGlyph) -> u8 {
        let point = point(0.0, 0.0);
        let (width, height) = Self::glyph_dimensions(&glyph);
        let mut buffer = vec![vec![0u16; width as usize + 1]; height as usize + 1];
        let glyph = glyph.clone().positioned(point);
        glyph.draw(|x, y, _| {
            buffer[y as usize][x as usize] = 1u16;
        });
        let total_pixels = ((buffer.len() + 1) * (buffer[0].len() + 1)) as u16;
        let sum = buffer
            .iter()
            .map(|vec| vec.iter().sum::<u16>())
            .sum::<u16>();
        (((sum as f64 * 255.0) / total_pixels as f64).round()) as u8
    }

    fn glyph_dimensions(glyph: &ScaledGlyph) -> (f32, f32) {
        let h_metrics = glyph.h_metrics();
        (
            h_metrics.advance_width + h_metrics.left_side_bearing,
            glyph.scale().y,
        )
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

impl Default for PixelDensityLut {
    fn default() -> Self {
        const LUT: [char; LUT_LENGTH] = [
            ' ', ' ', ' ', ' ', ' ', '.', '\'', '\'', '`', '`', '_', '-', '-', '-', '-', '-', '"',
            '"', '"', '"', '|', '|', '|', '|', '|', '!', '!', '!', '!', '!', '!', '!', '!', '!',
            ';', '~', '~', '=', '^', '^', '^', '^', '^', '^', '^', ']', ']', ']', 'r', 'r', 'r',
            '[', '<', '<', '>', '>', '>', '1', '1', '1', '1', '1', '1', '1', '1', 'n', 'u', 'u',
            's', 's', '(', '(', 'z', 'z', 'z', '+', 'o', 'o', 'o', 'x', 'E', 'E', 'E', 'm', 'j',
            '}', '?', '?', 'i', 'i', 'h', 'l', 'Z', 'N', 'N', 'w', 'w', 'U', 'd', 'd', 'J', 'J',
            '4', '4', 'f', 'f', 'f', 'X', 'X', 'O', 'O', 'Y', 'Y', 'Y', 'Y', '@', '@', '@', '@',
            'y', '&', '&', '&', '&', '$', '%', '%', '%', '%', '%', '%', 'Q', 'Q', 'Q', 'Q', 'Q',
            'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q',
            'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q',
            'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q',
            'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q',
            'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q',
            'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q',
            'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q', 'Q',
            'Q',
        ];
        Self::from_lut(LUT)
    }
}

impl Index<u8> for PixelDensityLut {
    type Output = char;

    fn index(&self, index: u8) -> &Self::Output {
        &self.char_lut[index as usize]
    }
}

#[cfg(test)]
mod char_pixel_density_lut_tests {
    use super::*;

    #[test]
    fn test_index() {
        let lut = super::PixelDensityLut::create_lut(&vec![('b', 2), ('a', 1)]);
        assert_eq!(lut[1], 'a');
        assert_eq!(lut[2], 'b');
    }

    #[test]
    fn test_average_pixel_density() {
        let scale = Scale::uniform(100.0);
        const FONT_BYTES: &[u8] = include_bytes!("../files/RobotoMono-Regular.ttf");
        let font = Font::try_from_bytes(FONT_BYTES).unwrap();
        let glyph = font.glyph('a').scaled(scale);
        let pixel_density = PixelDensityLut::average_pixel_density(&glyph);
        dbg!(pixel_density);
    }
}
