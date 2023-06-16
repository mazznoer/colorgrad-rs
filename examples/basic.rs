use colorgrad::Gradient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let width = 1300.0;
    let height = 70.0;

    // preset gradient
    let grad = colorgrad::preset::rainbow();

    let imgbuf = image::ImageBuffer::from_fn(width as u32, height as u32, |x, _| {
        image::Rgba(grad.at(x as f32 / width).to_rgba8())
    });
    imgbuf.save("gradient-preset.png")?;

    // custom gradient
    let grad = colorgrad::GradientBuilder::new()
        .html_colors(&["deeppink", "gold", "seagreen"])
        .build::<colorgrad::CatmullRomGradient>()?;

    let imgbuf = image::ImageBuffer::from_fn(width as u32, height as u32, |x, _| {
        image::Rgba(grad.at(x as f32 / width).to_rgba8())
    });
    imgbuf.save("gradient-custom.png")?;

    Ok(())
}
