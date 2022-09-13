use colorgrad::Color;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::BufReader;

const GGR_STR: &'static str = include_str!("../examples/ggr/My_Gradient.ggr");

const POSITIONS: [f64; 3] = [0.03, 0.5, 0.97];

fn bench_gimp_gradient(c: &mut Criterion) {
    let fg_color = Color::new(0.0, 0.0, 0.0, 1.0);
    let bg_color = Color::new(1.0, 1.0, 1.0, 1.0);
    let (gradient, _) =
        colorgrad::parse_ggr(BufReader::new(GGR_STR.as_bytes()), &fg_color, &bg_color).unwrap();

    for pos in POSITIONS {
        c.bench_function(&format!("GimpGradient t={pos}"), |b| {
            b.iter(|| {
                gradient.at(black_box(pos));
            })
        });
    }
}

criterion_group!(gimp_gradient, bench_gimp_gradient,);
criterion_main!(gimp_gradient);
