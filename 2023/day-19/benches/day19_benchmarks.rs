// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_19::part1::calculate_accepted_parts_sum as calculate_accepted_parts_sum_part1;
use day_19::part2::calculate_accepted_combinations as calculate_accepted_combinations_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_19_part1_calculate_accepted_parts_sum", |b| {
        b.iter(|| calculate_accepted_parts_sum_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_19_part2_calculate_accepted_combinations", |b| {
        b.iter(|| calculate_accepted_combinations_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
