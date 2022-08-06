use colorgrad::BlendMode;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn build_gradient(mode: BlendMode) -> colorgrad::Gradient {
    colorgrad::CustomGradient::new()
        .html_colors(&[
            "#87e575", "#e88ef2", "#7398ef", "#65c3f2", "#3e52a0", "#b659db", "#75b7ff", "#7555ba",
            "#fceac4", "#e8009e", "#cc7c26", "#e175f4", "#f959e7", "#31828e", "#e4bef7", "#a9fcc6",
            "#c122d6", "#81f9e1", "#caea81", "#47d192", "#db579d", "#ead36b", "#3c2bbc", "#9de544",
            "#e8e476", "#055d66", "#77c90c", "#bff49a", "#6b76db", "#3cf720", "#61bace", "#aa3405",
            "#a588d8", "#e2aef9", "#c0eff9", "#9b043b", "#b2ffe0", "#64e092", "#ff4cab", "#56d356",
            "#e185e2", "#ff72f3", "#ff4fbe", "#0a9366", "#dbc2f9", "#6cbacc", "#893009", "#13afaa",
            "#5208ad", "#9b1426", "#71e06d", "#c2ff0c", "#ce4244", "#ffebb5", "#169bf9", "#e58eb5",
            "#3c3ab2", "#2afca5", "#5946c4", "#ea7352", "#f46bbb", "#264daf", "#edaada", "#c6baf4",
            "#d984e8", "#61dd5f", "#1f26b7", "#f99345", "#b2d624", "#f911e2", "#bf882a", "#81f48b",
            "#a3ffba", "#13c139", "#dd7752", "#db755c", "#fcbdf2", "#f455b2", "#7414e2", "#074575",
            "#7cffef", "#dd778a", "#db55cb", "#7aa7cc", "#fcbfd2", "#b7f799", "#a65bc6", "#f242ff",
            "#f9c0b3", "#9890db", "#d01be8", "#20870e", "#f4426b", "#def260", "#521efc", "#ffbcc6",
            "#e285b9", "#0ed6f9", "#7825ed", "#f2c6ff", "#cdb2f4", "#5fd374", "#fc838d", "#27bec6",
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

fn bench_linear_oklab(c: &mut Criterion) {
    let grad = build_gradient(BlendMode::Oklab);
    c.bench_function("LinearGradient (oklab)", |b| {
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

fn bench_pos1(c: &mut Criterion) {
    let grad = build_gradient(BlendMode::Rgb);
    c.bench_function("LinearGradient position 0.01", |b| {
        b.iter(|| {
            grad.at(black_box(0.01));
        })
    });
}

fn bench_pos2(c: &mut Criterion) {
    let grad = build_gradient(BlendMode::Rgb);
    c.bench_function("LinearGradient position 0.5", |b| {
        b.iter(|| {
            grad.at(black_box(0.5));
        })
    });
}

fn bench_pos3(c: &mut Criterion) {
    let grad = build_gradient(BlendMode::Rgb);
    c.bench_function("LinearGradient position 0.99", |b| {
        b.iter(|| {
            grad.at(black_box(0.99));
        })
    });
}

criterion_group!(
    linear_gradient,
    bench_linear_rgb,
    bench_linear_lrgb,
    bench_linear_oklab,
    bench_linear_hsv,
    bench_pos1,
    bench_pos2,
    bench_pos3,
);
criterion_main!(linear_gradient);
