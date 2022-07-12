use crate::{convert_colors, interp_angle, linear_interpolation, BlendMode, Color, GradientBase};

#[derive(Debug)]
pub(crate) struct LinearGradient {
    stops: Vec<(f64, [f64; 4])>,
    dmin: f64,
    dmax: f64,
    mode: BlendMode,
    first_color: Color,
    last_color: Color,
}

impl LinearGradient {
    pub(crate) fn new(colors: Vec<Color>, positions: Vec<f64>, mode: BlendMode) -> Self {
        let dmin = positions[0];
        let dmax = positions[positions.len() - 1];
        let first_color = colors[0].clone();
        let last_color = colors[colors.len() - 1].clone();
        let colors = convert_colors(&colors, mode);
        Self {
            stops: positions
                .iter()
                .zip(colors.iter())
                .map(|(p, c)| (*p, *c))
                .collect(),
            dmin,
            dmax,
            mode,
            first_color,
            last_color,
        }
    }
}

impl GradientBase for LinearGradient {
    fn at(&self, t: f64) -> Color {
        if t <= self.dmin {
            return self.first_color.clone();
        }

        if t >= self.dmax {
            return self.last_color.clone();
        }

        for segment in self.stops.windows(2) {
            let (pos_0, col_0) = &segment[0];
            let (pos_1, col_1) = &segment[1];
            if (*pos_0 <= t) && (t <= *pos_1) {
                let t = (t - pos_0) / (pos_1 - pos_0);
                let [a, b, c, d] = linear_interpolation(col_0, col_1, t);
                match self.mode {
                    BlendMode::Rgb => return Color::new(a, b, c, d),
                    BlendMode::LinearRgb => return Color::from_linear_rgba(a, b, c, d),
                    BlendMode::Oklab => return Color::from_oklaba(a, b, c, d),
                    BlendMode::Hsv => {
                        let hue = interp_angle(col_0[0], col_1[0], t);
                        return Color::from_hsva(hue, b, c, d);
                    }
                }
            }
        }

        self.last_color.clone()
    }
}
