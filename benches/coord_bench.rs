use coordtransform::*;
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const LON: f64 = 116.404;
const LAT: f64 = 39.915;

const MERCATOR_X: f64 = 12_958_752.0;
const MERCATOR_Y: f64 = 4_825_923.0;

//
// BD09 <-> GCJ02
//

fn bench_bd09_to_gcj02(c: &mut Criterion) {
    c.bench_function("bd09_to_gcj02", |b| {
        b.iter(|| black_box(bd09_to_gcj02(black_box(LON), black_box(LAT))))
    });
}

fn bench_bd09_to_gcj02_fast(c: &mut Criterion) {
    c.bench_function("bd09_to_gcj02_fast", |b| {
        b.iter(|| black_box(bd09_to_gcj02_fast(black_box(LON), black_box(LAT))))
    });
}

fn bench_gcj02_to_bd09(c: &mut Criterion) {
    c.bench_function("gcj02_to_bd09", |b| {
        b.iter(|| black_box(gcj02_to_bd09(black_box(LON), black_box(LAT))))
    });
}

//
// WGS84 <-> GCJ02
//

fn bench_wgs84_to_gcj02(c: &mut Criterion) {
    c.bench_function("wgs84_to_gcj02", |b| {
        b.iter(|| black_box(wgs84_to_gcj02(black_box(LON), black_box(LAT))))
    });
}

fn bench_wgs84_to_gcj02_in_mainland(c: &mut Criterion) {
    c.bench_function("wgs84_to_gcj02_in_mainland", |b| {
        b.iter(|| black_box(wgs84_to_gcj02_in_mainland(black_box(LON), black_box(LAT))))
    });
}

fn bench_gcj02_to_wgs84(c: &mut Criterion) {
    c.bench_function("gcj02_to_wgs84", |b| {
        b.iter(|| black_box(gcj02_to_wgs84(black_box(LON), black_box(LAT))))
    });
}

fn bench_gcj02_to_wgs84_fast(c: &mut Criterion) {
    c.bench_function("gcj02_to_wgs84_fast", |b| {
        b.iter(|| black_box(gcj02_to_wgs84_fast(black_box(LON), black_box(LAT))))
    });
}

fn bench_gcj02_to_wgs84_in_mainland(c: &mut Criterion) {
    c.bench_function("gcj02_to_wgs84_in_mainland", |b| {
        b.iter(|| black_box(gcj02_to_wgs84_in_mainland(black_box(LON), black_box(LAT))))
    });
}

//
// WGS84 <-> BD09
//

fn bench_wgs84_to_bd09(c: &mut Criterion) {
    c.bench_function("wgs84_to_bd09", |b| {
        b.iter(|| black_box(wgs84_to_bd09(black_box(LON), black_box(LAT))))
    });
}

fn bench_bd09_to_wgs84(c: &mut Criterion) {
    c.bench_function("bd09_to_wgs84", |b| {
        b.iter(|| black_box(bd09_to_wgs84(black_box(LON), black_box(LAT))))
    });
}

//
// WGS84 <-> EPSG:3857
//

fn bench_wgs84_to_epsg3857(c: &mut Criterion) {
    c.bench_function("wgs84_to_epsg3857", |b| {
        b.iter(|| black_box(wgs84_to_epsg3857(black_box(LON), black_box(LAT))))
    });
}

fn bench_epsg3857_to_wgs84(c: &mut Criterion) {
    c.bench_function("epsg3857_to_wgs84", |b| {
        b.iter(|| {
            black_box(epsg3857_to_wgs84(
                black_box(MERCATOR_X),
                black_box(MERCATOR_Y),
            ))
        })
    });
}

//
// GCJ02 <-> EPSG:3857
//

fn bench_gcj02_to_epsg3857(c: &mut Criterion) {
    c.bench_function("gcj02_to_epsg3857", |b| {
        b.iter(|| black_box(gcj02_to_epsg3857(black_box(LON), black_box(LAT))))
    });
}

fn bench_epsg3857_to_gcj02(c: &mut Criterion) {
    c.bench_function("epsg3857_to_gcj02", |b| {
        b.iter(|| {
            black_box(epsg3857_to_gcj02(
                black_box(MERCATOR_X),
                black_box(MERCATOR_Y),
            ))
        })
    });
}

//
// BD09 <-> EPSG:3857
//

fn bench_bd09_to_epsg3857(c: &mut Criterion) {
    c.bench_function("bd09_to_epsg3857", |b| {
        b.iter(|| black_box(bd09_to_epsg3857(black_box(LON), black_box(LAT))))
    });
}

fn bench_epsg3857_to_bd09(c: &mut Criterion) {
    c.bench_function("epsg3857_to_bd09", |b| {
        b.iter(|| {
            black_box(epsg3857_to_bd09(
                black_box(MERCATOR_X),
                black_box(MERCATOR_Y),
            ))
        })
    });
}

//
// China region lookup
//

fn bench_is_in_gcj02_region_inside(c: &mut Criterion) {
    c.bench_function("is_in_gcj02_region/inside", |b| {
        b.iter(|| black_box(is_in_gcj02_region(black_box(LON), black_box(LAT))))
    });
}

fn bench_is_in_gcj02_region_outside(c: &mut Criterion) {
    // Auckland: should be rejected very early by the bounding box.
    c.bench_function("is_in_gcj02_region/outside", |b| {
        b.iter(|| black_box(is_in_gcj02_region(black_box(174.7633), black_box(-36.8485))))
    });
}

//
// Outside-China fast path
//

fn bench_wgs84_to_gcj02_outside(c: &mut Criterion) {
    c.bench_function("wgs84_to_gcj02/outside_china", |b| {
        b.iter(|| black_box(wgs84_to_gcj02(black_box(174.7633), black_box(-36.8485))))
    });
}

fn bench_wgs84_to_bd09_outside(c: &mut Criterion) {
    c.bench_function("wgs84_to_bd09/outside_china", |b| {
        b.iter(|| black_box(wgs84_to_bd09(black_box(174.7633), black_box(-36.8485))))
    });
}

//
// In-place batch conversion
//

fn bench_wgs84_to_gcj02_batch(c: &mut Criterion) {
    const POINTS: [(f64, f64); 256] = [(116.404, 39.915); 256];

    c.bench_function("wgs84_to_gcj02_in_place/256", |b| {
        b.iter_batched(
            || POINTS,
            |mut points| {
                wgs84_to_gcj02_in_place(black_box(&mut points));
                black_box(points)
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_wgs84_to_gcj02_mainland_batch(c: &mut Criterion) {
    const POINTS: [(f64, f64); 256] = [(116.404, 39.915); 256];

    c.bench_function("wgs84_to_gcj02_in_mainland_in_place/256", |b| {
        b.iter_batched(
            || POINTS,
            |mut points| {
                wgs84_to_gcj02_in_mainland_in_place(black_box(&mut points));
                black_box(points)
            },
            BatchSize::SmallInput,
        )
    });
}

fn bench_gcj02_to_wgs84_batch(c: &mut Criterion) {
    const POINTS: [(f64, f64); 256] = [(116.41024449916938, 39.91640428150164); 256];

    c.bench_function("gcj02_to_wgs84_in_place/256", |b| {
        b.iter_batched(
            || POINTS,
            |mut points| {
                gcj02_to_wgs84_in_place(black_box(&mut points));
                black_box(points)
            },
            BatchSize::SmallInput,
        )
    });
}

criterion_group!(
    benches,
    // BD09 <-> GCJ02
    bench_bd09_to_gcj02,
    bench_bd09_to_gcj02_fast,
    bench_gcj02_to_bd09,
    // WGS84 <-> GCJ02
    bench_wgs84_to_gcj02,
    bench_wgs84_to_gcj02_in_mainland,
    bench_gcj02_to_wgs84,
    bench_gcj02_to_wgs84_fast,
    bench_gcj02_to_wgs84_in_mainland,
    // WGS84 <-> BD09
    bench_wgs84_to_bd09,
    bench_bd09_to_wgs84,
    // EPSG:3857
    bench_wgs84_to_epsg3857,
    bench_epsg3857_to_wgs84,
    bench_gcj02_to_epsg3857,
    bench_epsg3857_to_gcj02,
    bench_bd09_to_epsg3857,
    bench_epsg3857_to_bd09,
    // Region lookup
    bench_is_in_gcj02_region_inside,
    bench_is_in_gcj02_region_outside,
    // Outside-China path
    bench_wgs84_to_gcj02_outside,
    bench_wgs84_to_bd09_outside,
    // Batch
    bench_wgs84_to_gcj02_batch,
    bench_wgs84_to_gcj02_mainland_batch,
    bench_gcj02_to_wgs84_batch,
);

criterion_main!(benches);
