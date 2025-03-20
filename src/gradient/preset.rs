//! # Preset Gradients
//!
//! ```
//! use colorgrad::Gradient;
//! let grad = colorgrad::preset::rainbow();
//!
//! assert_eq!(grad.domain(), (0.0, 1.0)); // all preset gradients are in the domain [0..1]
//! assert_eq!(grad.at(0.25).to_rgba8(), [255, 94, 99, 255]);
//! assert_eq!(grad.at(0.75).to_rgba8(), [26, 199, 194, 255]);
//! assert_eq!(grad.at(0.37).to_hex_string(), "#f2a42f");
//! ```

use std::f32::consts::{FRAC_PI_3, PI};

use crate::{linspace, BasisGradient, BlendMode, Color, Gradient};

const PI2_3: f32 = PI * 2.0 / 3.0;

// Sinebow

#[derive(Debug, Clone)]
pub struct SinebowGradient {}

pub fn sinebow() -> SinebowGradient {
    SinebowGradient {}
}

impl Gradient for SinebowGradient {
    fn at(&self, t: f32) -> Color {
        let t = (0.5 - t) * PI;
        Color::new(
            t.sin().powi(2).clamp(0.0, 1.0),
            (t + FRAC_PI_3).sin().powi(2).clamp(0.0, 1.0),
            (t + PI2_3).sin().powi(2).clamp(0.0, 1.0),
            1.0,
        )
    }
}

// Turbo

#[derive(Debug, Clone)]
pub struct TurboGradient {}

pub fn turbo() -> TurboGradient {
    TurboGradient {}
}

impl Gradient for TurboGradient {
    fn at(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let r = (34.61
            + t * (1172.33 - t * (10793.56 - t * (33300.12 - t * (38394.49 - t * 14825.05)))))
            .round();
        let g = (23.31 + t * (557.33 + t * (1225.33 - t * (3574.96 - t * (1073.77 + t * 707.56)))))
            .round();
        let b = (27.2
            + t * (3211.1 - t * (15327.97 - t * (27814.0 - t * (22569.18 - t * 6838.66)))))
            .round();
        Color::new(
            (r / 255.0).clamp(0.0, 1.0),
            (g / 255.0).clamp(0.0, 1.0),
            (b / 255.0).clamp(0.0, 1.0),
            1.0,
        )
    }
}

// Cividis

#[derive(Debug, Clone)]
pub struct CividisGradient {}

pub fn cividis() -> CividisGradient {
    CividisGradient {}
}

impl Gradient for CividisGradient {
    fn at(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let r = (-4.54 - t * (35.34 - t * (2381.73 - t * (6402.7 - t * (7024.72 - t * 2710.57)))))
            .round();
        let g =
            (32.49 + t * (170.73 + t * (52.82 - t * (131.46 - t * (176.58 - t * 67.37))))).round();
        let b = (81.24
            + t * (442.36 - t * (2482.43 - t * (6167.24 - t * (6614.94 - t * 2475.67)))))
            .round();
        Color::new(
            (r / 255.0).clamp(0.0, 1.0),
            (g / 255.0).clamp(0.0, 1.0),
            (b / 255.0).clamp(0.0, 1.0),
            1.0,
        )
    }
}

// Cubehelix

#[derive(Debug, Clone)]
struct Cubehelix {
    h: f32,
    s: f32,
    l: f32,
}

impl Cubehelix {
    fn to_color(&self) -> Color {
        let h = (self.h + 120.0).to_radians();
        let l = self.l;
        let a = self.s * l * (1.0 - l);

        let cosh = h.cos();
        let sinh = h.sin();

        let r = l - a * (0.14861 * cosh - 1.78277 * sinh);
        let g = l - a * (0.29227 * cosh + 0.90649 * sinh);
        let b = l + a * (1.97294 * cosh);

        Color::new(r.clamp(0.0, 1.0), g.clamp(0.0, 1.0), b.clamp(0.0, 1.0), 1.0)
    }

    fn interpolate(&self, other: &Cubehelix, t: f32) -> Cubehelix {
        Cubehelix {
            h: self.h + t * (other.h - self.h),
            s: self.s + t * (other.s - self.s),
            l: self.l + t * (other.l - self.l),
        }
    }
}

// Cubehelix gradient

#[derive(Debug, Clone)]
pub struct CubehelixGradient {
    start: Cubehelix,
    end: Cubehelix,
}

impl Gradient for CubehelixGradient {
    fn at(&self, t: f32) -> Color {
        self.start
            .interpolate(&self.end, t.clamp(0.0, 1.0))
            .to_color()
    }
}

pub fn cubehelix_default() -> CubehelixGradient {
    CubehelixGradient {
        start: Cubehelix {
            h: 300.0,
            s: 0.5,
            l: 0.0,
        },
        end: Cubehelix {
            h: -240.0,
            s: 0.5,
            l: 1.0,
        },
    }
}

pub fn warm() -> CubehelixGradient {
    CubehelixGradient {
        start: Cubehelix {
            h: -100.0,
            s: 0.75,
            l: 0.35,
        },
        end: Cubehelix {
            h: 80.0,
            s: 1.5,
            l: 0.8,
        },
    }
}

pub fn cool() -> CubehelixGradient {
    CubehelixGradient {
        start: Cubehelix {
            h: 260.0,
            s: 0.75,
            l: 0.35,
        },
        end: Cubehelix {
            h: 80.0,
            s: 1.5,
            l: 0.8,
        },
    }
}

// Rainbow

#[derive(Debug, Clone)]
pub struct RainbowGradient {}

pub fn rainbow() -> RainbowGradient {
    RainbowGradient {}
}

impl Gradient for RainbowGradient {
    fn at(&self, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let ts = (t - 0.5).abs();
        Cubehelix {
            h: 360.0 * t - 100.0,
            s: 1.5 - 1.5 * ts,
            l: 0.8 - 0.9 * ts,
        }
        .to_color()
    }
}

// ---

fn build_preset(html_colors: &[&str]) -> BasisGradient {
    let colors = html_colors
        .iter()
        .map(|s| csscolorparser::parse(s).unwrap())
        .collect::<Vec<_>>();
    let pos = linspace(0.0, 1.0, colors.len());
    BasisGradient::new(&colors, pos, BlendMode::Rgb)
}

macro_rules! preset {
    ($colors:expr; $name:ident) => {
        pub fn $name() -> BasisGradient {
            build_preset($colors)
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
