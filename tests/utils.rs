use colorgrad::Color;

#[allow(dead_code)]
pub fn colors2hex(colors: &[Color]) -> Vec<String> {
    let mut res = Vec::with_capacity(colors.len());
    for c in colors {
        res.push(c.to_css_hex());
    }
    res
}

#[macro_export]
macro_rules! cmp_hex {
    ($color:expr, $hex:expr) => {
        assert_eq!($color.to_css_hex(), $hex);
    };
}
