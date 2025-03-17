use colorgrad::Gradient;

mod utils;
use utils::*;

#[test]
fn inverse() {
    let grad = colorgrad::GradientBuilder::new()
        .html_colors(&["#000", "#fff"])
        .build::<colorgrad::LinearGradient>()
        .unwrap();

    let inv = grad.inverse();

    assert_eq!(grad.domain(), (0.0, 1.0));
    assert_eq!(inv.domain(), (0.0, 1.0));

    assert_eq!(inv.at(0.0).to_rgba8(), grad.at(1.0).to_rgba8());
    assert_eq!(inv.at(0.3).to_rgba8(), grad.at(0.7).to_rgba8());
    assert_eq!(inv.at(0.5).to_rgba8(), grad.at(0.5).to_rgba8());
    assert_eq!(inv.at(0.7).to_rgba8(), grad.at(0.3).to_rgba8());
    assert_eq!(inv.at(1.0).to_rgba8(), grad.at(0.0).to_rgba8());

    assert_eq!(inv.repeat_at(-0.9).to_hex_string(), "#e6e6e6");
    assert_eq!(inv.repeat_at(1.1).to_hex_string(), "#e6e6e6");

    assert_eq!(inv.reflect_at(-0.9).to_hex_string(), "#191919");
    assert_eq!(inv.reflect_at(1.1).to_hex_string(), "#191919");

    assert_eq!(
        colors2hex(&grad.colors(5)),
        &["#000000", "#404040", "#808080", "#bfbfbf", "#ffffff"]
    );

    assert_eq!(
        colors2hex(&inv.colors(5)),
        &["#ffffff", "#bfbfbf", "#808080", "#404040", "#000000"]
    );

    // Custom domain

    let grad = colorgrad::GradientBuilder::new()
        .html_colors(&["#000", "#fff"])
        .domain(&[-100.0, 100.0])
        .build::<colorgrad::LinearGradient>()
        .unwrap();

    let inv = grad.inverse();

    assert_eq!(grad.domain(), (-100.0, 100.0));
    assert_eq!(inv.domain(), (-100.0, 100.0));

    assert_eq!(inv.at(-100.0).to_rgba8(), grad.at(100.0).to_rgba8());
    assert_eq!(inv.at(-50.0).to_rgba8(), grad.at(50.0).to_rgba8());
    assert_eq!(inv.at(0.0).to_rgba8(), grad.at(0.0).to_rgba8());
    assert_eq!(inv.at(50.0).to_rgba8(), grad.at(-50.0).to_rgba8());
    assert_eq!(inv.at(100.0).to_rgba8(), grad.at(-100.0).to_rgba8());

    assert_eq!(
        colors2hex(&grad.colors(5)),
        &["#000000", "#404040", "#808080", "#bfbfbf", "#ffffff"]
    );

    assert_eq!(
        colors2hex(&inv.colors(5)),
        &["#ffffff", "#bfbfbf", "#808080", "#404040", "#000000"]
    );
}
