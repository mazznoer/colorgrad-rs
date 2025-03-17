use csscolorparser::Color;

use crate::{Gradient, SharpGradient};

impl Gradient for Box<dyn Gradient> {
    fn at(&self, t: f32) -> crate::Color {
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

    fn colors(&self, n: usize) -> Vec<Color> {
        (**self).colors(n)
    }

    fn sharp(&self, segment: u16, smoothness: f32) -> SharpGradient {
        (**self).sharp(segment, smoothness)
    }

    fn boxed(self) -> Box<dyn Gradient>
    where
        Self: 'static,
    {
        Box::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::preset;

    use super::*;

    #[test]
    fn boxed_gradients() {
        let gradient = preset::rainbow().boxed();
        assert_eq!(gradient.at(0.0).to_rgba8(), [110, 64, 170, 255]);
        assert_eq!(gradient.repeat_at(1.25).to_rgba8(), [255, 94, 99, 255]);
        assert_eq!(gradient.reflect_at(1.25).to_rgba8(), [77, 199, 194, 255]);
        assert_eq!(gradient.domain(), (0.0, 1.0));
        assert_eq!(gradient.colors(3).len(), 3);
        assert_eq!(gradient.sharp(3, 0.0).colors(3).len(), 3);
    }
}
