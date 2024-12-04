// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_04::part1::count_xmas_occurrences as count_xmas_occurrences_part1;
use day_04::part2::count_xmas_occurrences as count_xmas_occurrences_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_04_part1_count_xmas_occurrences", |b| {
        b.iter(|| count_xmas_occurrences_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_04_part2_count_xmas_occurrences", |b| {
        b.iter(|| count_xmas_occurrences_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
