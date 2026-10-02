use alloc::sync::Arc;
use alloc::vec::Vec;
use core::convert::TryFrom;

use crate::utils::{convert_colors, interpolate_linear};
use crate::{BlendMode, Color, Gradient, GradientBuilder, GradientBuilderError};

#[cfg_attr(
    all(feature = "named-colors", feature = "preset"),
    doc = r##"
## Using `GradientBuilder`

```
# fn main() -> Result<(), Box<dyn std::error::Error>> {
use colorgrad::Gradient;

let grad = colorgrad::GradientBuilder::new()
    .html_colors(&["deeppink", "gold", "seagreen"])
    .mode(colorgrad::BlendMode::Oklab)
    .build::<colorgrad::LinearGradient>()?;
# Ok(())
# }
```

## Converting from another gradient

```
use colorgrad::Gradient;

let og = colorgrad::preset::rainbow();

let lg = og.linearize(0.005);
```

## From raw color stops

```
# fn main() -> Result<(), Box<dyn std::error::Error>> {
let stops = vec![
    (0.000, [0.87843, 0.47451, 0.64706, 1.00000]),
    (0.250, [0.76078, 0.97647, 0.98824, 1.00000]),
    (0.500, [0.07843, 0.56078, 0.66667, 1.00000]),
    (0.750, [0.49804, 0.07451, 0.86667, 1.00000]),
    (1.000, [0.86667, 0.75294, 0.09412, 1.00000]),
];

let lg = colorgrad::LinearGradient::from_rgba_data(stops).unwrap();
# Ok(())
# }
```
"##
)]
#[derive(Debug, Clone)]
pub struct LinearGradient {
    stops: Arc<[(f32, [f32; 4])]>,
    domain: (f32, f32),
    mode: BlendMode,
    first_color: Color,
    last_color: Color,
}

impl LinearGradient {
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

    pub fn stops(&self) -> &[(f32, [f32; 4])] {
        &self.stops
    }

    pub fn mode(&self) -> BlendMode {
        self.mode
    }

    pub fn from_rgba_data(stops: impl Into<Vec<(f32, [f32; 4])>>) -> Option<Self> {
        let stops = stops.into();

        if stops.len() < 2 {
            return None;
        }

        let mut prev = f32::NEG_INFINITY;

        for (t, rgba) in &stops {
            if !t.is_finite() {
                return None;
            }
            for c in rgba {
                if !c.is_finite() {
                    return None;
                }
            }
            if *t < prev {
                return None;
            }
            prev = *t;
        }

        let (dmin, c0) = stops[0];
        let (dmax, c1) = stops[stops.len() - 1];

        Some(Self {
            stops: stops.into(),
            domain: (dmin, dmax),
            mode: BlendMode::Rgb,
            first_color: Color::from(c0),
            last_color: Color::from(c1),
        })
    }
}

impl Gradient for LinearGradient {
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

        let [a, b, c, d] = interpolate_linear(col_0, col_1, t);

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

impl TryFrom<&mut GradientBuilder> for LinearGradient {
    type Error = GradientBuilderError;

    fn try_from(gb: &mut GradientBuilder) -> Result<Self, Self::Error> {
        gb.prepare_build()?;
        Ok(Self::new(&gb.colors, &gb.positions, gb.mode))
    }
}
