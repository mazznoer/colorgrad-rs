use colorgrad::{Gradient, preset};
use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn bench_sinebow(c: &mut Criterion) {
    let grad = preset::sinebow();
    c.bench_function("preset sinebow", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_rainbow(c: &mut Criterion) {
    let grad = preset::rainbow();
    c.bench_function("preset rainbow", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_turbo(c: &mut Criterion) {
    let grad = preset::turbo();
    c.bench_function("preset turbo", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_cividis(c: &mut Criterion) {
    let grad = preset::cividis();
    c.bench_function("preset cividis", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_cubehelix(c: &mut Criterion) {
    let grad = preset::cubehelix_default();
    c.bench_function("preset cubehelix_default", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_warm(c: &mut Criterion) {
    let grad = preset::warm();
    c.bench_function("preset warm", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_cool(c: &mut Criterion) {
    let grad = preset::cool();
    c.bench_function("preset cool", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_spectral(c: &mut Criterion) {
    let grad = preset::spectral();
    c.bench_function("preset spectral", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

criterion_group!(
    preset_gradients,
    bench_sinebow,
    bench_rainbow,
    bench_turbo,
    bench_cividis,
    bench_cubehelix,
    bench_warm,
    bench_cool,
    bench_spectral,
);
criterion_main!(preset_gradients);
