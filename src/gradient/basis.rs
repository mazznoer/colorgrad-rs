use std::convert::TryFrom;

use crate::{convert_colors, BlendMode, Color, Gradient, GradientBuilder, GradientBuilderError};

// Basis spline algorithm adapted from:
// https://github.com/d3/d3-interpolate/blob/master/src/basis.js

#[inline]
fn basis(t1: f32, v0: f32, v1: f32, v2: f32, v3: f32) -> f32 {
    let t2 = t1 * t1;
    let t3 = t2 * t1;
    ((1.0 - 3.0 * t1 + 3.0 * t2 - t3) * v0
        + (4.0 - 6.0 * t2 + 3.0 * t3) * v1
        + (1.0 + 3.0 * t1 + 3.0 * t2 - 3.0 * t3) * v2
        + t3 * v3)
        / 6.0
}

#[derive(Debug, Clone)]
pub struct BasisGradient {
    values: Vec<[f32; 4]>,
    positions: Vec<f32>,
    domain: (f32, f32),
    mode: BlendMode,
    first_color: Color,
    last_color: Color,
}

impl BasisGradient {
    pub(crate) fn new(colors: Vec<Color>, positions: Vec<f32>, mode: BlendMode) -> Self {
        let dmin = positions[0];
        let dmax = positions[positions.len() - 1];
        let first_color = colors[0].clone();
        let last_color = colors[colors.len() - 1].clone();
        Self {
            values: convert_colors(&colors, mode),
            positions,
            domain: (dmin, dmax),
            mode,
            first_color,
            last_color,
        }
    }
}

impl Gradient for BasisGradient {
    fn at(&self, t: f32) -> Color {
        if t <= self.domain.0 {
            return self.first_color.clone();
        }

        if t >= self.domain.1 {
            return self.last_color.clone();
        }

        if t.is_nan() {
            return Color::new(0.0, 0.0, 0.0, 1.0);
        }

        let mut low = 0;
        let mut high = self.positions.len();
        let n = high - 1;

        loop {
            if low >= high {
                break;
            }
            let mid = (low + high) / 2;
            if self.positions[mid] < t {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        if low == 0 {
            low = 1;
        }

        let pos0 = self.positions[low - 1];
        let pos1 = self.positions[low];
        let val0 = self.values[low - 1];
        let val1 = self.values[low];
        let i = low - 1;
        let t = (t - pos0) / (pos1 - pos0);
        let mut zz = [0.0; 4];

        for (j, (v1, v2)) in val0.iter().zip(val1.iter()).enumerate() {
            let v0 = if i > 0 {
                self.values[i - 1][j]
            } else {
                2.0 * v1 - v2
            };

            let v3 = if i < (n - 1) {
                self.values[i + 2][j]
            } else {
                2.0 * v2 - v1
            };

            zz[j] = basis(t, v0, *v1, *v2, v3);
        }

        let [c0, c1, c2, c3] = zz;

        match self.mode {
            BlendMode::Rgb => Color::new(c0, c1, c2, c3),
            BlendMode::LinearRgb => Color::from_linear_rgba(c0, c1, c2, c3),
            BlendMode::Oklab => Color::from_oklaba(c0, c1, c2, c3),
        }
    }

    fn domain(&self) -> (f32, f32) {
        self.domain
    }
}

impl TryFrom<&GradientBuilder> for BasisGradient {
    type Error = GradientBuilderError;

    fn try_from(gb: &GradientBuilder) -> Result<Self, Self::Error> {
        let (colors, positions) = gb.build_()?;
        Ok(Self::new(colors, positions, gb.mode))
    }
}
