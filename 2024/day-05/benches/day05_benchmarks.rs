// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_05::part1::sum_middle_pages_of_correct_updates as sum_middle_pages_of_correct_updates_part1;
use day_05::part2::sum_middle_pages_of_corrected_updates as sum_middle_pages_of_corrected_updates_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_05_part1_sum_middle_pages_of_correct_updates", |b| {
        b.iter(|| sum_middle_pages_of_correct_updates_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_05_part2_sum_middle_pages_of_corrected_updates", |b| {
        b.iter(|| sum_middle_pages_of_corrected_updates_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
