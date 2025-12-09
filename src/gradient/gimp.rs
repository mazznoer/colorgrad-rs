// References:
// https://gitlab.gnome.org/GNOME/gimp/-/blob/master/devel-docs/ggr.txt
// https://gitlab.gnome.org/GNOME/gimp/-/blob/master/app/core/gimpgradient.c
// https://gitlab.gnome.org/GNOME/gimp/-/blob/master/app/core/gimpgradient-load.c

use std::error;
use std::f32::consts::{FRAC_PI_2, LN_2, PI};
use std::fmt;
use std::io::BufRead;
use std::string::{String, ToString};
use std::vec::Vec;

use crate::interpolate_linear;
use crate::Color;
use crate::Gradient;

#[derive(Debug)]
pub struct ParseGgrError {
    message: &'static str,
    line: usize,
}

impl error::Error for ParseGgrError {}

impl fmt::Display for ParseGgrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (line {})", self.message, self.line)
    }
}

#[derive(Debug, Copy, Clone)]
enum BlendingType {
    Linear,
    Curved,
    Sinusoidal,
    SphericalIncreasing,
    SphericalDecreasing,
    Step,
}

#[derive(Debug, Copy, Clone)]
enum ColoringType {
    Rgb,
    HsvCw,
    HsvCcw,
}

#[derive(Debug, Clone)]
struct GimpSegment {
    // Left endpoint color
    lcolor: [f32; 4],
    // Right endpoint color
    rcolor: [f32; 4],
    // Left endpoint coordinate
    lpos: f32,
    // Midpoint coordinate
    mpos: f32,
    // Right endpoint coordinate
    rpos: f32,
    // Blending function type
    blending_type: BlendingType,
    // Coloring type
    coloring_type: ColoringType,
}

/// Parse GIMP gradient (ggr)
///
/// # Example
///
/// ```
/// use colorgrad::{Color, Gradient};
/// use std::fs::File;
/// use std::io::BufReader;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let input = File::open("examples/ggr/Abstract_1.ggr")?;
/// let buf = BufReader::new(input);
/// let col = Color::default();
/// let grad = colorgrad::GimpGradient::new(buf, &col, &col)?;
///
/// assert_eq!(grad.name(), "Abstract 1");
/// # Ok(())
/// # }
/// ```
/// ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/ggr_abstract_1.png)
#[derive(Debug, Clone)]
pub struct GimpGradient {
    name: String,
    segments: Vec<GimpSegment>,
    dmin: f32,
    dmax: f32,
}

impl GimpGradient {
    pub fn new<R>(r: R, foreground: &Color, background: &Color) -> Result<Self, ParseGgrError>
    where
        R: BufRead,
    {
        parse_ggr(r, foreground, background)
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Gradient for GimpGradient {
    fn at(&self, t: f32) -> Color {
        if t < self.dmin || t > self.dmax || t.is_nan() {
            return Color::new(0.0, 0.0, 0.0, 1.0);
        }

        let mut low = 0;
        let mut high = self.segments.len();
        let mut mid = 0;

        while low < high {
            mid = (low + high) / 2;
            if t > self.segments[mid].rpos {
                low = mid + 1;
            } else if t < self.segments[mid].lpos {
                high = mid;
            } else {
                break;
            }
        }

        let seg = &self.segments[mid];
        let seg_len = seg.rpos - seg.lpos;

        let (middle, pos) = if seg_len < f32::EPSILON {
            (0.5, 0.5)
        } else {
            ((seg.mpos - seg.lpos) / seg_len, (t - seg.lpos) / seg_len)
        };

        let f = match seg.blending_type {
            BlendingType::Linear => calc_linear_factor(middle, pos),
            BlendingType::Curved => {
                if middle < f32::EPSILON {
                    match seg.coloring_type {
                        ColoringType::Rgb => {
                            let [r, g, b, a] = seg.rcolor;
                            return Color::new(r, g, b, a);
                        }
                        _ => {
                            let [h, s, v, a] = seg.rcolor;
                            return Color::from_hsva(h, s, v, a);
                        }
                    }
                } else if (1.0 - middle).abs() < f32::EPSILON {
                    match seg.coloring_type {
                        ColoringType::Rgb => {
                            let [r, g, b, a] = seg.lcolor;
                            return Color::new(r, g, b, a);
                        }
                        _ => {
                            let [h, s, v, a] = seg.lcolor;
                            return Color::from_hsva(h, s, v, a);
                        }
                    }
                } else {
                    (-LN_2 * pos.log10() / middle.log10()).exp()
                }
            }
            BlendingType::Sinusoidal => {
                let f = calc_linear_factor(middle, pos);
                ((-FRAC_PI_2 + (PI * f)).sin() + 1.0) / 2.0
            }
            BlendingType::SphericalIncreasing => {
                let f = calc_linear_factor(middle, pos) - 1.0;
                (1.0 - f * f).sqrt()
            }
            BlendingType::SphericalDecreasing => {
                let f = calc_linear_factor(middle, pos);
                1.0 - (1.0 - f * f).sqrt()
            }
            BlendingType::Step => {
                if pos >= middle {
                    match seg.coloring_type {
                        ColoringType::Rgb => {
                            let [r, g, b, a] = seg.rcolor;
                            return Color::new(r, g, b, a);
                        }
                        _ => {
                            let [h, s, v, a] = seg.rcolor;
                            return Color::from_hsva(h, s, v, a);
                        }
                    }
                } else {
                    match seg.coloring_type {
                        ColoringType::Rgb => {
                            let [r, g, b, a] = seg.lcolor;
                            return Color::new(r, g, b, a);
                        }
                        _ => {
                            let [h, s, v, a] = seg.lcolor;
                            return Color::from_hsva(h, s, v, a);
                        }
                    }
                }
            }
        };

        match seg.coloring_type {
            ColoringType::Rgb => Color::from(interpolate_linear(&seg.lcolor, &seg.rcolor, f)),
            ColoringType::HsvCcw => blend_hsv_ccw(&seg.lcolor, &seg.rcolor, f),
            ColoringType::HsvCw => blend_hsv_cw(&seg.lcolor, &seg.rcolor, f),
        }
    }
}

#[inline]
fn calc_linear_factor(middle: f32, pos: f32) -> f32 {
    if pos <= middle {
        if middle < f32::EPSILON {
            0.0
        } else {
            0.5 * pos / middle
        }
    } else {
        let pos = pos - middle;
        let middle = 1.0 - middle;

        if middle < f32::EPSILON {
            1.0
        } else {
            0.5 + 0.5 * pos / middle
        }
    }
}

fn parse_ggr<R: BufRead>(
    r: R,
    foreground: &Color,
    background: &Color,
) -> Result<GimpGradient, ParseGgrError> {
    let mut segments = Vec::new();
    let mut seg_n = 0;
    let mut seg_x = 0;
    let mut name = "".to_string();

    for (line_no, line) in r.lines().enumerate() {
        if let Ok(s) = line {
            if line_no == 0 {
                let s = s.trim_start_matches('\u{feff}');
                if s != "GIMP Gradient" {
                    return Err(ParseGgrError {
                        message: "invalid header",
                        line: 1,
                    });
                }

                continue;
            } else if line_no == 1 {
                if !s.starts_with("Name:") {
                    return Err(ParseGgrError {
                        message: "invalid header",
                        line: 2,
                    });
                }

                name = s[5..].trim().to_string();
                continue;
            } else if line_no == 2 {
                if let Ok(n) = s.parse::<usize>() {
                    seg_n = n;
                } else {
                    return Err(ParseGgrError {
                        message: "invalid header",
                        line: 3,
                    });
                }
                continue;
            }

            if line_no >= seg_n + 3 {
                break;
            }

            seg_x += 1;

            if let Some(seg) = parse_segment(&s, foreground, background) {
                segments.push(seg);
            } else {
                return Err(ParseGgrError {
                    message: "invalid segment",
                    line: line_no + 1,
                });
            }
        }
    }

    if seg_x < seg_n {
        return Err(ParseGgrError {
            message: "wrong segments count",
            line: 3,
        });
    }

    if segments.is_empty() {
        return Err(ParseGgrError {
            message: "no segment",
            line: 4,
        });
    }

    Ok(GimpGradient {
        name,
        segments,
        dmin: 0.0,
        dmax: 1.0,
    })
}

fn parse_segment(s: &str, foreground: &Color, background: &Color) -> Option<GimpSegment> {
    let d: Result<Vec<f32>, _> = s.split_whitespace().map(|x| x.parse()).collect();

    let Ok(d) = d else {
        return None;
    };

    if d.len() != 13 && d.len() != 15 {
        return None;
    }

    let blending_type = match d[11] as isize {
        0 => BlendingType::Linear,
        1 => BlendingType::Curved,
        2 => BlendingType::Sinusoidal,
        3 => BlendingType::SphericalIncreasing,
        4 => BlendingType::SphericalDecreasing,
        5 => BlendingType::Step,
        _ => return None,
    };

    let coloring_type = match d[12] as isize {
        0 => ColoringType::Rgb,
        1 => ColoringType::HsvCcw,
        2 => ColoringType::HsvCw,
        _ => return None,
    };

    let lcolor_code = if d.len() == 15 { d[13] as isize } else { 0 };
    let rcolor_code = if d.len() == 15 { d[14] as isize } else { 0 };

    let lcolor = match lcolor_code {
        0 => Color::new(d[3], d[4], d[5], d[6]),
        1 => foreground.clone(),
        2 => {
            let [r, g, b, _] = foreground.to_array();
            Color::new(r, g, b, 0.0)
        }
        3 => background.clone(),
        4 => {
            let [r, g, b, _] = background.to_array();
            Color::new(r, g, b, 0.0)
        }
        _ => return None,
    };

    let rcolor = match rcolor_code {
        0 => Color::new(d[7], d[8], d[9], d[10]),
        1 => foreground.clone(),
        2 => {
            let [r, g, b, _] = foreground.to_array();
            Color::new(r, g, b, 0.0)
        }
        3 => background.clone(),
        4 => {
            let [r, g, b, _] = background.to_array();
            Color::new(r, g, b, 0.0)
        }
        _ => return None,
    };

    let lcolor = match coloring_type {
        ColoringType::Rgb => lcolor.to_array(),
        _ => lcolor.to_hsva(),
    };

    let rcolor = match coloring_type {
        ColoringType::Rgb => rcolor.to_array(),
        _ => rcolor.to_hsva(),
    };

    Some(GimpSegment {
        lcolor,
        rcolor,
        lpos: d[0],
        mpos: d[1],
        rpos: d[2],
        blending_type,
        coloring_type,
    })
}

fn blend_hsv_ccw(c1: &[f32; 4], c2: &[f32; 4], t: f32) -> Color {
    let [h1, s1, v1, a1] = c1;
    let [h2, s2, v2, a2] = c2;

    let hue = if h1 < h2 {
        h1 + ((h2 - h1) * t)
    } else {
        let h = h1 + ((360.0 - (h1 - h2)) * t);

        if h > 360.0 {
            h - 360.0
        } else {
            h
        }
    };

    Color::from_hsva(
        hue,
        s1 + t * (s2 - s1),
        v1 + t * (v2 - v1),
        a1 + t * (a2 - a1),
    )
}

fn blend_hsv_cw(c1: &[f32; 4], c2: &[f32; 4], t: f32) -> Color {
    let [h1, s1, v1, a1] = c1;
    let [h2, s2, v2, a2] = c2;

    let hue = if h2 < h1 {
        h1 - ((h1 - h2) * t)
    } else {
        let h = h1 - ((360.0 - (h2 - h1)) * t);

        if h < 0.0 {
            h + 360.0
        } else {
            h
        }
    };

    Color::from_hsva(
        hue,
        s1 + t * (s2 - s1),
        v1 + t * (v2 - v1),
        a1 + t * (a2 - a1),
    )
}
