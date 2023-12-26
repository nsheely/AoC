// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_18::part1::calculate_lava_capacity as calculate_lava_capacity_part1;
use day_18::part2::calculate_lava_capacity as calculate_lava_capacity_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_18_part1_calculate_lava_capacity", |b| {
        b.iter(|| calculate_lava_capacity_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_18_part2_calculate_lava_capacity", |b| {
        b.iter(|| calculate_lava_capacity_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
