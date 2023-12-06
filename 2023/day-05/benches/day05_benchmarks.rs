// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_05::part1::find_lowest_location as find_lowest_location_part1;
use day_05::part2::find_lowest_location as find_lowest_location_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_05_part1_find_lowest_locationm", |b| {
        b.iter(|| find_lowest_location_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_05_part2_find_lowest_location", |b| {
        b.iter(|| find_lowest_location_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
