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
//! assert_eq!(g.domain(), (0., 1.)); // all preset gradients are in the domain [0..1]
//! assert_eq!(g.at(0.5).rgba_u8(), (175, 240, 91, 255));
//! assert_eq!(g.at(0.5).to_hex_string(), "#aff05b");
//! ```
//!
//! Custom gradient:
//! ```
//! # use std::error::Error;
//! use colorgrad::Color;
//!
//! # fn main() -> Result<(), Box<dyn Error>> {
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
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let grad = colorgrad::CustomGradient::new()
//!         .html_colors(&["deeppink", "gold", "seagreen"])
//!         .build()?;
//!
//!     let w = 1500;
//!     let h = 70;
//!     let fw = w as f64;
//!
//!     let mut imgbuf = image::ImageBuffer::new(w, h);
//!
//!     for (x, _, pixel) in imgbuf.enumerate_pixels_mut() {
//!         let (r, g, b, _) = grad.at(x as f64 / fw).rgba_u8();
//!         *pixel = image::Rgb([r, g, b]);
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
//!     let w = 600;
//!     let h = 350;
//!     let scale = 0.015;
//!
//!     let grad = colorgrad::rainbow().sharp(5, 0.15);
//!     let ns = noise::OpenSimplex::new();
//!     let mut imgbuf = image::ImageBuffer::new(w, h);
//!
//!     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//!         let t = ns.get([x as f64 * scale, y as f64 * scale]);
//!         let (r, g, b, _) = grad.at(remap(t, -0.5, 0.5, 0.0, 1.0)).rgba_u8();
//!         *pixel = image::Rgb([r, g, b]);
//!     }
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

#![allow(clippy::many_single_char_names)]

pub use csscolorparser::Color;

use std::error::Error as StdError;
use std::f64::consts::PI;
use std::fmt;

mod spline;
use spline::{preset_spline, spline_gradient};

const PI1_3: f64 = PI / 3.;
const PI2_3: f64 = PI * 2. / 3.;

/// Color blending mode
#[derive(Debug, Copy, Clone)]
pub enum BlendMode {
    Rgb,
    LinearRgb,
    Hsv,
    Oklab,
}

/// Interpolation mode
#[derive(Debug, Copy, Clone)]
pub enum Interpolation {
    Linear,
    Basis,
    CatmullRom,
}

#[derive(Debug)]
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

impl StdError for CustomGradientError {}

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
            .at(self.dmin + modulo(t, 1.) * (self.dmax - self.dmin))
    }

    /// Get color at certain position
    pub fn reflect_at(&self, t: f64) -> Color {
        let t = norm(t, self.dmin, self.dmax);
        self.gradient
            .at(self.dmin + (modulo(1. + t, 2.) - 1.).abs() * (self.dmax - self.dmin))
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
        if segment < 2 {
            let gradbase = SharpGradient {
                colors: vec![self.gradient.at(self.dmin)],
                pos: vec![self.dmin, self.dmax],
                n: 0,
                dmin: self.dmin,
                dmax: self.dmax,
            };
            return Gradient {
                gradient: Box::new(gradbase),
                dmin: self.dmin,
                dmax: self.dmax,
            };
        }
        if smoothness > 0. {
            return sharp_gradient_x(self, segment, smoothness);
        }
        sharp_gradient(self, segment)
    }
}

#[derive(Debug)]
struct LinearGradient {
    colors: Vec<Color>,
    pos: Vec<f64>,
    count: usize,
    dmin: f64,
    dmax: f64,
    mode: BlendMode,
}

impl GradientBase for LinearGradient {
    fn at(&self, t: f64) -> Color {
        if t < self.dmin {
            return self.colors[0].clone();
        }
        if t > self.dmax {
            return self.colors[self.count].clone();
        }
        for (pos, col) in self.pos.windows(2).zip(self.colors.windows(2)) {
            if (pos[0] <= t) && (t <= pos[1]) {
                let t = (t - pos[0]) / (pos[1] - pos[0]);
                match self.mode {
                    BlendMode::Rgb => return col[0].interpolate_rgb(&col[1], t),
                    BlendMode::LinearRgb => return col[0].interpolate_lrgb(&col[1], t),
                    BlendMode::Hsv => return col[0].interpolate_hsv(&col[1], t),
                    BlendMode::Oklab => return col[0].interpolate_oklab(&col[1], t),
                }
            }
        }
        self.colors[0].clone()
    }
}

#[derive(Debug)]
struct SharpGradient {
    colors: Vec<Color>,
    pos: Vec<f64>,
    n: usize,
    dmin: f64,
    dmax: f64,
}

impl GradientBase for SharpGradient {
    fn at(&self, t: f64) -> Color {
        if t < self.dmin {
            return self.colors[0].clone();
        }
        if t > self.dmax {
            return self.colors[self.n].clone();
        }
        for (pos, col) in self.pos.windows(2).zip(self.colors.iter()) {
            if (pos[0] <= t) && (t <= pos[1]) {
                return col.clone();
            }
        }
        self.colors[0].clone()
    }
}

fn sharp_gradient(grad: &Gradient, n: usize) -> Gradient {
    let (dmin, dmax) = grad.domain();
    let gradbase = SharpGradient {
        colors: grad.colors(n),
        pos: linspace(dmin, dmax, n + 1),
        n: n - 1,
        dmin,
        dmax,
    };
    Gradient {
        gradient: Box::new(gradbase),
        dmin,
        dmax,
    }
}

#[derive(Debug)]
struct SharpGradientX {
    colors: Vec<Color>,
    pos: Vec<f64>,
    dmin: f64,
    dmax: f64,
    last_idx: usize,
}

impl GradientBase for SharpGradientX {
    fn at(&self, t: f64) -> Color {
        if t < self.dmin {
            return self.colors[0].clone();
        }
        if t > self.dmax {
            return self.colors[self.last_idx].clone();
        }
        for (i, (pos, col)) in self.pos.windows(2).zip(self.colors.windows(2)).enumerate() {
            if (pos[0] <= t) && (t <= pos[1]) {
                if i % 2 == 0 {
                    return col[0].clone();
                }
                let t = (t - pos[0]) / (pos[1] - pos[0]);
                return col[0].interpolate_rgb(&col[1], t);
            }
        }
        self.colors[0].clone()
    }
}

fn sharp_gradient_x(grad: &Gradient, n: usize, t: f64) -> Gradient {
    let mut colors = Vec::with_capacity(n * 2);
    for c in grad.colors(n) {
        colors.push(c.clone());
        colors.push(c.clone());
    }
    let (dmin, dmax) = grad.domain();
    let t = clamp0_1(t) * (dmax - dmin) / n as f64 / 4.;
    let p = linspace(dmin, dmax, n + 1);
    let mut pos = Vec::with_capacity(n * 2);
    let mut j = 0;
    for i in 0..n {
        pos.push(p[i]);
        if j > 0 {
            pos[j] += t;
        }
        j += 1;
        pos.push(p[i + 1]);
        if j < colors.len() - 1 {
            pos[j] -= t;
        }
        j += 1;
    }
    let gradbase = SharpGradientX {
        colors,
        pos,
        last_idx: n * 2 - 1,
        dmin,
        dmax,
    };
    Gradient {
        gradient: Box::new(gradbase),
        dmin,
        dmax,
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
///         Color::from_rgb(0., 0., 1.),
///     ])
///     .build()?;
///
/// assert_eq!(grad.domain(), (0., 1.)); // default domain
/// assert_eq!(grad.at(0.).rgba_u8(), (255, 0, 0, 255));
/// assert_eq!(grad.at(1.).rgba_u8(), (0, 0, 255, 255));
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
///     .domain(&[0., 100.])
///     .mode(colorgrad::BlendMode::Rgb)
///     .build()?;
///
/// assert_eq!(grad.at(0.).rgba_u8(), (255, 20, 147, 255));
/// assert_eq!(grad.at(100.).rgba_u8(), (46, 139, 87, 255));
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
            vec![Color::from_rgb(0., 0., 0.), Color::from_rgb(1., 1., 1.)]
        } else if self.colors.len() == 1 {
            vec![self.colors[0].clone(), self.colors[0].clone()]
        } else {
            self.colors.to_vec()
        };

        let pos = if self.pos.is_empty() {
            linspace(0., 1., colors.len())
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
            let gradbase = LinearGradient {
                colors: colors.to_vec(),
                pos: pos.to_vec(),
                count: colors.len() - 1,
                dmin: pos[0],
                dmax: pos[pos.len() - 1],
                mode: self.mode,
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

// --- Preset

macro_rules! preset {
    ($colors:expr; $name:ident) => {
        pub fn $name() -> Gradient {
            preset_spline($colors)
        }
    };
}

// Diverging

preset!(&["#543005", "#8c510a", "#bf812d", "#dfc27d", "#f6e8c3", "#f5f5f5", "#c7eae5", "#80cdc1", "#35978f", "#01665e", "#003c30"]; br_bg);
preset!(&["#40004b", "#762a83", "#9970ab", "#c2a5cf", "#e7d4e8", "#f7f7f7", "#d9f0d3", "#a6dba0", "#5aae61", "#1b7837", "#00441b"]; pr_gn);
preset!(&["#8e0152", "#c51b7d", "#de77ae", "#f1b6da", "#fde0ef", "#f7f7f7", "#e6f5d0", "#b8e186", "#7fbc41", "#4d9221", "#276419"]; pi_yg);
preset!(&["#2d004b", "#542788", "#8073ac", "#b2abd2", "#d8daeb", "#f7f7f7", "#fee0b6", "#fdb863", "#e08214", "#b35806", "#7f3b08"]; pu_or);
preset!(&["#67001f", "#b2182b", "#d6604d", "#f4a582", "#fddbc7", "#f7f7f7", "#d1e5f0", "#92c5de", "#4393c3", "#2166ac", "#053061"]; rd_bu);
preset!(&["#67001f", "#b2182b", "#d6604d", "#f4a582", "#fddbc7", "#ffffff", "#e0e0e0", "#bababa", "#878787", "#4d4d4d", "#1a1a1a"]; rd_gy);
preset!(&["#a50026", "#d73027", "#f46d43", "#fdae61", "#fee090", "#ffffbf", "#e0f3f8", "#abd9e9", "#74add1", "#4575b4", "#313695"]; rd_yl_bu);
preset!(&["#a50026", "#d73027", "#f46d43", "#fdae61", "#fee08b", "#ffffbf", "#d9ef8b", "#a6d96a", "#66bd63", "#1a9850", "#006837"]; rd_yl_gn);
preset!(&["#9e0142", "#d53e4f", "#f46d43", "#fdae61", "#fee08b", "#ffffbf", "#e6f598", "#abdda4", "#66c2a5", "#3288bd", "#5e4fa2"]; spectral);

// Sequential (Single Hue)

preset!(&["#f7fbff", "#deebf7", "#c6dbef", "#9ecae1", "#6baed6", "#4292c6", "#2171b5", "#08519c", "#08306b"]; blues);
preset!(&["#f7fcf5", "#e5f5e0", "#c7e9c0", "#a1d99b", "#74c476", "#41ab5d", "#238b45", "#006d2c", "#00441b"]; greens);
preset!(&["#ffffff", "#f0f0f0", "#d9d9d9", "#bdbdbd", "#969696", "#737373", "#525252", "#252525", "#000000"]; greys);
preset!(&["#fff5eb", "#fee6ce", "#fdd0a2", "#fdae6b", "#fd8d3c", "#f16913", "#d94801", "#a63603", "#7f2704"]; oranges);
preset!(&["#fcfbfd", "#efedf5", "#dadaeb", "#bcbddc", "#9e9ac8", "#807dba", "#6a51a3", "#54278f", "#3f007d"]; purples);
preset!(&["#fff5f0", "#fee0d2", "#fcbba1", "#fc9272", "#fb6a4a", "#ef3b2c", "#cb181d", "#a50f15", "#67000d"]; reds);

// Sequential (Multi-Hue)

preset!(&["#440154", "#482777", "#3f4a8a", "#31678e", "#26838f", "#1f9d8a", "#6cce5a", "#b6de2b", "#fee825"]; viridis);
preset!(&["#000004", "#170b3a", "#420a68", "#6b176e", "#932667", "#bb3654", "#dd513a", "#f3771a", "#fca50a", "#f6d644", "#fcffa4"]; inferno);
preset!(&["#000004", "#140e37", "#3b0f70", "#641a80", "#8c2981", "#b63679", "#de4968", "#f66f5c", "#fe9f6d", "#fece91", "#fcfdbf"]; magma);
preset!(&["#0d0887", "#42039d", "#6a00a8", "#900da3", "#b12a90", "#cb4678", "#e16462", "#f1834b", "#fca636", "#fccd25", "#f0f921"]; plasma);
preset!(&["#f7fcfd", "#e5f5f9", "#ccece6", "#99d8c9", "#66c2a4", "#41ae76", "#238b45", "#006d2c", "#00441b"]; bu_gn);
preset!(&["#f7fcfd", "#e0ecf4", "#bfd3e6", "#9ebcda", "#8c96c6", "#8c6bb1", "#88419d", "#810f7c", "#4d004b"]; bu_pu);
preset!(&["#f7fcf0", "#e0f3db", "#ccebc5", "#a8ddb5", "#7bccc4", "#4eb3d3", "#2b8cbe", "#0868ac", "#084081"]; gn_bu);
preset!(&["#fff7ec", "#fee8c8", "#fdd49e", "#fdbb84", "#fc8d59", "#ef6548", "#d7301f", "#b30000", "#7f0000"]; or_rd);
preset!(&["#fff7fb", "#ece2f0", "#d0d1e6", "#a6bddb", "#67a9cf", "#3690c0", "#02818a", "#016c59", "#014636"]; pu_bu_gn);
preset!(&["#fff7fb", "#ece7f2", "#d0d1e6", "#a6bddb", "#74a9cf", "#3690c0", "#0570b0", "#045a8d", "#023858"]; pu_bu);
preset!(&["#f7f4f9", "#e7e1ef", "#d4b9da", "#c994c7", "#df65b0", "#e7298a", "#ce1256", "#980043", "#67001f"]; pu_rd);
preset!(&["#fff7f3", "#fde0dd", "#fcc5c0", "#fa9fb5", "#f768a1", "#dd3497", "#ae017e", "#7a0177", "#49006a"]; rd_pu);
preset!(&["#ffffd9", "#edf8b1", "#c7e9b4", "#7fcdbb", "#41b6c4", "#1d91c0", "#225ea8", "#253494", "#081d58"]; yl_gn_bu);
preset!(&["#ffffe5", "#f7fcb9", "#d9f0a3", "#addd8e", "#78c679", "#41ab5d", "#238443", "#006837", "#004529"]; yl_gn);
preset!(&["#ffffe5", "#fff7bc", "#fee391", "#fec44f", "#fe9929", "#ec7014", "#cc4c02", "#993404", "#662506"]; yl_or_br);
preset!(&["#ffffcc", "#ffeda0", "#fed976", "#feb24c", "#fd8d3c", "#fc4e2a", "#e31a1c", "#bd0026", "#800026"]; yl_or_rd);

// ---

macro_rules! preset_fn {
    ($name:ident; $gradient_base:expr) => {
        pub fn $name() -> Gradient {
            Gradient {
                gradient: Box::new($gradient_base),
                dmin: 0.,
                dmax: 1.,
            }
        }
    };
}

// Sinebow

struct SinebowGradient {}

impl GradientBase for SinebowGradient {
    fn at(&self, t: f64) -> Color {
        let t = (0.5 - t) * PI;
        Color::from_rgb(
            clamp0_1(t.sin().powi(2)),
            clamp0_1((t + PI1_3).sin().powi(2)),
            clamp0_1((t + PI2_3).sin().powi(2)),
        )
    }
}

preset_fn!(sinebow; SinebowGradient{});

// Turbo

struct TurboGradient {}

impl GradientBase for TurboGradient {
    fn at(&self, t: f64) -> Color {
        let t = clamp0_1(t);
        let r = (34.61
            + t * (1172.33 - t * (10793.56 - t * (33300.12 - t * (38394.49 - t * 14825.05)))))
            .round();
        let g = (23.31 + t * (557.33 + t * (1225.33 - t * (3574.96 - t * (1073.77 + t * 707.56)))))
            .round();
        let b = (27.2
            + t * (3211.1 - t * (15327.97 - t * (27814.0 - t * (22569.18 - t * 6838.66)))))
            .round();
        Color::from_rgb(clamp0_1(r / 255.), clamp0_1(g / 255.), clamp0_1(b / 255.))
    }
}

preset_fn!(turbo; TurboGradient{});

// Cividis

struct CividisGradient {}

impl GradientBase for CividisGradient {
    fn at(&self, t: f64) -> Color {
        let t = clamp0_1(t);
        let r = (-4.54 - t * (35.34 - t * (2381.73 - t * (6402.7 - t * (7024.72 - t * 2710.57)))))
            .round();
        let g =
            (32.49 + t * (170.73 + t * (52.82 - t * (131.46 - t * (176.58 - t * 67.37))))).round();
        let b = (81.24
            + t * (442.36 - t * (2482.43 - t * (6167.24 - t * (6614.94 - t * 2475.67)))))
            .round();
        Color::from_rgb(clamp0_1(r / 255.), clamp0_1(g / 255.), clamp0_1(b / 255.))
    }
}

preset_fn!(cividis; CividisGradient{});

// Cubehelix

#[derive(Debug)]
struct Cubehelix {
    h: f64,
    s: f64,
    l: f64,
}

impl Cubehelix {
    fn to_color(&self) -> Color {
        let h = (self.h + 120.).to_radians();
        let l = self.l;
        let a = self.s * l * (1. - l);
        let cosh = h.cos();
        let sinh = h.sin();
        let r = l - a * (0.14861 * cosh - 1.78277 * sinh).min(1.);
        let g = l - a * (0.29227 * cosh + 0.90649 * sinh).min(1.);
        let b = l + a * (1.97294 * cosh);
        Color::from_rgb(clamp0_1(r), clamp0_1(g), clamp0_1(b))
    }

    fn interpolate(&self, other: &Cubehelix, t: f64) -> Cubehelix {
        Cubehelix {
            h: self.h + t * (other.h - self.h),
            s: self.s + t * (other.s - self.s),
            l: self.l + t * (other.l - self.l),
        }
    }
}

// Cubehelix gradient

#[derive(Debug)]
struct CubehelixGradient {
    start: Cubehelix,
    end: Cubehelix,
}

impl GradientBase for CubehelixGradient {
    fn at(&self, t: f64) -> Color {
        self.start.interpolate(&self.end, clamp0_1(t)).to_color()
    }
}

preset_fn!(cubehelix_default; CubehelixGradient {
    start: Cubehelix {
        h: 300.,
        s: 0.5,
        l: 0.,
    },
    end: Cubehelix {
        h: -240.,
        s: 0.5,
        l: 1.,
    },
});

preset_fn!(warm; CubehelixGradient {
    start: Cubehelix {
        h: -100.,
        s: 0.75,
        l: 0.35,
    },
    end: Cubehelix {
        h: 80.,
        s: 1.5,
        l: 0.8,
    },
});

preset_fn!(cool; CubehelixGradient {
    start: Cubehelix {
        h: 260.,
        s: 0.75,
        l: 0.35,
    },
    end: Cubehelix {
        h: 80.,
        s: 1.5,
        l: 0.8,
    },
});

// Rainbow

struct RainbowGradient {}

impl GradientBase for RainbowGradient {
    fn at(&self, t: f64) -> Color {
        let t = clamp0_1(t);
        let ts = (t - 0.5).abs();
        Cubehelix {
            h: 360. * t - 100.,
            s: 1.5 - 1.5 * ts,
            l: 0.8 - 0.9 * ts,
        }
        .to_color()
    }
}

preset_fn!(rainbow; RainbowGradient{});

// ---

fn linspace(min: f64, max: f64, n: usize) -> Vec<f64> {
    if n == 1 {
        return vec![min];
    }
    let d = max - min;
    let l = n as f64 - 1.;
    (0..n).map(|i| min + (i as f64 * d) / l).collect()
}

fn clamp0_1(t: f64) -> f64 {
    t.clamp(0., 1.)
}

fn modulo(x: f64, y: f64) -> f64 {
    (x % y + y) % y
}

// Map t from range [a, b] to range [0, 1]
fn norm(t: f64, a: f64, b: f64) -> f64 {
    (t - a) * (1. / (b - a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linspace() {
        assert_eq!(linspace(0., 1., 0), vec![]);
        assert_eq!(linspace(0., 1., 1), vec![0.]);
        assert_eq!(linspace(0., 1., 2), vec![0., 1.]);
        assert_eq!(linspace(0., 1., 3), vec![0., 0.5, 1.]);
        assert_eq!(linspace(-1., 1., 5), vec![-1., -0.5, 0., 0.5, 1.]);
        assert_eq!(linspace(0., 100., 5), vec![0., 25., 50., 75., 100.]);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(clamp0_1(-0.01), 0.);
        assert_eq!(clamp0_1(1.01), 1.);
        assert_eq!(clamp0_1(0.99), 0.99);
        assert_eq!(clamp0_1(0.01), 0.01);
        assert_eq!(clamp0_1(0.), 0.);
        assert_eq!(clamp0_1(1.), 1.);
    }
}
