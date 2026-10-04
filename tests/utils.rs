#![allow(dead_code)]

use std::borrow::Borrow;

use colorgrad::Color;

pub fn color_diff(a: Color, b: Color) -> f32 {
    let a = a.clamp();
    let b = b.clamp();

    let dr = a.r - b.r;
    let dg = a.g - b.g;
    let db = a.b - b.b;
    let da = a.a - b.a;

    (dr * dr + dg * dg + db * db + da * da).sqrt() / 2.0
}

pub fn colors2hex<T, U>(colors: T) -> Vec<String>
where
    T: IntoIterator<Item = U>,
    U: Borrow<Color>,
{
    colors.into_iter().map(|c| c.borrow().to_string()).collect()
}

#[macro_export]
macro_rules! cmp_hex {
    ($color:expr, $hex:expr) => {
        assert_eq!($color.to_string(), $hex);
    };
}
