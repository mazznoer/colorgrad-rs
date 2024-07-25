use std::convert::TryFrom;
use std::{error, fmt};

use crate::{css_gradient, linspace, BlendMode, Color};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum GradientBuilderError {
    InvalidHtmlColor(Vec<String>),
    InvalidCssGradient,
    WrongDomainCount,
    WrongDomain,
}

impl fmt::Display for GradientBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Self::InvalidHtmlColor(ref colors) => {
                write!(
                    f,
                    "invalid html colors: {}",
                    colors
                        .iter()
                        .map(|x| format!("'{}'", x))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            Self::InvalidCssGradient => f.write_str("invalid css gradient"),
            Self::WrongDomainCount => f.write_str("wrong domain count"),
            Self::WrongDomain => f.write_str("wrong domain"),
        }
    }
}

impl error::Error for GradientBuilderError {}

/// Create custom gradient
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// use colorgrad::{Color, Gradient};
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let grad = colorgrad::GradientBuilder::new()
///     .colors(&[
///         Color::from_rgba8(255, 0, 0, 255),
///         Color::new(0.0, 0.0, 1.0, 1.0),
///     ])
///     .build::<colorgrad::LinearGradient>()?;
///
/// assert_eq!(grad.domain(), (0.0, 1.0)); // default domain
/// assert_eq!(grad.at(0.0).to_rgba8(), [255, 0, 0, 255]);
/// assert_eq!(grad.at(1.0).to_rgba8(), [0, 0, 255, 255]);
/// # Ok(())
/// # }
/// ```
///
/// ## Using web color format string
///
/// ```
/// # use std::error::Error;
/// use colorgrad::Gradient;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let grad = colorgrad::GradientBuilder::new()
///     .html_colors(&["deeppink", "gold", "seagreen"])
///     .domain(&[0.0, 100.0])
///     .mode(colorgrad::BlendMode::Rgb)
///     .build::<colorgrad::LinearGradient>()?;
///
/// assert_eq!(grad.at(0.0).to_rgba8(), [255, 20, 147, 255]);
/// assert_eq!(grad.at(100.0).to_rgba8(), [46, 139, 87, 255]);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct GradientBuilder {
    colors: Vec<Color>,
    pos: Vec<f32>,
    pub(crate) mode: BlendMode,
    invalid_html_colors: Vec<String>,
    invalid_css_gradient: bool,
}

impl GradientBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            colors: Vec::new(),
            pos: Vec::new(),
            mode: BlendMode::Rgb,
            invalid_html_colors: Vec::new(),
            invalid_css_gradient: false,
        }
    }

    /// Set gradient color
    pub fn colors<'a>(&'a mut self, colors: &[Color]) -> &'a mut Self {
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
    pub fn html_colors<'a, S: AsRef<str> + ToString>(
        &'a mut self,
        html_colors: &[S],
    ) -> &'a mut Self {
        for s in html_colors {
            if let Ok(c) = csscolorparser::parse(s.as_ref()) {
                self.colors.push(c);
            } else {
                self.invalid_html_colors.push(s.to_string());
            }
        }
        self
    }

    /// Set the gradient domain and/or color position.
    pub fn domain<'a>(&'a mut self, pos: &[f32]) -> &'a mut Self {
        self.pos = pos.to_vec();
        self
    }

    /// Set the color blending mode
    pub fn mode(&mut self, mode: BlendMode) -> &mut Self {
        self.mode = mode;
        self
    }

    /// Parse [CSS gradient](https://developer.mozilla.org/en-US/docs/Web/CSS/gradient/linear-gradient) format
    ///
    /// ```
    /// # use std::error::Error;
    /// # fn main() -> Result<(), Box<dyn Error>> {
    /// let grad = colorgrad::GradientBuilder::new()
    ///     .css("#fff, 75%, blue")
    ///     .build::<colorgrad::LinearGradient>()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn css<'a>(&'a mut self, s: &str) -> &'a mut Self {
        if let Some((colors, pos)) = css_gradient::parse(s, self.mode) {
            self.invalid_css_gradient = false;
            self.colors = colors;
            self.pos = pos;
        } else {
            self.invalid_css_gradient = true;
        }
        self
    }

    pub fn build<'a, T>(&'a self) -> Result<T, T::Error>
    where
        T: TryFrom<&'a Self, Error = GradientBuilderError>,
    {
        T::try_from(self)
    }

    /// Build the gradient
    pub(crate) fn build_(&self) -> Result<(Vec<Color>, Vec<f32>), GradientBuilderError> {
        if !self.invalid_html_colors.is_empty() {
            return Err(GradientBuilderError::InvalidHtmlColor(
                self.invalid_html_colors.clone(),
            ));
        }

        if self.invalid_css_gradient {
            return Err(GradientBuilderError::InvalidCssGradient);
        }

        let colors = if self.colors.is_empty() {
            vec![
                Color::new(0.0, 0.0, 0.0, 1.0),
                Color::new(1.0, 1.0, 1.0, 1.0),
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
                    return Err(GradientBuilderError::WrongDomain);
                }
            }
            self.pos.to_vec()
        } else if self.pos.len() == 2 {
            if self.pos[0] >= self.pos[1] {
                return Err(GradientBuilderError::WrongDomain);
            }
            linspace(self.pos[0], self.pos[1], colors.len())
        } else {
            return Err(GradientBuilderError::WrongDomainCount);
        };

        Ok((colors, pos))
    }
}
