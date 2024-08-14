use colorgrad::Gradient;

mod utils;
use utils::*;

#[test]
fn basic() {
    let g = colorgrad::GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .mode(colorgrad::BlendMode::Rgb)
        .build::<colorgrad::LinearGradient>()
        .unwrap();

    assert_eq!(g.at(0.00).to_hex_string(), "#ff0000");
    assert_eq!(g.at(0.25).to_hex_string(), "#808000");
    assert_eq!(g.at(0.50).to_hex_string(), "#00ff00");
    assert_eq!(g.at(0.75).to_hex_string(), "#008080");
    assert_eq!(g.at(1.00).to_hex_string(), "#0000ff");

    assert_eq!(
        colors2hex(&g.colors(5)),
        &["#ff0000", "#808000", "#00ff00", "#008080", "#0000ff"]
    );

    assert_eq!(g.at(-0.1).to_hex_string(), "#ff0000");
    assert_eq!(g.at(1.11).to_hex_string(), "#0000ff");
    assert_eq!(g.at(f32::NAN).to_hex_string(), "#000000");
}
