use colorgrad::{BlendMode, Gradient, GradientBuilder, LinearGradient};

mod utils;
use utils::*;

#[test]
fn helper() {
    let threshold = 0.005;

    // test helper functions

    let stops = [
        (0.0, [1.0, 0.0, 0.0, 1.0]),
        (0.5, [0.0, 1.0, 0.0, 1.0]),
        (0.5, [0.0, 1.0, 0.0, 1.0]), // duplicate stop
        (1.0, [0.0, 0.0, 1.0, 1.0]),
    ];
    let og = LinearGradient::from_rgba_data(&stops).unwrap();
    assert_eq!(check_linear_stops(&og, &stops, threshold), false);

    let stops = [
        (0.0, [1.0, 0.0, 0.0, 1.0]),
        (0.5, [1.0, 0.0, 0.0, 1.0]), // unnecessary stop
        (1.0, [1.0, 0.0, 0.0, 1.0]),
    ];
    let og = LinearGradient::from_rgba_data(&stops).unwrap();
    assert_eq!(check_linear_stops(&og, &stops, threshold), false);
}

#[test]
fn simple() {
    // LinearGradient + Rgb
    // 2 colors

    let og = GradientBuilder::new()
        .css("#f00, #00f")
        .mode(BlendMode::Rgb)
        .build::<LinearGradient>()
        .unwrap();

    let threshold = 0.005;
    let lg = og.linearize(threshold);

    assert_eq!(lg.stops().len(), 2);
    assert_eq!(lg.mode(), BlendMode::Rgb);

    let expected = ["#ff0000", "#0000ff"];
    assert_eq!(colors2hex(lg.colors(2)), &expected);

    // 4 colors

    let og = GradientBuilder::new()
        .css("#00f, #f00, #fff, #ff0")
        .mode(BlendMode::Rgb)
        .build::<LinearGradient>()
        .unwrap();

    let lg = og.linearize(threshold);
    let stops = lg.stops();

    assert_eq!(stops.len(), 4);

    assert!(check_linear_stops(&og, stops, threshold));

    let colors = ["#0000ff", "#ff0000", "#ffffff", "#ffff00"];
    assert_eq!(colors2hex(lg.colors(4)), &colors);
}

#[test]
fn complex() {
    // 1

    let og = GradientBuilder::new()
        .css("#00f, #f00, #fff, #ff0")
        .mode(BlendMode::Oklab)
        .build::<colorgrad::BasisGradient>()
        .unwrap();

    let threshold = 0.005;
    let lg = og.linearize(threshold);
    let stops = lg.stops();

    assert_eq!(stops.len(), 21);

    /*for (i, (t, rgba)) in stops.iter().enumerate() {
        let c = colorgrad::Color::from(*rgba);
        println!("  {i} {t:.3} {:?}", c.to_rgba8());
    }*/

    assert!(check_linear_stops(&lg, stops, threshold));

    // 2

    let og = GradientBuilder::new()
        .css("#ff1493, #ffd700 67%, #2e8b57")
        .mode(BlendMode::Oklab)
        .build::<colorgrad::SmoothstepGradient>()
        .unwrap();

    let threshold = 0.007;
    let lg = og.linearize(threshold);
    let stops = lg.stops();

    assert!(check_linear_stops(&og, stops, threshold));

    let expected = [
        "#ff1493", "#ff288d", "#ff3c86", "#ff5080", "#ff657a", "#ff7973", "#ff8d6d", "#ffa167",
        "#ffb055", "#ffbe42", "#ffcc30", "#ffd316", "#ffd700", "#efd11b", "#cdc436", "#a1b349",
        "#71a151", "#479256", "#2e8b57",
    ];
    assert_eq!(colors2hex(lg.colors(19)), &expected);

    let g2 = LinearGradient::from_rgba_data(stops).unwrap();
    let colors1 = lg.colors(177);
    let colors2 = g2.colors(177);
    for (a, b) in colors1.zip(colors2) {
        assert_eq!(a.to_rgba8(), b.to_rgba8());
    }
}

// --- Helper functions

// grad -> the original gradient
fn check_linear_stops(grad: &dyn Gradient, stops: &[(f32, [f32; 4])], threshold: f32) -> bool {
    let mut last_idx = 0;
    let mut unnecessary = 0;
    let mut duplicates = 0;

    for i in 1..stops.len() - 1 {
        let t_prev = stops[last_idx].0;
        let t_curr = stops[i].0;

        // duplicate position
        if (t_prev - t_curr).abs() < f32::EPSILON {
            duplicates += 1;
            last_idx = i;
            println!("duplicate @ {i}");
            continue;
        }

        let t_next = stops[i + 1].0;
        let lerp_factor = (t_curr - t_prev) / (t_next - t_prev);

        let color_a = grad.at(t_prev).clamp();
        let color_c = grad.at(t_next).clamp();

        let color_b_actual = grad.at(t_curr).clamp();
        let color_b_linear = color_a.interpolate_rgb(&color_c, lerp_factor);

        let diff = color_diff(color_b_actual, color_b_linear);

        if diff < threshold {
            unnecessary += 1;
            println!("unnecessary @ {i}, diff {diff:?}");
        } else {
            last_idx = i;
        }
    }

    unnecessary == 0 && duplicates == 0
}
