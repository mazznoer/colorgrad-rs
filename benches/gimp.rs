use colorgrad::{Color, GimpGradient, Gradient};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::io::BufReader;

const GGR1: &'static str = include_str!("../examples/ggr/My_Gradient.ggr");
const GGR2: &'static str = include_str!("../examples/ggr/test_hsv.ggr");

const POSITIONS: [f32; 4] = [0.03, 0.45, 0.55, 0.97];

fn bench_gimp_gradient(c: &mut Criterion) {
    let fg_color = Color::new(0.0, 0.0, 0.0, 1.0);
    let bg_color = Color::new(1.0, 1.0, 1.0, 1.0);
    let grad1 = GimpGradient::new(BufReader::new(GGR1.as_bytes()), &fg_color, &bg_color).unwrap();
    let grad2 = GimpGradient::new(BufReader::new(GGR2.as_bytes()), &fg_color, &bg_color).unwrap();

    for pos in POSITIONS {
        c.bench_function(&format!("GimpGradient RGB t={pos}"), |b| {
            b.iter(|| {
                grad1.at(black_box(pos));
            })
        });
    }

    for pos in POSITIONS {
        c.bench_function(&format!("GimpGradient HSV t={pos}"), |b| {
            b.iter(|| {
                grad2.at(black_box(pos));
            })
        });
    }
}

criterion_group!(gimp_gradient, bench_gimp_gradient,);
criterion_main!(gimp_gradient);
