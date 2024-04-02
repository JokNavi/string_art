use rusttype::{Font, Scale};


#[derive(Debug, Clone)]
pub struct TextArtBuilder<'a> {
    font: Option<Font<'a>>,
    scale: Option<Scale>,
}

impl<'a> TextArtBuilder<'a> {

    pub fn new() -> Self {
        Self {
            font: None,
            scale: None,
        }
    }
}