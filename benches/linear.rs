use colorgrad::BlendMode;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn build_gradient(mode: BlendMode) -> colorgrad::Gradient {
    colorgrad::CustomGradient::new()
        .html_colors(&[
            "#8dd3c7", "#ffffb3", "#bebada", "#fb8072", "#80b1d3", "#fdb462", "#b3de69", "#fccde5",
            "#d9d9d9", "#bc80bd", "#ccebc5", "#ffed6f",
        ])
        .mode(mode)
        .interpolation(colorgrad::Interpolation::Linear)
        .build()
        .unwrap()
}

fn bench_linear_rgb(c: &mut Criterion) {
    let grad = build_gradient(BlendMode::Rgb);
    c.bench_function("LinearGradient (rgb)", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_linear_lrgb(c: &mut Criterion) {
    let grad = build_gradient(BlendMode::LinearRgb);
    c.bench_function("LinearGradient (linear-rgb)", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_linear_hsv(c: &mut Criterion) {
    let grad = build_gradient(BlendMode::Hsv);
    c.bench_function("LinearGradient (hsv)", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_linear_oklab(c: &mut Criterion) {
    let grad = build_gradient(BlendMode::Oklab);
    c.bench_function("LinearGradient (oklab)", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

criterion_group!(
    linear_gradient,
    bench_linear_rgb,
    bench_linear_lrgb,
    bench_linear_hsv,
    bench_linear_oklab
);
criterion_main!(linear_gradient);
