pub mod config;
use config::*;
use minifb::Window;
/// Initializes and returns a minifb window instance. Configuration for the window is in window/config.rs
pub fn init() -> Result<Window, minifb::Error> {
    Window::new(WINDOW_NAME, WINDOW_X, WINDOW_Y, WINDOW_OPTIONS)
}