use colorgrad::GradientColors;

#[allow(dead_code)]
pub fn colors2hex(colors: GradientColors) -> Vec<String> {
    colors.map(|c| c.to_css_hex()).collect()
}

#[macro_export]
macro_rules! cmp_hex {
    ($color:expr, $hex:expr) => {
        assert_eq!($color.to_css_hex(), $hex);
    };
}
