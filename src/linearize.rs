use alloc::boxed::Box;
use alloc::vec::Vec;

use libm::sqrtf;

use crate::{Color, LinearGradient};

const MAX_DEPTH: u32 = 7;
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
    let mut positions = Vec::with_capacity(500);
    positions.push(min);

    let mut t0 = min;
    let mut c0 = grad(t0).clamp();

    // Adaptive Sampling
    for i in 1..=SEEDS {
        let t1 = min + (max - min) * (i as f32 / SEEDS as f32);
        let c1 = grad(t1).clamp();

        // Subdivide interval [t0, t1]
        subdivide(&grad, t0, t1, c0, c1, threshold, 0, &mut positions);

        // Push right seed boundary
        positions.push(t1);

        t0 = t1;
        c0 = c1;
    }

    // Prune Unnecessary Points
    let positions = remove_unnecessary(&grad, &positions, threshold);

    let stops: Vec<_> = positions.iter().map(|&t| (t, grad(t).to_array())).collect();

    LinearGradient::from_rgba_data(stops).unwrap()
}

fn subdivide<'a>(
    grad: &(dyn Fn(f32) -> Color + 'a),
    t0: f32,
    t1: f32,
    c0: Color,
    c1: Color,
    threshold: f32,
    depth: u32,
    stops: &mut Vec<f32>,
) {
    if depth >= MAX_DEPTH {
        return;
    }

    let mid = (t0 + t1) / 2.0;
    let c_mid_actual = grad(mid).clamp();
    let c_mid_linear = c0.interpolate_rgb(&c1, 0.5).clamp();

    if color_diff(c_mid_actual, c_mid_linear) > threshold {
        // Left branch (t0 -> mid)
        subdivide(grad, t0, mid, c0, c_mid_actual, threshold, depth + 1, stops);

        // In-order midpoint push
        stops.push(mid);

        // Right branch (mid -> t1)
        subdivide(grad, mid, t1, c_mid_actual, c1, threshold, depth + 1, stops);
    }
}

fn remove_unnecessary<'a>(
    grad: &(dyn Fn(f32) -> Color + 'a),
    pos: &[f32],
    threshold: f32,
) -> Vec<f32> {
    if pos.len() <= 2 {
        return pos.to_vec();
    }

    let mut out = Vec::with_capacity(pos.len());
    out.push(pos[0]);
    let mut last_idx = 0;

    for i in 1..pos.len() - 1 {
        let t_prev = pos[last_idx];
        let t_curr = pos[i];

        // skip duplicate position
        if (t_prev - t_curr).abs() < f32::EPSILON {
            continue;
        }

        let t_next = pos[i + 1];
        let lerp_factor = (t_curr - t_prev) / (t_next - t_prev);

        let c_prev = grad(t_prev);
        let c_next = grad(t_next);
        let c_curr_actual = grad(t_curr).clamp();
        let c_curr_linear = c_prev.interpolate_rgb(&c_next, lerp_factor).clamp();

        if color_diff(c_curr_actual, c_curr_linear) > threshold {
            out.push(t_curr);
            last_idx = i;
        }
    }

    out.push(*pos.last().unwrap());
    out
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
