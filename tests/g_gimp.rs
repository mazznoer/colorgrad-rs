use std::io::BufReader;

use colorgrad::{Color, GimpGradient, Gradient};

mod utils;

#[test]
fn parse_gimp_gradients() {
    let col = Color::default();
    let red = Color::new(1.0, 0.0, 0.0, 1.0);
    let blue = Color::new(0.0, 0.0, 1.0, 1.0);

    // Black to white
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 0 0";
    let g = GimpGradient::new(BufReader::new(ggr.as_bytes()), &col, &col).unwrap();

    assert_eq!(g.name(), "My Gradient");
    assert_eq!(g.domain(), (0.0, 1.0));

    cmp_hex!(g.at(0.0), "#000000");
    cmp_hex!(g.at(1.0), "#ffffff");

    cmp_hex!(g.at(-0.1), "#000000");
    cmp_hex!(g.at(1.11), "#000000");
    cmp_hex!(g.at(f32::NEG_INFINITY), "#000000");
    cmp_hex!(g.at(f32::INFINITY), "#000000");
    cmp_hex!(g.at(f32::NAN), "#000000");

    // Foreground to background
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 1 3";
    let g = GimpGradient::new(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    cmp_hex!(g.at(0.0), "#ff0000");
    cmp_hex!(g.at(1.0), "#0000ff");

    // Background to foreground
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 3 1";
    let g = GimpGradient::new(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    cmp_hex!(g.at(0.0), "#0000ff");
    cmp_hex!(g.at(1.0), "#ff0000");

    // Foreground transparent to background transparent
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 2 4";
    let g = GimpGradient::new(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    cmp_hex!(g.at(0.0), "#ff000000");
    cmp_hex!(g.at(1.0), "#0000ff00");

    // Background transparent to foreground transparent
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 4 2";
    let g = GimpGradient::new(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    cmp_hex!(g.at(0.0), "#0000ff00");
    cmp_hex!(g.at(1.0), "#ff000000");

    // Blending function: step
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 1 0 0 1 0 0 1 1 5 0 0 0";
    let g = GimpGradient::new(BufReader::new(ggr.as_bytes()), &col, &col).unwrap();

    cmp_hex!(g.at(0.00), "#ff0000");
    cmp_hex!(g.at(0.25), "#ff0000");
    cmp_hex!(g.at(0.49), "#ff0000");
    cmp_hex!(g.at(0.51), "#0000ff");
    cmp_hex!(g.at(0.75), "#0000ff");
    cmp_hex!(g.at(1.00), "#0000ff");

    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.75 1 1 0 0 1 0 0 1 1 5 0 0 0";
    let g = GimpGradient::new(BufReader::new(ggr.as_bytes()), &col, &col).unwrap();

    cmp_hex!(g.at(0.00), "#ff0000");
    cmp_hex!(g.at(0.25), "#ff0000");
    cmp_hex!(g.at(0.50), "#ff0000");
    cmp_hex!(g.at(0.74), "#ff0000");
    cmp_hex!(g.at(0.76), "#0000ff");
    cmp_hex!(g.at(0.90), "#0000ff");
    cmp_hex!(g.at(1.00), "#0000ff");

    // Coloring type: HSV CCW (white to blue)
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 1 1 1 1 0 0 1 1 0 1 0 0";
    let g = GimpGradient::new(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    cmp_hex!(g.at(0.0), "#ffffff");
    cmp_hex!(g.at(0.5), "#80ff80");
    cmp_hex!(g.at(1.0), "#0000ff");

    // Coloring type: HSV CW (white to blue)
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 1 1 1 1 0 0 1 1 0 2 0 0";
    let g = GimpGradient::new(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    cmp_hex!(g.at(0.0), "#ffffff");
    cmp_hex!(g.at(0.5), "#ff80ff");
    cmp_hex!(g.at(1.0), "#0000ff");

    // UTF-8 with BOM
    let ggr = include_str!("../examples/ggr/UTF_8_BOM.ggr");
    let g = GimpGradient::new(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    assert_eq!(g.name(), "Pelangi");

    cmp_hex!(g.at(0.0), "#24579e");
    cmp_hex!(g.at(1.0), "#4878a8");
}

#[test]
fn invalid_format() {
    let col = Color::default();

    let test_data = [
        ("GIMP Pallete\n9", "invalid header (line 1)"),
        ("GIMP Gradient\n6", "invalid header (line 2)"),
        (
            "GIMP Gradient\nName: Gradient\nx",
            "invalid header (line 3)",
        ),
        (
            "GIMP Gradient\nName: Gradient\n1\n0 0.5 1",
            "invalid segment (line 4)",
        ),
        (
            "GIMP Gradient\nName: Gradient\n3\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 0 0",
            "wrong segments count (line 3)",
        ),
        ("GIMP Gradient\nName: Gradient\n0", "no segment (line 4)"),
    ];

    for (ggr, err_msg) in test_data {
        let res = GimpGradient::new(BufReader::new(ggr.as_bytes()), &col, &col);
        assert_eq!(res.unwrap_err().to_string(), err_msg);
    }

    let invalid_segments = [
        "GIMP Gradient\nName: Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 6 0 0 0",
        "GIMP Gradient\nName: Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 3 0 0",
        "GIMP Gradient\nName: Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 5 0",
        "GIMP Gradient\nName: Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 0 5",
        "GIMP Gradient\nName: Gradient\n1\n0 0.5 1 0 0 0 A 1 1 1 A 0 0 0 0",
        // NaN & infinite
        "GIMP Gradient\nName: Gradient\n1\n0 0.5 nan 0 0 0 1 1 1 1 1 0 0 0 0",
        "GIMP Gradient\nName: Gradient\n1\n0 0.5 inf 0 0 0 1 1 1 1 1 0 0 0 0",
    ];

    for ggr in invalid_segments {
        let res = GimpGradient::new(BufReader::new(ggr.as_bytes()), &col, &col);
        assert!(res.is_err());
    }
}
