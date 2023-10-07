/// Pixles are represented by u32 RGB values:
/// Bits 32 through 24 are unused
/// Bits 24 through 16 represent Red value
/// Bits 16 through 8 represent Green value
/// Bits 8 through 0 represent Blue value
pub type Pixel = u32;
pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> Pixel {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b    //shift rgb values to correct places in u32, perform bitwise or to concatonate them together
}

pub struct Image {
    data: Vec<Pixel>,
    height: usize,
    width: usize,
    size: usize
}
impl Image {
    pub fn height(&self) -> usize {self.height}
    pub fn width(&self) -> usize {self.width}
    pub fn size(&self) -> usize {self.size}
    pub fn data(&self) -> &Vec<Pixel> {&self.data}
    pub fn data_as_mut(&mut self) -> &mut Vec<Pixel> {&mut self.data}
    /// Draws a pixel at a specified position.
    /// color: see Pixel::from_u8_rgb
    /// position: tuple representing x y position on the Image
    /// Images are drawn starting at the top left most position (0, 0) to bottom right most position (width, height)
    pub fn draw_pixel(&mut self, color: Pixel, position: (i32, i32)) {
        let index: usize = (position.0 + (position.1 * self.width as i32)) as usize;
        if index < self.size {
            self.data[index] = color;
        }
    }
    /// Draws an Image on this Image.
    /// other: the image to draw
    /// position: tuple representing the xy position on this Image to start drawing other
    pub fn draw_image(&mut self, other: &Image, position: (i32, i32)) {
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
            if x.is_negative() || 
               y.is_negative() ||
               x as usize >= self.width || 
               y as usize >= self.height { 
                continue; 
            }
            //Calculate and check position in self.data
            let self_pos = x as usize + (y as usize * self.width);
            if self_pos >= self.size { 
                break; 
            }
            self.data[self_pos] = *pixel;
        }
    }
}

/// Returns an empty image
/// color: Optional color, if None image will be black
/// width: width of image
/// height: height of image
/// returns an image object
pub fn empty_image(color: Option<Pixel>, width: usize, height: usize) -> Image {
    let image_data: Vec<Pixel>;
    let size = width * height;
    match color {
        Some(pixel) => {
            image_data = vec!(pixel; size);
        }, None => {
            image_data = vec!(from_u8_rgb(0, 0, 0); size);
        }
    }

    Image {
        width: width,
        height: height,
        size: size,
        data: image_data
    }
}