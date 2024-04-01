use std::{error::Error, fmt::Display};

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub enum CharPixelDensityError {
    NoGlyphForChar,
    TooLargeGlyph,
    MissingBoundingBox,
}

impl Display for CharPixelDensityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CharPixelDensityError::NoGlyphForChar => write!(f, "No glyph for char"),
            CharPixelDensityError::TooLargeGlyph =>  write!(f, "Glyph is too large for drawing within the pixel boundary"),
            CharPixelDensityError::MissingBoundingBox => write!(f, "No bounding box for glyph"),
        }
    }
}

impl Error for CharPixelDensityError { 

}