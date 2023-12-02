// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_02::part1::sum_of_possible_game_ids as sum_of_possible_game_ids_part1;
use day_02::part2::sum_of_product_of_min_cubes as sum_of_product_of_min_cubes_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_02_part1_sum_of_possible_game_ids", |b| {
        b.iter(|| sum_of_possible_game_ids_part1(black_box(document), 12, 13, 14))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_02_part2_sum_of_product_of_min_cubes", |b| {
        b.iter(|| sum_of_product_of_min_cubes_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
