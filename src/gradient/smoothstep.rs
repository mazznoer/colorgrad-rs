use alloc::sync::Arc;
use core::convert::TryFrom;

use crate::utils::{convert_colors, interpolate_smoothstep};
use crate::{BlendMode, Color, Gradient, GradientBuilder, GradientBuilderError};

#[derive(Debug, Clone)]
pub struct SmoothstepGradient {
    stops: Arc<[(f32, [f32; 4])]>,
    domain: (f32, f32),
    mode: BlendMode,
    first_color: Color,
    last_color: Color,
}

impl SmoothstepGradient {
    pub(crate) fn new(colors: &[Color], positions: &[f32], mode: BlendMode) -> Self {
        assert!(
            !colors.is_empty() && colors.len() == positions.len(),
            "colors and positions must be non-empty and of the same length"
        );
        assert!(colors.len() >= 2, "minimal 2 colors");

        let dmin = positions[0];
        let dmax = positions[positions.len() - 1];
        let first_color = colors[0];
        let last_color = colors[colors.len() - 1];

        Self {
            stops: positions
                .iter()
                .copied()
                .zip(convert_colors(colors, mode))
                .collect::<Arc<[_]>>(),
            domain: (dmin, dmax),
            mode,
            first_color,
            last_color,
        }
    }
}

impl Gradient for SmoothstepGradient {
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

        let low = self.stops.partition_point(|stop| stop.0 < t);

        let (pos_0, col_0) = self.stops[low - 1];
        let (pos_1, col_1) = self.stops[low];

        // Guard against division by zero if two stops share the exact same position.
        let diff = pos_1 - pos_0;
        let t = if diff > 0.0 { (t - pos_0) / diff } else { 1.0 };

        let [a, b, c, d] = interpolate_smoothstep(col_0, col_1, t);

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

impl TryFrom<&mut GradientBuilder> for SmoothstepGradient {
    type Error = GradientBuilderError;

    fn try_from(gb: &mut GradientBuilder) -> Result<Self, Self::Error> {
        gb.prepare_build()?;
        Ok(Self::new(&gb.colors, &gb.positions, gb.mode))
    }
}
