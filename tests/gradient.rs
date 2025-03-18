use colorgrad::{Gradient, GradientBuilder, LinearGradient};

mod utils;
use utils::*;

#[test]
fn spread_inside_domain() {
    let g = GradientBuilder::new()
        .html_colors(&["#00f", "#fff"])
        .build::<LinearGradient>()
        .unwrap();

    assert_eq!(g.at(0.0).to_rgba8(), g.repeat_at(0.0).to_rgba8());
    assert_eq!(g.at(0.0).to_rgba8(), g.reflect_at(0.0).to_rgba8());

    assert_eq!(g.at(0.01).to_rgba8(), g.repeat_at(0.01).to_rgba8());
    assert_eq!(g.at(0.01).to_rgba8(), g.reflect_at(0.01).to_rgba8());

    assert_eq!(g.at(0.25).to_rgba8(), g.repeat_at(0.25).to_rgba8());
    assert_eq!(g.at(0.25).to_rgba8(), g.reflect_at(0.25).to_rgba8());

    assert_eq!(g.at(0.5).to_rgba8(), g.repeat_at(0.5).to_rgba8());
    assert_eq!(g.at(0.5).to_rgba8(), g.reflect_at(0.5).to_rgba8());

    assert_eq!(g.at(0.75).to_rgba8(), g.repeat_at(0.75).to_rgba8());
    assert_eq!(g.at(0.75).to_rgba8(), g.reflect_at(0.75).to_rgba8());

    assert_eq!(g.at(0.999).to_rgba8(), g.repeat_at(0.999).to_rgba8());
    assert_eq!(g.at(0.999).to_rgba8(), g.reflect_at(0.999).to_rgba8());
}

#[test]
fn spread_repeat() {
    let g = GradientBuilder::new()
        .html_colors(&["#000", "#fff"])
        .build::<LinearGradient>()
        .unwrap();

    assert_eq!(g.repeat_at(-2.0).to_hex_string(), "#000000");
    assert_eq!(g.repeat_at(-1.9).to_hex_string(), "#1a1a1a");
    assert_eq!(g.repeat_at(-1.5).to_hex_string(), "#808080");
    assert_eq!(g.repeat_at(-1.1).to_hex_string(), "#e6e6e6");

    assert_eq!(g.repeat_at(-1.0).to_hex_string(), "#000000");
    assert_eq!(g.repeat_at(-0.9).to_hex_string(), "#1a1a1a");
    assert_eq!(g.repeat_at(-0.5).to_hex_string(), "#808080");
    assert_eq!(g.repeat_at(-0.1).to_hex_string(), "#e6e6e6");

    assert_eq!(g.repeat_at(0.0).to_hex_string(), "#000000");
    assert_eq!(g.repeat_at(0.1).to_hex_string(), "#1a1a1a");
    assert_eq!(g.repeat_at(0.5).to_hex_string(), "#808080");
    assert_eq!(g.repeat_at(0.9).to_hex_string(), "#e6e6e6");

    assert_eq!(g.repeat_at(1.0).to_hex_string(), "#000000");
    assert_eq!(g.repeat_at(1.1).to_hex_string(), "#1a1a1a");
    assert_eq!(g.repeat_at(1.5).to_hex_string(), "#808080");
    assert_eq!(g.repeat_at(1.9).to_hex_string(), "#e6e6e6");

    assert_eq!(g.repeat_at(2.0).to_hex_string(), "#000000");
    assert_eq!(g.repeat_at(2.1).to_hex_string(), "#191919");
    assert_eq!(g.repeat_at(2.5).to_hex_string(), "#808080");
    assert_eq!(g.repeat_at(2.9).to_hex_string(), "#e6e6e6");
}

#[test]
fn spread_reflect() {
    let g = GradientBuilder::new()
        .html_colors(&["#000", "#fff"])
        .build::<LinearGradient>()
        .unwrap();

    assert_eq!(g.reflect_at(-2.0).to_hex_string(), "#000000");
    assert_eq!(g.reflect_at(-1.9).to_hex_string(), "#1a1a1a");
    assert_eq!(g.reflect_at(-1.5).to_hex_string(), "#808080");
    assert_eq!(g.reflect_at(-1.1).to_hex_string(), "#e6e6e6");

    assert_eq!(g.reflect_at(-1.0).to_hex_string(), "#ffffff");
    assert_eq!(g.reflect_at(-0.9).to_hex_string(), "#e6e6e6");
    assert_eq!(g.reflect_at(-0.5).to_hex_string(), "#808080");
    assert_eq!(g.reflect_at(-0.1).to_hex_string(), "#191919");

    assert_eq!(g.reflect_at(0.0).to_hex_string(), "#000000");
    assert_eq!(g.reflect_at(0.1).to_hex_string(), "#191919");
    assert_eq!(g.reflect_at(0.5).to_hex_string(), "#808080");
    assert_eq!(g.reflect_at(0.9).to_hex_string(), "#e6e6e6");

    assert_eq!(g.reflect_at(1.0).to_hex_string(), "#ffffff");
    assert_eq!(g.reflect_at(1.1).to_hex_string(), "#e6e6e6");
    assert_eq!(g.reflect_at(1.5).to_hex_string(), "#808080");
    assert_eq!(g.reflect_at(1.9).to_hex_string(), "#191919");

    assert_eq!(g.reflect_at(2.0).to_hex_string(), "#000000");
    assert_eq!(g.reflect_at(2.1).to_hex_string(), "#191919");
    assert_eq!(g.reflect_at(2.5).to_hex_string(), "#808080");
    assert_eq!(g.reflect_at(2.9).to_hex_string(), "#e6e6e6");
}

#[test]
fn colors() {
    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .build::<LinearGradient>()
        .unwrap();

    assert_eq!(g.domain(), (0.0, 1.0));

    assert_eq!(g.colors(0).len(), 0);

    assert_eq!(colors2hex(&g.colors(1)), &["#ff0000",]);

    assert_eq!(colors2hex(&g.colors(2)), &["#ff0000", "#0000ff",]);

    assert_eq!(
        colors2hex(&g.colors(3)),
        &["#ff0000", "#00ff00", "#0000ff",]
    );

    assert_eq!(
        colors2hex(&g.colors(5)),
        &["#ff0000", "#808000", "#00ff00", "#008080", "#0000ff",]
    );

    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .domain(&[-1.0, 1.0])
        .build::<LinearGradient>()
        .unwrap();

    assert_eq!(g.domain(), (-1.0, 1.0));

    assert_eq!(
        colors2hex(&g.colors(3)),
        &["#ff0000", "#00ff00", "#0000ff",]
    );

    assert_eq!(
        colors2hex(&g.colors(5)),
        &["#ff0000", "#808000", "#00ff00", "#008080", "#0000ff",]
    );
}

#[test]
fn box_clone() {
    let g: Box<dyn Gradient> = Box::new(GradientBuilder::new().build::<LinearGradient>().unwrap());
    //let _: Box<dyn Gradient> = g.clone();
}
