extern crate colorgrad;
extern crate image;

fn main() {
    let grad = colorgrad::CustomGradient::new()
        .html_colors(&["deeppink", "gold", "seagreen"])
        .build()
        .unwrap();

    let w = 1500;
    let h = 70;
    let fw = w as f64;

    let mut imgbuf = image::ImageBuffer::new(w, h);

    for (x, _y, pixel) in imgbuf.enumerate_pixels_mut() {
        let (r, g, b, _) = grad.at(x as f64 / fw).rgba_u8();
        *pixel = image::Rgb([r, g, b]);
    }

    imgbuf.save("gradient.png").unwrap();
}
