use divan::{black_box, Bencher};

pub mod char_brightnesses;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn copy_from_slice(bencher: Bencher) {
    let src = (0..100).collect::<Vec<i32>>();
    let mut dst = vec![0; src.len()];

    bencher.bench_local(move || {
        black_box(&mut dst).copy_from_slice(black_box(&src));
    });
}

