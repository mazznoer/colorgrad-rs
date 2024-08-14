use colorgrad::{
    BasisGradient, BlendMode, CatmullRomGradient, Gradient, GradientBuilder, LinearGradient,
};

#[test]
fn blend_modes() {
    // Blend mode RGB
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .mode(BlendMode::Rgb)
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    // Blend mode Linear RGB
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .mode(BlendMode::LinearRgb)
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    // Blend mode Oklab
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .mode(BlendMode::Oklab)
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);
}

#[test]
fn interpolation_modes() {
    // Interpolation linear
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    // Interpolation catmull-rom
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .build::<CatmullRomGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    // Interpolation basis
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .build::<BasisGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert!(g.at(0.5).to_rgba8() != [255, 255, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 0, 255, 255]);
}
