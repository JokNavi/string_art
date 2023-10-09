#![feature(test)]
extern crate test;

#[cfg(test)]
mod benchmarks {
    use rusttype::{Font, Scale};
    use string_art::string_art::char_brightness_map::CharBrightnesses;
    use test::{black_box, Bencher};

    const CHARS: &str = "!\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~";
    const FONT_BYTES: &[u8; 86908] =
        include_bytes!("/home/joknavi/.local/share/fonts/RobotoMono-Regular.ttf");
    const SCALE: u8 = 50;

    #[bench]
    fn bench_new(b: &mut Bencher) {
        let font = &Font::try_from_bytes(FONT_BYTES).unwrap();
        b.iter(|| {
            for _ in 1..100 {
                black_box(CharBrightnesses::new(black_box(CHARS), font, SCALE));
            }
        });
    }

    #[bench]
    fn bench_default(b: &mut Bencher) {
        let font = &Font::try_from_bytes(FONT_BYTES).unwrap();
        b.iter(|| {
            for _ in 1..100 {
                black_box(CharBrightnesses::default());
            }
        });
    }
}
