use colorgrad::{
    BlendMode, Color, Gradient, GradientBuilder, GradientBuilderError, LinearGradient,
};

#[test]
fn builder() {
    // Builder pattern style 2
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
