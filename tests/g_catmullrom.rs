use colorgrad::Gradient;

mod utils;
use utils::*;

#[test]
fn basic() {
    let g = colorgrad::GradientBuilder::new()
        .html_colors(&["#ff1493", "#ffd700", "#2e8b57"])
        .mode(colorgrad::BlendMode::Rgb)
        .build::<colorgrad::CatmullRomGradient>()
        .unwrap();

    assert_eq!(g.domain(), (0.0, 1.0));

    cmp_hex!(g.at(0.00), "#ff1493");
    cmp_hex!(g.at(0.25), "#ff8e37");
    cmp_hex!(g.at(0.50), "#ffd700");
    cmp_hex!(g.at(0.75), "#b1bb21");
    cmp_hex!(g.at(1.00), "#2e8b57");

    let colors = [
        "#ff1493", "#ff2685", "#ff3b76", "#ff5165", "#ff6754", "#ff7e43", "#ff9333", "#ffa724",
        "#ffb817", "#ffc70c", "#ffd105", "#ffd601", "#fed700", "#f8d503", "#edd107", "#decb0e",
        "#ccc415", "#b6bd1e", "#9fb428", "#87ab32", "#6fa33c", "#589a46", "#42924f", "#2e8b57",
    ];
    assert_eq!(colors2hex(g.colors(24)), &colors);

    cmp_hex!(g.at(-0.1), "#ff1493");
    cmp_hex!(g.at(1.11), "#2e8b57");
    cmp_hex!(g.at(f32::NEG_INFINITY), "#ff1493");
    cmp_hex!(g.at(f32::INFINITY), "#2e8b57");
    cmp_hex!(g.at(f32::NAN), "#000000");
}

#[test]
fn custom_domain() {
    let g = colorgrad::GradientBuilder::new()
        .html_colors(&["#00f", "#f00", "#fff", "#ff0"])
        .domain(&[-0.75, 0.6])
        .mode(colorgrad::BlendMode::Rgb)
        .build::<colorgrad::CatmullRomGradient>()
        .unwrap();

    assert_eq!(g.domain(), (-0.75, 0.6));

    let colors = [
        "#0000ff", "#1b00e4", "#3900c6", "#5900a6", "#7a0085", "#9a0065", "#b80047", "#d2002d",
        "#e80017", "#f70008", "#fe0001", "#ff0303", "#ff1212", "#ff2a2a", "#ff4a4a", "#ff6d6d",
        "#ff9292", "#ffb5b5", "#ffd5d5", "#ffeded", "#fffcfc", "#fffffe", "#fffff7", "#ffffe8",
        "#ffffd2", "#ffffb8", "#ffff9a", "#ffff7a", "#ffff59", "#ffff39", "#ffff1b", "#ffff00",
    ];
    assert_eq!(colors2hex(g.colors(32)), &colors);

    cmp_hex!(g.at(-0.8), "#0000ff");
    cmp_hex!(g.at(0.63), "#ffff00");
}

#[test]
fn custom_pos() {
    let g = colorgrad::GradientBuilder::new()
        .html_colors(&["#2ec149", "#efd037", "#ed6212", "#db1ed5", "#544fdd"])
        .domain(&[20.0, 25.0, 37.0, 59.0, 87.0])
        .mode(colorgrad::BlendMode::Rgb)
        .build::<colorgrad::CatmullRomGradient>()
        .unwrap();

    assert_eq!(g.domain(), (20.0, 87.0));

    let colors = [
        "#2ec149", "#7fc743", "#cfce3d", "#efd037", "#efcb31", "#efbe2b", "#eead24", "#ee991d",
        "#ee8417", "#ed7013", "#ed6112", "#ec5b16", "#eb541f", "#ea4d2c", "#e9463d", "#e83f50",
        "#e73865", "#e5327a", "#e42c90", "#e227a4", "#e023b6", "#de20c5", "#dc1ed0", "#da1ed5",
        "#d41fd6", "#ce20d7", "#c822d7", "#c024d8", "#b827d8", "#b02ad9", "#a72dd9", "#9e31da",
        "#9535da", "#8b39db", "#823ddb", "#7841db", "#6f45dc", "#6548dc", "#5d4cdd", "#544fdd",
    ];
    assert_eq!(colors2hex(g.colors(40)), &colors);

    cmp_hex!(g.at(19.0), "#2ec149");
    cmp_hex!(g.at(88.0), "#544fdd");
}
