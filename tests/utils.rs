use std::borrow::Borrow;

use colorgrad::Color;

#[allow(dead_code)]
pub fn colors2hex<T, U>(colors: T) -> Vec<String>
where
    T: IntoIterator<Item = U>,
    U: Borrow<Color>,
{
    colors
        .into_iter()
        .map(|c| c.borrow().to_css_hex().to_string())
        .collect()
}

#[macro_export]
macro_rules! cmp_hex {
    ($color:expr, $hex:expr) => {
        assert_eq!($color.to_css_hex().to_string(), $hex);
    };
}
