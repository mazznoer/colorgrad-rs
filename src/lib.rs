//! # Overview
//!
//! Rust color scales library for data visualization, charts, games, maps, generative art and others.
//!
//! ## Usage
//!
//! Using preset gradient:
//! ```ignore
//! let g = colorgrad::rainbow();
//!
//! assert_eq!(g.domain(), (0.0, 1.0)); // all preset gradients are in the domain [0..1]
//! assert_eq!(g.at(0.5).to_rgba8(), [175, 240, 91, 255]);
//! assert_eq!(g.at(0.5).to_hex_string(), "#aff05b");
//! ```
//!
//! Custom gradient:
//! ```ignore
//! use colorgrad::Color;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let g = colorgrad::CustomGradient::new()
//!     .colors(&[
//!         Color::from_rgba8(255, 0, 0, 255),
//!         Color::from_rgba8(0, 255, 0, 255),
//!     ])
//!     .build()?;
//!
//! assert_eq!(g.at(0.0).to_rgba8(), [255, 0, 0, 255]);
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
//! ```rust,ignore
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
//!         let rgba = grad.at(x as f32 / width as f32).to_rgba8();
//!         *pixel = image::Rgba(rgba);
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
//!         let t = ns.get([x as f32 * scale, y as f32 * scale]);
//!         let rgba = grad.at(remap(t, -0.5, 0.5, 0.0, 1.0)).to_rgba8();
//!         *pixel = image::Rgba(rgba);
//!     }
//!
//!     imgbuf.save("noise.png").unwrap();
//! }
//!
//! // Map t which is in range [a, b] to range [c, d]
//! fn remap(t: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
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

pub use csscolorparser::{Color, ParseColorError};

mod builder;
pub use builder::{GradientBuilder, GradientBuilderError};

mod gradient;
pub use gradient::basis::BasisGradient;
pub use gradient::catmull_rom::CatmullRomGradient;
pub use gradient::linear::LinearGradient;
pub use gradient::sharp::SharpGradient;

#[cfg(feature = "preset")]
pub use gradient::preset;

#[cfg(feature = "ggr")]
pub use gradient::gimp::{GimpGradient, ParseGgrError};

/// Color blending mode
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BlendMode {
    Rgb,
    LinearRgb,
    Oklab,
}

pub trait Gradient {
    /// Get color at certain position
    fn at(&self, t: f32) -> Color;

    /// Get color at certain position
    fn repeat_at(&self, t: f32) -> Color {
        let (dmin, dmax) = self.domain();
        let t = norm(t, dmin, dmax);
        self.at(dmin + modulo(t, 1.0) * (dmax - dmin))
    }

    /// Get color at certain position
    fn reflect_at(&self, t: f32) -> Color {
        let (dmin, dmax) = self.domain();
        let t = norm(t, dmin, dmax);
        self.at(dmin + (modulo(1.0 + t, 2.0) - 1.0).abs() * (dmax - dmin))
    }

    /// Get the gradient's domain min and max
    fn domain(&self) -> (f32, f32) {
        (0.0, 1.0)
    }

    /// Get n colors evenly spaced across gradient
    fn colors(&self, n: usize) -> Vec<Color> {
        let (dmin, dmax) = self.domain();

        linspace(dmin, dmax, n)
            .iter()
            .map(|&t| self.at(t))
            .collect()
    }

    /// Get new hard-edge gradient
    ///
    /// ```ignore
    /// let g = colorgrad::rainbow();
    /// ```
    /// ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rainbow.png)
    ///
    /// ```ignore
    /// let g = colorgrad::rainbow().sharp(11, 0.);
    /// ```
    /// ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/rainbow-sharp.png)
    fn sharp(&self, segment: u16, smoothness: f32) -> SharpGradient {
        let colors = if segment > 1 {
            self.colors(segment.into())
        } else {
            vec![self.at(self.domain().0), self.at(self.domain().0)]
        };
        SharpGradient::new(&colors, self.domain(), smoothness)
    }
}

fn convert_colors(colors: &[Color], mode: BlendMode) -> Vec<[f32; 4]> {
    colors
        .iter()
        .map(|c| match mode {
            BlendMode::Rgb => c.to_array(),
            BlendMode::LinearRgb => c.to_linear_rgba(),
            BlendMode::Oklab => c.to_oklaba(),
        })
        .collect()
}

fn linspace(min: f32, max: f32, n: usize) -> Vec<f32> {
    if n == 1 {
        return vec![min];
    }

    let d = max - min;
    let l = n as f32 - 1.0;
    (0..n).map(|i| min + (i as f32 * d) / l).collect()
}

#[inline]
fn modulo(x: f32, y: f32) -> f32 {
    (x % y + y) % y
}

#[inline]
// Map t from range [a, b] to range [0, 1]
fn norm(t: f32, a: f32, b: f32) -> f32 {
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
