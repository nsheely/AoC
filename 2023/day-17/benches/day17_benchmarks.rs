// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_17::part1::least_heat_loss as least_heat_loss_part1;
use day_17::part2::least_heat_loss as least_heat_loss_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_17_part1_least_heat_loss", |b| {
        b.iter(|| least_heat_loss_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_17_part2_least_heat_loss", |b| {
        b.iter(|| least_heat_loss_part2(black_box(document)))
    });
}


criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
