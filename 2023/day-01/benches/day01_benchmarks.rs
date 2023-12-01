// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_01::part1::sum_calibration_values as sum_calibration_values_part1;
use day_01::part2::sum_calibration_values as sum_calibration_values_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt"); // Adjust the path to your input file
    c.bench_function("part1_sum_calibration_values", |b| {
        b.iter(|| sum_calibration_values_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt"); // Adjust the path to your input file
    c.bench_function("part2_sum_calibration_values", |b| {
        b.iter(|| sum_calibration_values_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
