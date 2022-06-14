use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_catmull_rom(c: &mut Criterion) {
    let grad = colorgrad::CustomGradient::new()
        .html_colors(&[
            "#8dd3c7", "#ffffb3", "#bebada", "#fb8072", "#80b1d3", "#fdb462", "#b3de69", "#fccde5",
            "#d9d9d9", "#bc80bd", "#ccebc5", "#ffed6f",
        ])
        .interpolation(colorgrad::Interpolation::CatmullRom)
        .build()
        .unwrap();

    c.bench_function("catmull_rom", |b| {
        b.iter(|| {
            grad.at(black_box(0.6));
        })
    });
}

criterion_group!(catmull_rom, bench_catmull_rom);
criterion_main!(catmull_rom);
