use divan::{black_box, Bencher};
use image::{io::Reader, DynamicImage, GrayImage};
use string_art::text_art::{TextArtImageEncoder, TextArtStringEncoder};

#[divan::bench]
fn text_art_text_encode(bencher: Bencher) {
    let image = GrayImage::new(1000, 1000);
    let image = DynamicImage::ImageLuma8(image);
    let text_art_encoder = TextArtStringEncoder::default();
    bencher.bench_local(move || {
        black_box(text_art_encoder.encode_alternating(image.clone()));
    });
}

#[divan::bench]
fn text_art_image_encode(bencher: Bencher) {
    let image = Reader::open("files/input/test-pattern-small.webp")
            .unwrap()
            .decode()
            .unwrap();
        let image_encoder = TextArtImageEncoder::default();
    bencher.bench_local(move || {
        black_box(image_encoder.encode(image.clone()));
    });
}


