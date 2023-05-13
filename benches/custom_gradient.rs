use colorgrad::{
    BasisGradient, BlendMode, CatmullRomGradient, Gradient, GradientBuilder, LinearGradient,
};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

const COLORS: [&'static str; 104] = [
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
];
const MODES: [BlendMode; 3] = [BlendMode::Rgb, BlendMode::LinearRgb, BlendMode::Oklab];
const POSITIONS: [f32; 3] = [0.03, 0.5, 0.97];

fn bench_linear_gradient(c: &mut Criterion) {
    for mode in MODES {
        let grad = GradientBuilder::new()
            .html_colors(&COLORS)
            .mode(mode)
            .build::<LinearGradient>()
            .unwrap();

        for pos in POSITIONS {
            c.bench_function(&format!("LinearGradient ({mode:?}) t={pos}"), |b| {
                b.iter(|| {
                    grad.at(black_box(pos));
                })
            });
        }
    }
}

fn bench_catmull_rom_gradient(c: &mut Criterion) {
    for mode in MODES {
        let grad = GradientBuilder::new()
            .html_colors(&COLORS)
            .mode(mode)
            .build::<CatmullRomGradient>()
            .unwrap();

        for pos in POSITIONS {
            c.bench_function(&format!("CatmullRomGradient ({mode:?}) t={pos}"), |b| {
                b.iter(|| {
                    grad.at(black_box(pos));
                })
            });
        }
    }
}

fn bench_basis_gradient(c: &mut Criterion) {
    for mode in MODES {
        let grad = GradientBuilder::new()
            .html_colors(&COLORS)
            .mode(mode)
            .build::<BasisGradient>()
            .unwrap();

        for pos in POSITIONS {
            c.bench_function(&format!("BasisGradient ({mode:?}) t={pos}"), |b| {
                b.iter(|| {
                    grad.at(black_box(pos));
                })
            });
        }
    }
}

criterion_group!(linear_gradient, bench_linear_gradient,);
criterion_group!(catmull_rom_gradient, bench_catmull_rom_gradient,);
criterion_group!(basis_gradient, bench_basis_gradient,);
criterion_main!(linear_gradient, catmull_rom_gradient, basis_gradient);
