use colorgrad::Gradient;

#[test]
fn inverse() {
    let grad = colorgrad::GradientBuilder::new()
        .html_colors(&["#fff", "#000"])
        .build::<colorgrad::LinearGradient>()
        .unwrap();

    let inv = grad.inverse();

    assert_eq!(grad.at(0.0).to_rgba8(), inv.at(1.0).to_rgba8());
    assert_eq!(grad.at(0.3).to_rgba8(), inv.at(0.7).to_rgba8());
    assert_eq!(grad.at(0.5).to_rgba8(), inv.at(0.5).to_rgba8());
    assert_eq!(grad.at(0.7).to_rgba8(), inv.at(0.3).to_rgba8());
    assert_eq!(grad.at(1.0).to_rgba8(), inv.at(0.0).to_rgba8());
}
