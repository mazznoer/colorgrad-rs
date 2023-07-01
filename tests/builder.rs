use colorgrad::{GradientBuilder, LinearGradient};

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
        g.unwrap_err().to_string(),
        "invalid html colors: 'bloodred', '#zzz'"
    );

    // Wrong domain #1
    let g = GradientBuilder::new()
        .html_colors(&["#777", "gold", "#bbb", "#f0f"])
        .domain(&[0.0, 0.75, 1.0])
        .build::<LinearGradient>();
    assert_eq!(g.unwrap_err().to_string(), "wrong domain count");

    // Wrong domain #2
    let g = GradientBuilder::new()
        .html_colors(&["#777", "gold", "#bbb", "#f0f"])
        .domain(&[0.0, 0.71, 0.7, 1.0])
        .build::<LinearGradient>();
    assert_eq!(g.unwrap_err().to_string(), "wrong domain");

    // Wrong domain #3
    let g = GradientBuilder::new()
        .html_colors(&["#777", "gold", "#bbb", "#f0f"])
        .domain(&[1.0, 0.0])
        .build::<LinearGradient>();
    assert_eq!(g.unwrap_err().to_string(), "wrong domain");
}
