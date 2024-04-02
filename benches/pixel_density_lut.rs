use divan::{black_box, Bencher};
use rusttype::{Font, Scale};
use string_art::pixel_density_lut::PixelDensityLut;

#[divan::bench]
fn pixel_density_lut(bencher: Bencher) {
    const CHARS: &str = " !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    const FONT_BYTES: &[u8] = include_bytes!("../files/RobotoMono-Regular.ttf");
    const SCALE: u8 = 50;
    let font = Font::try_from_bytes(FONT_BYTES).unwrap();
    let scale = Scale::uniform(SCALE as f32);

    bencher.bench_local(move || {
        black_box(PixelDensityLut::new(CHARS, &font, scale));
    });
}
