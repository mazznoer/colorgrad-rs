use colorgrad::{BlendMode, Color, CustomGradient};

#[test]
fn custom_gradient() {
    // Custom gradient default
    let g = CustomGradient::new().build().unwrap();
    assert_eq!(g.domain(), (0., 1.));
    assert_eq!(g.at(0.).to_hex_string(), "#000000");
    assert_eq!(g.at(1.).to_hex_string(), "#ffffff");
    assert_eq!(format!("{:?}", g), "Gradient { dmin: 0.0, dmax: 1.0 }");

    // Custom colors
    let g = CustomGradient::new()
        .colors(&[
            Color::from_rgb(1., 0., 0.),
            Color::from_rgb(1., 1., 0.),
            Color::from_rgb(0., 0., 1.),
        ])
        .build()
        .unwrap();
    assert_eq!(g.domain(), (0., 1.));
    assert_eq!(g.at(0.0).to_hex_string(), "#ff0000");
    assert_eq!(g.at(0.5).to_hex_string(), "#ffff00");
    assert_eq!(g.at(1.0).to_hex_string(), "#0000ff");

    // Custom colors #2
    let g = CustomGradient::new()
        .html_colors(&["#00f", "#00ffff"])
        .colors(&[Color::from_rgba(1., 1., 0., 0.5)])
        .html_colors(&["lime"])
        .build()
        .unwrap();
    let colors = g.colors(4);
    assert_eq!(colors[0].rgba_u8(), (0, 0, 255, 255));
    assert_eq!(colors[1].rgba_u8(), (0, 255, 255, 255));
    assert_eq!(colors[2].rgba_u8(), (255, 255, 0, 128));
    assert_eq!(colors[3].rgba_u8(), (0, 255, 0, 255));

    // Single color
    let g = CustomGradient::new()
        .colors(&[Color::from_rgb(1., 0., 0.)])
        .build()
        .unwrap();
    assert_eq!(g.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g.at(0.5).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g.at(1.0).rgba_u8(), (255, 0, 0, 255));

    // Builder pattern style 2
    let mut gb = CustomGradient::new();
    gb.colors(&[
        Color::from_rgb_u8(255, 0, 0),
        Color::from_rgb_u8(0, 0, 255),
        Color::from_rgb_u8(0, 255, 0),
    ]);
    gb.domain(&[0.0, 0.5, 1.0]);
    gb.mode(BlendMode::Rgb);
    let g = gb.build().unwrap();
    assert_eq!(g.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g.at(0.5).rgba_u8(), (0, 0, 255, 255));
    assert_eq!(g.at(1.0).rgba_u8(), (0, 255, 0, 255));
}

#[test]
fn custom_gradient_blend_mode() {
    // Blend mode RGB
    let g = CustomGradient::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .mode(BlendMode::Rgb)
        .build()
        .unwrap();
    assert_eq!(g.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g.at(0.5).rgba_u8(), (255, 255, 0, 255));
    assert_eq!(g.at(1.0).rgba_u8(), (0, 0, 255, 255));

    // Blend mode Linear RGB
    let g = CustomGradient::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .mode(BlendMode::LinearRgb)
        .build()
        .unwrap();
    assert_eq!(g.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g.at(0.5).rgba_u8(), (255, 255, 0, 255));
    assert_eq!(g.at(1.0).rgba_u8(), (0, 0, 255, 255));

    // Blend mode HSV
    let g = CustomGradient::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .mode(BlendMode::Hsv)
        .build()
        .unwrap();
    assert_eq!(g.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g.at(0.5).rgba_u8(), (255, 255, 0, 255));
    assert_eq!(g.at(1.0).rgba_u8(), (0, 0, 255, 255));

    // Blend mode Oklab
    let g = CustomGradient::new()
        .html_colors(&["#f00", "#ff0", "#00f"])
        .mode(BlendMode::Oklab)
        .build()
        .unwrap();
    assert_eq!(g.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g.at(0.5).rgba_u8(), (255, 255, 0, 255));
    assert_eq!(g.at(1.0).rgba_u8(), (0, 0, 255, 255));
}

#[test]
fn custom_gradient_error() {
    // Invalid HTML colors
    let g = CustomGradient::new()
        .html_colors(&["#777", "bloodred", "#bbb", "#zzz"])
        .build();
    assert_eq!(g.unwrap_err().to_string(), "Invalid html color");

    // Wrong domain #1
    let g = CustomGradient::new()
        .html_colors(&["#777", "gold", "#bbb", "#f0f"])
        .domain(&[0., 0.75, 1.])
        .build();
    assert_eq!(g.unwrap_err().to_string(), "Wrong domain count");

    // Wrong domain #2
    let g = CustomGradient::new()
        .html_colors(&["#777", "gold", "#bbb", "#f0f"])
        .domain(&[0., 0.71, 0.7, 1.])
        .build();
    assert_eq!(g.unwrap_err().to_string(), "Wrong domain");

    // Wrong domain #3
    let g = CustomGradient::new()
        .html_colors(&["#777", "gold", "#bbb", "#f0f"])
        .domain(&[1., 0.])
        .build();
    assert_eq!(g.unwrap_err().to_string(), "Wrong domain");
}

#[test]
fn custom_gradient_domain() {
    // Custom domain #1
    let g = CustomGradient::new()
        .html_colors(&["yellow", "blue", "lime"])
        .domain(&[0., 100.])
        .build()
        .unwrap();
    assert_eq!(g.at(0.).to_hex_string(), "#ffff00");
    assert_eq!(g.at(50.).to_hex_string(), "#0000ff");
    assert_eq!(g.at(100.).to_hex_string(), "#00ff00");

    assert_eq!(g.at(-10.).to_hex_string(), "#ffff00");
    assert_eq!(g.at(110.).to_hex_string(), "#00ff00");
    assert_eq!(g.at(f64::NAN).to_hex_string(), "#ffff00");

    // Custom domain #2
    let g = CustomGradient::new()
        .html_colors(&["yellow", "blue", "lime"])
        .domain(&[-1., 1.])
        .build()
        .unwrap();
    assert_eq!(g.at(-1.).to_hex_string(), "#ffff00");
    assert_eq!(g.at(0.).to_hex_string(), "#0000ff");
    assert_eq!(g.at(1.).to_hex_string(), "#00ff00");

    assert_eq!(g.at(-2.).to_hex_string(), "#ffff00");
    assert_eq!(g.at(2.).to_hex_string(), "#00ff00");

    // Custom color position #1
    let g = CustomGradient::new()
        .html_colors(&["yellow", "blue", "lime"])
        .domain(&[0., 0.75, 1.])
        .build()
        .unwrap();
    assert_eq!(g.at(0.).to_hex_string(), "#ffff00");
    assert_eq!(g.at(0.75).to_hex_string(), "#0000ff");
    assert_eq!(g.at(1.).to_hex_string(), "#00ff00");

    assert_eq!(g.at(-10.).to_hex_string(), "#ffff00");
    assert_eq!(g.at(110.).to_hex_string(), "#00ff00");

    // Custom color position #2
    let g = CustomGradient::new()
        .html_colors(&["yellow", "blue", "lime", "red"])
        .domain(&[15., 25., 29., 63.])
        .build()
        .unwrap();
    assert_eq!(g.at(15.).to_hex_string(), "#ffff00");
    assert_eq!(g.at(25.).to_hex_string(), "#0000ff");
    assert_eq!(g.at(29.).to_hex_string(), "#00ff00");
    assert_eq!(g.at(63.).to_hex_string(), "#ff0000");

    assert_eq!(g.at(10.).to_hex_string(), "#ffff00");
    assert_eq!(g.at(64.).to_hex_string(), "#ff0000");
}

#[test]
fn sharp_gradient() {
    let grad = CustomGradient::new()
        .html_colors(&["red", "lime", "blue"])
        .build()
        .unwrap();

    let g0 = grad.sharp(0, 0.);
    assert_eq!(g0.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g0.at(0.5).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g0.at(0.1).rgba_u8(), (255, 0, 0, 255));

    let g1 = grad.sharp(1, 0.);
    assert_eq!(g1.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g1.at(0.5).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g1.at(0.1).rgba_u8(), (255, 0, 0, 255));

    let g3 = grad.sharp(3, 0.);
    assert_eq!(g3.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g3.at(0.1).rgba_u8(), (255, 0, 0, 255));

    assert_eq!(g3.at(0.4).rgba_u8(), (0, 255, 0, 255));
    assert_eq!(g3.at(0.5).rgba_u8(), (0, 255, 0, 255));
    assert_eq!(g3.at(0.6).rgba_u8(), (0, 255, 0, 255));

    assert_eq!(g3.at(0.9).rgba_u8(), (0, 0, 255, 255));
    assert_eq!(g3.at(1.0).rgba_u8(), (0, 0, 255, 255));

    assert_eq!(g3.at(-0.1).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g3.at(1.1).rgba_u8(), (0, 0, 255, 255));
    assert_eq!(g3.at(f64::NAN).rgba_u8(), (255, 0, 0, 255));

    let grad = CustomGradient::new()
        .html_colors(&["red", "lime", "blue"])
        .domain(&[-1., 1.])
        .build()
        .unwrap();

    let g2 = grad.sharp(2, 0.);
    assert_eq!(g2.at(-1.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g2.at(-0.5).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g2.at(-0.1).rgba_u8(), (255, 0, 0, 255));

    assert_eq!(g2.at(0.1).rgba_u8(), (0, 0, 255, 255));
    assert_eq!(g2.at(0.5).rgba_u8(), (0, 0, 255, 255));
    assert_eq!(g2.at(1.0).rgba_u8(), (0, 0, 255, 255));
}

#[test]
fn sharp_gradient_x() {
    let g = CustomGradient::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .build()
        .unwrap();

    let g0 = g.sharp(0, 0.1);
    assert_eq!(g0.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g0.at(0.5).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g0.at(1.0).rgba_u8(), (255, 0, 0, 255));

    let g1 = g.sharp(1, 0.1);
    assert_eq!(g1.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g1.at(0.5).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g1.at(1.0).rgba_u8(), (255, 0, 0, 255));

    let g = g.sharp(3, 0.1);
    assert_eq!(g.at(0.0).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g.at(0.1).rgba_u8(), (255, 0, 0, 255));

    assert_eq!(g.at(1. / 3.).rgba_u8(), (128, 128, 0, 255));

    assert_eq!(g.at(0.45).rgba_u8(), (0, 255, 0, 255));
    assert_eq!(g.at(0.50).rgba_u8(), (0, 255, 0, 255));
    assert_eq!(g.at(0.55).rgba_u8(), (0, 255, 0, 255));

    assert_eq!(g.at(1. / 3. * 2.).rgba_u8(), (0, 128, 128, 255));

    assert_eq!(g.at(0.9).rgba_u8(), (0, 0, 255, 255));
    assert_eq!(g.at(1.0).rgba_u8(), (0, 0, 255, 255));

    assert_eq!(g.at(-0.5).rgba_u8(), (255, 0, 0, 255));
    assert_eq!(g.at(1.5).rgba_u8(), (0, 0, 255, 255));
    assert_eq!(g.at(f64::NAN).rgba_u8(), (255, 0, 0, 255));
}

#[test]
fn colors() {
    let g = CustomGradient::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .build()
        .unwrap();

    let colors0 = g.colors(0);
    assert_eq!(colors0.len(), 0);

    let colors1 = g.colors(1);
    assert_eq!(colors1.len(), 1);
    assert_eq!(colors1[0].to_hex_string(), "#ff0000");

    let colors2 = g.colors(2);
    assert_eq!(colors2.len(), 2);
    assert_eq!(colors2[0].to_hex_string(), "#ff0000");
    assert_eq!(colors2[1].to_hex_string(), "#0000ff");

    let colors3 = g.colors(3);
    assert_eq!(colors3.len(), 3);
    assert_eq!(colors3[0].to_hex_string(), "#ff0000");
    assert_eq!(colors3[1].to_hex_string(), "#00ff00");
    assert_eq!(colors3[2].to_hex_string(), "#0000ff");

    let g = CustomGradient::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .domain(&[-1., 1.])
        .build()
        .unwrap();

    let colors5 = g.colors(5);
    assert_eq!(colors5.len(), 5);
    assert_eq!(colors5[0].to_hex_string(), "#ff0000");
    assert_eq!(colors5[1].to_hex_string(), "#808000");
    assert_eq!(colors5[2].to_hex_string(), "#00ff00");
    assert_eq!(colors5[3].to_hex_string(), "#008080");
    assert_eq!(colors5[4].to_hex_string(), "#0000ff");
}

#[test]
fn preset() {
    let g = colorgrad::viridis();
    assert_eq!(g.at(0.0).to_hex_string(), "#440154");
    assert_eq!(g.at(0.5).to_hex_string(), "#27838e");
    assert_eq!(g.at(1.0).to_hex_string(), "#fee825");
    assert_eq!(g.at(f64::NAN).to_hex_string(), "#000000");

    let g = colorgrad::greys();
    assert_eq!(g.at(0.).to_hex_string(), "#ffffff");
    assert_eq!(g.at(1.).to_hex_string(), "#000000");

    let g = colorgrad::turbo();
    assert_eq!(g.at(0.).to_hex_string(), "#23171b");
    assert_eq!(g.at(1.).to_hex_string(), "#900c00");

    let g = colorgrad::cividis();
    assert_eq!(g.at(0.).to_hex_string(), "#002051");
    assert_eq!(g.at(1.).to_hex_string(), "#fdea45");

    let g = colorgrad::cubehelix_default();
    assert_eq!(g.at(0.).to_hex_string(), "#000000");
    assert_eq!(g.at(1.).to_hex_string(), "#ffffff");

    let g = colorgrad::warm();
    assert_eq!(g.at(0.).to_hex_string(), "#6e40aa");
    assert_eq!(g.at(1.).to_hex_string(), "#aff05b");

    let g = colorgrad::cool();
    assert_eq!(g.at(0.).to_hex_string(), "#6e40aa");
    assert_eq!(g.at(1.).to_hex_string(), "#aff05b");
}

#[test]
fn cyclic() {
    let g = colorgrad::rainbow();
    assert_eq!(g.at(0.).rgba_u8(), g.at(1.).rgba_u8());

    let g = colorgrad::sinebow();
    assert_eq!(g.at(0.).rgba_u8(), g.at(1.).rgba_u8());
}
