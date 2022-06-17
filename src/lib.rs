//! # Overview
//!
//! Rust color scales library for data visualization, charts, games, maps, generative art and others.
//!
//! ## Usage
//!
//! Using preset gradient:
//! ```
//! let g = colorgrad::rainbow();
//!
//! assert_eq!(g.domain(), (0.0, 1.0)); // all preset gradients are in the domain [0..1]
//! assert_eq!(g.at(0.5).rgba_u8(), (175, 240, 91, 255));
//! assert_eq!(g.at(0.5).to_hex_string(), "#aff05b");
//! ```
//!
//! Custom gradient:
//! ```
//! use colorgrad::Color;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let g = colorgrad::CustomGradient::new()
//!     .colors(&[
//!         Color::from_rgb_u8(255, 0, 0),
//!         Color::from_rgb_u8(0, 255, 0),
//!     ])
//!     .build()?;
//!
//! assert_eq!(g.at(0.0).rgba_u8(), (255, 0, 0, 255));
//! assert_eq!(g.at(0.0).to_hex_string(), "#ff0000");
//! assert_eq!(g.at(1.0).to_hex_string(), "#00ff00");
//! # Ok(())
//! # }
//! ```
//!
//! ## Examples
//!
//! ### Gradient Image
//!
//! ```rust
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let grad = colorgrad::CustomGradient::new()
//!         .html_colors(&["deeppink", "gold", "seagreen"])
//!         .build()?;
//!
//!     let width = 1500;
//!     let height = 70;
//!
//!     let mut imgbuf = image::ImageBuffer::new(width, height);
//!
//!     for (x, _, pixel) in imgbuf.enumerate_pixels_mut() {
//!         let (r, g, b, a) = grad.at(x as f64 / width as f64).rgba_u8();
//!         *pixel = image::Rgba([r, g, b, a]);
//!     }
//!
//!     imgbuf.save("gradient.png")?;
//!     Ok(())
//! }
//! ```
//!
//! Example output:
//!
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/example-gradient.png)
//!
//! ### Colored Noise
//!
//! ```rust,ignore
//! use noise::NoiseFn;
//!
//! fn main() {
//!     let scale = 0.015;
//!
//!     let grad = colorgrad::rainbow().sharp(5, 0.15);
//!     let ns = noise::OpenSimplex::new();
//!     let mut imgbuf = image::ImageBuffer::new(600, 350);
//!
//!     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//!         let t = ns.get([x as f64 * scale, y as f64 * scale]);
//!         let (r, g, b, a) = grad.at(remap(t, -0.5, 0.5, 0.0, 1.0)).rgba_u8();
//!         *pixel = image::Rgba([r, g, b, a]);
//!     }
//!
//!     imgbuf.save("noise.png").unwrap();
//! }
//!
//! // Map t which is in range [a, b] to range [c, d]
//! fn remap(t: f64, a: f64, b: f64, c: f64, d: f64) -> f64 {
//!     (t - a) * ((d - c) / (b - a)) + c
//! }
//! ```
//!
//! Example output:
//!
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/example-noise.png)
//!
//! ## Preset Gradients
//!
//! [colorgrad::cubehelix_default()](fn.cubehelix_default.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/cubehelix_default.png)
//!
//! [colorgrad::turbo()](fn.turbo.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/turbo.png)
//!
//! [colorgrad::spectral()](fn.spectral.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/spectral.png)
//!
//! [colorgrad::viridis()](fn.viridis.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/viridis.png)
//!
//! [colorgrad::magma()](fn.magma.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/magma.png)
//!
//! [colorgrad::rainbow()](fn.rainbow.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rainbow.png)
//!
//! [colorgrad::sinebow()](fn.sinebow.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/sinebow.png)
//!
//! See more complete gradient preview and examples at [Github](https://github.com/mazznoer/colorgrad-rs).

use std::fmt;

pub use csscolorparser::{Color, ParseColorError};

mod builder;
pub use builder::{CustomGradient, CustomGradientError};

mod gradient;
pub use gradient::gimp::{parse_ggr, ParseGgrError};
use gradient::linear::LinearGradient;
pub use gradient::preset::*;
use gradient::sharp::SharpGradient;
use gradient::spline::spline_gradient;

/// Color blending mode
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BlendMode {
    Rgb,
    LinearRgb,
    Hsv,
    Oklab,
}

/// Interpolation mode
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Interpolation {
    Linear,
    Basis,
    CatmullRom,
}

trait GradientBase {
    fn at(&self, t: f64) -> Color;
}

/// The gradient
pub struct Gradient {
    gradient: Box<dyn GradientBase + Send + Sync>,
    dmin: f64,
    dmax: f64,
}

impl fmt::Debug for Gradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Gradient")
            .field("dmin", &self.dmin)
            .field("dmax", &self.dmax)
            .finish()
    }
}

impl Gradient {
    /// Get color at certain position
    pub fn at(&self, t: f64) -> Color {
        self.gradient.at(t)
    }

    /// Get color at certain position
    pub fn repeat_at(&self, t: f64) -> Color {
        let t = norm(t, self.dmin, self.dmax);
        self.gradient
            .at(self.dmin + modulo(t, 1.0) * (self.dmax - self.dmin))
    }

    /// Get color at certain position
    pub fn reflect_at(&self, t: f64) -> Color {
        let t = norm(t, self.dmin, self.dmax);
        self.gradient
            .at(self.dmin + (modulo(1.0 + t, 2.0) - 1.0).abs() * (self.dmax - self.dmin))
    }

    /// Get n colors evenly spaced across gradient
    pub fn colors(&self, n: usize) -> Vec<Color> {
        linspace(self.dmin, self.dmax, n)
            .iter()
            .map(|&t| self.gradient.at(t))
            .collect()
    }

    /// Get the gradient's domain min and max
    pub fn domain(&self) -> (f64, f64) {
        (self.dmin, self.dmax)
    }

    /// Get new hard-edge gradient
    ///
    /// ```
    /// let g = colorgrad::rainbow();
    /// ```
    /// ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rainbow.png)
    ///
    /// ```
    /// let g = colorgrad::rainbow().sharp(11, 0.);
    /// ```
    /// ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/rainbow-sharp.png)
    pub fn sharp(&self, segment: usize, smoothness: f64) -> Gradient {
        let colors = if segment > 1 {
            self.colors(segment)
        } else {
            vec![self.at(self.dmin), self.at(self.dmin)]
        };
        let gradbase = SharpGradient::new(&colors, self.domain(), smoothness);
        Gradient {
            gradient: Box::new(gradbase),
            dmin: self.dmin,
            dmax: self.dmax,
        }
    }
}

// ---

fn convert_colors(colors: &[Color], mode: BlendMode) -> Vec<[f64; 4]> {
    let mut result = Vec::with_capacity(colors.len());
    for col in colors.iter() {
        let (a, b, c, d) = match mode {
            BlendMode::Rgb => (col.r, col.g, col.b, col.a),
            BlendMode::LinearRgb => col.to_linear_rgba(),
            BlendMode::Oklab => col.to_oklaba(),
            BlendMode::Hsv => col.to_hsva(),
        };
        result.push([a, b, c, d]);
    }
    result
}

fn linear_interpolation(a: &[f64; 4], b: &[f64; 4], t: f64) -> [f64; 4] {
    [
        a[0] + t * (b[0] - a[0]),
        a[1] + t * (b[1] - a[1]),
        a[2] + t * (b[2] - a[2]),
        a[3] + t * (b[3] - a[3]),
    ]
}

#[inline]
fn interp_angle(a0: f64, a1: f64, t: f64) -> f64 {
    let delta = (((a1 - a0) % 360.0) + 540.0) % 360.0 - 180.0;
    (a0 + t * delta + 360.0) % 360.0
}

fn linspace(min: f64, max: f64, n: usize) -> Vec<f64> {
    if n == 1 {
        return vec![min];
    }

    let d = max - min;
    let l = n as f64 - 1.0;
    (0..n).map(|i| min + (i as f64 * d) / l).collect()
}

#[inline]
fn modulo(x: f64, y: f64) -> f64 {
    (x % y + y) % y
}

#[inline]
// Map t from range [a, b] to range [0, 1]
fn norm(t: f64, a: f64, b: f64) -> f64 {
    (t - a) * (1.0 / (b - a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linspace() {
        assert_eq!(linspace(0.0, 1.0, 0), vec![]);
        assert_eq!(linspace(0.0, 1.0, 1), vec![0.0]);
        assert_eq!(linspace(0.0, 1.0, 2), vec![0.0, 1.0]);
        assert_eq!(linspace(0.0, 1.0, 3), vec![0.0, 0.5, 1.0]);
        assert_eq!(linspace(-1.0, 1.0, 5), vec![-1.0, -0.5, 0.0, 0.5, 1.0]);
        assert_eq!(linspace(0.0, 100.0, 5), vec![0.0, 25.0, 50.0, 75.0, 100.0]);
    }
}
