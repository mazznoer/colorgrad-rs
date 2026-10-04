use alloc::sync::Arc;
use alloc::vec::Vec;

use crate::utils::{convert_colors, interpolate_smoothstep};
use crate::{BlendMode, Color, Gradient};

#[cfg_attr(
    feature = "preset",
    doc = r##"
```
use colorgrad::Gradient;

let g = colorgrad::preset::rainbow().sharp(11, 0.0);
```"##
)]
#[derive(Debug, Clone)]
pub struct SharpGradient {
    stops: Arc<[(f32, [f32; 4])]>,
    domain: (f32, f32),
    first_color: Color,
    last_color: Color,
}

impl SharpGradient {
    pub(crate) fn new(colors_in: &[Color], domain: (f32, f32), t: f32) -> Self {
        let n = colors_in.len();
        assert!(n >= 2, "need 2 or more colors");

        let colors: Vec<Color> = colors_in.iter().flat_map(|&c| [c, c]).collect();

        let t = if !t.is_finite() {
            0.1
        } else {
            t.clamp(0.0, 1.0)
        };

        let segment_len = (domain.1 - domain.0) / (n as f32);
        let t_offset = t * segment_len / 4.0;

        let mut positions = Vec::with_capacity(n * 2);

        for i in 0..n {
            let mut p_start = domain.0 + (i as f32) * segment_len;
            let mut p_end = domain.0 + ((i + 1) as f32) * segment_len;

            if i > 0 {
                p_start += t_offset;
            }
            if i < n - 1 {
                p_end -= t_offset;
            }

            positions.push(p_start);
            positions.push(p_end);
        }

        let colors = convert_colors(&colors, BlendMode::Rgb);
        let first_color = colors_in[0];
        let last_color = colors_in[n - 1];

        Self {
            stops: positions.into_iter().zip(colors).collect::<Arc<[_]>>(),
            domain,
            first_color,
            last_color,
        }
    }
}

impl Gradient for SharpGradient {
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

        // Clamp `low` safely to avoid out-of-bounds panics from float precision quirks
        let low = low.clamp(1, self.stops.len() - 1);

        let i = low - 1;
        let (pos_0, col_0) = self.stops[i];
        let (pos_1, col_1) = self.stops[low];

        // An even index implies we are sitting inside a solid color block
        if i & 1 == 0 {
            return Color::new(col_0[0], col_0[1], col_0[2], col_0[3]);
        }

        // Prevent division by zero NaNs if stops perfectly overlap
        let diff = pos_1 - pos_0;
        let t = if diff > 0.0 { (t - pos_0) / diff } else { 1.0 };

        let [a, b, c, d] = interpolate_smoothstep(col_0, col_1, t);
        Color::new(a, b, c, d)
    }

    fn domain(&self) -> (f32, f32) {
        self.domain
    }
}
