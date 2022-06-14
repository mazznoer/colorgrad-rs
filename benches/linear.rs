use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_linear(c: &mut Criterion) {
    let grad = colorgrad::CustomGradient::new()
        .html_colors(&[
            "#8dd3c7", "#ffffb3", "#bebada", "#fb8072", "#80b1d3", "#fdb462", "#b3de69", "#fccde5",
            "#d9d9d9", "#bc80bd", "#ccebc5", "#ffed6f",
        ])
        .interpolation(colorgrad::Interpolation::Linear)
        .build()
        .unwrap();

    c.bench_function("linear", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

criterion_group!(linear, bench_linear);
criterion_main!(linear);
