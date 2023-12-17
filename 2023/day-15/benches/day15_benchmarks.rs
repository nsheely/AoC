// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_15::part1::compute_hash as compute_hash_part1;
use day_15::part2::compute_focusing_power as compute_focusing_power_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_15_part1_compute_hash", |b| {
        b.iter(|| compute_hash_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_15_part2_compute_focusing_power", |b| {
        b.iter(|| compute_focusing_power_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
