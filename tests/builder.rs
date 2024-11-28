use colorgrad::{
    BlendMode, Color, Gradient, GradientBuilder, GradientBuilderError, LinearGradient,
};

mod utils;
use utils::*;

#[test]
fn builder() {
    // Default colors

    let g = GradientBuilder::new().build::<LinearGradient>().unwrap();
    assert_eq!(g.domain(), (0.0, 1.0));
    assert_eq!(g.at(0.0).to_hex_string(), "#000000");
    assert_eq!(g.at(1.0).to_hex_string(), "#ffffff");

    // Single color

    let g = GradientBuilder::new()
        .colors(&[Color::new(1.0, 0.0, 0.0, 1.0)])
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [255, 0, 0, 255]);

    // Default domain

    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(0.0).to_hex_string(), "#ff0000");
    assert_eq!(g.at(0.5).to_hex_string(), "#00ff00");
    assert_eq!(g.at(1.0).to_hex_string(), "#0000ff");

    // Custom domain

    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .domain(&[-100.0, 100.0])
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(-100.0).to_hex_string(), "#ff0000");
    assert_eq!(g.at(0.0).to_hex_string(), "#00ff00");
    assert_eq!(g.at(100.0).to_hex_string(), "#0000ff");

    // Color position

    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f", "#f0f"])
        .domain(&[13.0, 27.3, 90.0, 97.5])
        .build::<LinearGradient>()
        .unwrap();
    assert_eq!(g.at(13.0).to_hex_string(), "#ff0000");
    assert_eq!(g.at(27.3).to_hex_string(), "#00ff00");
    assert_eq!(g.at(90.0).to_hex_string(), "#0000ff");
    assert_eq!(g.at(97.5).to_hex_string(), "#ff00ff");

    // Multiple colors, custom domain

    let cols1 = vec!["#00f", "#00ffff"];
    let cols2 = vec!["lime".to_string()];
    let mut gb = GradientBuilder::new();
    gb.html_colors(&cols1);
    gb.colors(&[Color::new(1.0, 1.0, 0.0, 0.5)]);
    gb.html_colors(&cols2);
    gb.domain(&[10.0, 50.0]);
    let g = gb.build::<LinearGradient>().unwrap();

    assert_eq!(
        &colors2hex(&gb.get_colors()),
        &["#0000ff", "#00ffff", "#ffff0080", "#00ff00"]
    );
    assert_eq!(
        &colors2hex(&g.colors(4)),
        &["#0000ff", "#00ffff", "#ffff0080", "#00ff00"]
    );

    // Filter stops

    let mut gb = GradientBuilder::new();
    gb.html_colors(&["gold", "red", "blue", "yellow", "black", "white", "plum"]);
    gb.domain(&[0.0, 0.0, 0.5, 0.5, 0.5, 1.0, 1.0]);
    gb.build::<LinearGradient>().unwrap();

    assert_eq!(&gb.get_positions(), &[0.0, 0.5, 0.5, 1.0]);
    assert_eq!(
        &colors2hex(&gb.get_colors()),
        &["#ff0000", "#0000ff", "#000000", "#ffffff"]
    );

    // Reusing builder

    let mut gb = GradientBuilder::new();
    gb.colors(&[
        Color::from_rgba8(255, 0, 0, 255),
        Color::from_rgba8(0, 0, 255, 255),
        Color::from_rgba8(0, 255, 0, 255),
    ]);
    gb.domain(&[0.0, 0.5, 1.0]);
    gb.mode(BlendMode::Rgb);

    let mut gb2 = gb.clone();

    let g = gb.build::<LinearGradient>().unwrap();
    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(0.5).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g.at(1.0).to_rgba8(), [0, 255, 0, 255]);

    // change color position
    gb2.domain(&[0.0, 35.0, 100.0]);
    let g = gb2.build::<LinearGradient>().unwrap();

    assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(g.at(35.0).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(g.at(100.0).to_rgba8(), [0, 255, 0, 255]);

    // Reset
    gb.reset();
    gb.colors(&[
        Color::from_rgba8(127, 0, 100, 255),
        Color::from_rgba8(50, 255, 0, 255),
    ]);
    gb.build::<LinearGradient>().unwrap();
    assert_eq!(gb.get_positions(), &[0.0, 1.0]);
    assert_eq!(&colors2hex(gb.get_colors()), &["#7f0064", "#32ff00"]);
}

#[test]
fn css_gradient() {
    let test_data = [
        (
            "red, lime 75%, blue",
            vec![0.0, 0.75, 1.0],
            vec!["#ff0000", "#00ff00", "#0000ff"],
        ),
        (
            "red 13%, lime, blue",
            vec![0.0, 0.13, 0.565, 1.0],
            vec!["#ff0000", "#ff0000", "#00ff00", "#0000ff"],
        ),
        (
            "red, lime, blue 100",
            vec![0.0, 50.0, 100.0],
            vec!["#ff0000", "#00ff00", "#0000ff"],
        ),
        (
            "red -100, lime, blue 100",
            vec![-100.0, 0.0, 100.0],
            vec!["#ff0000", "#00ff00", "#0000ff"],
        ),
        (
            "red, lime -10, blue 15, gold",
            vec![0.0, 15.0],
            vec!["#00ff00", "#0000ff"],
        ),
    ];

    for (s, positions, colors) in test_data {
        let mut gb = GradientBuilder::new();
        gb.css(s).build::<LinearGradient>().unwrap();
        assert_eq!(gb.get_positions(), &positions);
        assert_eq!(&colors2hex(gb.get_colors()), &colors);
    }

    // Invalid format

    let invalid_css = [
        "",
        " ",
        "0, red, lime",
        "red, lime, 100%",
        "deeppink, 0.4, 0.9, pink",
        "0%, 100%",
    ];

    for s in invalid_css {
        let g = GradientBuilder::new().css(s).build::<LinearGradient>();
        assert_eq!(g.unwrap_err(), GradientBuilderError::InvalidCssGradient);
    }
}

#[test]
fn builder_error() {
    // Invalid HTML colors
    let g = GradientBuilder::new()
        .html_colors(&["#777", "bloodred", "#bbb", "#zzz"])
        .build::<LinearGradient>();
    assert_eq!(
        g.as_ref().unwrap_err(),
        &GradientBuilderError::InvalidHtmlColors(vec!["bloodred".to_string(), "#zzz".to_string()])
    );
    assert_eq!(
        g.unwrap_err().to_string(),
        "invalid html colors: 'bloodred', '#zzz'"
    );

    // Invalid domain
    let g = GradientBuilder::new()
        .html_colors(&["#777", "gold", "#bbb", "#f0f"])
        .domain(&[0.0, 0.75, 1.0])
        .build::<LinearGradient>();
    assert_eq!(g.unwrap_err(), GradientBuilderError::InvalidDomain);

    // Invalid domain
    let g = GradientBuilder::new()
        .html_colors(&["#777", "gold", "#bbb", "#f0f"])
        .domain(&[0.0, 0.71, 0.7, 1.0])
        .build::<LinearGradient>();
    assert_eq!(g.unwrap_err(), GradientBuilderError::InvalidDomain);

    // Invalid domain
    let g = GradientBuilder::new()
        .html_colors(&["#777", "gold", "#bbb", "#f0f"])
        .domain(&[1.0, 0.0])
        .build::<LinearGradient>();
    assert_eq!(g.unwrap_err(), GradientBuilderError::InvalidDomain);

    // Invalid domain
    let g = GradientBuilder::new()
        .html_colors(&["#777", "#bbb"])
        .domain(&[2.0, 1.0])
        .build::<LinearGradient>();
    assert_eq!(g.unwrap_err(), GradientBuilderError::InvalidDomain);

    // Invalid CSS gradient
    let g = GradientBuilder::new()
        .css("#f00, 30%, 55%, #00f")
        .build::<LinearGradient>();
    assert_eq!(g.unwrap_err(), GradientBuilderError::InvalidCssGradient);

    // Invalid stops
    let g = GradientBuilder::new()
        .html_colors(&["#777", "#f0f", "#f00"])
        .domain(&[0.0, 0.0, 0.0])
        .build::<LinearGradient>();
    assert_eq!(g.unwrap_err(), GradientBuilderError::InvalidStops);
}
