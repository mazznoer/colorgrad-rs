use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::utils::convert_colors;
use crate::{BlendMode, Color, Gradient, GradientBuilder, GradientBuilderError};

#[cfg(not(feature = "std"))]
use crate::utils::FloatExt;

// Catmull-Rom spline algorithm adapted from:
// https://qroph.github.io/2018/07/30/smooth-paths-using-catmull-rom-splines.html

#[inline]
fn catmull_segment_coeffs(v0: f32, v1: f32, v2: f32, v3: f32) -> [f32; 4] {
    let t1 = (v0 - v1).abs().sqrt();
    let dt21 = (v1 - v2).abs().sqrt();
    let dt32 = (v2 - v3).abs().sqrt();

    let t2 = t1 + dt21;
    let dt31 = dt21 + dt32;

    let m1_raw = dt21 * ((v1 - v0) / t1 - (v2 - v0) / t2 + (v2 - v1) / dt21);
    let m2_raw = dt21 * ((v2 - v1) / dt21 - (v3 - v1) / dt31 + (v3 - v2) / dt32);

    let m1 = if m1_raw.is_nan() { 0.0 } else { m1_raw };
    let m2 = if m2_raw.is_nan() { 0.0 } else { m2_raw };

    let a = 2.0 * v1 - 2.0 * v2 + m1 + m2;
    let b = -3.0 * v1 + 3.0 * v2 - 2.0 * m1 - m2;
    let c = m1;
    let d = v1;

    [a, b, c, d]
}

#[cfg_attr(
    feature = "named-colors",
    doc = r##"
```
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# use colorgrad::{GradientBuilder, CatmullRomGradient};
use colorgrad::Gradient;

let grad = GradientBuilder::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .build::<CatmullRomGradient>()?;
# Ok(())
# }
```
"##
)]
#[derive(Debug, Clone)]
pub struct CatmullRomGradient {
    segments: Arc<[[[f32; 4]; 4]]>,
    positions: Arc<[f32]>,
    domain: (f32, f32),
    mode: BlendMode,
    first_color: Color,
    last_color: Color,
}

impl CatmullRomGradient {
    pub(crate) fn new(colors: &[Color], positions: Vec<f32>, mode: BlendMode) -> Self {
        let n = colors.len();
        assert!(
            n >= 2 && n == positions.len(),
            "Gradient requires at least 2 stops, and colors length must match positions length"
        );

        let converted: Vec<_> = convert_colors(colors, mode).collect();
        let mut segments = Vec::with_capacity(n - 1);

        for i in 0..(n - 1) {
            let mut segment = [[0.0f32; 4]; 4];

            for ch in 0..4 {
                let v1 = converted[i][ch];
                let v2 = converted[i + 1][ch];

                let v0 = if i == 0 {
                    2.0 * converted[0][ch] - converted[1][ch]
                } else {
                    converted[i - 1][ch]
                };

                let v3 = if i + 2 < n {
                    converted[i + 2][ch]
                } else {
                    2.0 * converted[n - 1][ch] - converted[n - 2][ch]
                };

                segment[ch] = catmull_segment_coeffs(v0, v1, v2, v3);
            }

            segments.push(segment);
        }

        let domain = (positions[0], positions[n - 1]);
        let first_color = colors[0];
        let last_color = colors[n - 1];

        Self {
            segments: segments.into(),
            positions: positions.into(),
            domain,
            mode,
            first_color,
            last_color,
        }
    }
}

impl Gradient for CatmullRomGradient {
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

        let pos0 = self.positions[low - 1];
        let pos1 = self.positions[low];

        let [seg_a, seg_b, seg_c, seg_d] = self.segments[low - 1];

        let t1 = (t - pos0) / (pos1 - pos0);
        let t2 = t1 * t1;
        let t3 = t2 * t1;

        let a = seg_a[0] * t3 + seg_a[1] * t2 + seg_a[2] * t1 + seg_a[3];
        let b = seg_b[0] * t3 + seg_b[1] * t2 + seg_b[2] * t1 + seg_b[3];
        let c = seg_c[0] * t3 + seg_c[1] * t2 + seg_c[2] * t1 + seg_c[3];
        let d = seg_d[0] * t3 + seg_d[1] * t2 + seg_d[2] * t1 + seg_d[3];

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

impl TryFrom<&mut GradientBuilder> for CatmullRomGradient {
    type Error = GradientBuilderError;

    fn try_from(gb: &mut GradientBuilder) -> Result<Self, Self::Error> {
        gb.prepare_build()?;
        Ok(Self::new(&gb.colors, gb.positions.clone(), gb.mode))
    }
}
