use crate::{linspace, Color, GradientBase};

#[derive(Debug, Clone)]
pub(crate) struct SharpGradient {
    stops: Vec<(f64, Color)>,
    domain: (f64, f64),
    first_color: Color,
    last_color: Color,
}

impl SharpGradient {
    pub(crate) fn new(colors_in: &[Color], domain: (f64, f64), t: f64) -> Self {
        let n = colors_in.len();
        let mut colors = Vec::with_capacity(n * 2);

        for c in colors_in {
            colors.push(c.clone());
            colors.push(c.clone());
        }

        let t = t.clamp(0.0, 1.0) * (domain.1 - domain.0) / n as f64 / 4.0;
        let p = linspace(domain.0, domain.1, n + 1);
        let mut positions = Vec::with_capacity(n * 2);
        let mut j = 0;

        for i in 0..n {
            positions.push(p[i]);

            if j > 0 {
                positions[j] += t;
            }

            j += 1;
            positions.push(p[i + 1]);

            if j < colors.len() - 1 {
                positions[j] -= t;
            }

            j += 1;
        }

        let first_color = colors_in[0].clone();
        let last_color = colors_in[n - 1].clone();

        Self {
            stops: positions
                .iter()
                .zip(colors.iter())
                .map(|(p, c)| (*p, c.clone()))
                .collect(),
            domain,
            first_color,
            last_color,
        }
    }
}

impl GradientBase for SharpGradient {
    fn at(&self, t: f64) -> Color {
        if t <= self.domain.0 {
            return self.first_color.clone();
        }

        if t >= self.domain.1 {
            return self.last_color.clone();
        }

        for (i, segment) in self.stops.windows(2).enumerate() {
            let (pos_0, col_0) = &segment[0];
            let (pos_1, col_1) = &segment[1];

            if (*pos_0 <= t) && (t <= *pos_1) {
                if i & 1 == 0 {
                    return col_0.clone();
                }
                let t = (t - pos_0) / (pos_1 - pos_0);
                return col_0.interpolate_rgb(col_1, t);
            }
        }

        Color::from_rgba(0.0, 0.0, 0.0, 1.0)
    }
}
