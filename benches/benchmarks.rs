use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_standard_temperature(c: &mut Criterion) {
    c.bench_function("atmosphere/standard_temperature", |b| {
        b.iter(|| badal::atmosphere::standard_temperature(black_box(5000.0)));
    });
}

fn bench_saturation_vp(c: &mut Criterion) {
    c.bench_function("moisture/saturation_vapor_pressure", |b| {
        b.iter(|| badal::moisture::saturation_vapor_pressure(black_box(20.0)));
    });
}

fn bench_coriolis(c: &mut Criterion) {
    c.bench_function("wind/coriolis_parameter", |b| {
        b.iter(|| badal::wind::coriolis_parameter(black_box(0.785)));
    });
}

fn bench_cloud_base(c: &mut Criterion) {
    c.bench_function("cloud/cloud_base_altitude", |b| {
        b.iter(|| badal::cloud::cloud_base_altitude(black_box(25.0), black_box(15.0)));
    });
}

fn bench_beaufort(c: &mut Criterion) {
    c.bench_function("wind/beaufort_scale", |b| {
        b.iter(|| badal::wind::beaufort_scale(black_box(12.0)));
    });
}

criterion_group!(benches, bench_standard_temperature, bench_saturation_vp, bench_coriolis, bench_cloud_base, bench_beaufort);
criterion_main!(benches);
