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

use std::{error, fmt};

mod gimp_gradient;
mod preset;
mod spline;

pub use csscolorparser::{Color, ParseColorError};
pub use gimp_gradient::{parse_ggr, ParseGgrError};
pub use preset::*;
use spline::spline_gradient;

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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CustomGradientError {
    InvalidHtmlColor(Vec<String>),
    WrongDomainCount,
    WrongDomain,
}

impl fmt::Display for CustomGradientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomGradientError::InvalidHtmlColor(ref colors) => {
                write!(
                    f,
                    "Invalid html colors:{}",
                    colors
                        .iter()
                        .map(|x| format!("'{}'", x))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            CustomGradientError::WrongDomainCount => f.write_str("Wrong domain count"),
            CustomGradientError::WrongDomain => f.write_str("Wrong domain"),
        }
    }
}

impl error::Error for CustomGradientError {}

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

#[derive(Debug)]
struct LinearGradient {
    stops: Vec<(f64, [f64; 4])>,
    dmin: f64,
    dmax: f64,
    mode: BlendMode,
    first_color: Color,
    last_color: Color,
}

impl GradientBase for LinearGradient {
    fn at(&self, t: f64) -> Color {
        if t <= self.dmin {
            return self.first_color.clone();
        }

        if t >= self.dmax {
            return self.last_color.clone();
        }

        for segment in self.stops.windows(2) {
            let (pos_0, col_0) = &segment[0];
            let (pos_1, col_1) = &segment[1];
            if (*pos_0 <= t) && (t <= *pos_1) {
                let t = (t - pos_0) / (pos_1 - pos_0);
                let [a, b, c, d] = linear_interpolation(col_0, col_1, t);
                match self.mode {
                    BlendMode::Rgb => return Color::from_rgba(a, b, c, d),
                    BlendMode::LinearRgb => return Color::from_linear_rgba(a, b, c, d),
                    BlendMode::Oklab => return Color::from_oklaba(a, b, c, d),
                    BlendMode::Hsv => {
                        let hue = interp_angle(col_0[0], col_1[0], t);
                        return Color::from_hsva(hue, b, c, d);
                    }
                }
            }
        }

        self.last_color.clone()
    }
}

#[derive(Debug, Clone)]
pub struct SharpGradient {
    stops: Vec<(f64, Color)>,
    domain: (f64, f64),
    first_color: Color,
    last_color: Color,
}

impl SharpGradient {
    fn new(colors_in: &[Color], domain: (f64, f64), t: f64) -> Self {
        let n = colors_in.len();
        let mut colors = Vec::with_capacity(n * 2);

        for c in colors_in {
            colors.push(c.clone());
            colors.push(c.clone());
        }

        let t = t.clamp(0.0, 1.0) * (domain.1 - domain.0) / n as f64 / 4.0;
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

        let first_color = colors_in[0].clone();
        let last_color = colors_in[n - 1].clone();

        Self {
            stops: positions
                .iter()
                .zip(colors.iter())
                .map(|(p, c)| (*p, c.clone()))
                .collect(),
            domain,
            first_color,
            last_color,
        }
    }
}

impl GradientBase for SharpGradient {
    fn at(&self, t: f64) -> Color {
        if t <= self.domain.0 {
            return self.first_color.clone();
        }

        if t >= self.domain.1 {
            return self.last_color.clone();
        }

        for (i, segment) in self.stops.windows(2).enumerate() {
            let (pos_0, col_0) = &segment[0];
            let (pos_1, col_1) = &segment[1];

            if (*pos_0 <= t) && (t <= *pos_1) {
                if i & 1 == 0 {
                    return col_0.clone();
                }
                let t = (t - pos_0) / (pos_1 - pos_0);
                return col_0.interpolate_rgb(col_1, t);
            }
        }

        Color::from_rgba(0.0, 0.0, 0.0, 1.0)
    }
}

/// Create custom gradient
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// use colorgrad::Color;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let grad = colorgrad::CustomGradient::new()
///     .colors(&[
///         Color::from_rgb_u8(255, 0, 0),
///         Color::from_rgb(0.0, 0.0, 1.0),
///     ])
///     .build()?;
///
/// assert_eq!(grad.domain(), (0.0, 1.0)); // default domain
/// assert_eq!(grad.at(0.0).rgba_u8(), (255, 0, 0, 255));
/// assert_eq!(grad.at(1.0).rgba_u8(), (0, 0, 255, 255));
/// # Ok(())
/// # }
/// ```
///
/// ## Using web color format string
///
/// ```
/// # use std::error::Error;
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let grad = colorgrad::CustomGradient::new()
///     .html_colors(&["deeppink", "gold", "seagreen"])
///     .domain(&[0.0, 100.0])
///     .mode(colorgrad::BlendMode::Rgb)
///     .build()?;
///
/// assert_eq!(grad.at(0.0).rgba_u8(), (255, 20, 147, 255));
/// assert_eq!(grad.at(100.0).rgba_u8(), (46, 139, 87, 255));
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct CustomGradient {
    colors: Vec<Color>,
    pos: Vec<f64>,
    mode: BlendMode,
    interpolation: Interpolation,
    invalid_html_colors: Vec<String>,
}

impl CustomGradient {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CustomGradient {
        CustomGradient {
            colors: Vec::new(),
            pos: Vec::new(),
            mode: BlendMode::Rgb,
            interpolation: Interpolation::Linear,
            invalid_html_colors: Vec::new(),
        }
    }

    /// Set gradient color
    pub fn colors<'a>(&'a mut self, colors: &[Color]) -> &'a mut CustomGradient {
        for c in colors {
            self.colors.push(c.clone());
        }
        self
    }

    /// Set gradient color using web / CSS color format.
    ///
    /// ## Supported Color Format
    ///
    /// * [Named colors](https://www.w3.org/TR/css-color-4/#named-colors)
    /// * RGB hexadecimal
    ///      + Short format `#rgb`
    ///      + Short format with alpha `#rgba`
    ///      + Long format `#rrggbb`
    ///      + Long format with alpha `#rrggbbaa`
    /// * `rgb()` and `rgba()`
    /// * `hsl()` and `hsla()`
    /// * `hwb()`
    /// * `hsv()` - not in CSS standard.
    pub fn html_colors<'a>(&'a mut self, html_colors: &[&str]) -> &'a mut CustomGradient {
        for s in html_colors {
            if let Ok(c) = csscolorparser::parse(s) {
                self.colors.push(c);
            } else {
                self.invalid_html_colors.push(s.to_string());
            }
        }
        self
    }

    /// Set the gradient domain and/or color position.
    pub fn domain<'a>(&'a mut self, pos: &[f64]) -> &'a mut CustomGradient {
        self.pos = pos.to_vec();
        self
    }

    /// Set the color blending mode
    #[allow(clippy::needless_lifetimes)]
    pub fn mode<'a>(&'a mut self, mode: BlendMode) -> &'a mut CustomGradient {
        self.mode = mode;
        self
    }

    /// Set the interpolation mode
    #[allow(clippy::needless_lifetimes)]
    pub fn interpolation<'a>(&'a mut self, mode: Interpolation) -> &'a mut CustomGradient {
        self.interpolation = mode;
        self
    }

    /// Build the gradient
    pub fn build(&self) -> Result<Gradient, CustomGradientError> {
        if !self.invalid_html_colors.is_empty() {
            return Err(CustomGradientError::InvalidHtmlColor(
                self.invalid_html_colors.clone(),
            ));
        }

        let colors = if self.colors.is_empty() {
            vec![
                Color::from_rgb(0.0, 0.0, 0.0),
                Color::from_rgb(1.0, 1.0, 1.0),
            ]
        } else if self.colors.len() == 1 {
            vec![self.colors[0].clone(), self.colors[0].clone()]
        } else {
            self.colors.to_vec()
        };

        let pos = if self.pos.is_empty() {
            linspace(0.0, 1.0, colors.len())
        } else if self.pos.len() == colors.len() {
            for p in self.pos.windows(2) {
                if p[0] > p[1] {
                    return Err(CustomGradientError::WrongDomain);
                }
            }
            self.pos.to_vec()
        } else if self.pos.len() == 2 {
            if self.pos[0] >= self.pos[1] {
                return Err(CustomGradientError::WrongDomain);
            }
            linspace(self.pos[0], self.pos[1], colors.len())
        } else {
            return Err(CustomGradientError::WrongDomainCount);
        };

        if let Interpolation::Linear = self.interpolation {
            let first_color = colors[0].clone();
            let last_color = colors[colors.len() - 1].clone();
            let colors = convert_colors(&colors, self.mode);
            let gradbase = LinearGradient {
                stops: pos
                    .iter()
                    .zip(colors.iter())
                    .map(|(p, c)| (*p, *c))
                    .collect(),
                dmin: pos[0],
                dmax: pos[pos.len() - 1],
                mode: self.mode,
                first_color,
                last_color,
            };

            return Ok(Gradient {
                gradient: Box::new(gradbase),
                dmin: pos[0],
                dmax: pos[pos.len() - 1],
            });
        }

        Ok(spline_gradient(
            &colors,
            &pos,
            self.mode,
            self.interpolation,
        ))
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
