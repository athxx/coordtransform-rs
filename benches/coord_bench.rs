use criterion::{black_box, criterion_group, criterion_main, Criterion};
use coordtransform::*;

fn bench_bd09_to_gcj02(c: &mut Criterion) {
    c.bench_function("bd09_to_gcj02", |b| {
        b.iter(|| bd09_to_gcj02(black_box(116.404), black_box(39.915)))
    });
}

fn bench_gcj02_to_bd09(c: &mut Criterion) {
    c.bench_function("gcj02_to_bd09", |b| {
        b.iter(|| gcj02_to_bd09(black_box(116.404), black_box(39.915)))
    });
}

fn bench_wgs84_to_bd09(c: &mut Criterion) {
    c.bench_function("wgs84_to_bd09", |b| {
        b.iter(|| wgs84_to_bd09(black_box(116.404), black_box(39.915)))
    });
}

fn bench_gcj02_to_wgs84(c: &mut Criterion) {
    c.bench_function("gcj02_to_wgs84", |b| {
        b.iter(|| gcj02_to_wgs84(black_box(116.404), black_box(39.915)))
    });
}

fn bench_bd09_to_wgs84(c: &mut Criterion) {
    c.bench_function("bd09_to_wgs84", |b| {
        b.iter(|| bd09_to_wgs84(black_box(116.404), black_box(39.915)))
    });
}

fn bench_wgs84_to_gcj02(c: &mut Criterion) {
    c.bench_function("wgs84_to_gcj02", |b| {
        b.iter(|| wgs84_to_gcj02(black_box(116.404), black_box(39.915)))
    });
}

criterion_group!(
    benches,
    bench_bd09_to_gcj02,
    bench_gcj02_to_bd09,
    bench_wgs84_to_bd09,
    bench_gcj02_to_wgs84,
    bench_bd09_to_wgs84,
    bench_wgs84_to_gcj02
);
criterion_main!(benches);