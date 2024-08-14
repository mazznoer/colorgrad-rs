use colorgrad::Color;

pub fn colors2hex(colors: &[Color]) -> Vec<String> {
    let mut res = Vec::with_capacity(colors.len());
    for c in colors {
        res.push(c.to_hex_string());
    }
    res
}
