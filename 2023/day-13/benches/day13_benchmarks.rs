// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_13::part1::sum_of_pattern_notes as sum_of_pattern_notes_part1;
use day_13::part2::sum_of_pattern_notes as sum_of_pattern_notes_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_13_part1_sum_of_pattern_notes", |b| {
        b.iter(|| sum_of_pattern_notes_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_13_part2_sum_of_pattern_notes", |b| {
        b.iter(|| sum_of_pattern_notes_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
