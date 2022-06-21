use std::{error, fmt};

use crate::{linspace, spline_gradient, BlendMode, Color, Gradient, Interpolation, LinearGradient};

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
#[derive(Debug, Clone)]
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
            let dmin = pos[0];
            let dmax = pos[pos.len() - 1];
            let gradbase = LinearGradient::new(colors, pos, self.mode);

            return Ok(Gradient {
                gradient: Box::new(gradbase),
                dmin,
                dmax,
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
