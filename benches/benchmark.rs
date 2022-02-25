use criterion::{black_box, criterion_group, criterion_main, Criterion};

use iqotw::y22::feb13::*;
use iqotw::y22::feb20::*;

fn feb13_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("keyboard_instructions");

    for input in ["car", "place", "hellothere", "onemorebecauseitsfun"] {
        group.bench_function(input, |b| b.iter(|| remote_control(black_box(input))));
    }

    group.finish();
}

fn feb20_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("feb_20/longest_subseq");

    for input in vec![
        vec![1, 9, 87, 3, 10, 4, 20, 2, 45],
        vec![36, 41, 56, 35, 91, 33, 34, 92, 43, 37, 42],
        vec![
            117, 148, 107, 70, 177, 155, 49, 191, 164, 190, 141, 128, 7, 192, 173, 197, 41, 131,
            143, 182, 95, 162, 66, 62, 59, 97, 157, 77, 68, 123, 147, 60, 34, 109, 84, 113, 44, 57,
            28, 187, 5, 165, 171, 100, 22, 43, 181, 176, 11, 151, 36, 200, 12, 54, 172, 69, 194,
            91, 61, 6, 46, 81, 118, 183, 79, 144, 52, 87, 88, 58, 111, 17, 189, 185, 76, 40, 50,
            23, 37, 130, 72, 78, 150, 71, 2, 96, 20, 80, 112, 124, 18, 89, 127, 110, 163, 142, 39,
            169, 140, 153,
        ],
    ] {
        group.bench_function(format!("length-{}", input.len()), |b| {
            b.iter(|| longest_sub_seq(black_box(&input)))
        });
    }

    group.finish();

    let mut group = c.benchmark_group("feb_20/longest_subseq_map");

    for input in vec![
        vec![1, 9, 87, 3, 10, 4, 20, 2, 45],
        vec![36, 41, 56, 35, 91, 33, 34, 92, 43, 37, 42],
        vec![
            117, 148, 107, 70, 177, 155, 49, 191, 164, 190, 141, 128, 7, 192, 173, 197, 41, 131,
            143, 182, 95, 162, 66, 62, 59, 97, 157, 77, 68, 123, 147, 60, 34, 109, 84, 113, 44, 57,
            28, 187, 5, 165, 171, 100, 22, 43, 181, 176, 11, 151, 36, 200, 12, 54, 172, 69, 194,
            91, 61, 6, 46, 81, 118, 183, 79, 144, 52, 87, 88, 58, 111, 17, 189, 185, 76, 40, 50,
            23, 37, 130, 72, 78, 150, 71, 2, 96, 20, 80, 112, 124, 18, 89, 127, 110, 163, 142, 39,
            169, 140, 153,
        ],
    ] {
        group.bench_function(format!("length-{}", input.len()), |b| {
            b.iter(|| longest_sub_seq_map(black_box(&input)))
        });
    }

    group.finish();
}

criterion_group!(benches, feb13_benchmark, feb20_benchmark);
criterion_main!(benches);

// TODO(Dan) - check benchmark result and also what else can be done. (optimize the thing.)
