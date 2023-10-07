use rusttype::{Font, Scale, ScaledGlyph, point};

pub const LUT_LENGTH: usize = u8::MAX as usize + 1;
pub const COLOR: f32 = 255.0;

#[derive(Debug)]
pub struct CharBrightnesses {
    char_lut: [char; LUT_LENGTH],
}

impl CharBrightnesses {
    pub fn new(chars: &str, font: &Font, scale: &Scale) -> Self {
        // if chars.contains("\n") {
        //     panic!("chars cannot contain any newlines.");
        // }
        let brightnesses_tuples = Self::get_brightness_tuples(chars, &font, &scale);
        // CharBrightnesses {
        //     char_lut: Self::brightness_tuples_to_lut(brightnesses_tuples),
        // }
        todo!();
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
            if v > 0.5 {
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
}

impl Default for CharBrightnesses {
    fn default() -> Self {
        //const LUT: [char; LUT_LENGTH] = [];
        //Self { char_lut: LUT }
        todo!();
    }
}
