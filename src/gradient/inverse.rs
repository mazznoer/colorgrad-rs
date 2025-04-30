use crate::{Color, Gradient};

#[cfg_attr(
    feature = "preset",
    doc = r##"
A special gradient that inverts the inner gradient.

The minimum value of the inner gradient will be the maximum value of the inverse gradient and
vice versa.

```
use colorgrad::Gradient;

let gradient = colorgrad::preset::magma();
let inverted = gradient.inverse();

assert_eq!(gradient.at(0.9).to_rgba8(), inverted.at(0.1).to_rgba8());

for color in inverted.colors_iter(15) {
    println!("{}", color.to_hex_string());
}
```
"##
)]
#[derive(Clone)]
pub struct InverseGradient<'a> {
    inner: Box<dyn Gradient + 'a>,
}

impl<'a> InverseGradient<'a> {
    pub fn new(inner: Box<dyn Gradient + 'a>) -> Self {
        Self { inner }
    }
}

impl Gradient for InverseGradient<'_> {
    fn at(&self, t: f32) -> Color {
        let (min, max) = self.inner.domain();
        self.inner.at(max - t + min)
    }

    fn domain(&self) -> (f32, f32) {
        self.inner.domain()
    }
}
