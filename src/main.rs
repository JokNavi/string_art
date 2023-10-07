pub mod window;
pub mod image;

use window::config::{
    WINDOW_X,
    WINDOW_Y
};
fn main() {
    //example initializing window and drawing an image
    let grey = image::from_u8_rgb(50, 50, 50);
    let image = image::empty_image(Some(grey), WINDOW_X, WINDOW_Y);
    let mut window = window::init().unwrap();
    loop {
        window.update_with_buffer(image.data().as_slice(), WINDOW_X, WINDOW_Y).unwrap();
    }
}
