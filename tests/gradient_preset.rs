use colorgrad::Gradient;

#[cfg(feature = "preset")]
#[test]
fn preset() {
    let g = colorgrad::preset::viridis();
    assert_eq!(g.at(0.0).to_hex_string(), "#440154");
    assert_eq!(g.at(0.5).to_hex_string(), "#27838e");
    assert_eq!(g.at(1.0).to_hex_string(), "#fee825");
    assert_eq!(g.at(f32::NAN).to_hex_string(), "#000000");

    let g = colorgrad::preset::greys();
    assert_eq!(g.at(0.0).to_hex_string(), "#ffffff");
    assert_eq!(g.at(1.0).to_hex_string(), "#000000");

    let g = colorgrad::preset::turbo();
    assert_eq!(g.at(0.0).to_hex_string(), "#23171b");
    assert_eq!(g.at(1.0).to_hex_string(), "#900c00");

    let g = colorgrad::preset::cividis();
    assert_eq!(g.at(0.0).to_hex_string(), "#002051");
    assert_eq!(g.at(1.0).to_hex_string(), "#fdea45");

    let g = colorgrad::preset::cubehelix_default();
    assert_eq!(g.at(0.0).to_hex_string(), "#000000");
    assert_eq!(g.at(1.0).to_hex_string(), "#ffffff");

    let g = colorgrad::preset::warm();
    assert_eq!(g.at(0.0).to_hex_string(), "#6e40aa");
    assert_eq!(g.at(1.0).to_hex_string(), "#aff05b");

    let g = colorgrad::preset::cool();
    assert_eq!(g.at(0.0).to_hex_string(), "#6e40aa");
    assert_eq!(g.at(1.0).to_hex_string(), "#aff05b");
}

#[cfg(feature = "preset")]
#[test]
fn cyclic() {
    let g = colorgrad::preset::rainbow();
    assert_eq!(g.at(0.0).to_rgba8(), g.at(1.0).to_rgba8());

    let g = colorgrad::preset::sinebow();
    assert_eq!(g.at(0.0).to_rgba8(), g.at(1.0).to_rgba8());
}
