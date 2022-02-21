use criterion::{black_box, criterion_group, criterion_main, Criterion};

use iqotw::y22::feb13::remote_control;

fn feb13_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("keyboard_instructions");

    for input in ["car", "place", "hellothere", "onemorebecauseitsfun"] {
        group.bench_function(input, |b| b.iter(|| remote_control(black_box(input))));
    }

    group.finish();
}

criterion_group!(benches, feb13_benchmark);
criterion_main!(benches);

// TODO(Dan) - check benchmark result and also what else can be done. (optimize the thing.)
