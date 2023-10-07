pub mod config;
use config::*;
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
