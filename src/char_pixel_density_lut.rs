use rusttype::{Font, Scale};
use std::ops::Index;

pub const LUT_LENGTH: usize = u8::MAX as usize + 1;

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
