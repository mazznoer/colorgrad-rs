use alloc::vec;
use alloc::vec::Vec;

use libm::sqrtf;

use crate::utils::linspace;
use crate::{BlendMode, Color, Gradient, LinearGradient};

const MAX_DEPTH: u32 = 19;

pub(crate) fn linearize(g: &dyn Gradient, threshold: f32) -> LinearGradient {
    let (min, max) = g.domain();
    let mut positions = Vec::new();
    let threshold = if !threshold.is_finite() {
        0.007
    } else {
        threshold.clamp(0.003, 0.035)
    };

    let initial_stops: Vec<_> = linspace(min, max, 37).collect();

    // Adaptive Sampling
    for i in 0..initial_stops.len() - 1 {
        let t0 = initial_stops[i];
        let t1 = initial_stops[i + 1];
        positions.push(t0);
        subdivide(g, t0, t1, threshold, 0, &mut positions);
    }
    positions.push(max);

    // Sorting & Precision Cleanup
    positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
    positions.dedup_by(|a, b| (*a - *b).abs() < f32::EPSILON);

    // Prune Unnecessary Points
    let positions = remove_unnecessary(g, &positions, threshold);

    // Map to Colors
    let colors: Vec<Color> = positions.iter().map(|&t| g.at(t).clamp()).collect();

    LinearGradient::new(&colors, &positions, BlendMode::Rgb)
}

fn subdivide(g: &dyn Gradient, t0: f32, t1: f32, threshold: f32, depth: u32, stops: &mut Vec<f32>) {
    if depth >= MAX_DEPTH {
        return;
    }
    let mid = (t0 + t1) / 2.0;
    let c_mid_linear = g.at(t0).interpolate_rgb(&g.at(t1), 0.5).clamp();

    if color_diff(g.at(mid).clamp(), c_mid_linear) > threshold {
        subdivide(g, t0, mid, threshold, depth + 1, stops);
        stops.push(mid);
        subdivide(g, mid, t1, threshold, depth + 1, stops);
    }
}

fn remove_unnecessary(g: &dyn Gradient, pos: &[f32], threshold: f32) -> Vec<f32> {
    if pos.len() <= 2 {
        return pos.to_vec();
    }
    let mut out = vec![pos[0]];
    let mut last_idx = 0;

    for i in 1..pos.len() - 1 {
        let t_prev = pos[last_idx];
        let t_next = pos[i + 1];
        let t_curr = pos[i];

        let lerp_factor = (t_curr - t_prev) / (t_next - t_prev);
        let predicted = g
            .at(t_prev)
            .interpolate_rgb(&g.at(t_next), lerp_factor)
            .clamp();

        if color_diff(g.at(t_curr).clamp(), predicted) > threshold {
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
