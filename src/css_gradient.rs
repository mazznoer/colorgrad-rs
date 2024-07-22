use crate::{BlendMode, Color};

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

    if stops[0].0.is_none() {
        return None;
    }

    for i in 0..stops.len() {
        if i == 0 && stops[i].1.is_none() {
            stops[i].1 = Some(0.0);
            continue;
        }

        if i == stops.len() - 1 {
            if stops[i].1.is_none() {
                stops[i].1 = Some(1.0);
            }
            break;
        }

        if stops[i].0.is_none() {
            if stops[i + 1].0.is_none() {
                return None;
            }
            let col1 = stops[i - 1].0.as_ref().unwrap();
            let col2 = stops[i + 1].0.as_ref().unwrap();
            let col = match mode {
                BlendMode::Rgb => col1.interpolate_rgb(col2, 0.5),
                BlendMode::LinearRgb => col1.interpolate_linear_rgb(col2, 0.5),
                BlendMode::Oklab => col1.interpolate_oklab(col2, 0.5),
                #[cfg(feature = "lab")]
                BlendMode::Lab => col1.interpolate_lab(col2, 0.5),
            };
            stops[i].0 = Some(col);
        }
    }

    if stops[0].1.unwrap() > 0.0 {
        stops.insert(0, (stops[0].0.clone(), Some(0.0)));
    }

    if stops[stops.len() - 1].1.unwrap() < 1.0 {
        stops.push((stops[stops.len() - 1].0.clone(), Some(1.0)));
    }

    for i in 0..stops.len() {
        if stops[i].1.is_none() {
            for j in (i + 1)..stops.len() {
                if let Some(next) = stops[j].1 {
                    let prev = stops[i - 1].1.unwrap();
                    stops[i].1 = Some(prev + (next - prev) / (j - i + 1) as f32);
                    break;
                }
            }
        }

        if i > 0 {
            stops[i].1 = Some(stops[i].1.unwrap().max(stops[i - 1].1.unwrap()));
        }
    }

    for (col, pos) in &stops {
        if col.is_none() || pos.is_none() {
            return None;
        }
    }

    let colors = stops
        .iter()
        .map(|(c, _)| c.clone().unwrap())
        .collect::<Vec<_>>();
    let pos = stops.iter().map(|(_, p)| p.unwrap()).collect::<Vec<_>>();
    Some((colors, pos))
}

fn parse_stop(stops: &mut Vec<(Option<Color>, Option<f32>)>, stop: &[&str]) -> bool {
    match stop.len() {
        1 => {
            if let Ok(c) = stop[0].parse::<Color>() {
                stops.push((Some(c), None));
            } else if let Some(pos) = parse_pos(stop[0]) {
                stops.push((None, Some(pos)));
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

            stops.push((col, p));
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

            stops.push((col.clone(), p1));
            stops.push((col, p2));
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
