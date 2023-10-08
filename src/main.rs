pub mod string_art;
pub mod window;

use window::{
    image::Image,
    WINDOW_NAME, WINDOW_OPTIONS, WINDOW_X, WINDOW_Y, pixel::Color,
};

use minifb::{Key, KeyRepeat, MouseMode, Window};

/// Initializes and returns a minifb window instance. Configuration for the window is in window/config.rs
pub fn init() -> Result<Window, minifb::Error> {
    Window::new(WINDOW_NAME, WINDOW_X, WINDOW_Y, WINDOW_OPTIONS)
}

fn handle_key_events(window: &Window) {
    window
        .get_keys_pressed(KeyRepeat::No)
        .iter()
        .for_each(|key| match key {
            Key::W => println!("pressed w"),
            Key::T => println!("pressed t"),
            Key::Escape => return,
            _ => (),
        });
}

fn draw_cursor_box(image: &mut Image, coords: (usize, usize), color: &Color) {
    image.splice_image(&Image::new(&color, 200, 200), coords);
}

fn main() -> minifb::Result<()> {
    let grey = Color::Grayscale(50);
    let white = Color::Grayscale(150);

    let mut window: Window = init()?;
    let mut image;
    
    loop {
        handle_key_events(&window);
        image = Image::new(&grey, WINDOW_X, WINDOW_Y);

        let coords = match window.get_mouse_pos(MouseMode::Clamp) {
            Some(pos) => (pos.0 as usize, pos.1 as usize),
            None => (0, 0),
        };
        draw_cursor_box(&mut image, coords, &white);

        window.update_with_buffer(image.data(), WINDOW_X, WINDOW_Y)?;
    }
}
