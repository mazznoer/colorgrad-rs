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

#[cfg(test)]
mod tests {
    use crate::CloneGradient;

    use super::*;

    #[test]
    fn inverse() {
        let rainbow = crate::preset::rainbow();
        let inverse = InverseGradient::new(rainbow.clone_gradient());

        // Note that some inversions may not be exact due to floating point errors (E.g.
        // `inverse.at(0.8) != rainbow.at(0.2)` at the last decimal place). If this test ever fails
        // with values that are very close, consider changing this test to check each the float
        // value of each color channel channel separately with an epsilon value.
        assert_eq!(inverse.at(0.0), rainbow.at(1.0));
        assert_eq!(inverse.at(0.3), rainbow.at(0.7));
        assert_eq!(inverse.at(0.5), rainbow.at(0.5));
        assert_eq!(inverse.at(0.7), rainbow.at(0.3));
        assert_eq!(inverse.at(1.0), rainbow.at(0.0));
    }
}
