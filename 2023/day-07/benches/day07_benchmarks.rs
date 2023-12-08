// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_07::part1::total_winnings as total_winnings_part1;
use day_07::part2::total_winnings as total_winnings_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_07_part1_total_winnings", |b| {
        b.iter(|| total_winnings_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_07_part2_total_winnings", |b| {
        b.iter(|| total_winnings_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
