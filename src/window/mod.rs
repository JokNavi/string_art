use minifb::{
    MouseMode,
    Window
};
/// Initializes and returns a minifb window instance. Configuration for the window is in window/config.rs
pub fn init() -> Result<Window, minifb::Error> {
    Window::new(WINDOW_NAME, WINDOW_X, WINDOW_Y, WINDOW_OPTIONS)
}

/// Returns mouse position on window
/// if the mouse position is not found returns 0,0
pub fn get_mouse_pos(window: &Window) -> (i32, i32) {
    match window.get_mouse_pos(MouseMode::Clamp) {
        Some(pos) => {
            (pos.0 as i32, pos.1 as i32)
        }, None => {
            (0, 0)
        }
    }
}


use minifb::{
    Scale,
    ScaleMode,
    WindowOptions
};

//Configuration for window
pub const WINDOW_X: usize = 1920; 
pub const WINDOW_Y: usize = 1080;
pub const WINDOW_SIZE: usize = WINDOW_X * WINDOW_Y;
pub const WINDOW_NAME: &str = "Ascii Art";

/// Documentation for WindowOptions: https://docs.rs/minifb/latest/minifb/struct.WindowOptions.html
pub const WINDOW_OPTIONS: WindowOptions = WindowOptions {
    borderless: false,
    title: true,
    resize: false,
    scale: Scale::X1,  //Scales images when rendered to the window: https://docs.rs/minifb/latest/minifb/enum.Scale.html
    scale_mode: ScaleMode::Center, //Determines how the window scales images: https://docs.rs/minifb/latest/minifb/enum.ScaleMode.html
    topmost: true,
    transparency: false,
    none: false
};
