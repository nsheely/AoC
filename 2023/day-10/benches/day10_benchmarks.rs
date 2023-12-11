// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_10::part1::find_longest_distance as find_longest_distance_part1;
use day_10::part2::num_enclosed_tiles as num_enclosed_tiles_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_10_part1_find_longest_distance", |b| {
        b.iter(|| find_longest_distance_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_10_part2_num_enclosed_tiles", |b| {
        b.iter(|| num_enclosed_tiles_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
