//! # Overview
//!
//! Rust color-scales library for maps, charts, data-visualization & creative coding.
//!
//! # Examples
//! Using preset gradient:
//! ```
//! let g = colorgrad::rainbow();
//! assert_eq!(g.domain(), (0., 1.)); // all preset gradients are in the domain 0..1
//! assert_eq!(g.at(0.5).rgba_u8(), (175, 239, 90, 255));
//! assert_eq!(g.at(0.5).to_hex_string(), "#afef5a");
//! ```
//!
//! Custom gradient:
//! ```
//! use colorgrad::Color;
//!
//! let g = colorgrad::CustomGradient::new()
//!     .colors(&[
//!         Color::from_rgb_u8(255, 0, 0),
//!         Color::from_rgb_u8(0, 255, 0),
//!     ])
//!     .build()
//!     .unwrap();
//!
//! assert_eq!(g.at(0.0).rgba_u8(), (255, 0, 0, 255));
//! assert_eq!(g.at(0.0).to_hex_string(), "#ff0000");
//! assert_eq!(g.at(1.0).to_hex_string(), "#00ff00");
//! ```
//!
//! [colorgrad::rainbow()](fn.rainbow.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad/master/doc/images/preset/Rainbow.png)
//!
//! [colorgrad::sinebow()](fn.sinebow.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad/master/doc/images/preset/Sinebow.png)
//!
//! [colorgrad::turbo()](fn.turbo.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad/master/doc/images/preset/Turbo.png)
//!
//! [colorgrad::spectral()](fn.spectral.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad/master/doc/images/preset/Spectral.png)
//!
//! See more complete gradient preview and examples at [Github](https://github.com/mazznoer/colorgrad-rs).

#![allow(clippy::many_single_char_names)]

extern crate csscolorparser;

pub use csscolorparser::Color;

use std::error::Error as StdError;
use std::f64::consts::PI;
use std::fmt;

const DEG2RAD: f64 = PI / 180.;
const PI1_3: f64 = PI / 3.;
const PI2_3: f64 = PI * 2. / 3.;

/// Color blending mode
#[derive(Debug, Copy, Clone)]
pub enum BlendMode {
    Rgb,
    Lrgb,
}

#[derive(Debug)]
pub enum CustomGradientError {
    InvalidHtmlColor,
    WrongDomainCount,
    WrongDomain,
}

impl fmt::Display for CustomGradientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomGradientError::InvalidHtmlColor => f.write_str("Invalid html color"),
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
    gradient: Box<dyn GradientBase>,
    dmin: f64,
    dmax: f64,
}

impl Gradient {
    /// Get color at certain position
    pub fn at(&self, t: f64) -> Color {
        self.gradient.at(t)
    }

    /// Get n colors evenly spaced across gradient
    pub fn colors(&self, n: usize) -> Vec<Color> {
        let mut colors = Vec::with_capacity(n);
        let d = self.dmax - self.dmin;
        let l = (n - 1) as f64;
        for i in 0..n {
            colors.push(self.at(self.dmin + (i as f64 * d) / l));
        }
        colors
    }

    /// Get the gradient's domain min and max
    pub fn domain(&self) -> (f64, f64) {
        (self.dmin, self.dmax)
    }

    /// Get new hard-edge gradient
    ///
    /// ```
    /// let g = colorgrad::spectral();
    /// ```
    /// ![img](https://raw.githubusercontent.com/mazznoer/colorgrad/master/doc/images/preset/Spectral.png)
    ///
    /// ```
    /// let g = colorgrad::spectral().sharp(19);
    /// ```
    /// ![img](https://raw.githubusercontent.com/mazznoer/colorgrad/master/doc/images/spectral-sharp.png)
    pub fn sharp(&self, n: usize) -> Gradient {
        let gradbase = GradientSharp {
            colors: self.colors(n),
            pos: linspace(self.dmin, self.dmax, n + 1),
            n,
            dmin: self.dmin,
            dmax: self.dmax,
        };
        Gradient {
            gradient: Box::new(gradbase),
            dmin: self.dmin,
            dmax: self.dmax,
        }
    }
}

#[derive(Debug)]
struct GradientX {
    colors: Vec<Color>,
    pos: Vec<f64>,
    count: usize,
    dmin: f64,
    dmax: f64,
    mode: BlendMode,
}

impl GradientBase for GradientX {
    fn at(&self, t: f64) -> Color {
        if t < self.dmin {
            return self.colors[0].clone();
        }
        if t > self.dmax {
            return self.colors[self.count - 1].clone();
        }
        for i in 0..(self.count - 1) {
            let p1 = self.pos[i];
            let p2 = self.pos[i + 1];

            if (p1 <= t) && (t <= p2) {
                let t = (t - p1) / (p2 - p1);
                let a = &self.colors[i];
                let b = &self.colors[i + 1];

                match self.mode {
                    BlendMode::Rgb => return a.interpolate_rgb(b, t),
                    BlendMode::Lrgb => return a.interpolate_lrgb(b, t),
                }
            }
        }
        self.colors[self.count - 1].clone()
    }
}

#[derive(Debug)]
struct GradientSharp {
    colors: Vec<Color>,
    pos: Vec<f64>,
    n: usize,
    dmin: f64,
    dmax: f64,
}

impl GradientBase for GradientSharp {
    fn at(&self, t: f64) -> Color {
        if t < self.dmin {
            return self.colors[0].clone();
        }
        if t > self.dmax {
            return self.colors[self.n - 1].clone();
        }
        for i in 0..self.n {
            if (self.pos[i] <= t) && (t <= self.pos[i + 1]) {
                return self.colors[i].clone();
            }
        }
        self.colors[self.n - 1].clone()
    }
}

/// Create custom gradient
///
/// # Examples
/// ```
/// use colorgrad::Color;
///
/// let grad = colorgrad::CustomGradient::new()
///     .colors(&[
///         Color::from_rgb_u8(255, 0, 0),
///         Color::from_rgb(0., 0., 1.),
///     ])
///     .build()
///     .unwrap();
///
/// assert_eq!(grad.domain(), (0., 1.)); // default domain
/// assert_eq!(grad.at(0.).rgba_u8(), (255, 0, 0, 255));
/// assert_eq!(grad.at(1.).rgba_u8(), (0, 0, 255, 255));
///
/// // Using web color format string
///
/// let grad = colorgrad::CustomGradient::new()
///     .html_colors(&["deeppink", "gold", "seagreen"])
///     .domain(&[0., 100.])
///     .mode(colorgrad::BlendMode::Rgb)
///     .build()
///     .unwrap();
///
/// assert_eq!(grad.at(0.).rgba_u8(), (255, 20, 147, 255));
/// assert_eq!(grad.at(100.).rgba_u8(), (46, 139, 87, 255));
/// ```
#[derive(Debug)]
pub struct CustomGradient {
    colors: Vec<Color>,
    pos: Vec<f64>,
    mode: BlendMode,
    invalid_html_colors: Vec<String>,
}

impl CustomGradient {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CustomGradient {
        CustomGradient {
            colors: Vec::new(),
            pos: Vec::new(),
            mode: BlendMode::Rgb,
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

    /// Set gradient color using HTML/ CSS color format.
    /// It support named colors, hexadecimal (`#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`), `rgb()`,
    /// `rgba()`, `hsl()`, `hsla()`, `hwb()`, and `hsv()`.
    pub fn html_colors<'a>(&'a mut self, html_colors: &[&str]) -> &'a mut CustomGradient {
        for s in html_colors {
            if let Ok(c) = csscolorparser::parse(s) {
                self.colors.push(c);
                continue;
            }
            self.invalid_html_colors.push(s.to_string());
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

    /// Build the gradient
    pub fn build(&self) -> Result<Gradient, CustomGradientError> {
        if !self.invalid_html_colors.is_empty() {
            return Err(CustomGradientError::InvalidHtmlColor);
        }

        let mut colors = self.colors.to_vec();

        if colors.is_empty() {
            colors = vec![Color::from_rgb(0., 0., 0.), Color::from_rgb(1., 1., 1.)];
        } else if colors.len() == 1 {
            colors.push(colors[0].clone());
        }

        let pos;

        if self.pos.is_empty() {
            pos = linspace(0., 1., colors.len());
        } else if self.pos.len() == colors.len() {
            pos = self.pos.to_vec();
        } else if self.pos.len() == 2 {
            pos = linspace(self.pos[0], self.pos[1], colors.len());
        } else {
            return Err(CustomGradientError::WrongDomainCount);
        }

        for i in 0..(pos.len() - 1) {
            if pos[i] >= pos[i + 1] {
                return Err(CustomGradientError::WrongDomain);
            }
        }

        let gradbase = GradientX {
            colors: colors.to_vec(),
            pos: pos.to_vec(),
            count: colors.len(),
            dmin: pos[0],
            dmax: pos[pos.len() - 1],
            mode: self.mode,
        };

        Ok(Gradient {
            gradient: Box::new(gradbase),
            dmin: pos[0],
            dmax: pos[pos.len() - 1],
        })
    }
}

// --- Preset

macro_rules! preset {
    ($colors:expr; $name:ident) => {
        pub fn $name() -> Gradient {
            CustomGradient::new().html_colors($colors).build().unwrap()
        }
    };
}

// Diverging

preset!(&["#543005", "#8c510a", "#bf812d", "#dfc27d", "#f6e8c3", "#f5f5f5", "#c7eae5", "#80cdc1", "#35978f", "#01665e", "#003c30"]; brbg);
preset!(&["#40004b", "#762a83", "#9970ab", "#c2a5cf", "#e7d4e8", "#f7f7f7", "#d9f0d3", "#a6dba0", "#5aae61", "#1b7837", "#00441b"]; prgn);
preset!(&["#8e0152", "#c51b7d", "#de77ae", "#f1b6da", "#fde0ef", "#f7f7f7", "#e6f5d0", "#b8e186", "#7fbc41", "#4d9221", "#276419"]; piyg);
preset!(&["#7f3b08", "#b35806", "#e08214", "#fdb863", "#fee0b6", "#f7f7f7", "#d8daeb", "#b2abd2", "#8073ac", "#542788", "#2d004b"]; puor);
preset!(&["#67001f", "#b2182b", "#d6604d", "#f4a582", "#fddbc7", "#f7f7f7", "#d1e5f0", "#92c5de", "#4393c3", "#2166ac", "#053061"]; rdbu);
preset!(&["#67001f", "#b2182b", "#d6604d", "#f4a582", "#fddbc7", "#ffffff", "#e0e0e0", "#bababa", "#878787", "#4d4d4d", "#1a1a1a"]; rdgy);
preset!(&["#a50026", "#d73027", "#f46d43", "#fdae61", "#fee090", "#ffffbf", "#e0f3f8", "#abd9e9", "#74add1", "#4575b4", "#313695"]; rdylbu);
preset!(&["#a50026", "#d73027", "#f46d43", "#fdae61", "#fee08b", "#ffffbf", "#d9ef8b", "#a6d96a", "#66bd63", "#1a9850", "#006837"]; rdylgn);
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
preset!(&["#f7fcfd", "#e5f5f9", "#ccece6", "#99d8c9", "#66c2a4", "#41ae76", "#238b45", "#006d2c", "#00441b"]; bugn);
preset!(&["#f7fcfd", "#e0ecf4", "#bfd3e6", "#9ebcda", "#8c96c6", "#8c6bb1", "#88419d", "#810f7c", "#4d004b"]; bupu);
preset!(&["#f7fcf0", "#e0f3db", "#ccebc5", "#a8ddb5", "#7bccc4", "#4eb3d3", "#2b8cbe", "#0868ac", "#084081"]; gnbu);
preset!(&["#fff7ec", "#fee8c8", "#fdd49e", "#fdbb84", "#fc8d59", "#ef6548", "#d7301f", "#b30000", "#7f0000"]; orrd);
preset!(&["#fff7fb", "#ece2f0", "#d0d1e6", "#a6bddb", "#67a9cf", "#3690c0", "#02818a", "#016c59", "#014636"]; pubugn);
preset!(&["#fff7fb", "#ece7f2", "#d0d1e6", "#a6bddb", "#74a9cf", "#3690c0", "#0570b0", "#045a8d", "#023858"]; pubu);
preset!(&["#f7f4f9", "#e7e1ef", "#d4b9da", "#c994c7", "#df65b0", "#e7298a", "#ce1256", "#980043", "#67001f"]; purd);
preset!(&["#fff7f3", "#fde0dd", "#fcc5c0", "#fa9fb5", "#f768a1", "#dd3497", "#ae017e", "#7a0177", "#49006a"]; rdpu);
preset!(&["#ffffd9", "#edf8b1", "#c7e9b4", "#7fcdbb", "#41b6c4", "#1d91c0", "#225ea8", "#253494", "#081d58"]; ylgnbu);
preset!(&["#ffffe5", "#f7fcb9", "#d9f0a3", "#addd8e", "#78c679", "#41ab5d", "#238443", "#006837", "#004529"]; ylgn);
preset!(&["#ffffe5", "#fff7bc", "#fee391", "#fec44f", "#fe9929", "#ec7014", "#cc4c02", "#993404", "#662506"]; ylorbr);
preset!(&["#ffffcc", "#ffeda0", "#fed976", "#feb24c", "#fd8d3c", "#fc4e2a", "#e31a1c", "#bd0026", "#800026"]; ylorrd);

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
        let h = (self.h + 120.) * DEG2RAD;
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
        self.start.interpolate(&self.end, t).to_color()
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
    let d = max - min;
    let l = (n - 1) as f64;
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        res.push(min + (i as f64 * d) / l);
    }
    res
}

fn clamp0_1(t: f64) -> f64 {
    if t < 0. {
        return 0.;
    }
    if t > 1. {
        return 1.;
    }
    t
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linspace() {
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
