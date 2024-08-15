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

    assert_eq!(g.at(0.00).to_hex_string(), "#ff0000");
    assert_eq!(g.at(0.25).to_hex_string(), "#857505");
    assert_eq!(g.at(0.50).to_hex_string(), "#2baa2b");
    assert_eq!(g.at(0.75).to_hex_string(), "#057585");
    assert_eq!(g.at(1.00).to_hex_string(), "#0000ff");

    assert_eq!(
        colors2hex(&g.colors(5)),
        &["#ff0000", "#857505", "#2baa2b", "#057585", "#0000ff"]
    );

    assert_eq!(g.at(-0.1).to_hex_string(), "#ff0000");
    assert_eq!(g.at(1.11).to_hex_string(), "#0000ff");
    assert_eq!(g.at(f32::NAN).to_hex_string(), "#000000");
}
