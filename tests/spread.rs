use colorgrad::CustomGradient;

#[test]
fn spread_inside_domain() {
    let g = colorgrad::blues();

    assert_eq!(g.at(0.0).rgba_u8(), g.repeat_at(0.0).rgba_u8());
    assert_eq!(g.at(0.0).rgba_u8(), g.reflect_at(0.0).rgba_u8());

    assert_eq!(g.at(0.01).rgba_u8(), g.repeat_at(0.01).rgba_u8());
    assert_eq!(g.at(0.01).rgba_u8(), g.reflect_at(0.01).rgba_u8());

    assert_eq!(g.at(0.25).rgba_u8(), g.repeat_at(0.25).rgba_u8());
    assert_eq!(g.at(0.25).rgba_u8(), g.reflect_at(0.25).rgba_u8());

    assert_eq!(g.at(0.5).rgba_u8(), g.repeat_at(0.5).rgba_u8());
    assert_eq!(g.at(0.5).rgba_u8(), g.reflect_at(0.5).rgba_u8());

    assert_eq!(g.at(0.75).rgba_u8(), g.repeat_at(0.75).rgba_u8());
    assert_eq!(g.at(0.75).rgba_u8(), g.reflect_at(0.75).rgba_u8());

    assert_eq!(g.at(0.999).rgba_u8(), g.repeat_at(0.999).rgba_u8());
    assert_eq!(g.at(0.999).rgba_u8(), g.reflect_at(0.999).rgba_u8());
}

#[test]
fn spread_repeat() {
    let g = CustomGradient::new()
        .html_colors(&["#000", "#fff"])
        .build()
        .unwrap();

    assert_eq!(g.repeat_at(-2.0).to_hex_string(), "#000000");
    assert_eq!(g.repeat_at(-1.9).to_hex_string(), "#1a1a1a");
    assert_eq!(g.repeat_at(-1.5).to_hex_string(), "#808080");
    assert_eq!(g.repeat_at(-1.1).to_hex_string(), "#e5e5e5");

    assert_eq!(g.repeat_at(-1.0).to_hex_string(), "#000000");
    assert_eq!(g.repeat_at(-0.9).to_hex_string(), "#191919");
    assert_eq!(g.repeat_at(-0.5).to_hex_string(), "#808080");
    assert_eq!(g.repeat_at(-0.1).to_hex_string(), "#e6e6e6");

    assert_eq!(g.repeat_at(0.0).to_hex_string(), "#000000");
    assert_eq!(g.repeat_at(0.1).to_hex_string(), "#1a1a1a");
    assert_eq!(g.repeat_at(0.5).to_hex_string(), "#808080");
    assert_eq!(g.repeat_at(0.9).to_hex_string(), "#e5e5e5");

    assert_eq!(g.repeat_at(1.0).to_hex_string(), "#000000");
    assert_eq!(g.repeat_at(1.1).to_hex_string(), "#1a1a1a");
    assert_eq!(g.repeat_at(1.5).to_hex_string(), "#808080");
    assert_eq!(g.repeat_at(1.9).to_hex_string(), "#e5e5e5");

    assert_eq!(g.repeat_at(2.0).to_hex_string(), "#000000");
    assert_eq!(g.repeat_at(2.1).to_hex_string(), "#1a1a1a");
    assert_eq!(g.repeat_at(2.5).to_hex_string(), "#808080");
    assert_eq!(g.repeat_at(2.9).to_hex_string(), "#e5e5e5");
}

#[test]
fn spread_reflect() {
    let g = CustomGradient::new()
        .html_colors(&["#000", "#fff"])
        .build()
        .unwrap();

    assert_eq!(g.reflect_at(-2.0).to_hex_string(), "#000000");
    assert_eq!(g.reflect_at(-1.9).to_hex_string(), "#1a1a1a");
    assert_eq!(g.reflect_at(-1.5).to_hex_string(), "#808080");
    assert_eq!(g.reflect_at(-1.1).to_hex_string(), "#e5e5e5");

    assert_eq!(g.reflect_at(-1.0).to_hex_string(), "#ffffff");
    assert_eq!(g.reflect_at(-0.9).to_hex_string(), "#e5e5e5");
    assert_eq!(g.reflect_at(-0.5).to_hex_string(), "#808080");
    assert_eq!(g.reflect_at(-0.1).to_hex_string(), "#1a1a1a");

    assert_eq!(g.reflect_at(0.0).to_hex_string(), "#000000");
    assert_eq!(g.reflect_at(0.1).to_hex_string(), "#1a1a1a");
    assert_eq!(g.reflect_at(0.5).to_hex_string(), "#808080");
    assert_eq!(g.reflect_at(0.9).to_hex_string(), "#e5e5e5");

    assert_eq!(g.reflect_at(1.0).to_hex_string(), "#ffffff");
    assert_eq!(g.reflect_at(1.1).to_hex_string(), "#e5e5e5");
    assert_eq!(g.reflect_at(1.5).to_hex_string(), "#808080");
    assert_eq!(g.reflect_at(1.9).to_hex_string(), "#1a1a1a");

    assert_eq!(g.reflect_at(2.0).to_hex_string(), "#000000");
    assert_eq!(g.reflect_at(2.1).to_hex_string(), "#1a1a1a");
    assert_eq!(g.reflect_at(2.5).to_hex_string(), "#808080");
    assert_eq!(g.reflect_at(2.9).to_hex_string(), "#e5e5e5");
}
