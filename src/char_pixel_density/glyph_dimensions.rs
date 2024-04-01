use rusttype::Rect;


#[derive(Debug, PartialEq, Clone, Copy, Eq)]
pub struct GlyphDimensions {
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