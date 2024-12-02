// benches/my_benchmarks.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day_02::part1::safe_reports as safe_reports_part1;
use day_02::part2::safe_reports as safe_reports_part2;

fn bench_part1(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_02_part1_safe_reports", |b| {
        b.iter(|| safe_reports_part1(black_box(document)))
    });
}

fn bench_part2(c: &mut Criterion) {
    let document = include_str!("../input/input1.txt");
    c.bench_function("day_02_part2_safe_reports", |b| {
        b.iter(|| safe_reports_part2(black_box(document)))
    });
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
