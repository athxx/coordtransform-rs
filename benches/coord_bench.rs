use criterion::{criterion_group, criterion_main, Criterion};
use coordtransform::*;
use std::hint::black_box;

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

fn bench_wgs84_to_epsg3857(c: &mut Criterion) {
    c.bench_function("wgs84_to_epsg3857", |b| {
        b.iter(|| wgs84_to_epsg3857(black_box(116.404), black_box(39.915)))
    });
}

fn bench_epsg3857_to_wgs84(c: &mut Criterion) {
    c.bench_function("epsg3857_to_wgs84", |b| {
        b.iter(|| epsg3857_to_wgs84(black_box(12958752.0), black_box(4825923.0)))
    });
}

fn bench_gcj02_to_epsg3857(c: &mut Criterion) {
    c.bench_function("gcj02_to_epsg3857", |b| {
        b.iter(|| gcj02_to_epsg3857(black_box(116.404), black_box(39.915)))
    });
}

fn bench_bd09_to_epsg3857(c: &mut Criterion) {
    c.bench_function("bd09_to_epsg3857", |b| {
        b.iter(|| bd09_to_epsg3857(black_box(116.404), black_box(39.915)))
    });
}

criterion_group!(
    benches,
    bench_bd09_to_gcj02,
    bench_gcj02_to_bd09,
    bench_wgs84_to_bd09,
    bench_gcj02_to_wgs84,
    bench_bd09_to_wgs84,
    bench_wgs84_to_gcj02,
    bench_wgs84_to_epsg3857,
    bench_epsg3857_to_wgs84,
    bench_gcj02_to_epsg3857,
    bench_bd09_to_epsg3857
);
criterion_main!(benches);