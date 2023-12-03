// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_03::part1::sum_of_part_numbers as sum_of_part_numbers_part1;
use day_03::part2::sum_of_gear_ratios as sum_of_gear_ratios_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_03_part1_sum_of_part_numbers", |b| {
        b.iter(|| sum_of_part_numbers_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_03_part2_sum_of_gear_ratios", |b| {
        b.iter(|| sum_of_gear_ratios_part2(black_box(document)))
    });
}


criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
