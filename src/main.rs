use char_map::char_brightness_map::CharBrightnesses;
use rusttype::Font;


pub mod char_map;

fn main() {
    let chars = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    let font = &Font::try_from_bytes(include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf")).unwrap();
    let scale = 255; 
    dbg!(CharBrightnesses::new(chars, font, scale));
}
