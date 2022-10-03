use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

use iqotw::y22::feb13::*;
use iqotw::y22::feb20::*;
use iqotw::y22::fib_like::*;
use iqotw::y22::mar7::*;

use std::collections::HashMap;

fn feb13_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("keyboard_instructions");

    for input in ["car", "place", "hellothere", "onemorebecauseitsfun"] {
        group.bench_function(input, |b| b.iter(|| remote_control(black_box(input))));
    }

    group.finish();
}

fn feb20_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("feb_20");

    let thousand = include_str!("random-1000.txt")
        .split(' ')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

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
        thousand,
    ] {
        group.bench_with_input(
            BenchmarkId::new("longest_sub_seq", format!("len-{}", input.len())),
            &input,
            |b, input| b.iter(|| longest_sub_seq(black_box(input))),
        );
        group.bench_with_input(
            BenchmarkId::new("longest_sub_seq_map", format!("len-{}", input.len())),
            &input,
            |b, input| b.iter(|| longest_sub_seq_map(black_box(input))),
        );
    }

    group.finish();
}

fn mar7_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("mar_7");

    let thousand: Vec<i32> = include_str!("random-1000.txt")
        .split(' ')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    group.bench_with_input("cmap-insert", &thousand, |b, input| {
        let mut cmap: CMap<usize, &i32> = CMap::new();

        b.iter(|| {
            input
                .iter()
                .enumerate()
                .for_each(|(idx, num)| cmap.insert(idx, num))
        })
    });

    group.bench_with_input("hashmap-insert", &thousand, |b, input| {
        let mut hash_map: HashMap<usize, &i32> = HashMap::new();
        b.iter(|| {
            input.iter().enumerate().for_each(|(idx, num)| {
                hash_map.insert(idx, num);
            })
        })
    });

    group.bench_with_input("cmap-get", &thousand, |b, input| {
        let mut cmap: CMap<usize, &i32> = CMap::new();
        input
            .iter()
            .enumerate()
            .for_each(|(idx, num)| cmap.insert(idx, num));

        b.iter(|| {
            input.iter().enumerate().for_each(|(idx, _)| {
                cmap.get(idx);
            })
        })
    });

    group.bench_with_input("hashmap-get", &thousand, |b, input| {
        let mut hash_map: HashMap<usize, &i32> = HashMap::new();
        input.iter().enumerate().for_each(|(idx, num)| {
            hash_map.insert(idx, num);
        });
        b.iter(|| {
            input.iter().enumerate().for_each(|(idx, _)| {
                hash_map.get(&idx);
            })
        })
    });

    group.finish();
}

fn fib_iterator_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("fib_iterator");

    for (n1, n2, len) in [(1, 1, 100), (140, 255, 150), (1, 1, 2000)] {
        group.bench_function(len.to_string(), |b| {
            b.iter(|| {
                let fibs = FibIterator::new(n1, n2, len);
                for n in fibs {
                    let _x = n;
                }
            })
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    feb13_benchmark,
    feb20_benchmark,
    mar7_benchmark,
    fib_iterator_benchmark
);
criterion_main!(benches);
