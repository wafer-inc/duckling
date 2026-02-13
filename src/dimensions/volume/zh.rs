use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{VolumeData, VolumeUnit};

fn zh_digit(ch: char) -> Option<f64> {
    match ch {
        '零' => Some(0.0),
        '一' => Some(1.0),
        '二' | '兩' | '两' => Some(2.0),
        '三' => Some(3.0),
        '四' => Some(4.0),
        '五' => Some(5.0),
        '六' => Some(6.0),
        '七' => Some(7.0),
        '八' => Some(8.0),
        '九' => Some(9.0),
        _ => None,
    }
}

fn parse_zh_integer(s: &str) -> Option<f64> {
    if s.is_empty() {
        return None;
    }
    if let Ok(v) = s.parse::<f64>() {
        return Some(v);
    }
    let mut total = 0.0f64;
    let mut current = 0.0f64;
    for ch in s.chars() {
        match ch {
            '十' => {
                let lhs = if current == 0.0 { 1.0 } else { current };
                total += lhs * 10.0;
                current = 0.0;
            }
            '百' => {
                let lhs = if current == 0.0 { 1.0 } else { current };
                total += lhs * 100.0;
                current = 0.0;
            }
            '千' => {
                let lhs = if current == 0.0 { 1.0 } else { current };
                total += lhs * 1000.0;
                current = 0.0;
            }
            _ => current = zh_digit(ch)?,
        }
    }
    Some(total + current)
}

fn parse_zh_number(s: &str) -> Option<f64> {
    if s == "半" {
        return Some(0.5);
    }
    if let Ok(v) = s.parse::<f64>() {
        return Some(v);
    }
    if let Some((d, n)) = s.split_once("分之") {
        let denom = parse_zh_integer(d)?;
        let numer = parse_zh_integer(n)?;
        return Some(numer / denom);
    }
    if let Some((int_part, frac_part)) = s.split_once('點').or_else(|| s.split_once('点')) {
        let int_v = parse_zh_integer(int_part)?;
        let mut place = 0.1;
        let mut frac_v = 0.0;
        for ch in frac_part.chars() {
            let d = zh_digit(ch)?;
            frac_v += d * place;
            place /= 10.0;
        }
        return Some(int_v + frac_v);
    }
    parse_zh_integer(s)
}

fn parse_unit(s: &str) -> Option<VolumeUnit> {
    match s.to_lowercase().as_str() {
        "升" | "公升" | "l" => Some(VolumeUnit::Litre),
        "毫升" | "ml" | "cc" => Some(VolumeUnit::Millilitre),
        "加侖" => Some(VolumeUnit::Gallon),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<number><zh volume unit>".to_string(),
            pattern: vec![regex("([0-9]+(?:\\.[0-9]+)?)\\s*(升|公升|l|毫升|ml|cc|加侖)")],
            production: Box::new(|nodes| {
                let (value, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.parse::<f64>().ok()?, parse_unit(m.group(2)?)?),
                    _ => return None,
                };
                Some(TokenData::Volume(VolumeData::new(value, unit)))
            }),
        },
        Rule {
            name: "<zh numeral><zh volume unit>".to_string(),
            pattern: vec![regex("([零一二兩两三四五六七八九十百千點点半]+)\\s*(升|公升|毫升|加侖)")],
            production: Box::new(|nodes| {
                let (value, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (parse_zh_number(m.group(1)?)?, parse_unit(m.group(2)?)?),
                    _ => return None,
                };
                Some(TokenData::Volume(VolumeData::new(value, unit)))
            }),
        },
        Rule {
            name: "fractional chinese volume".to_string(),
            pattern: vec![regex("([一二兩两三四五六七八九十百千]+分之一|半)\\s*(升|公升|加侖|湯匙|茶匙)")],
            production: Box::new(|nodes| {
                let (value, unit_text) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (parse_zh_number(m.group(1)?)?, m.group(2)?),
                    _ => return None,
                };
                let (unit, scaled) = match unit_text {
                    "湯匙" => (VolumeUnit::Millilitre, value * 15.0),
                    "茶匙" => (VolumeUnit::Millilitre, value * 5.0),
                    _ => (parse_unit(unit_text)?, value),
                };
                Some(TokenData::Volume(VolumeData::new(scaled, unit)))
            }),
        },
        Rule {
            name: "<zh numeral> teaspoon".to_string(),
            pattern: vec![regex("([零一二兩两三四五六七八九十百千]+)茶匙")],
            production: Box::new(|nodes| {
                let value = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => parse_zh_number(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Volume(VolumeData::new(
                    value * 5.0,
                    VolumeUnit::Millilitre,
                )))
            }),
        },
        Rule {
            name: "corpus one-third tablespoon".to_string(),
            pattern: vec![regex("三分一湯匙")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(
                    5.0,
                    VolumeUnit::Millilitre,
                )))
            }),
        },
        Rule {
            name: "between volumes (zh)".to_string(),
            pattern: vec![regex("([0-9零一二兩两三四五六七八九十百千]+)(升|公升|l)?\\s*(-|~|至|到)\\s*([0-9零一二兩两三四五六七八九十百千]+)\\s*(升|公升|l)?")],
            production: Box::new(|nodes| {
                let (from, to, u1, u2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => {
                        let f = parse_zh_number(m.group(1)?)?;
                        let t = parse_zh_number(m.group(4)?)?;
                        (f, t, m.group(2), m.group(5))
                    }
                    _ => return None,
                };
                if from >= to {
                    return None;
                }
                let unit = parse_unit(u2.or(u1).unwrap_or("升"))?;
                Some(TokenData::Volume(
                    VolumeData::unit_only(unit).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "at most volume (zh)".to_string(),
            pattern: vec![regex("(最多\\s*([0-9零一二兩两三四五六七八九十百千]+)\\s*(個)?\\s*(加侖|升|公升|l|毫升|ml|cc)|([0-9零一二兩两三四五六七八九十百千]+)\\s*(加侖|升|公升|l|毫升|ml|cc)\\s*(或)?以下)")],
            production: Box::new(|nodes| {
                let (value_text, unit_text) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => {
                        if let (Some(v), Some(u)) = (m.group(2), m.group(4)) {
                            (v, u)
                        } else {
                            (m.group(5)?, m.group(6)?)
                        }
                    }
                    _ => return None,
                };
                let value = parse_zh_number(value_text)?;
                let unit = parse_unit(unit_text)?;
                Some(TokenData::Volume(
                    VolumeData::unit_only(unit).with_max(value),
                ))
            }),
        },
        Rule {
            name: "at least volume (zh)".to_string(),
            pattern: vec![regex("(至少|最少|起碼)\\s*([0-9零一二兩两三四五六七八九十百千]+)\\s*(毫升|ml|cc|升|公升|l)|([0-9零一二兩两三四五六七八九十百千]+)\\s*(毫升|ml|cc|升|公升|l)\\s*(或)?以上")],
            production: Box::new(|nodes| {
                let (value_text, unit_text) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => {
                        if let (Some(v), Some(u)) = (m.group(2), m.group(3)) {
                            (v, u)
                        } else {
                            (m.group(4)?, m.group(5)?)
                        }
                    }
                    _ => return None,
                };
                let value = parse_zh_number(value_text)?;
                let unit = parse_unit(unit_text)?;
                Some(TokenData::Volume(
                    VolumeData::unit_only(unit).with_min(value),
                ))
            }),
        },
    ]
}
