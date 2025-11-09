//! # Preset Gradients
//!
//! ```
//! use colorgrad::Gradient;
//! let grad = colorgrad::preset::rainbow();
//!
//! assert_eq!(grad.domain(), (0.0, 1.0)); // all preset gradients are in the domain [0..1]
//! assert_eq!(grad.at(0.25).to_rgba8(), [255, 94, 99, 255]);
//! assert_eq!(grad.at(0.75).to_rgba8(), [26, 199, 194, 255]);
//! assert_eq!(grad.at(0.37).to_css_hex(), "#f2a42f");
//! ```

use alloc::vec::Vec;
use core::f32::consts::{FRAC_PI_3, PI};
use libm::{cosf, roundf, sinf};

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
        let r = sinf(t);
        let g = sinf(t + FRAC_PI_3);
        let b = sinf(t + PI2_3);
        Color::new(
            (r * r).clamp(0.0, 1.0),
            (g * g).clamp(0.0, 1.0),
            (b * b).clamp(0.0, 1.0),
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
        let r = roundf(
            34.61 + t * (1172.33 - t * (10793.56 - t * (33300.12 - t * (38394.49 - t * 14825.05)))),
        );
        let g = roundf(
            23.31 + t * (557.33 + t * (1225.33 - t * (3574.96 - t * (1073.77 + t * 707.56)))),
        );
        let b = roundf(
            27.2 + t * (3211.1 - t * (15327.97 - t * (27814.0 - t * (22569.18 - t * 6838.66)))),
        );
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
        let r = roundf(
            -4.54 - t * (35.34 - t * (2381.73 - t * (6402.7 - t * (7024.72 - t * 2710.57)))),
        );
        let g =
            roundf(32.49 + t * (170.73 + t * (52.82 - t * (131.46 - t * (176.58 - t * 67.37)))));
        let b = roundf(
            81.24 + t * (442.36 - t * (2482.43 - t * (6167.24 - t * (6614.94 - t * 2475.67)))),
        );
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
        let h = (self.h + 120.0) * (PI / 180.0);
        let l = self.l;
        let a = self.s * l * (1.0 - l);

        let cosh = cosf(h);
        let sinh = sinf(h);

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

fn build_preset(colors: &[u32]) -> BasisGradient {
    fn to_color(c: &u32) -> Color {
        Color::from_rgba8(
            ((c >> 16) & 0xff) as _,
            ((c >> 8) & 0xff) as _,
            (c & 0xff) as _,
            255,
        )
    }
    let colors = colors.iter().map(to_color).collect::<Vec<_>>();
    let pos = linspace(0.0, 1.0, colors.len()).collect();
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

preset!(&[0x543005, 0x8c510a, 0xbf812d, 0xdfc27d, 0xf6e8c3, 0xf5f5f5, 0xc7eae5, 0x80cdc1, 0x35978f, 0x01665e, 0x003c30]; br_bg);
preset!(&[0x40004b, 0x762a83, 0x9970ab, 0xc2a5cf, 0xe7d4e8, 0xf7f7f7, 0xd9f0d3, 0xa6dba0, 0x5aae61, 0x1b7837, 0x00441b]; pr_gn);
preset!(&[0x8e0152, 0xc51b7d, 0xde77ae, 0xf1b6da, 0xfde0ef, 0xf7f7f7, 0xe6f5d0, 0xb8e186, 0x7fbc41, 0x4d9221, 0x276419]; pi_yg);
preset!(&[0x2d004b, 0x542788, 0x8073ac, 0xb2abd2, 0xd8daeb, 0xf7f7f7, 0xfee0b6, 0xfdb863, 0xe08214, 0xb35806, 0x7f3b08]; pu_or);
preset!(&[0x67001f, 0xb2182b, 0xd6604d, 0xf4a582, 0xfddbc7, 0xf7f7f7, 0xd1e5f0, 0x92c5de, 0x4393c3, 0x2166ac, 0x053061]; rd_bu);
preset!(&[0x67001f, 0xb2182b, 0xd6604d, 0xf4a582, 0xfddbc7, 0xffffff, 0xe0e0e0, 0xbababa, 0x878787, 0x4d4d4d, 0x1a1a1a]; rd_gy);
preset!(&[0xa50026, 0xd73027, 0xf46d43, 0xfdae61, 0xfee090, 0xffffbf, 0xe0f3f8, 0xabd9e9, 0x74add1, 0x4575b4, 0x313695]; rd_yl_bu);
preset!(&[0xa50026, 0xd73027, 0xf46d43, 0xfdae61, 0xfee08b, 0xffffbf, 0xd9ef8b, 0xa6d96a, 0x66bd63, 0x1a9850, 0x006837]; rd_yl_gn);
preset!(&[0x9e0142, 0xd53e4f, 0xf46d43, 0xfdae61, 0xfee08b, 0xffffbf, 0xe6f598, 0xabdda4, 0x66c2a5, 0x3288bd, 0x5e4fa2]; spectral);

// Sequential (Single Hue)

preset!(&[0xf7fbff, 0xdeebf7, 0xc6dbef, 0x9ecae1, 0x6baed6, 0x4292c6, 0x2171b5, 0x08519c, 0x08306b]; blues);
preset!(&[0xf7fcf5, 0xe5f5e0, 0xc7e9c0, 0xa1d99b, 0x74c476, 0x41ab5d, 0x238b45, 0x006d2c, 0x00441b]; greens);
preset!(&[0xffffff, 0xf0f0f0, 0xd9d9d9, 0xbdbdbd, 0x969696, 0x737373, 0x525252, 0x252525, 0x000000]; greys);
preset!(&[0xfff5eb, 0xfee6ce, 0xfdd0a2, 0xfdae6b, 0xfd8d3c, 0xf16913, 0xd94801, 0xa63603, 0x7f2704]; oranges);
preset!(&[0xfcfbfd, 0xefedf5, 0xdadaeb, 0xbcbddc, 0x9e9ac8, 0x807dba, 0x6a51a3, 0x54278f, 0x3f007d]; purples);
preset!(&[0xfff5f0, 0xfee0d2, 0xfcbba1, 0xfc9272, 0xfb6a4a, 0xef3b2c, 0xcb181d, 0xa50f15, 0x67000d]; reds);

// Sequential (Multi-Hue)

preset!(&[0x440154, 0x482777, 0x3f4a8a, 0x31678e, 0x26838f, 0x1f9d8a, 0x6cce5a, 0xb6de2b, 0xfee825]; viridis);
preset!(&[0x000004, 0x170b3a, 0x420a68, 0x6b176e, 0x932667, 0xbb3654, 0xdd513a, 0xf3771a, 0xfca50a, 0xf6d644, 0xfcffa4]; inferno);
preset!(&[0x000004, 0x140e37, 0x3b0f70, 0x641a80, 0x8c2981, 0xb63679, 0xde4968, 0xf66f5c, 0xfe9f6d, 0xfece91, 0xfcfdbf]; magma);
preset!(&[0x0d0887, 0x42039d, 0x6a00a8, 0x900da3, 0xb12a90, 0xcb4678, 0xe16462, 0xf1834b, 0xfca636, 0xfccd25, 0xf0f921]; plasma);
preset!(&[0xf7fcfd, 0xe5f5f9, 0xccece6, 0x99d8c9, 0x66c2a4, 0x41ae76, 0x238b45, 0x006d2c, 0x00441b]; bu_gn);
preset!(&[0xf7fcfd, 0xe0ecf4, 0xbfd3e6, 0x9ebcda, 0x8c96c6, 0x8c6bb1, 0x88419d, 0x810f7c, 0x4d004b]; bu_pu);
preset!(&[0xf7fcf0, 0xe0f3db, 0xccebc5, 0xa8ddb5, 0x7bccc4, 0x4eb3d3, 0x2b8cbe, 0x0868ac, 0x084081]; gn_bu);
preset!(&[0xfff7ec, 0xfee8c8, 0xfdd49e, 0xfdbb84, 0xfc8d59, 0xef6548, 0xd7301f, 0xb30000, 0x7f0000]; or_rd);
preset!(&[0xfff7fb, 0xece2f0, 0xd0d1e6, 0xa6bddb, 0x67a9cf, 0x3690c0, 0x02818a, 0x016c59, 0x014636]; pu_bu_gn);
preset!(&[0xfff7fb, 0xece7f2, 0xd0d1e6, 0xa6bddb, 0x74a9cf, 0x3690c0, 0x0570b0, 0x045a8d, 0x023858]; pu_bu);
preset!(&[0xf7f4f9, 0xe7e1ef, 0xd4b9da, 0xc994c7, 0xdf65b0, 0xe7298a, 0xce1256, 0x980043, 0x67001f]; pu_rd);
preset!(&[0xfff7f3, 0xfde0dd, 0xfcc5c0, 0xfa9fb5, 0xf768a1, 0xdd3497, 0xae017e, 0x7a0177, 0x49006a]; rd_pu);
preset!(&[0xffffd9, 0xedf8b1, 0xc7e9b4, 0x7fcdbb, 0x41b6c4, 0x1d91c0, 0x225ea8, 0x253494, 0x081d58]; yl_gn_bu);
preset!(&[0xffffe5, 0xf7fcb9, 0xd9f0a3, 0xaddd8e, 0x78c679, 0x41ab5d, 0x238443, 0x006837, 0x004529]; yl_gn);
preset!(&[0xffffe5, 0xfff7bc, 0xfee391, 0xfec44f, 0xfe9929, 0xec7014, 0xcc4c02, 0x993404, 0x662506]; yl_or_br);
preset!(&[0xffffcc, 0xffeda0, 0xfed976, 0xfeb24c, 0xfd8d3c, 0xfc4e2a, 0xe31a1c, 0xbd0026, 0x800026]; yl_or_rd);
