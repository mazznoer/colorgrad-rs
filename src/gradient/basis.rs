use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::utils::convert_colors;
use crate::{BlendMode, Color, Gradient, GradientBuilder, GradientBuilderError};

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

#[cfg_attr(
    feature = "named-colors",
    doc = r##"
```
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# use colorgrad::{GradientBuilder, BasisGradient};
use colorgrad::Gradient;

let grad = GradientBuilder::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .build::<BasisGradient>()?;
# Ok(())
# }
```
"##
)]
#[derive(Debug, Clone)]
pub struct BasisGradient {
    values: Arc<[[f32; 4]]>,
    positions: Arc<[f32]>,
    domain: (f32, f32),
    mode: BlendMode,
    first_color: Color,
    last_color: Color,
}

impl BasisGradient {
    pub(crate) fn new(colors: &[Color], positions: Vec<f32>, mode: BlendMode) -> Self {
        let n = colors.len();
        assert!(
            n >= 2 && n == positions.len(),
            "Gradient requires at least 2 stops, and colors length must match positions length"
        );

        let domain = (positions[0], positions[n - 1]);
        let first_color = colors[0];
        let last_color = colors[n - 1];

        Self {
            values: convert_colors(colors, mode).collect(),
            positions: positions.into(),
            domain,
            mode,
            first_color,
            last_color,
        }
    }
}

impl Gradient for BasisGradient {
    fn at(&self, t: f32) -> Color {
        if t.is_nan() {
            return Color::new(0.0, 0.0, 0.0, 1.0);
        }

        if t <= self.domain.0 {
            return self.first_color;
        }

        if t >= self.domain.1 {
            return self.last_color;
        }

        let low = self.positions.partition_point(|&p| p < t);

        // Safely clamp bounds in case of floating-point inaccuracies
        let low = low.clamp(1, self.positions.len() - 1);

        let i = low - 1;
        let n = self.positions.len() - 1;

        let pos0 = self.positions[i];
        let pos1 = self.positions[low];
        let val0 = self.values[i];
        let val1 = self.values[low];

        // Prevent division-by-zero
        let diff = pos1 - pos0;
        let t = if diff > 0.0 { (t - pos0) / diff } else { 1.0 };

        let [a, b, c, d] = core::array::from_fn(|j| {
            let v1 = val0[j];
            let v2 = val1[j];

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

            basis(t, v0, v1, v2, v3)
        });

        match self.mode {
            BlendMode::Rgb => Color::new(a, b, c, d),
            BlendMode::LinearRgb => Color::from_linear_rgba(a, b, c, d),
            BlendMode::Oklab => Color::from_oklaba(a, b, c, d),
            BlendMode::Lab => Color::from_laba(a, b, c, d),
        }
    }

    fn domain(&self) -> (f32, f32) {
        self.domain
    }
}

impl TryFrom<&mut GradientBuilder> for BasisGradient {
    type Error = GradientBuilderError;

    fn try_from(gb: &mut GradientBuilder) -> Result<Self, Self::Error> {
        gb.prepare_build()?;
        Ok(Self::new(&gb.colors, gb.positions.clone(), gb.mode))
    }
}
