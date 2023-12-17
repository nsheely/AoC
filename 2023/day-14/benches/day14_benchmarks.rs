// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_14::part1::tilt_and_sum_load as tilt_and_sum_load_part1;
use day_14::part2::tilt_and_sum_load as tilt_and_sum_load_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_14_part1_tilt_and_sum_load", |b| {
        b.iter(|| tilt_and_sum_load_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_14_part2_tilt_and_sum_load", |b| {
        b.iter(|| tilt_and_sum_load_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
