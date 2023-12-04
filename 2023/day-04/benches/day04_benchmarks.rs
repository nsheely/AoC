// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_04::part1::scratchcard_winnings_sum as scratchcard_winnings_sum_part1;
use day_04::part2::total_scratchcards as total_scratchcards_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_04_part1_scratchcard_winnings_sum", |b| {
        b.iter(|| scratchcard_winnings_sum_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_04_part2_total_scratchcards", |b| {
        b.iter(|| total_scratchcards_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
