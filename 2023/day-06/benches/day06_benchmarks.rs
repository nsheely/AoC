// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_06::part1::product_of_number_of_ways_to_win as product_of_number_of_ways_to_win_part1;
use day_06::part2::number_of_ways_to_win as number_of_ways_to_win_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_06_part1_product_of_number_of_ways_to_win", |b| {
        b.iter(|| product_of_number_of_ways_to_win_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_06_part2_mumber_of_ways_to_win", |b| {
        b.iter(|| number_of_ways_to_win_part2(black_box(document)))
    });
}


criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
