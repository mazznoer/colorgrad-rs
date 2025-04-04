use crate::{Color, Gradient, SharpGradient};

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

    fn colors(&self, n: usize) -> Vec<Color> {
        (**self).colors(n)
    }

    fn sharp(&self, segment: u16, smoothness: f32) -> SharpGradient {
        (**self).sharp(segment, smoothness)
    }

    fn boxed<'a>(self) -> Box<dyn Gradient + 'a>
    where
        Self: 'a,
    {
        Box::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Gradient, GradientBuilder, LinearGradient};

    #[test]
    fn boxed_gradients() {
        let gradient = GradientBuilder::new()
            .html_colors(&["#fff", "#000"])
            .build::<LinearGradient>()
            .unwrap()
            .boxed();

        assert_eq!(gradient.at(0.0).to_rgba8(), [255, 255, 255, 255]);
        assert_eq!(gradient.repeat_at(1.25).to_rgba8(), [191, 191, 191, 255]);
        assert_eq!(gradient.reflect_at(1.25).to_rgba8(), [64, 64, 64, 255]);
        assert_eq!(gradient.domain(), (0.0, 1.0));
        assert_eq!(gradient.colors(3).len(), 3);
        assert_eq!(gradient.sharp(3, 0.0).colors(3).len(), 3);
    }
}
