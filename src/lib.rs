//! # Overview
//!
//! Rust color scales library for data visualization, charts, games, maps, generative art and others.
//!
//! ## Usage
//!
#![cfg_attr(
    feature = "preset",
    doc = r##"
Using preset gradient:
```
use colorgrad::Gradient;
let g = colorgrad::preset::rainbow();

assert_eq!(g.domain(), (0.0, 1.0)); // all preset gradients are in the domain [0..1]
assert_eq!(g.at(0.5).to_rgba8(), [175, 240, 91, 255]);
assert_eq!(g.at(0.5).to_hex_string(), "#aff05b");
```"##
)]
//!
//! Custom gradient:
//! ```
//! use colorgrad::{Color, Gradient};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let g = colorgrad::GradientBuilder::new()
//!     .colors(&[
//!         Color::from_rgba8(255, 0, 0, 255),
//!         Color::from_rgba8(0, 255, 0, 255),
//!     ])
//!     .build::<colorgrad::LinearGradient>()?;
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
#![cfg_attr(
    feature = "named-colors",
    doc = r##"
```
use colorgrad::Gradient;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grad = colorgrad::GradientBuilder::new()
        .html_colors(&["deeppink", "gold", "seagreen"])
        .build::<colorgrad::CatmullRomGradient>()?;

    let width = 1500;
    let height = 70;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, _, pixel) in imgbuf.enumerate_pixels_mut() {
        let rgba = grad.at(x as f32 / width as f32).to_rgba8();
        *pixel = image::Rgba(rgba);
    }

    imgbuf.save("gradient.png")?;
    Ok(())
}
```

Example output:

![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/example-gradient.png)
"##
)]
//!
//! ### Colored Noise
//!
//! ```ignore
//! use colorgrad::Gradient;
//! use noise::NoiseFn;
//!
//! fn main() {
//!     let scale = 0.015;
//!
//!     let grad = colorgrad::preset::rainbow().sharp(5, 0.15);
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
//! [colorgrad::preset::cubehelix_default()](preset)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/cubehelix_default.png)
//!
//! [colorgrad::preset::turbo()](preset)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/turbo.png)
//!
//! [colorgrad::preset::spectral()](preset)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/spectral.png)
//!
//! [colorgrad::preset::viridis()](preset)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/viridis.png)
//!
//! [colorgrad::preset::magma()](preset)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/magma.png)
//!
//! [colorgrad::preset::rainbow()](preset)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rainbow.png)
//!
//! [colorgrad::preset::sinebow()](preset)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/sinebow.png)
//!
//! See more complete gradient preview and examples at [Github](https://github.com/mazznoer/colorgrad-rs).

pub use csscolorparser::{Color, ParseColorError};

mod builder;
pub use builder::{GradientBuilder, GradientBuilderError};

mod css_gradient;

mod gradient;
pub use gradient::basis::BasisGradient;
pub use gradient::catmull_rom::CatmullRomGradient;
pub use gradient::inverse::InverseGradient;
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
    #[cfg(feature = "lab")]
    Lab,
}

pub trait Gradient: CloneGradient {
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
            .map(|&t| self.at(t).clamp())
            .collect()
    }

    #[cfg_attr(
        feature = "preset",
        doc = r##"
    Get new hard-edge gradient

    ```
    let g = colorgrad::preset::rainbow();
    ```
    ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rainbow.png)

    ```
    use colorgrad::Gradient;
    let g = colorgrad::preset::rainbow().sharp(11, 0.0);
    ```
    ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/rainbow-sharp.png)
    "##
    )]
    fn sharp(&self, segment: u16, smoothness: f32) -> SharpGradient {
        let colors = if segment > 1 {
            self.colors(segment.into())
        } else {
            vec![self.at(self.domain().0), self.at(self.domain().0)]
        };
        SharpGradient::new(&colors, self.domain(), smoothness)
    }

    /// Get a new gradient that inverts the gradient
    ///
    /// The minimum value of the inner gradient will be the maximum value of the inverse gradient and
    /// vice versa.
    ///
    /// # Example
    ///
    /// ```
    /// use colorgrad::Gradient;
    /// let rainbow = colorgrad::preset::rainbow();
    /// let inverse = rainbow.inverse();
    /// ```
    fn inverse(&self) -> InverseGradient {
        InverseGradient::new(self.clone_gradient())
    }
}

pub trait CloneGradient {
    fn clone_gradient(&self) -> Box<dyn Gradient>;
}

impl<T> CloneGradient for T
where
    T: Gradient + Clone + 'static,
{
    fn clone_gradient(&self) -> Box<dyn Gradient> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Gradient> {
    fn clone(&self) -> Self {
        self.clone_gradient()
    }
}

fn convert_colors(colors: &[Color], mode: BlendMode) -> Vec<[f32; 4]> {
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

    #[test]
    fn inverse() {
        let rainbow = preset::rainbow();
        let inverse = rainbow.inverse();

        // Note that this test is just to make sure the inverse trait method is working correctly.
        // The values are properly tested in the inverse module tests.
        assert_eq!(inverse.at(0.0), rainbow.at(1.0));
        assert_eq!(inverse.at(1.0), rainbow.at(0.0));
    }
}
