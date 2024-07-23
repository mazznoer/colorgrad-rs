use crate::{BlendMode, Color};

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

#[allow(clippy::question_mark)]
pub(crate) fn parse(s: &str, mode: BlendMode) -> Option<(Vec<Color>, Vec<f32>)> {
    let mut stops = Vec::new();

    for stop in split_by_comma(s) {
        if !parse_stop(&mut stops, &split_by_space(stop)) {
            return None;
        }
    }

    if stops.is_empty() {
        return None;
    }

    if stops[0].col.is_none() {
        return None;
    }

    for i in 0..stops.len() {
        if i == 0 && stops[i].pos.is_none() {
            stops[i].pos = Some(0.0);
            continue;
        }

        if i == stops.len() - 1 {
            if stops[i].pos.is_none() {
                stops[i].pos = Some(1.0);
            }
            break;
        }

        if stops[i].col.is_none() {
            if stops[i + 1].col.is_none() {
                return None;
            }
            let col1 = stops[i - 1].col.as_ref().unwrap();
            let col2 = stops[i + 1].col.as_ref().unwrap();
            let col = match mode {
                BlendMode::Rgb => col1.interpolate_rgb(col2, 0.5),
                BlendMode::LinearRgb => col1.interpolate_linear_rgb(col2, 0.5),
                BlendMode::Oklab => col1.interpolate_oklab(col2, 0.5),
                #[cfg(feature = "lab")]
                BlendMode::Lab => col1.interpolate_lab(col2, 0.5),
            };
            stops[i].col = Some(col);
        }
    }

    if stops[0].pos.unwrap() > 0.0 {
        stops.insert(0, Stop::new(stops[0].col.clone(), Some(0.0)));
    }

    if stops[stops.len() - 1].pos.unwrap() < 1.0 {
        stops.push(Stop::new(stops[stops.len() - 1].col.clone(), Some(1.0)));
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

    for stop in &stops {
        if !stop.valid() {
            return None;
        }
    }

    let colors = stops
        .iter()
        .map(|stop| stop.col.clone().unwrap())
        .collect::<Vec<_>>();
    let positions = stops
        .iter()
        .map(|stop| stop.pos.unwrap())
        .collect::<Vec<_>>();
    Some((colors, positions))
}

fn parse_stop(stops: &mut Vec<Stop>, stop: &[&str]) -> bool {
    match stop.len() {
        1 => {
            if let Ok(c) = stop[0].parse::<Color>() {
                stops.push(Stop::new(Some(c), None));
            } else if let Some(pos) = parse_pos(stop[0]) {
                stops.push(Stop::new(None, Some(pos)));
            } else {
                return false;
            }
        }
        2 => {
            let col = if let Ok(c) = stop[0].parse::<Color>() {
                Some(c)
            } else {
                return false;
            };

            let p = if let Some(pos) = parse_pos(stop[1]) {
                Some(pos)
            } else {
                return false;
            };

            stops.push(Stop::new(col, p));
        }
        3 => {
            let col = if let Ok(c) = stop[0].parse::<Color>() {
                Some(c)
            } else {
                return false;
            };

            let p1 = if let Some(pos) = parse_pos(stop[1]) {
                Some(pos)
            } else {
                return false;
            };

            let p2 = if let Some(pos) = parse_pos(stop[2]) {
                Some(pos)
            } else {
                return false;
            };

            stops.push(Stop::new(col.clone(), p1));
            stops.push(Stop::new(col, p2));
        }
        _ => {
            return false;
        }
    }
    true
}

fn parse_pos(s: &str) -> Option<f32> {
    s.strip_suffix('%')
        .and_then(|s| s.parse().ok().map(|t: f32| t / 100.0))
        .or_else(|| s.parse().ok())
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
