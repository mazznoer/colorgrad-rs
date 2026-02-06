use colorgrad::Gradient;

mod utils;
use utils::*;

#[test]
fn inverse() {
    macro_rules! cmp_rgba8 {
        ($a:expr, $b:expr) => {
            assert_eq!($a.to_rgba8(), $b.to_rgba8());
        };
    }

    let grad = colorgrad::GradientBuilder::new()
        .html_colors(&["#000", "#fff"])
        .build::<colorgrad::LinearGradient>()
        .unwrap();

    let inv = grad.inverse();

    assert_eq!(grad.domain(), (0.0, 1.0));
    assert_eq!(inv.domain(), (0.0, 1.0));

    cmp_rgba8!(inv.at(0.0), grad.at(1.0));
    cmp_rgba8!(inv.at(0.3), grad.at(0.7));
    cmp_rgba8!(inv.at(0.5), grad.at(0.5));
    cmp_rgba8!(inv.at(0.7), grad.at(0.3));
    cmp_rgba8!(inv.at(1.0), grad.at(0.0));

    cmp_hex!(inv.repeat_at(-0.9), "#e6e6e6");
    cmp_hex!(inv.repeat_at(1.1), "#e6e6e6");

    cmp_hex!(inv.reflect_at(-0.9), "#191919");
    cmp_hex!(inv.reflect_at(1.1), "#191919");

    assert_eq!(
        colors2hex(grad.colors(5)),
        &["#000000", "#404040", "#808080", "#bfbfbf", "#ffffff"]
    );

    assert_eq!(
        colors2hex(inv.colors(5)),
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

    cmp_rgba8!(inv.at(-100.0), grad.at(100.0));
    cmp_rgba8!(inv.at(-50.0), grad.at(50.0));
    cmp_rgba8!(inv.at(0.0), grad.at(0.0));
    cmp_rgba8!(inv.at(50.0), grad.at(-50.0));
    cmp_rgba8!(inv.at(100.0), grad.at(-100.0));

    assert_eq!(
        colors2hex(grad.colors(5)),
        &["#000000", "#404040", "#808080", "#bfbfbf", "#ffffff"]
    );

    assert_eq!(
        colors2hex(inv.colors(5)),
        &["#ffffff", "#bfbfbf", "#808080", "#404040", "#000000"]
    );
}
