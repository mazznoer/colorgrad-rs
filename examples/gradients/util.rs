use colorgrad::{Color, Gradient};
use image::{imageops, ImageBuffer, Rgba};

pub fn gradient_image<T>(gradient: &T, width: u32, height: u32) -> ImageBuffer<Rgba<u16>, Vec<u16>>
where
    T: Gradient + ?Sized,
{
    let (dmin, dmax) = gradient.domain();
    ImageBuffer::from_fn(width, height, |x, _| {
        let rgba = gradient
            .at(remap(x as f32, 0.0, width as f32, dmin, dmax))
            .to_rgba16();
        Rgba(rgba)
    })
}

fn rgb_plot<T>(
    grad: &T,
    width: u32,
    height: u32,
    pos: Option<&[f32]>,
) -> ImageBuffer<Rgba<u16>, Vec<u16>>
where
    T: Gradient + ?Sized,
{
    let mut imgbuf = ImageBuffer::from_pixel(
        width,
        height,
        Rgba(Color::new(0.9, 0.9, 0.9, 1.0).to_rgba16()),
    );
    let (dmin, dmax) = grad.domain();
    let fw = width as f32;
    let y1 = 0.0;
    let y2 = height as f32;

    if let Some(pos) = pos {
        let color = Rgba(Color::new(0.75, 0.75, 0.75, 1.0).to_rgba16());

        for t in pos {
            let x = remap(*t, dmin, dmax, 0.0, fw - 1.0) as u32;
            for y in 0..height {
                let pixel = imgbuf.get_pixel_mut(x, y);
                *pixel = color;
            }
        }
    };

    for x in 0..width {
        let col = grad.at(remap(x as f32, 0.0, fw, dmin, dmax));
        let yr = remap(col.r, 0.0, 1.0, y2, y1);
        let yg = remap(col.g, 0.0, 1.0, y2, y1);
        let yb = remap(col.b, 0.0, 1.0, y2, y1);
        let yl = remap(color_luminance(&col), 0.0, 1.0, y2, y1);

        if (y1..y2).contains(&yr) {
            let pixel = imgbuf.get_pixel_mut(x, yr as u32);
            *pixel = Rgba([65535, 0, 0, 65535]);
        }

        if (y1..y2).contains(&yg) {
            let pixel = imgbuf.get_pixel_mut(x, yg as u32);
            *pixel = Rgba([0, 32767, 0, 65535]);
        }

        if (y1..y2).contains(&yb) {
            let pixel = imgbuf.get_pixel_mut(x, yb as u32);
            *pixel = Rgba([0, 0, 65535, 65535]);
        }

        continue;

        if (y1..y2).contains(&yl) {
            let pixel = imgbuf.get_pixel_mut(x, yl as u32);
            *pixel = Rgba([32767, 32767, 32767, 65535]);
        }
    }
    imgbuf
}

pub fn grad_rgb_plot<T>(
    grad: &T,
    width: u32,
    height: u32,
    padding: u32,
    pos: Option<&[f32]>,
) -> ImageBuffer<Rgba<u16>, Vec<u16>>
where
    T: Gradient + ?Sized,
{
    let w = width + padding * 2;
    let h = height * 2 + padding * 3;
    let mut imgbuf =
        ImageBuffer::from_pixel(w, h, Rgba(Color::new(1.0, 1.0, 1.0, 1.0).to_rgba16()));

    let grad_img = gradient_image(grad, width, height);
    imageops::replace(&mut imgbuf, &grad_img, padding.into(), padding.into());

    let plot_img = rgb_plot(grad, width, height, pos);
    imageops::replace(
        &mut imgbuf,
        &plot_img,
        padding.into(),
        (height + padding * 2).into(),
    );

    imgbuf
}

// Map t in range [a, b] to range [c, d]
fn remap(t: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
    (t - a) * ((d - c) / (b - a)) + c
}

fn color_luminance(col: &Color) -> f32 {
    fn lum(t: f32) -> f32 {
        if t <= 0.03928 {
            t / 12.92
        } else {
            ((t + 0.055) / 1.055).powf(2.4)
        }
    }

    0.2126 * lum(col.r) + 0.7152 * lum(col.g) + 0.0722 * lum(col.b)
}
