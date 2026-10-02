use colorgrad::Gradient;

mod utils;
use utils::*;

#[test]
fn basic() {
    let g = colorgrad::GradientBuilder::new()
        .css("#00f, #f00, #fff, #ff0")
        .mode(colorgrad::BlendMode::Rgb)
        .build::<colorgrad::SmoothstepGradient>()
        .unwrap();

    cmp_hex!(g.at(0.0), "#0000ff");
    cmp_hex!(g.at(1.0 / 3.0), "#ff0000");
    cmp_hex!(g.at(2.0 / 3.0), "#ffffff");
    cmp_hex!(g.at(1.0), "#ffff00");

    let colors = [
        "#0000ff", "#0700f8", "#1900e6", "#3400cb", "#5500aa", "#790086", "#9e0061", "#c1003e",
        "#de0021", "#f3000c", "#fe0001", "#ff0303", "#ff1212", "#ff2a2a", "#ff4a4a", "#ff6d6d",
        "#ff9292", "#ffb5b5", "#ffd5d5", "#ffeded", "#fffcfc", "#fffffe", "#fffff3", "#ffffde",
        "#ffffc1", "#ffff9e", "#ffff79", "#ffff55", "#ffff34", "#ffff19", "#ffff07", "#ffff00",
    ];

    assert_eq!(colors2hex(g.colors(32)), &colors);

    cmp_hex!(g.at(-0.1), "#0000ff");
    cmp_hex!(g.at(1.11), "#ffff00");
    cmp_hex!(g.at(f32::NAN), "#000000");
}
