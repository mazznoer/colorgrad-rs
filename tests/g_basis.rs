use colorgrad::Gradient;

mod utils;
use utils::*;

#[test]
fn basic() {
    let g = colorgrad::GradientBuilder::new()
        .html_colors(&["#ff1493", "#ffd700", "#2e8b57"])
        .mode(colorgrad::BlendMode::Rgb)
        .build::<colorgrad::BasisGradient>()
        .unwrap();

    assert_eq!(g.domain(), (0.0, 1.0));

    cmp_hex!(g.at(0.00), "#ff1493");
    cmp_hex!(g.at(0.25), "#fb704e");
    cmp_hex!(g.at(0.50), "#dcaa27");
    cmp_hex!(g.at(0.75), "#92ab30");
    cmp_hex!(g.at(1.00), "#2e8b57");

    let colors = [
        "#ff1493", "#ff2586", "#ff367a", "#fe466d", "#fe5662", "#fc6556", "#fa734c", "#f78142",
        "#f38c3a", "#ee9733", "#e8a02d", "#e1a729", "#d7ac26", "#cdaf25", "#c1b126", "#b4b128",
        "#a5af2b", "#96ac2f", "#86a834", "#75a43a", "#649e41", "#529848", "#40924f", "#2e8b57",
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
        .build::<colorgrad::BasisGradient>()
        .unwrap();

    assert_eq!(g.domain(), (-0.75, 0.6));

    let colors = [
        "#0000ff", "#1900e6", "#3100ce", "#4901b7", "#6002a1", "#77058d", "#8c087c", "#a00d6d",
        "#b21461", "#c21c59", "#d02755", "#dc3356", "#e6425b", "#ee5263", "#f4646e", "#f87679",
        "#fb8986", "#fd9b91", "#fead9c", "#ffbda4", "#ffcca9", "#ffd8aa", "#ffe3a6", "#ffeb9e",
        "#fff292", "#fff783", "#fffa72", "#fffd5e", "#fffe48", "#ffff31", "#ffff19", "#ffff00",
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
        .build::<colorgrad::BasisGradient>()
        .unwrap();

    assert_eq!(g.domain(), (20.0, 87.0));

    let colors = [
        "#2ec149", "#6fc543", "#a8c53c", "#d0bb33", "#dbb22f", "#e3a82c", "#e89c29", "#eb9029",
        "#ec822b", "#eb7530", "#ea6839", "#e96140", "#e85b48", "#e75552", "#e64f5c", "#e44a66",
        "#e24571", "#df407c", "#dd3c87", "#d93992", "#d5369d", "#d134a7", "#cc32b0", "#c631b8",
        "#c131bd", "#bc31c2", "#b632c7", "#b033cb", "#a934ce", "#a235d1", "#9b37d3", "#9439d5",
        "#8c3bd7", "#853ed9", "#7d41da", "#7543db", "#6d46db", "#6549dc", "#5c4cdd", "#544fdd",
    ];
    assert_eq!(colors2hex(g.colors(40)), &colors);

    cmp_hex!(g.at(19.0), "#2ec149");
    cmp_hex!(g.at(88.0), "#544fdd");
}
