use crate::{BlendMode, Color};

pub(crate) fn convert_colors(colors: &[Color], mode: BlendMode) -> Vec<[f32; 4]> {
    colors
        .iter()
        .map(|c| match mode {
            BlendMode::Rgb => c.to_array(),
            BlendMode::LinearRgb => c.to_linear_rgba(),
            BlendMode::Oklab => c.to_oklaba(),
            #[cfg(feature = "lab")]
            BlendMode::Lab => c.to_laba(),
        })
        .collect()
}

#[inline]
pub(crate) fn interpolate_linear(a: &[f32; 4], b: &[f32; 4], t: f32) -> [f32; 4] {
    [
        a[0] + t * (b[0] - a[0]),
        a[1] + t * (b[1] - a[1]),
        a[2] + t * (b[2] - a[2]),
        a[3] + t * (b[3] - a[3]),
    ]
}

pub(crate) fn linspace(min: f32, max: f32, n: usize) -> Vec<f32> {
    if n == 1 {
        return vec![min];
    }
    let d = max - min;
    let l = n as f32 - 1.0;
    (0..n).map(|i| min + (i as f32 * d) / l).collect()
}

#[inline]
pub(crate) fn modulo(x: f32, y: f32) -> f32 {
    (x % y + y) % y
}

// Map t from range [a, b] to range [0, 1]
#[inline]
pub(crate) fn norm(t: f32, a: f32, b: f32) -> f32 {
    (t - a) * (1.0 / (b - a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utils() {
        let empty: [f32; 0] = [];
        assert_eq!(linspace(0.0, 1.0, 0), empty);
        assert_eq!(linspace(0.0, 1.0, 1), [0.0]);
        assert_eq!(linspace(0.0, 1.0, 2), [0.0, 1.0]);
        assert_eq!(linspace(0.0, 1.0, 3), [0.0, 0.5, 1.0]);
        assert_eq!(linspace(-1.0, 1.0, 5), [-1.0, -0.5, 0.0, 0.5, 1.0]);
        assert_eq!(linspace(0.0, 100.0, 5), [0.0, 25.0, 50.0, 75.0, 100.0]);

        assert_eq!(modulo(7.0, 10.0), 7.0);
        assert_eq!(modulo(17.0, 10.0), 7.0);

        assert_eq!(norm(0.79, 0.0, 1.0), 0.79);
        assert_eq!(norm(16.0, 0.0, 100.0), 0.16);
        assert_eq!(norm(20.0, 15.0, 25.0), 0.5);
    }
}
