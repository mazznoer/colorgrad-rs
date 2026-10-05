use alloc::boxed::Box;
use alloc::vec::Vec;

use libm::sqrtf;

use crate::{Color, LinearGradient};

const MAX_DEPTH: u32 = 8;
const SEEDS: usize = 36;

pub(crate) fn linearize<'a>(
    grad: Box<dyn Fn(f32) -> Color + 'a>,
    domain: (f32, f32),
    threshold: f32,
) -> LinearGradient {
    let threshold = if !threshold.is_finite() {
        0.007
    } else {
        threshold.clamp(0.001, 0.035)
    };

    let (min, max) = domain;
    let mut t0 = min;
    let mut c0 = grad(t0).clamp();
    let mut sf = StreamFilter::new(threshold, t0, c0, 500);

    // Adaptive Sampling
    for i in 1..=SEEDS {
        let t1 = min + (max - min) * (i as f32 / SEEDS as f32);
        let c1 = grad(t1).clamp();

        // Subdivide interval [t0, t1]
        subdivide(&grad, t0, t1, c0, c1, threshold, 0, &mut sf);

        // Push right seed boundary
        sf.push(t1, c1);

        t0 = t1;
        c0 = c1;
    }

    let positions = sf.finish();
    let stops: Vec<_> = positions.iter().map(|&t| (t, grad(t).to_array())).collect();

    LinearGradient::from_rgba_data(stops).unwrap()
}

#[allow(clippy::too_many_arguments)]
fn subdivide<'a>(
    grad: &(dyn Fn(f32) -> Color + 'a),
    t0: f32,
    t1: f32,
    c0: Color,
    c1: Color,
    threshold: f32,
    depth: u32,
    sf: &mut StreamFilter,
) {
    if depth > MAX_DEPTH {
        return;
    }

    let mid = (t0 + t1) / 2.0;
    let c_mid_actual = grad(mid).clamp();
    let c_mid_linear = c0.interpolate_rgb(&c1, 0.5);

    if color_diff(c_mid_actual, c_mid_linear) > threshold {
        // Left branch (t0 -> mid)
        subdivide(grad, t0, mid, c0, c_mid_actual, threshold, depth + 1, sf);

        // In-order midpoint push
        sf.push(mid, c_mid_actual);

        // Right branch (mid -> t1)
        subdivide(grad, mid, t1, c_mid_actual, c1, threshold, depth + 1, sf);
    }
}

/// Streaming pruner that maintains a 3-point sliding window with cached colors
/// to perform on-the-fly collinear pruning and deduplication.
struct StreamFilter {
    threshold: f32,
    committed: Vec<f32>,
    color_a: Color,
    candidate: Option<(f32, Color)>,
}

impl StreamFilter {
    fn new(threshold: f32, min: f32, initial_color: Color, capacity: usize) -> Self {
        let mut committed = Vec::with_capacity(capacity);
        committed.push(min);
        Self {
            threshold,
            committed,
            color_a: initial_color,
            candidate: None,
        }
    }

    fn push(&mut self, c: f32, color_c: Color) {
        let a = *self.committed.last().unwrap();

        // Skip duplicates close to last committed point
        if (c - a).abs() < f32::EPSILON {
            return;
        }

        if let Some((b, color_b)) = self.candidate {
            // Deduplicate candidate against incoming point
            if (c - b).abs() < f32::EPSILON {
                self.candidate = Some((c, color_c));
                return;
            }

            // Evaluate if candidate B is collinear between A and C using cached colors
            let lerp_factor = (b - a) / (c - a);
            let c_curr_linear = self.color_a.interpolate_rgb(&color_c, lerp_factor);

            if color_diff(color_b, c_curr_linear) > self.threshold {
                // B is essential -> commit B and update last committed color A
                self.committed.push(b);
                self.color_a = color_b;
            }
            // If B is collinear, it drops automatically when C replaces it
        }

        self.candidate = Some((c, color_c));
    }

    fn finish(mut self) -> Vec<f32> {
        if let Some((c, _)) = self.candidate {
            let a = *self.committed.last().unwrap();
            if (c - a).abs() >= f32::EPSILON {
                self.committed.push(c);
            }
        }
        self.committed
    }
}

// Euclidean distance between two colors in RGBA space, normalized to [0.0, 1.0].
fn color_diff(a: Color, b: Color) -> f32 {
    let dr = a.r - b.r;
    let dg = a.g - b.g;
    let db = a.b - b.b;
    let da = a.a - b.a;
    sqrtf(dr * dr + dg * dg + db * db + da * da) / 2.0
}

#[cfg(test)]
mod t {
    use super::{Color, color_diff};

    #[test]
    fn color_diff_() {
        let a = Color::new(1.0, 1.0, 1.0, 1.0);
        let b = Color::new(0.0, 0.0, 0.0, 0.0);

        assert!((color_diff(a, b) - 1.0).abs() < f32::EPSILON);
        assert!((color_diff(a, a)).abs() < f32::EPSILON);
        assert!((color_diff(b, b)).abs() < f32::EPSILON);
    }
}
