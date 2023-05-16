use std::convert::TryFrom;

use crate::{convert_colors, BlendMode, Color, Gradient, GradientBuilder, GradientBuilderError};

/// ```
/// # use std::error::Error;
/// use colorgrad::Gradient;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let grad = colorgrad::GradientBuilder::new()
///     .html_colors(&["deeppink", "gold", "seagreen"])
///     .build::<colorgrad::LinearGradient>()?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct LinearGradient {
    stops: Vec<(f32, [f32; 4])>,
    domain: (f32, f32),
    mode: BlendMode,
    first_color: Color,
    last_color: Color,
}

impl LinearGradient {
    pub(crate) fn new(colors: Vec<Color>, positions: Vec<f32>, mode: BlendMode) -> Self {
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
            domain: (dmin, dmax),
            mode,
            first_color,
            last_color,
        }
    }
}

impl Gradient for LinearGradient {
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
        let mut high = self.stops.len();

        loop {
            if low >= high {
                break;
            }
            let mid = (low + high) / 2;
            if self.stops[mid].0 < t {
                low = mid + 1;
            } else {
                high = mid;
            }
        }

        if low == 0 {
            low = 1;
        }

        let (pos_0, col_0) = self.stops[low - 1];
        let (pos_1, col_1) = self.stops[low];
        let t = (t - pos_0) / (pos_1 - pos_0);
        let [a, b, c, d] = linear_interpolation(&col_0, &col_1, t);

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

impl TryFrom<&GradientBuilder> for LinearGradient {
    type Error = GradientBuilderError;

    fn try_from(gb: &GradientBuilder) -> Result<Self, Self::Error> {
        let (colors, positions) = gb.build_()?;
        Ok(Self::new(colors, positions, gb.mode))
    }
}

#[inline]
pub(crate) fn linear_interpolation(a: &[f32; 4], b: &[f32; 4], t: f32) -> [f32; 4] {
    [
        a[0] + t * (b[0] - a[0]),
        a[1] + t * (b[1] - a[1]),
        a[2] + t * (b[2] - a[2]),
        a[3] + t * (b[3] - a[3]),
    ]
}
