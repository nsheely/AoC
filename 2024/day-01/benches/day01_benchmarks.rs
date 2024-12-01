// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_01::part1::distance_between_lists as distance_between_lists_part1;
use day_01::part2::similarity_score as lists_similarity_score_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_01_part1_distance_between_lists", |b| {
        b.iter(|| distance_between_lists_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_01_part2_lists_similarity_score", |b| {
        b.iter(|| lists_similarity_score_part2(black_box(document)))
    });
}


criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
