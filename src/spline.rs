use crate::{Color, Gradient, GradientBase};

fn basis(t1: f64, v0: f64, v1: f64, v2: f64, v3: f64) -> f64 {
    let t2 = t1 * t1;
    let t3 = t2 * t1;
    ((1. - 3. * t1 + 3. * t2 - t3) * v0
        + (4. - 6. * t2 + 3. * t3) * v1
        + (1. + 3. * t1 + 3. * t2 - 3. * t3) * v2
        + t3 * v3)
        / 6.
}

struct Spline {
    values: Vec<f64>,
}

impl Spline {
    fn at(&self, t: f64) -> f64 {
        let n = self.values.len() - 1;
        let (t, i) = if t <= 0. {
            (0., 0)
        } else if t >= 1. {
            (1., n - 1)
        } else {
            (t, (n as f64 * t) as usize)
        };
        let v1 = self.values[i];
        let v2 = self.values[i + 1];
        let v0 = if i > 0 {
            self.values[i - 1]
        } else {
            2. * v1 - v2
        };
        let v3 = if i < (n - 1) {
            self.values[i + 2]
        } else {
            2. * v2 - v1
        };
        basis((t - i as f64 / n as f64) * n as f64, v0, v1, v2, v3)
    }
}

struct SplineGradient {
    r: Spline,
    g: Spline,
    b: Spline,
}

impl GradientBase for SplineGradient {
    fn at(&self, t: f64) -> Color {
        Color::from_rgb(self.r.at(t), self.g.at(t), self.b.at(t))
    }
}

pub(crate) fn preset_spline(html_colors: &[&str]) -> Gradient {
    let mut colors = Vec::new();
    for s in html_colors {
        if let Ok(c) = csscolorparser::parse(s) {
            colors.push(c);
        }
    }
    let gradbase = SplineGradient {
        r: Spline {
            values: colors.iter().map(|c| c.red()).collect(),
        },
        g: Spline {
            values: colors.iter().map(|c| c.green()).collect(),
        },
        b: Spline {
            values: colors.iter().map(|c| c.blue()).collect(),
        },
    };
    Gradient {
        gradient: Box::new(gradbase),
        dmin: 0.,
        dmax: 1.,
    }
}
