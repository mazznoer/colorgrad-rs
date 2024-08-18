use crate::{convert_colors, linspace, BlendMode, Color, Gradient};

/// ```ignore
/// use colorgrad::Gradient;
///
/// let g = colorgrad::preset::rainbow().sharp(11, 0.);
/// ```
#[derive(Debug, Clone)]
pub struct SharpGradient {
    stops: Vec<(f32, [f32; 4])>,
    domain: (f32, f32),
    first_color: Color,
    last_color: Color,
}

impl SharpGradient {
    pub(crate) fn new(colors_in: &[Color], domain: (f32, f32), t: f32) -> Self {
        let n = colors_in.len();
        let mut colors = Vec::with_capacity(n * 2);

        for c in colors_in {
            colors.push(c.clone());
            colors.push(c.clone());
        }

        let t = t.clamp(0.0, 1.0) * (domain.1 - domain.0) / n as f32 / 4.0;
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

        let colors = convert_colors(&colors, BlendMode::Rgb);
        let first_color = colors_in[0].clone();
        let last_color = colors_in[n - 1].clone();

        Self {
            stops: positions
                .iter()
                .zip(colors.iter())
                .map(|(p, c)| (*p, *c))
                .collect(),
            domain,
            first_color,
            last_color,
        }
    }
}

impl Gradient for SharpGradient {
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

        while low < high {
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

        let i = low - 1;
        let (pos_0, col_0) = &self.stops[i];
        let (pos_1, col_1) = &self.stops[low];

        if i & 1 == 0 {
            return Color::new(col_0[0], col_0[1], col_0[2], col_0[3]);
        }

        let t = (t - pos_0) / (pos_1 - pos_0);
        let [a, b, c, d] = smoothstep(col_0, col_1, t);
        Color::new(a, b, c, d)
    }

    fn domain(&self) -> (f32, f32) {
        self.domain
    }
}

#[inline]
fn smoothstep(a: &[f32; 4], b: &[f32; 4], t: f32) -> [f32; 4] {
    [
        (b[0] - a[0]) * (3.0 - t * 2.0) * t * t + a[0],
        (b[1] - a[1]) * (3.0 - t * 2.0) * t * t + a[1],
        (b[2] - a[2]) * (3.0 - t * 2.0) * t * t + a[2],
        (b[3] - a[3]) * (3.0 - t * 2.0) * t * t + a[3],
    ]
}
