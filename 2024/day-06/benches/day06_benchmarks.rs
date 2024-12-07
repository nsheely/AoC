// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_06::part1::guard_positions as guard_positions_part1;
use day_06::part2::count_guard_loop_additions as count_guard_loop_additions_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_06_part1_guard_positions", |b| {
        b.iter(|| guard_positions_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_06_part2_count_guard_loop_additions", |b| {
        b.iter(|| count_guard_loop_additions_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
