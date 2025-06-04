use crate::{BlendMode, Color};

#[derive(Debug, PartialEq)]
struct Stop {
    col: Option<Color>,
    pos: Option<f32>,
}

impl Stop {
    fn new(col: Option<Color>, pos: Option<f32>) -> Self {
        Self { col, pos }
    }

    fn valid(&self) -> bool {
        self.col.is_some() && self.pos.is_some()
    }
}

pub struct CSSGradientParser {
    dmin: f32,
    dmax: f32,
    mode: BlendMode,
    stops: Vec<Stop>,
}

impl CSSGradientParser {
    pub fn new() -> Self {
        Self {
            dmin: 0.0,
            dmax: 1.0,
            mode: BlendMode::Rgb,
            stops: Vec::new(),
        }
    }

    pub fn set_domain(&mut self, min: f32, max: f32) {
        assert!(min < max);
        self.dmin = min;
        self.dmax = max;
    }

    pub fn set_mode(&mut self, mode: BlendMode) {
        self.mode = mode;
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        self.dmin = 0.0;
        self.dmax = 1.0;
        self.mode = BlendMode::Rgb;
        self.stops.clear();
    }

    #[allow(clippy::question_mark)]
    pub fn parse(&mut self, s: &str) -> Option<(Vec<Color>, Vec<f32>)> {
        if self.dmin >= self.dmax {
            return None;
        }

        for stop in split_by_comma(s) {
            if !self.parse_stop(stop) {
                return None;
            }
        }

        let stops = &mut self.stops;

        if stops.is_empty() {
            return None;
        }

        if stops[0].col.is_none() {
            return None;
        }

        if stops[0].pos.is_none() {
            stops[0].pos = Some(self.dmin);
        }

        for i in 0..stops.len() {
            if i == stops.len() - 1 {
                if stops[i].pos.is_none() {
                    stops[i].pos = Some(self.dmax);
                }
                break;
            }

            if stops[i].col.is_none() {
                if stops[i + 1].col.is_none() {
                    return None;
                }
                let col1 = stops[i - 1].col.as_ref().unwrap();
                let col2 = stops[i + 1].col.as_ref().unwrap();
                let col = match self.mode {
                    BlendMode::Rgb => col1.interpolate_rgb(col2, 0.5),
                    BlendMode::LinearRgb => col1.interpolate_linear_rgb(col2, 0.5),
                    BlendMode::Oklab => col1.interpolate_oklab(col2, 0.5),
                    #[cfg(feature = "lab")]
                    BlendMode::Lab => col1.interpolate_lab(col2, 0.5),
                };
                stops[i].col = Some(col);
            }
        }

        if stops[0].pos.unwrap() > self.dmin {
            stops.insert(0, Stop::new(stops[0].col.clone(), Some(self.dmin)));
        }

        if stops[stops.len() - 1].pos.unwrap() < self.dmax {
            stops.push(Stop::new(
                stops[stops.len() - 1].col.clone(),
                Some(self.dmax),
            ));
        }

        for i in 0..stops.len() {
            if stops[i].pos.is_none() {
                for j in (i + 1)..stops.len() {
                    if let Some(next) = stops[j].pos {
                        let prev = stops[i - 1].pos.unwrap();
                        stops[i].pos = Some(prev + (next - prev) / (j - i + 1) as f32);
                        break;
                    }
                }
            }

            if i > 0 {
                stops[i].pos = Some(stops[i].pos.unwrap().max(stops[i - 1].pos.unwrap()));
            }
        }

        for stop in &self.stops {
            if !stop.valid() {
                return None;
            }
        }

        let positions: Vec<_> = self.stops.iter().map(|s| s.pos.unwrap()).collect();
        let colors: Vec<_> = self.stops.iter().map(|s| s.col.clone().unwrap()).collect();
        Some((colors, positions))
    }

    #[rustfmt::skip]
    pub fn parse_stop(&mut self, s: &str) -> bool {
        match split_by_space(s)[..] {
            [s] => {
                if let Ok(color) = s.parse::<Color>() {
                    self.stops.push(Stop::new(Some(color), None));
                } else if let Some(position) = self.parse_pos(s) {
                    self.stops.push(Stop::new(None, Some(position)));
                } else {
                    return false;
                }
            }
            [color, position] => {
                let (
                    Ok(color),
                    Some(position),
                ) = (
                    color.parse::<Color>(),
                    self.parse_pos(position),
                ) else {
                    return false;
                };
                self.stops.push(Stop::new(Some(color), Some(position)));
            }
            [color, position1, position2] => {
                let (
                    Ok(color),
                    Some(position1),
                    Some(position2),
                ) = (
                    color.parse::<Color>(),
                    self.parse_pos(position1),
                    self.parse_pos(position2),
                ) else {
                    return false;
                };
                self.stops.push(Stop::new(Some(color.clone()), Some(position1)));
                self.stops.push(Stop::new(Some(color), Some(position2)));
            }
            _ => {
                return false;
            }
        }
        true
    }

    #[rustfmt::skip]
    pub fn parse_pos(&self, s: &str) -> Option<f32> {
        s.strip_suffix('%')
            .and_then(|s| {
                s.parse().ok().map(|t: f32| {
                    t / 100.0 * (self.dmax - self.dmin) + self.dmin
                })
            })
            .or_else(|| s.parse().ok())
    }
}

fn split_by_comma(s: &str) -> Vec<&str> {
    let mut res = Vec::new();
    let mut start = 0;
    let mut inside = false;

    for (i, c) in s.chars().enumerate() {
        if c == ',' && !inside {
            res.push(&s[start..i]);
            start = i + 1;
        } else if c == '(' {
            inside = true;
        } else if c == ')' {
            inside = false;
        }
    }
    res.push(&s[start..]);
    res
}

fn split_by_space(s: &str) -> Vec<&str> {
    let mut res = Vec::new();
    let mut start = 0;
    let mut inside = false;

    for (i, c) in s.chars().enumerate() {
        if c == ' ' && !inside {
            if !s[start..i].is_empty() {
                res.push(&s[start..i]);
            }
            start = i + 1;
        } else if c == '(' {
            inside = true;
        } else if c == ')' {
            inside = false;
        }
    }
    if !s[start..].is_empty() {
        res.push(&s[start..]);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn utils() {
        assert_eq!(split_by_comma("red, #fff, lime"), ["red", " #fff", " lime"]);
        assert_eq!(
            split_by_comma("#ff0000, rgb(255, 0, 0), hsv(120, 50%, 50%) 75%, blue"),
            [
                "#ff0000",
                " rgb(255, 0, 0)",
                " hsv(120, 50%, 50%) 75%",
                " blue"
            ]
        );

        assert_eq!(
            split_by_space("rgb(0, 0, 150) 0.75 1"),
            ["rgb(0, 0, 150)", "0.75", "1"]
        );
        assert_eq!(
            split_by_space("hsv(360, 50%, 30%) 0% 35%"),
            ["hsv(360, 50%, 30%)", "0%", "35%"]
        );
    }

    #[test]
    fn test_parse_pos() {
        let mut gp = CSSGradientParser::new();

        assert_eq!(gp.parse_pos("0.5"), Some(0.5));
        assert_eq!(gp.parse_pos("50%"), Some(0.5));
        assert_eq!(gp.parse_pos("1.1"), Some(1.1));
        assert_eq!(gp.parse_pos("100%"), Some(1.0));
        assert_eq!(gp.parse_pos("75%"), Some(0.75));

        assert_eq!(gp.parse_pos(""), None);
        assert_eq!(gp.parse_pos("50x%"), None);
        assert_eq!(gp.parse_pos("y"), None);

        gp.set_domain(10.0, 30.0);

        assert_eq!(gp.parse_pos("0%"), Some(10.0));
        assert_eq!(gp.parse_pos("50%"), Some(20.0));
        assert_eq!(gp.parse_pos("100%"), Some(30.0));
        assert_eq!(gp.parse_pos("17"), Some(17.0));
    }

    #[test]
    fn test_parse_stop() {
        fn c(s: &str) -> Color {
            s.parse::<Color>().unwrap()
        }
        assert_eq!(c("#ff0000"), Color::new(1.0, 0.0, 0.0, 1.0));
        assert_ne!(c("#ff0001"), Color::new(1.0, 0.0, 0.0, 1.0));

        let mut gp = CSSGradientParser::new();

        // color only
        assert!(gp.parse_stop("#f00"));
        assert_eq!(gp.stops[0], Stop::new(Some(c("#f00")), None));

        // position only
        assert!(gp.parse_stop("75%"));
        assert_eq!(gp.stops[1], Stop::new(None, Some(0.75)));

        // color & position
        assert!(gp.parse_stop("#f00 10%"));
        assert_eq!(gp.stops[2], Stop::new(Some(c("#f00")), Some(0.1)));

        // color & double positions
        assert!(gp.parse_stop("#ff0 0% 50%"));
        assert_eq!(gp.stops[3], Stop::new(Some(c("#ff0")), Some(0.0)));
        assert_eq!(gp.stops[4], Stop::new(Some(c("#ff0")), Some(0.5)));

        assert_eq!(gp.stops.len(), 5);

        // invalid
        assert!(!gp.parse_stop(""));
        assert!(!gp.parse_stop("#zbb"));
        assert!(!gp.parse_stop("0x%"));

        assert!(!gp.parse_stop("#000 x"));
        assert!(!gp.parse_stop("#xyz 10%"));

        assert!(!gp.parse_stop("#f00 50% x"));
        assert!(!gp.parse_stop("#f00 x 0%"));
        assert!(!gp.parse_stop("#ffm 20% 30%"));

        assert!(!gp.parse_stop("#f00 20% 30% 50%"));
        assert_eq!(gp.stops.len(), 5);
    }

    fn colors2hex(colors: Vec<Color>) -> Vec<String> {
        colors.iter().map(|c| c.to_css_hex()).collect()
    }

    #[test]
    fn parse_css_gradient() {
        let mut gp = CSSGradientParser::new();

        let s = "#f00, #0f0";

        let (colors, positions) = gp.parse(s).unwrap();
        assert_eq!(colors2hex(colors), ["#ff0000", "#00ff00"]);
        assert_eq!(positions, [0.0, 1.0]);

        gp.reset();
        gp.set_domain(-10.0, 10.0);
        let (colors, positions) = gp.parse(s).unwrap();
        assert_eq!(colors2hex(colors), ["#ff0000", "#00ff00"]);
        assert_eq!(positions, [-10.0, 10.0]);

        let s = "#f00, #00f 75%, #0f0";

        gp.reset();
        let (colors, positions) = gp.parse(s).unwrap();
        assert_eq!(colors2hex(colors), ["#ff0000", "#0000ff", "#00ff00"]);
        assert_eq!(positions, [0.0, 0.75, 1.0]);

        gp.reset();
        gp.set_domain(0.0, 100.0);
        let (colors, positions) = gp.parse(s).unwrap();
        assert_eq!(colors2hex(colors), ["#ff0000", "#0000ff", "#00ff00"]);
        assert_eq!(positions, [0.0, 75.0, 100.0]);

        let s = "#f00, #0f0 15, #00f";

        gp.reset();
        gp.set_domain(0.0, 20.0);
        let (colors, positions) = gp.parse(s).unwrap();
        assert_eq!(colors2hex(colors), ["#ff0000", "#00ff00", "#0000ff"]);
        assert_eq!(positions, [0.0, 15.0, 20.0]);
    }
}
