use colorgrad::{Gradient, GradientBuilder, LinearGradient};

#[test]
fn sharp_gradient() {
    let grad = GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .build::<LinearGradient>()
        .unwrap();

    let g0 = grad.sharp(0, 0.0);
    assert_eq!(g0.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g0.at(0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g0.at(0.1).to_rgba8(), [255, 0, 0, 255]);

    let g1 = grad.sharp(1, 0.0);
    assert_eq!(g1.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g1.at(0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g1.at(0.1).to_rgba8(), [255, 0, 0, 255]);

    let g3 = grad.sharp(3, 0.0);
    assert_eq!(g3.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g3.at(0.1).to_rgba8(), [255, 0, 0, 255]);

    assert_eq!(g3.at(0.4).to_rgba8(), [0, 255, 0, 255]);
    assert_eq!(g3.at(0.5).to_rgba8(), [0, 255, 0, 255]);
    assert_eq!(g3.at(0.6).to_rgba8(), [0, 255, 0, 255]);

    assert_eq!(g3.at(0.9).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g3.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    assert_eq!(g3.at(-0.1).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g3.at(1.1).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g3.at(f32::NAN).to_rgba8(), [0, 0, 0, 255]);

    let grad = GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .domain(&[-1.0, 1.0])
        .build::<LinearGradient>()
        .unwrap();

    let g2 = grad.sharp(2, 0.0);
    assert_eq!(g2.at(-1.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g2.at(-0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g2.at(-0.1).to_rgba8(), [255, 0, 0, 255]);

    assert_eq!(g2.at(0.1).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g2.at(0.5).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g2.at(1.0).to_rgba8(), [0, 0, 255, 255]);
}

#[test]
fn sharp_gradient_with_smoothness() {
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .build::<LinearGradient>()
        .unwrap();

    let g0 = g.sharp(0, 0.1);
    assert_eq!(g0.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g0.at(0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g0.at(1.0).to_rgba8(), [255, 0, 0, 255]);

    let g1 = g.sharp(1, 0.1);
    assert_eq!(g1.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g1.at(0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g1.at(1.0).to_rgba8(), [255, 0, 0, 255]);

    let g = g.sharp(3, 0.1);
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.1).to_rgba8(), [255, 0, 0, 255]);

    assert_eq!(g.at(1.0 / 3.0).to_rgba8(), [128, 128, 0, 255]);

    assert_eq!(g.at(0.45).to_rgba8(), [0, 255, 0, 255]);
    assert_eq!(g.at(0.50).to_rgba8(), [0, 255, 0, 255]);
    assert_eq!(g.at(0.55).to_rgba8(), [0, 255, 0, 255]);

    assert_eq!(g.at(1.0 / 3.0 * 2.0).to_rgba8(), [0, 128, 128, 255]);

    assert_eq!(g.at(0.9).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    assert_eq!(g.at(-0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(1.5).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g.at(f32::NAN).to_rgba8(), [0, 0, 0, 255]);
}
