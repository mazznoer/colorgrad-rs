use colorgrad::{BlendMode, Color, Gradient, GradientBuilder, LinearGradient};

mod utils;
use utils::*;

#[test]
fn spread_inside_domain() {
    macro_rules! cmp_rgba8 {
        ($a:expr, $b:expr) => {
            assert_eq!($a.to_rgba8(), $b.to_rgba8());
        };
    }

    let g = GradientBuilder::new()
        .html_colors(&["#00f", "#fff"])
        .build::<LinearGradient>()
        .unwrap();

    cmp_rgba8!(g.at(0.0), g.repeat_at(0.0));
    cmp_rgba8!(g.at(0.0), g.reflect_at(0.0));

    cmp_rgba8!(g.at(0.01), g.repeat_at(0.01));
    cmp_rgba8!(g.at(0.01), g.reflect_at(0.01));

    cmp_rgba8!(g.at(0.25), g.repeat_at(0.25));
    cmp_rgba8!(g.at(0.25), g.reflect_at(0.25));

    cmp_rgba8!(g.at(0.5), g.repeat_at(0.5));
    cmp_rgba8!(g.at(0.5), g.reflect_at(0.5));

    cmp_rgba8!(g.at(0.75), g.repeat_at(0.75));
    cmp_rgba8!(g.at(0.75), g.reflect_at(0.75));

    cmp_rgba8!(g.at(0.999), g.repeat_at(0.999));
    cmp_rgba8!(g.at(0.999), g.reflect_at(0.999));
}

#[test]
fn spread_repeat() {
    let g = GradientBuilder::new()
        .html_colors(&["#000", "#fff"])
        .build::<LinearGradient>()
        .unwrap();

    cmp_hex!(g.repeat_at(-2.0), "#000000");
    cmp_hex!(g.repeat_at(-1.9), "#1a1a1a");
    cmp_hex!(g.repeat_at(-1.5), "#808080");
    cmp_hex!(g.repeat_at(-1.1), "#e6e6e6");

    cmp_hex!(g.repeat_at(-1.0), "#000000");
    cmp_hex!(g.repeat_at(-0.9), "#1a1a1a");
    cmp_hex!(g.repeat_at(-0.5), "#808080");
    cmp_hex!(g.repeat_at(-0.1), "#e6e6e6");

    cmp_hex!(g.repeat_at(0.0), "#000000");
    cmp_hex!(g.repeat_at(0.1), "#1a1a1a");
    cmp_hex!(g.repeat_at(0.5), "#808080");
    cmp_hex!(g.repeat_at(0.9), "#e6e6e6");

    cmp_hex!(g.repeat_at(1.0), "#000000");
    cmp_hex!(g.repeat_at(1.1), "#1a1a1a");
    cmp_hex!(g.repeat_at(1.5), "#808080");
    cmp_hex!(g.repeat_at(1.9), "#e6e6e6");

    cmp_hex!(g.repeat_at(2.0), "#000000");
    cmp_hex!(g.repeat_at(2.1), "#191919");
    cmp_hex!(g.repeat_at(2.5), "#808080");
    cmp_hex!(g.repeat_at(2.9), "#e6e6e6");
}

#[test]
fn spread_reflect() {
    let g = GradientBuilder::new()
        .html_colors(&["#000", "#fff"])
        .build::<LinearGradient>()
        .unwrap();

    cmp_hex!(g.reflect_at(-2.0), "#000000");
    cmp_hex!(g.reflect_at(-1.9), "#1a1a1a");
    cmp_hex!(g.reflect_at(-1.5), "#808080");
    cmp_hex!(g.reflect_at(-1.1), "#e6e6e6");

    cmp_hex!(g.reflect_at(-1.0), "#ffffff");
    cmp_hex!(g.reflect_at(-0.9), "#e6e6e6");
    cmp_hex!(g.reflect_at(-0.5), "#808080");
    cmp_hex!(g.reflect_at(-0.1), "#191919");

    cmp_hex!(g.reflect_at(0.0), "#000000");
    cmp_hex!(g.reflect_at(0.1), "#191919");
    cmp_hex!(g.reflect_at(0.5), "#808080");
    cmp_hex!(g.reflect_at(0.9), "#e6e6e6");

    cmp_hex!(g.reflect_at(1.0), "#ffffff");
    cmp_hex!(g.reflect_at(1.1), "#e6e6e6");
    cmp_hex!(g.reflect_at(1.5), "#808080");
    cmp_hex!(g.reflect_at(1.9), "#191919");

    cmp_hex!(g.reflect_at(2.0), "#000000");
    cmp_hex!(g.reflect_at(2.1), "#191919");
    cmp_hex!(g.reflect_at(2.5), "#808080");
    cmp_hex!(g.reflect_at(2.9), "#e6e6e6");
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
fn colors_iter() {
    fn hex(c: Color) -> String {
        c.to_css_hex()
    }

    macro_rules! cmp {
        ($a:expr, $b:expr) => {
            assert_eq!($a.map(hex).as_deref(), Some($b));
        };
    }

    let g = GradientBuilder::new()
        .html_colors(&["#000", "#f00", "#ff0", "#fff"])
        .mode(BlendMode::Rgb)
        .build::<LinearGradient>()
        .unwrap();

    let mut it = g.colors_iter(0);
    assert_eq!(it.len(), 0);
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    let mut it = g.colors_iter(1);
    cmp!(it.next(), "#000000");
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    let mut it = g.colors_iter(1);
    cmp!(it.next_back(), "#000000");
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    let mut it = g.colors_iter(2);
    cmp!(it.next(), "#000000");
    cmp!(it.next(), "#ffffff");
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    let mut it = g.colors_iter(4);
    cmp!(it.next(), "#000000");
    cmp!(it.next(), "#ff0000");
    cmp!(it.next(), "#ffff00");
    cmp!(it.next(), "#ffffff");
    assert_eq!(it.len(), 0);
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    let mut it = g.colors_iter(4);
    cmp!(it.next_back(), "#ffffff");
    cmp!(it.next_back(), "#ffff00");
    cmp!(it.next_back(), "#ff0000");
    cmp!(it.next_back(), "#000000");
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    let mut it = g.colors_iter(4);
    assert_eq!(it.len(), 4);
    cmp!(it.next(), "#000000");
    assert_eq!(it.len(), 3);
    cmp!(it.next_back(), "#ffffff");
    assert_eq!(it.len(), 2);
    cmp!(it.next(), "#ff0000");
    assert_eq!(it.len(), 1);
    cmp!(it.next_back(), "#ffff00");
    assert_eq!(it.len(), 0);
    assert_eq!(it.next(), None);
    assert_eq!(it.next_back(), None);

    let mut it = g.colors_iter(999);
    cmp!(it.next(), "#000000");
    cmp!(it.next_back(), "#ffffff");
    assert_eq!(it.len(), 997);
    assert_eq!(it.count(), 997);

    let colors: Vec<_> = g.colors_iter(73).collect();
    assert_eq!(colors.len(), 73);

    // reverse
    let mut it = g.colors_iter(4).rev();
    cmp!(it.next(), "#ffffff");
    cmp!(it.next(), "#ffff00");
    cmp!(it.next(), "#ff0000");
    cmp!(it.next(), "#000000");
    assert_eq!(it.len(), 0);

    // compare with Gradient::colors()
    let colors: Vec<_> = g.colors_iter(27).collect();
    assert_eq!(g.colors(27), colors);

    // --- Custom gradient domain

    let g = GradientBuilder::new()
        .html_colors(&["#f00", "#0f0", "#00f"])
        .domain(&[-5.0, 17.0])
        .mode(BlendMode::Rgb)
        .build::<LinearGradient>()
        .unwrap();

    let mut it = g.colors_iter(3);
    cmp!(it.next(), "#ff0000");
    cmp!(it.next(), "#00ff00");
    cmp!(it.next(), "#0000ff");
    assert_eq!(it.next(), None);

    // reverse
    let mut it = g.colors_iter(3).rev();
    cmp!(it.next(), "#0000ff");
    cmp!(it.next(), "#00ff00");
    cmp!(it.next(), "#ff0000");
    assert_eq!(it.next(), None);

    // compare with Gradient::colors()
    let colors: Vec<_> = g.colors_iter(10).collect();
    assert_eq!(g.colors(10), colors);
}

#[test]
fn boxed_gradients() {
    let gradient = GradientBuilder::new()
        .html_colors(&["#fff", "#000"])
        .build::<LinearGradient>()
        .unwrap()
        .boxed();

    assert_eq!(gradient.at(0.0).to_rgba8(), [255, 255, 255, 255]);
    assert_eq!(gradient.repeat_at(1.25).to_rgba8(), [191, 191, 191, 255]);
    assert_eq!(gradient.reflect_at(1.25).to_rgba8(), [64, 64, 64, 255]);
    assert_eq!(gradient.domain(), (0.0, 1.0));
    assert_eq!(gradient.colors(3).len(), 3);
    assert_eq!(gradient.sharp(3, 0.0).colors(3).len(), 3);
}

#[test]
fn others() {
    let gd: Box<dyn Gradient> = Box::new(GradientBuilder::new().build::<LinearGradient>().unwrap());
    let _: Box<dyn Gradient> = gd.clone();
    let _ = gd.clone().boxed();
    let _ = gd.inverse();
    let _ = gd.inverse().boxed();
    let _ = gd.clone_boxed();

    let gd: &dyn Gradient = &GradientBuilder::new().build::<LinearGradient>().unwrap();
    let _ = gd.inverse();
    let _ = gd.inverse().boxed();
    let _ = gd.clone_boxed();
}

#[test]
fn impl_gradient() {
    // Default domain

    #[derive(Clone)]
    struct MyGradient1 {}

    impl Gradient for MyGradient1 {
        fn at(&self, t: f32) -> Color {
            if t < 0.5 {
                Color::new(0.0, 0.0, 1.0, 1.0)
            } else {
                Color::new(1.0, 0.0, 0.0, 1.0)
            }
        }
    }

    let g = MyGradient1 {};
    assert_eq!(g.domain(), (0.0, 1.0));
    cmp_hex!(g.at(0.00), "#0000ff");
    cmp_hex!(g.at(0.49), "#0000ff");
    cmp_hex!(g.at(0.51), "#ff0000");
    cmp_hex!(g.at(1.00), "#ff0000");

    // Custom domain

    #[derive(Clone)]
    struct MyGradient2 {}

    impl Gradient for MyGradient2 {
        fn at(&self, t: f32) -> Color {
            if (t as usize / 10) & 1 == 0 {
                Color::new(0.0, 0.0, 1.0, 1.0)
            } else {
                Color::new(1.0, 0.0, 0.0, 1.0)
            }
        }

        fn domain(&self) -> (f32, f32) {
            (1.0, 99.0)
        }
    }

    let g = MyGradient2 {};
    assert_eq!(g.domain(), (1.0, 99.0));
    cmp_hex!(g.at(25.0), "#0000ff");
    assert_eq!(
        colors2hex(&g.colors(10)),
        &[
            "#0000ff", "#ff0000", "#0000ff", "#ff0000", "#0000ff", "#ff0000", "#0000ff", "#ff0000",
            "#0000ff", "#ff0000",
        ]
    );
}
