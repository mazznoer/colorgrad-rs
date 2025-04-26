use colorgrad::Gradient;

mod utils;
use utils::*;

#[test]
fn basic() {
    let g = colorgrad::GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .mode(colorgrad::BlendMode::Rgb)
        .build::<colorgrad::BasisGradient>()
        .unwrap();

    cmp_hex!(g.at(0.00), "#ff0000");
    cmp_hex!(g.at(0.25), "#857505");
    cmp_hex!(g.at(0.50), "#2baa2b");
    cmp_hex!(g.at(0.75), "#057585");
    cmp_hex!(g.at(1.00), "#0000ff");

    assert_eq!(
        colors2hex(&g.colors(5)),
        &["#ff0000", "#857505", "#2baa2b", "#057585", "#0000ff"]
    );

    cmp_hex!(g.at(-0.1), "#ff0000");
    cmp_hex!(g.at(1.11), "#0000ff");
    cmp_hex!(g.at(f32::NAN), "#000000");
}
