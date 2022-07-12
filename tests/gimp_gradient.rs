use colorgrad::{parse_ggr, Color};
use std::io::BufReader;

#[test]
fn parse_gimp_gradients() {
    let col = Color::default();
    let red = Color::new(1.0, 0.0, 0.0, 1.0);
    let blue = Color::new(0.0, 0.0, 1.0, 1.0);

    // Black to white
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 0 0";
    let (grad, name) = parse_ggr(BufReader::new(ggr.as_bytes()), &col, &col).unwrap();

    assert_eq!(name, "My Gradient");
    assert_eq!(grad.domain(), (0.0, 1.0));
    assert_eq!(grad.at(0.0).to_rgba8(), [0, 0, 0, 255]);
    assert_eq!(grad.at(1.0).to_rgba8(), [255, 255, 255, 255]);
    assert_eq!(grad.at(-0.5).to_rgba8(), [0, 0, 0, 255]);
    assert_eq!(grad.at(1.5).to_rgba8(), [255, 255, 255, 255]);
    assert_eq!(grad.at(f64::NAN).to_rgba8(), [0, 0, 0, 255]);

    // Foreground to background
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 1 3";
    let (grad, _) = parse_ggr(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    assert_eq!(grad.at(0.0).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(grad.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    // Background to foreground
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 3 1";
    let (grad, _) = parse_ggr(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    assert_eq!(grad.at(0.0).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(grad.at(1.0).to_rgba8(), [255, 0, 0, 255]);

    // Foreground transparent to background transparent
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 2 4";
    let (grad, _) = parse_ggr(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    assert_eq!(grad.at(0.0).to_rgba8(), [255, 0, 0, 0]);
    assert_eq!(grad.at(1.0).to_rgba8(), [0, 0, 255, 0]);

    // Background transparent to foreground transparent
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 4 2";
    let (grad, _) = parse_ggr(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    assert_eq!(grad.at(0.0).to_rgba8(), [0, 0, 255, 0]);
    assert_eq!(grad.at(1.0).to_rgba8(), [255, 0, 0, 0]);

    // Blending function: step
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 1 0 0 1 0 0 1 1 5 0 0 0";
    let (grad, _) = parse_ggr(BufReader::new(ggr.as_bytes()), &col, &col).unwrap();

    assert_eq!(grad.at(0.00).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(grad.at(0.25).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(grad.at(0.49).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(grad.at(0.51).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(grad.at(0.75).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(grad.at(1.00).to_rgba8(), [0, 0, 255, 255]);

    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.75 1 1 0 0 1 0 0 1 1 5 0 0 0";
    let (grad, _) = parse_ggr(BufReader::new(ggr.as_bytes()), &col, &col).unwrap();

    assert_eq!(grad.at(0.00).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(grad.at(0.25).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(grad.at(0.50).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(grad.at(0.74).to_rgba8(), [255, 0, 0, 255]);
    assert_eq!(grad.at(0.76).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(grad.at(0.90).to_rgba8(), [0, 0, 255, 255]);
    assert_eq!(grad.at(1.00).to_rgba8(), [0, 0, 255, 255]);

    // Coloring type: HSV CCW (white to blue)
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 1 1 1 1 0 0 1 1 0 1 0 0";
    let (grad, _) = parse_ggr(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    assert_eq!(grad.at(0.0).to_rgba8(), [255, 255, 255, 255]);
    assert_eq!(grad.at(0.5).to_rgba8(), [128, 255, 128, 255]);
    assert_eq!(grad.at(1.0).to_rgba8(), [0, 0, 255, 255]);

    // Coloring type: HSV CW (white to blue)
    let ggr = "GIMP Gradient\nName: My Gradient\n1\n0 0.5 1 1 1 1 1 0 0 1 1 0 2 0 0";
    let (grad, _) = parse_ggr(BufReader::new(ggr.as_bytes()), &red, &blue).unwrap();

    assert_eq!(grad.at(0.0).to_rgba8(), [255, 255, 255, 255]);
    assert_eq!(grad.at(0.5).to_rgba8(), [255, 128, 255, 255]);
    assert_eq!(grad.at(1.0).to_rgba8(), [0, 0, 255, 255]);
}

#[test]
fn invalid_format() {
    let col = Color::default();

    let test_data = vec![
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
        let res = parse_ggr(BufReader::new(ggr.as_bytes()), &col, &col);
        assert_eq!(res.unwrap_err().to_string(), err_msg);
    }

    let invalid_segments = vec![
        "GIMP Gradient\nName: Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 6 0 0 0",
        "GIMP Gradient\nName: Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 3 0 0",
        "GIMP Gradient\nName: Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 5 0",
        "GIMP Gradient\nName: Gradient\n1\n0 0.5 1 0 0 0 1 1 1 1 1 0 0 0 5",
        "GIMP Gradient\nName: Gradient\n1\n0 0.5 1 0 0 0 A 1 1 1 A 0 0 0 0",
    ];

    for ggr in invalid_segments {
        let res = parse_ggr(BufReader::new(ggr.as_bytes()), &col, &col);
        assert!(res.is_err());
    }
}
