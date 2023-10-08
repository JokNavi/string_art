use rusttype::{point, Font, Scale, ScaledGlyph};

pub const LUT_LENGTH: usize = u8::MAX as usize + 1;
pub const COLOR: f32 = 255.0;
pub const BRIGHTNESS_CUTOFF: f32 = 0.5;

#[derive(Debug, PartialEq)]
pub struct CharBrightnesses {
    char_lut: [char; LUT_LENGTH],
}

impl CharBrightnesses {
    pub fn new(chars: &str, font: &Font, scale: u8) -> Self {
        let brightnesses_tuples =
            Self::get_brightness_tuples(chars, &font, &Scale::uniform(scale as f32));
        CharBrightnesses {
            char_lut: Self::brightness_tuples_to_lut(brightnesses_tuples),
        }
    }

    pub fn get_brightness_tuples(chars: &str, font: &Font, scale: &Scale) -> Vec<(char, u8)> {
        let mut brightnesses = vec![(' ', u8::MIN); chars.len()];
        for (i, char) in chars.char_indices() {
            unsafe {
                *brightnesses.get_unchecked_mut(i) = (
                    char,
                    Self::average_brightness(font.glyph(char).scaled(*scale)),
                );
            }
        }

        brightnesses
    }

    fn average_brightness(glyph: ScaledGlyph) -> u8 {
        let (glyph_width, glyph_height) = Self::glyph_dimensions(&glyph);
        let buffer_column = vec![u8::MIN; glyph_height as usize];
        let mut buffer = vec![buffer_column; glyph_width as usize];
        let total_pixels = glyph_width * glyph_height;
        glyph.positioned(point(0.0, 0.0)).draw(|x, y, v| {
            if v > BRIGHTNESS_CUTOFF {
                buffer[x as usize][y as usize] = 1u8;
            }
        });
        let brightness = buffer
            .into_iter()
            .map(|row| row.iter().sum::<u8>() as u16) //Only works if scale is <= then u8::MAX;
            .sum::<u16>();
        ((brightness as f32 * COLOR) / total_pixels) as u8
    }

    fn glyph_dimensions(glyph: &ScaledGlyph) -> (f32, f32) {
        let h_metrics = glyph.h_metrics();
        (
            h_metrics.advance_width + h_metrics.left_side_bearing,
            glyph.scale().y,
        )
    }

    fn brightness_tuples_to_lut(tuples: Vec<(char, u8)>) -> [char; LUT_LENGTH] {
        let mut lut = ['\x00'; LUT_LENGTH];
        let mut offset = [u8::MAX; LUT_LENGTH];

        for (char, brightness) in tuples {
            lut[brightness as usize] = char;
            offset[brightness as usize] = 0u8;

            let mut i = 0usize;
            while i < brightness as usize {
                let new_offset = brightness - i as u8;
                if offset[i] < new_offset {
                    break;
                }
                unsafe {
                    *offset.get_unchecked_mut(i) = new_offset;
                    *lut.get_unchecked_mut(i) = char;
                }

                i += 1;
            }

            let mut i = (brightness + 1) as usize;
            while i < LUT_LENGTH {
                let new_offset = i as u8 - brightness;
                if offset[i] < new_offset {
                    break;
                }
                unsafe {
                    *offset.get_unchecked_mut(i) = new_offset;
                    *lut.get_unchecked_mut(i) = char;
                }
                i += 1;
            }
        }
        lut
    }
}

impl Default for CharBrightnesses {
    fn default() -> Self {
        const LUT: [char; LUT_LENGTH] = [
            '`', '`', '`', '.', '.', '.', ',', ':', '"', '_', '_', '!', '!', '!', '|', '~', '/',
            '\\', 'r', '=', '=', '1', '1', '}', 'v', 'v', 'i', 'c', 't', 'x', 'z', 's', 's', 'o',
            'y', 'y', 'k', 'h', 'E', 'U', 'w', 'p', 'S', 'O', 'm', '8', 'g', '@', 'Q', 'Q', 'B',
            'N', 'N', 'M', 'M', 'M', 'M', 'M', 'M', 'M', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W', 'W',
            'W',
        ];
        Self { char_lut: LUT }
    }
}


#[cfg(test)]
mod char_brightnesses_tests {
    use super::*;

    #[test]
    fn test_get_brightness_tuples() {
        let chars = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
        let font = &Font::try_from_bytes(include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf")).unwrap();
        let scale = 255; 
        let new = CharBrightnesses::new(chars, font, scale);
        let default = CharBrightnesses::default();
        assert_eq!(new, default);
    }
}
