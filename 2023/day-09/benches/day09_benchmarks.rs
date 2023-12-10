// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_09::part1::extrapolate_and_sum as extrapolate_and_sum_part1;
use day_09::part2::extrapolate_and_sum as extrapolate_and_sum_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_09_part1_extrapolate_and_sum", |b| {
        b.iter(|| extrapolate_and_sum_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_09_part2_extrapolate_and_sum", |b| {
        b.iter(|| extrapolate_and_sum_part2(black_box(document)))
    });
}


criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
