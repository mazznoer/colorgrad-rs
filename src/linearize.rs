use alloc::vec;
use alloc::vec::Vec;

use libm::powf;

use crate::utils::linspace;
use crate::{BlendMode, Color, Gradient, LinearGradient};

const MAX_DEPTH: u32 = 7;

pub(crate) fn linearize(g: &dyn Gradient, threshold: f32) -> LinearGradient {
    let (min, max) = g.domain();
    let mut positions = Vec::new();
    let threshold_sq = powf(threshold.clamp(0.005, 0.1), 2.0);

    let initial_stops: Vec<_> = linspace(min, max, 17).collect();

    // Adaptive Sampling
    for i in 0..initial_stops.len() - 1 {
        let t0 = initial_stops[i];
        let t1 = initial_stops[i + 1];
        positions.push(t0);
        subdivide(g, t0, t1, threshold_sq, 0, &mut positions);
    }
    positions.push(max);

    // Sorting & Precision Cleanup
    positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
    positions.dedup_by(|a, b| (*a - *b).abs() < 1e-6);

    // Prune Unnecessary Points
    let positions = remove_unnecessary(g, &positions, threshold_sq);

    // Map to Colors
    let colors: Vec<Color> = positions.iter().map(|&t| g.at(t)).collect();

    LinearGradient::new(&colors, &positions, BlendMode::Rgb)
}

fn subdivide(g: &dyn Gradient, t0: f32, t1: f32, thresh_sq: f32, depth: u32, stops: &mut Vec<f32>) {
    if depth >= MAX_DEPTH {
        return;
    }
    let mid = (t0 + t1) / 2.0;
    let c_mid_linear = g.at(t0).interpolate_rgb(&g.at(t1), 0.5);

    if color_diff_sq(g.at(mid), c_mid_linear) > thresh_sq {
        subdivide(g, t0, mid, thresh_sq, depth + 1, stops);
        stops.push(mid);
        subdivide(g, mid, t1, thresh_sq, depth + 1, stops);
    }
}

fn remove_unnecessary(g: &dyn Gradient, pos: &[f32], thresh_sq: f32) -> Vec<f32> {
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
        let predicted = g.at(t_prev).interpolate_rgb(&g.at(t_next), lerp_factor);

        if color_diff_sq(g.at(t_curr), predicted) > thresh_sq {
            out.push(t_curr);
            last_idx = i;
        }
    }
    out.push(*pos.last().unwrap());
    out
}

// Squared distance
fn color_diff_sq(c1: Color, c2: Color) -> f32 {
    powf(c1.r - c2.r, 2.0)
        + powf(c1.g - c2.g, 2.0)
        + powf(c1.b - c2.b, 2.0)
        + powf(c1.a - c2.a, 2.0)
}
