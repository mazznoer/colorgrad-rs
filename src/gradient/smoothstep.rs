use alloc::vec::Vec;
use core::convert::TryFrom;

use crate::utils::{convert_colors, interpolate_smoothstep};
use crate::{BlendMode, Color, Gradient, GradientBuilder, GradientBuilderError};

#[derive(Debug, Clone)]
pub struct SmoothstepGradient {
    stops: Vec<(f32, [f32; 4])>,
    domain: (f32, f32),
    mode: BlendMode,
    first_color: Color,
    last_color: Color,
}

impl SmoothstepGradient {
    pub(crate) fn new(colors: &[Color], positions: &[f32], mode: BlendMode) -> Self {
        let dmin = positions[0];
        let dmax = positions[positions.len() - 1];
        let first_color = colors[0].clone();
        let last_color = colors[colors.len() - 1].clone();
        let colors = convert_colors(colors, mode);
        Self {
            stops: positions.iter().zip(colors).map(|(p, c)| (*p, c)).collect(),
            domain: (dmin, dmax),
            mode,
            first_color,
            last_color,
        }
    }
}

impl Gradient for SmoothstepGradient {
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

        let idx = self.stops.partition_point(|&d| d.0 < t);

        let [i0, i1] = if idx == 0 { [0, 1] } else { [idx - 1, idx] };

        let (pos_0, col_0) = self.stops[i0];
        let (pos_1, col_1) = self.stops[i1];
        let t = (t - pos_0) / (pos_1 - pos_0);
        let [a, b, c, d] = interpolate_smoothstep(&col_0, &col_1, t);

        match self.mode {
            BlendMode::Rgb => Color::new(a, b, c, d),
            BlendMode::LinearRgb => Color::from_linear_rgba(a, b, c, d),
            BlendMode::Oklab => Color::from_oklaba(a, b, c, d),
            #[cfg(feature = "lab")]
            BlendMode::Lab => Color::from_laba(a, b, c, d),
        }
    }

    fn domain(&self) -> (f32, f32) {
        self.domain
    }
}

impl TryFrom<&mut GradientBuilder> for SmoothstepGradient {
    type Error = GradientBuilderError;

    fn try_from(gb: &mut GradientBuilder) -> Result<Self, Self::Error> {
        gb.prepare_build()?;
        Ok(Self::new(&gb.colors, &gb.positions, gb.mode))
    }
}
