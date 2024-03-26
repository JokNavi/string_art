use super::color::Color;
pub struct Image {
    data: Vec<u32>,
    height: usize,
    width: usize,
    size: usize,
}
impl Image {
    pub fn new(color: &Color, width: usize, height: usize) -> Image {
        let size = width * height;
        Image {
            width: width,
            height: height,
            size: size,
            data: vec![color.to_u32(); size],
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn size(&self) -> usize {
        self.size
    }
    pub fn data(&self) -> &[u32] {
        &self.data
    }
    pub fn data_as_mut(&mut self) -> &mut [u32] {
        &mut self.data
    }
    /// Draws a pixel at a specified position.
    /// position: tuple representing x y position on the Image
    /// Images are drawn starting at the top left most position (0, 0) to bottom right most position (width, height)
    pub fn draw_pixel(&mut self, color: &Color, position: (usize, usize)) {
        let index = position.1 * self.width + position.0;
        self.data[index] = color.to_u32();
    }
    
    pub unsafe fn draw_pixel_unchecked(&mut self, color: &Color, position: (usize, usize)) {
        let index = position.1 * self.width + position.0;
        unsafe {
            *self.data.get_unchecked_mut(index) = color.to_u32();
        }
    }

    /// Draws an Image on this Image.
    /// other: the image to draw
    /// position: tuple representing the xy position on this Image to start drawing other
    pub fn splice_image(&mut self, other: &Image, position: (usize, usize)) {
        let initial_x = position.0;
        let mut y = position.1;
        let mut x = initial_x;
        let mut current_other_x = 0;
        for pixel in other.data() {
            if current_other_x == other.width() {
                y += 1;
                x = initial_x;
                current_other_x = 0;
            }
            current_other_x += 1;
            x += 1;
            //Do not draw objects outside of self bounds
            if x >= self.width || y >= self.height {
                continue;
            }
            //Calculate and check position in self.data
            let self_pos = x + (y * self.width);
            if self_pos >= self.size {
                break;
            }
            self.data[self_pos] = *pixel;
        }
    }
}
