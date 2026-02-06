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

    cmp_hex!(g.at(0.00), "#ff0000");
    cmp_hex!(g.at(0.25), "#808000");
    cmp_hex!(g.at(0.50), "#00ff00");
    cmp_hex!(g.at(0.75), "#008080");
    cmp_hex!(g.at(1.00), "#0000ff");

    assert_eq!(
        colors2hex(g.colors(5)),
        &["#ff0000", "#808000", "#00ff00", "#008080", "#0000ff"]
    );

    cmp_hex!(g.at(-0.1), "#ff0000");
    cmp_hex!(g.at(1.11), "#0000ff");
    cmp_hex!(g.at(f32::NAN), "#000000");
}
