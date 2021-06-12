use crate::{linspace, BlendMode, Color, Gradient, GradientBase, Interpolation};

trait Interpolator {
    fn at(&self, t: f64) -> f64;
}

// Adapted from https://qroph.github.io/2018/07/30/smooth-paths-using-catmull-rom-splines.html

struct CatmullRomInterpolator {
    segments: Vec<[f64; 4]>,
    pos: Vec<f64>,
}

impl CatmullRomInterpolator {
    fn new(values: &[f64], pos: &[f64]) -> CatmullRomInterpolator {
        let alpha = 0.5;
        let tension = 0.0;
        let n = values.len();

        let mut vals = Vec::with_capacity(n + 2);
        vals.push(2.0 * values[0] - values[1]);

        for v in values.iter() {
            vals.push(*v);
        }

        vals.push(2.0 * values[n - 1] - values[n - 2]);

        let mut segments = Vec::new();

        for i in 1..(vals.len() - 2) {
            let v0 = vals[i - 1];
            let v1 = vals[i];
            let v2 = vals[i + 1];
            let v3 = vals[i + 2];

            let t0 = 0.0;
            let t1 = t0 + (v0 - v1).abs().powf(alpha);
            let t2 = t1 + (v1 - v2).abs().powf(alpha);
            let t3 = t2 + (v2 - v3).abs().powf(alpha);

            let m1 = (1.0 - tension)
                * (t2 - t1)
                * ((v0 - v1) / (t0 - t1) - (v0 - v2) / (t0 - t2) + (v1 - v2) / (t1 - t2));
            let m2 = (1.0 - tension)
                * (t2 - t1)
                * ((v1 - v2) / (t1 - t2) - (v1 - v3) / (t1 - t3) + (v2 - v3) / (t2 - t3));
            let m1 = if m1.is_nan() { 0.0 } else { m1 };
            let m2 = if m2.is_nan() { 0.0 } else { m2 };

            let a = 2.0 * v1 - 2.0 * v2 + m1 + m2;
            let b = -3.0 * v1 + 3.0 * v2 - 2.0 * m1 - m2;
            let c = m1;
            let d = v1;

            segments.push([a, b, c, d]);
        }

        CatmullRomInterpolator {
            pos: pos.to_vec(),
            segments,
        }
    }
}

impl Interpolator for CatmullRomInterpolator {
    fn at(&self, t: f64) -> f64 {
        for (pos, seg) in self.pos.windows(2).zip(&self.segments) {
            if (pos[0] <= t) && (t <= pos[1]) {
                let t1 = (t - pos[0]) / (pos[1] - pos[0]);
                let t2 = t1 * t1;
                let t3 = t2 * t1;
                return seg[0] * t3 + seg[1] * t2 + seg[2] * t1 + seg[3];
            }
        }
        0.0
    }
}

// Adapted from https://github.com/d3/d3-interpolate/blob/master/src/basis.js

#[inline]
fn basis(t1: f64, v0: f64, v1: f64, v2: f64, v3: f64) -> f64 {
    let t2 = t1 * t1;
    let t3 = t2 * t1;
    ((1.0 - 3.0 * t1 + 3.0 * t2 - t3) * v0
        + (4.0 - 6.0 * t2 + 3.0 * t3) * v1
        + (1.0 + 3.0 * t1 + 3.0 * t2 - 3.0 * t3) * v2
        + t3 * v3)
        / 6.0
}

struct BasisInterpolator {
    values: Vec<f64>,
    pos: Vec<f64>,
}

impl BasisInterpolator {
    fn new(values: &[f64], pos: &[f64]) -> BasisInterpolator {
        BasisInterpolator {
            values: values.to_vec(),
            pos: pos.to_vec(),
        }
    }
}

impl Interpolator for BasisInterpolator {
    fn at(&self, t: f64) -> f64 {
        let n = self.values.len() - 1;

        for (i, (pos, val)) in self.pos.windows(2).zip(self.values.windows(2)).enumerate() {
            if (pos[0] <= t) && (t <= pos[1]) {
                let t = (t - pos[0]) / (pos[1] - pos[0]);
                let v1 = val[0];
                let v2 = val[1];

                let v0 = if i > 0 {
                    self.values[i - 1]
                } else {
                    2.0 * v1 - v2
                };

                let v3 = if i < (n - 1) {
                    self.values[i + 2]
                } else {
                    2.0 * v2 - v1
                };

                return basis(t, v0, v1, v2, v3);
            }
        }
        0.0
    }
}

#[derive(Debug)]
struct SplineGradient<T: Interpolator> {
    a: T,
    b: T,
    c: T,
    d: T,
    dmin: f64,
    dmax: f64,
    mode: BlendMode,
}

impl<T: Interpolator> GradientBase for SplineGradient<T> {
    fn at(&self, t: f64) -> Color {
        if t.is_nan() {
            return Color::from_rgb(0.0, 0.0, 0.0);
        }

        let t = t.clamp(self.dmin, self.dmax);

        match self.mode {
            BlendMode::Rgb => {
                Color::from_rgba(self.a.at(t), self.b.at(t), self.c.at(t), self.d.at(t))
            }
            BlendMode::LinearRgb => {
                Color::from_linear_rgba(self.a.at(t), self.b.at(t), self.c.at(t), self.d.at(t))
            }
            BlendMode::Oklab => {
                Color::from_oklaba(self.a.at(t), self.b.at(t), self.c.at(t), self.d.at(t))
            }
            BlendMode::Hsv => {
                Color::from_hsva(self.a.at(t), self.b.at(t), self.c.at(t), self.d.at(t))
            }
        }
    }
}

pub(crate) fn spline_gradient(
    colors: &[Color],
    pos: &[f64],
    space: BlendMode,
    interpolation: Interpolation,
) -> Gradient {
    let n = colors.len();
    let mut a = Vec::with_capacity(n);
    let mut b = Vec::with_capacity(n);
    let mut c = Vec::with_capacity(n);
    let mut d = Vec::with_capacity(n);

    for col in colors.iter() {
        let (c1, c2, c3, c4) = match space {
            BlendMode::Rgb => col.rgba(),
            BlendMode::LinearRgb => col.to_linear_rgba(),
            BlendMode::Oklab => col.to_oklaba(),
            BlendMode::Hsv => col.to_hsva(),
        };
        a.push(c1);
        b.push(c2);
        c.push(c3);
        d.push(c4);
    }

    let dmin = pos[0];
    let dmax = pos[n - 1];

    match interpolation {
        Interpolation::CatmullRom => {
            let gradbase = SplineGradient {
                a: CatmullRomInterpolator::new(&a, &pos),
                b: CatmullRomInterpolator::new(&b, &pos),
                c: CatmullRomInterpolator::new(&c, &pos),
                d: CatmullRomInterpolator::new(&d, &pos),
                dmin,
                dmax,
                mode: space,
            };

            Gradient {
                gradient: Box::new(gradbase),
                dmin,
                dmax,
            }
        }
        _ => {
            let gradbase = SplineGradient {
                a: BasisInterpolator::new(&a, &pos),
                b: BasisInterpolator::new(&b, &pos),
                c: BasisInterpolator::new(&c, &pos),
                d: BasisInterpolator::new(&d, &pos),
                dmin,
                dmax,
                mode: space,
            };

            Gradient {
                gradient: Box::new(gradbase),
                dmin,
                dmax,
            }
        }
    }
}

pub(crate) fn preset_spline(html_colors: &[&str]) -> Gradient {
    let mut colors = Vec::new();

    for s in html_colors {
        if let Ok(c) = csscolorparser::parse(s) {
            colors.push(c);
        }
    }

    let pos = linspace(0.0, 1.0, colors.len());
    spline_gradient(&colors, &pos, BlendMode::Rgb, Interpolation::Basis)
}
