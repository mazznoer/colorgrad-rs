use colorgrad::BlendMode;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn build_gradient(mode: BlendMode) -> colorgrad::Gradient {
    colorgrad::CustomGradient::new()
        .html_colors(&[
            "#8dd3c7", "#ffffb3", "#bebada", "#fb8072", "#80b1d3", "#fdb462", "#b3de69", "#fccde5",
            "#d9d9d9", "#bc80bd", "#ccebc5", "#ffed6f",
        ])
        .mode(mode)
        .interpolation(colorgrad::Interpolation::CatmullRom)
        .build()
        .unwrap()
}

fn bench_catmull_rom_rgb(c: &mut Criterion) {
    let grad = build_gradient(BlendMode::Rgb);
    c.bench_function("CatmullRomGradient (rgb)", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_catmull_rom_lrgb(c: &mut Criterion) {
    let grad = build_gradient(BlendMode::LinearRgb);
    c.bench_function("CatmullRomGradient (linear-rgb)", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_catmull_rom_oklab(c: &mut Criterion) {
    let grad = build_gradient(BlendMode::Oklab);
    c.bench_function("CatmullRomGradient (oklab)", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

fn bench_catmull_rom_hsv(c: &mut Criterion) {
    let grad = build_gradient(BlendMode::Hsv);
    c.bench_function("CatmullRomGradient (hsv)", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

criterion_group!(
    catmull_rom_gradient,
    bench_catmull_rom_rgb,
    bench_catmull_rom_lrgb,
    bench_catmull_rom_oklab,
    bench_catmull_rom_hsv
);
criterion_main!(catmull_rom_gradient);
