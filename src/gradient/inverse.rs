use crate::Gradient;

/// A gradient that inverts the gradient of another gradient.
///
/// The minimum value of the inner gradient will be the maximum value of the inverse gradient and
/// vice versa.
#[derive(Clone)]
pub struct InverseGradient {
    inner: Box<dyn Gradient>,
}

impl InverseGradient {
    pub fn new(inner: Box<dyn Gradient>) -> Self {
        Self { inner }
    }
}

impl Gradient for InverseGradient {
    fn at(&self, t: f32) -> crate::Color {
        let (min, max) = self.inner.domain();
        self.inner.at(max - t * (max - min))
    }
}
