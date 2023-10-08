pub mod window;
pub mod image;
use image::{
    Image,
    Pixel,
    empty_image,
    from_u8_rgb
};
use window::config::{
    WINDOW_X,
    WINDOW_Y
};
use minifb::{
    Key,
    Window
};
fn main() {
    //example initializing window and drawing an image
    let grey: Pixel = from_u8_rgb(50, 50, 50);
    let white: Pixel = from_u8_rgb(150, 150, 150);
    let mut image: Image;
    let mut window: Window = window::init().unwrap();
    loop {
        //get input 
        if window.is_key_down(Key::Escape) {
            return; 
        }
        //get mouse pos
        let mp = window::get_mouse_pos(&window);
        //draw background
        image = empty_image(Some(grey), WINDOW_X, WINDOW_Y);
        //draw box on mouse pos
        image.draw_image(&empty_image(Some(white), 200, 200), mp);
        //update window
        window.update_with_buffer(image.data().as_slice(), WINDOW_X, WINDOW_Y).unwrap();
        
    }