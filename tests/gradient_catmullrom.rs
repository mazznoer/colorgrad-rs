use colorgrad::Gradient;

mod utils;
use utils::*;

#[test]
fn basic() {
    let g = colorgrad::GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .mode(colorgrad::BlendMode::Rgb)
        .build::<colorgrad::CatmullRomGradient>()
        .unwrap();

    assert_eq!(g.at(0.00).to_hex_string(), "#ff0000");
    assert_eq!(g.at(0.25).to_hex_string(), "#609f00");
    assert_eq!(g.at(0.50).to_hex_string(), "#00ff00");
    assert_eq!(g.at(0.75).to_hex_string(), "#009f60");
    assert_eq!(g.at(1.00).to_hex_string(), "#0000ff");

    assert_eq!(
        colors2hex(&g.colors(5)),
        &["#ff0000", "#609f00", "#00ff00", "#009f60", "#0000ff"]
    );

    assert_eq!(g.at(-0.1).to_hex_string(), "#ff0000");
    assert_eq!(g.at(1.11).to_hex_string(), "#0000ff");
    assert_eq!(g.at(f32::NAN).to_hex_string(), "#000000");
}
