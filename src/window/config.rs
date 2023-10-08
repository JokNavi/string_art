extern crate minifb;

use minifb::{
    Scale,
    ScaleMode,
    WindowOptions
};

//Configuration for window
pub const WINDOW_X: usize = 1920; 
pub const WINDOW_Y: usize = 1080;
pub const WINDOW_SIZE: usize = WINDOW_X * WINDOW_Y;
pub const WINDOW_NAME: &str = "String Art";
/// Documentation for WindowOptions: https://docs.rs/minifb/latest/minifb/struct.WindowOptions.html
pub const WINDOW_OPTIONS: WindowOptions = WindowOptions {
    borderless: false,
    title: true,
    resize: false,
    scale: Scale:: X1,  //Scales images when rendered to the window: https://docs.rs/minifb/latest/minifb/enum.Scale.html
    scale_mode: ScaleMode::Center, //Determines how the window scales images: https://docs.rs/minifb/latest/minifb/enum.ScaleMode.html
    topmost: true,
    transparency: false,
    none: false
};