use divan::{black_box, Bencher};
use image::{DynamicImage, GrayImage};
use string_art::text_art::TextArtEncoder;

#[divan::bench]
fn text_art_encode(bencher: Bencher) {
    let image = GrayImage::new(1000, 1000);
    let image = DynamicImage::ImageLuma8(image);
    let text_art_encoder = TextArtEncoder::default();
    bencher.bench_local(move || {
        black_box(text_art_encoder.encode_alternating(&image));
    });
}
