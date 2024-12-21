use std::fs;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

use day11::part1_and_2;


pub fn criterion_benchmark(c: &mut Criterion) {
    let input = fs::read_to_string("input.txt").unwrap();
    c.bench_function("Part 1", |b| b.iter(|| part1_and_2(black_box(&input), 25)));
    c.bench_function("Part 2", |b| b.iter(|| part1_and_2(black_box(&input), 75)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);