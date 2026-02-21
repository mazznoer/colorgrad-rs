use alloc::boxed::Box;
use alloc::vec;

use crate::utils::{linspace, modulo, norm};
use crate::{Color, InverseGradient, SharpGradient};

/// Color blending mode
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BlendMode {
    Rgb,
    LinearRgb,
    Oklab,
    Lab,
}

/// All gradient types in `colorgrad` implement `Gradient` trait.
///
/// You can also implement `Gradient` for your own types.
///
/// ```
/// use colorgrad::{Color, Gradient};
///
/// #[derive(Clone)]
/// struct MyRedGradient {}
///
/// impl Gradient for MyRedGradient {
///     fn at(&self, t: f32) -> Color {
///         Color::new(1.0, 0.0, 0.0, 1.0)
///     }
/// }
///
/// let g = MyRedGradient{};
/// assert_eq!(g.domain(), (0.0, 1.0));
/// assert_eq!(g.at(0.1).to_css_hex().to_string(), "#ff0000");
///
/// for color in g.colors(25) {
///     println!("{:?}", color.to_rgba8());
/// }
/// ```
pub trait Gradient: CloneGradient {
    /// Get color at certain position
    fn at(&self, t: f32) -> Color;

    /// Get color at certain position (**repeat** mode)
    fn repeat_at(&self, t: f32) -> Color {
        let (dmin, dmax) = self.domain();
        let t = norm(t, dmin, dmax);
        self.at(dmin + modulo(t, 1.0) * (dmax - dmin))
    }

    /// Get color at certain position (**reflect** mode)
    fn reflect_at(&self, t: f32) -> Color {
        let (dmin, dmax) = self.domain();
        let t = norm(t, dmin, dmax);
        self.at(dmin + (modulo(1.0 + t, 2.0) - 1.0).abs() * (dmax - dmin))
    }

    /// Get the gradient's domain min and max
    fn domain(&self) -> (f32, f32) {
        (0.0, 1.0)
    }

    /// Returns iterator for `n` colors evenly spaced across gradient
    fn colors(&self, n: usize) -> GradientColors<'_>
    where
        Self: Sized,
    {
        GradientColors::new(self, n)
    }

    #[deprecated(since = "0.9.0", note = "Use `colors()` instead")]
    /// Returns iterator for `n` colors evenly spaced across gradient
    fn colors_iter(&self, n: usize) -> GradientColors<'_>
    where
        Self: Sized,
    {
        GradientColors::new(self, n)
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
        let (dmin, dmax) = self.domain();
        let colors = if segment > 1 {
            linspace(dmin, dmax, segment.into())
                .map(|t| self.at(t))
                .collect()
        } else {
            vec![self.at(dmin), self.at(dmin)]
        };
        SharpGradient::new(&colors, (dmin, dmax), smoothness)
    }

    #[cfg_attr(
        feature = "preset",
        doc = r##"
Convert gradient to boxed trait object

This is a convenience function, which is useful when you want to store gradients with
different types in a collection, or when you want to return a gradient from a function but
the type is not known at compile time.

## Examples

```
# fn main() -> Result<(), Box<dyn std::error::Error>> {
# let is_rainbow = true;
# use colorgrad::{BlendMode, LinearGradient, GradientBuilder};
use colorgrad::Gradient;

let g = if is_rainbow {
    colorgrad::preset::rainbow().boxed()
} else {
    colorgrad::preset::sinebow().boxed()
};

// Vector of different gradient types

let g2: LinearGradient = GradientBuilder::new()
    .css("#a52a2a, 35%, #ffd700")
    .mode(BlendMode::Oklab)
    .build()?;

let gradients = vec![
    g2.sharp(7, 0.0).boxed(),
    g2.boxed(),
    colorgrad::preset::magma().boxed(),
    colorgrad::preset::turbo().boxed(),
];
# Ok(())
# }
```
"##
    )]
    fn boxed<'a>(self) -> Box<dyn Gradient + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
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
    ///
    /// let grad = colorgrad::GradientBuilder::new()
    ///     .html_colors(&["#fff", "#000"])
    ///     .build::<colorgrad::LinearGradient>()
    ///     .unwrap();
    ///
    /// let inverse = grad.inverse();
    /// ```
    fn inverse<'a>(&self) -> InverseGradient<'_>
    where
        Self: 'a,
    {
        InverseGradient::new(self.clone_boxed())
    }
}

pub trait CloneGradient {
    fn clone_boxed<'s>(&self) -> Box<dyn Gradient + 's>
    where
        Self: 's;
}

impl<T> CloneGradient for T
where
    T: Gradient + Clone,
{
    fn clone_boxed<'s>(&self) -> Box<dyn Gradient + 's>
    where
        Self: 's,
    {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Gradient + '_> {
    fn clone(&self) -> Self {
        (**self).clone_boxed()
    }
}

impl Gradient for Box<dyn Gradient + '_> {
    fn at(&self, t: f32) -> Color {
        (**self).at(t)
    }

    fn repeat_at(&self, t: f32) -> Color {
        (**self).repeat_at(t)
    }

    fn reflect_at(&self, t: f32) -> Color {
        (**self).reflect_at(t)
    }

    fn domain(&self) -> (f32, f32) {
        (**self).domain()
    }

    fn colors(&self, n: usize) -> GradientColors<'_> {
        GradientColors::new(self, n)
    }

    fn sharp(&self, segment: u16, smoothness: f32) -> SharpGradient {
        (**self).sharp(segment, smoothness)
    }

    fn inverse<'a>(&self) -> InverseGradient<'_>
    where
        Self: 'a,
    {
        (**self).inverse()
    }

    fn boxed<'a>(self) -> Box<dyn Gradient + 'a>
    where
        Self: 'a,
    {
        Box::new(self)
    }
}

#[cfg_attr(
    feature = "preset",
    doc = r##"
Iterator for evenly spaced colors across gradient

## Examples

```
use colorgrad::Gradient;

let gradient = colorgrad::preset::magma();

for color in gradient.colors(15) {
    println!("{:?}", color.to_rgba8());
}

// reverse order

for color in gradient.colors(15).rev() {
    println!("{:?}", color.to_rgba8());
}
```
"##
)]
#[derive(Clone)]
pub struct GradientColors<'a> {
    gradient: &'a dyn Gradient,
    a_idx: usize,
    b_idx: usize,
    max: f32,
}

impl<'a> GradientColors<'a> {
    pub fn new(gradient: &'a dyn Gradient, total: usize) -> Self {
        Self {
            gradient,
            a_idx: 0,
            b_idx: total,
            max: if total == 0 { 0.0 } else { (total - 1) as f32 },
        }
    }
}

impl Iterator for GradientColors<'_> {
    type Item = Color;

    fn next(&mut self) -> Option<Self::Item> {
        if self.a_idx == self.b_idx {
            return None;
        }
        let (dmin, dmax) = self.gradient.domain();
        let t = dmin + (self.a_idx as f32 * (dmax - dmin)) / self.max;
        self.a_idx += 1;
        Some(self.gradient.at(t))
    }
}

impl DoubleEndedIterator for GradientColors<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.a_idx == self.b_idx {
            return None;
        }
        let (dmin, dmax) = self.gradient.domain();
        self.b_idx -= 1;
        let t = dmin + (self.b_idx as f32 * (dmax - dmin)) / self.max;
        Some(self.gradient.at(t))
    }
}

impl ExactSizeIterator for GradientColors<'_> {
    fn len(&self) -> usize {
        self.b_idx - self.a_idx
    }
}
