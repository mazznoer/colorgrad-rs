// References:
// https://gitlab.gnome.org/GNOME/gimp/-/blob/master/devel-docs/ggr.txt
// https://gitlab.gnome.org/GNOME/gimp/-/blob/master/app/core/gimpgradient.c
// https://gitlab.gnome.org/GNOME/gimp/-/blob/master/app/core/gimpgradient-load.c

use crate::{Color, Gradient, GradientBase};

use std::{
    error,
    f64::consts::{FRAC_PI_2, LN_2, PI},
    fmt,
    io::BufRead,
};

#[derive(Debug)]
pub struct ParseGgrError {
    message: String,
    line: usize,
}

impl error::Error for ParseGgrError {}

impl fmt::Display for ParseGgrError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (line {})", &self.message, self.line)
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

#[derive(Debug)]
struct GimpSegment {
    // Left endpoint color
    lcolor: Color,
    // Right endpoint color
    rcolor: Color,
    // Left endpoint coordinate
    lpos: f64,
    // Midpoint coordinate
    mpos: f64,
    // Right endpoint coordinate
    rpos: f64,
    // Blending function type
    blending_type: BlendingType,
    // Coloring type
    coloring_type: ColoringType,
}

#[derive(Debug)]
struct GimpGradient {
    segments: Vec<GimpSegment>,
    dmin: f64,
    dmax: f64,
}

impl GradientBase for GimpGradient {
    fn at(&self, t: f64) -> Color {
        if t <= self.dmin {
            return self.segments[0].lcolor.clone();
        }

        if t >= self.dmax {
            return self.segments[self.segments.len() - 1].rcolor.clone();
        }

        if t.is_nan() {
            return Color::new(0.0, 0.0, 0.0, 1.0);
        }

        let mut low = 0;
        let mut high = self.segments.len();
        let mut mid = 0;

        loop {
            if low >= high {
                break;
            }
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

        let (middle, pos) = if seg_len < f64::EPSILON {
            (0.5, 0.5)
        } else {
            ((seg.mpos - seg.lpos) / seg_len, (t - seg.lpos) / seg_len)
        };

        let f = match seg.blending_type {
            BlendingType::Linear => calc_linear_factor(middle, pos),
            BlendingType::Curved => {
                if middle < f64::EPSILON {
                    return seg.rcolor.clone();
                } else if (1.0 - middle).abs() < f64::EPSILON {
                    return seg.lcolor.clone();
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
                    return seg.rcolor.clone();
                } else {
                    return seg.lcolor.clone();
                }
            }
        };

        match seg.coloring_type {
            ColoringType::Rgb => seg.lcolor.interpolate_rgb(&seg.rcolor, f),
            ColoringType::HsvCcw => blend_hsv_ccw(&seg.lcolor, &seg.rcolor, f),
            ColoringType::HsvCw => blend_hsv_cw(&seg.lcolor, &seg.rcolor, f),
        }
    }
}

#[inline]
fn calc_linear_factor(middle: f64, pos: f64) -> f64 {
    if pos <= middle {
        if middle < f64::EPSILON {
            0.0
        } else {
            0.5 * pos / middle
        }
    } else {
        let pos = pos - middle;
        let middle = 1.0 - middle;

        if middle < f64::EPSILON {
            1.0
        } else {
            0.5 + 0.5 * pos / middle
        }
    }
}

/// Parse GIMP gradient (ggr)
///
/// # Example
///
/// ```
/// use colorgrad::Color;
/// use std::fs::File;
/// use std::io::BufReader;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let input = File::open("examples/ggr/Abstract_1.ggr")?;
/// let buf = BufReader::new(input);
/// let fg = Color::new(0.0, 0.0, 0.0, 1.0);
/// let bg = Color::new(1.0, 1.0, 1.0, 1.0);
/// let (grad, name) = colorgrad::parse_ggr(buf, &fg, &bg)?;
///
/// assert_eq!(name, "Abstract 1");
/// # Ok(())
/// # }
/// ```
/// ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/ggr_abstract_1.png)
pub fn parse_ggr<R: BufRead>(
    r: R,
    foreground: &Color,
    background: &Color,
) -> Result<(Gradient, String), ParseGgrError> {
    let mut segments = Vec::new();
    let mut seg_n = 0;
    let mut seg_x = 0;
    let mut name = "".to_string();

    for (line_no, line) in r.lines().enumerate() {
        if let Ok(s) = line {
            if line_no == 0 {
                if s != "GIMP Gradient" {
                    return Err(ParseGgrError {
                        message: "invalid header".to_string(),
                        line: 1,
                    });
                }

                continue;
            } else if line_no == 1 {
                if !s.starts_with("Name:") {
                    return Err(ParseGgrError {
                        message: "invalid header".to_string(),
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
                        message: "invalid header".to_string(),
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
                    message: "invalid segment".to_string(),
                    line: line_no + 1,
                });
            }
        }
    }

    if seg_x < seg_n {
        return Err(ParseGgrError {
            message: "wrong segments count".to_string(),
            line: 3,
        });
    }

    if segments.is_empty() {
        return Err(ParseGgrError {
            message: "no segment".to_string(),
            line: 4,
        });
    }

    let gradbase = GimpGradient {
        segments,
        dmin: 0.0,
        dmax: 1.0,
    };

    Ok((
        Gradient {
            gradient: Box::new(gradbase),
            dmin: 0.0,
            dmax: 1.0,
        },
        name,
    ))
}

fn parse_segment(s: &str, foreground: &Color, background: &Color) -> Option<GimpSegment> {
    let d: Result<Vec<_>, _> = s.split_whitespace().map(|x| x.parse::<f64>()).collect();

    let d = if let Ok(t) = d {
        t
    } else {
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

fn blend_hsv_ccw(c1: &Color, c2: &Color, t: f64) -> Color {
    let (h1, s1, v1, a1) = c1.to_hsva();
    let (h2, s2, v2, a2) = c2.to_hsva();

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

fn blend_hsv_cw(c1: &Color, c2: &Color, t: f64) -> Color {
    let (h1, s1, v1, a1) = c1.to_hsva();
    let (h2, s2, v2, a2) = c2.to_hsva();

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
