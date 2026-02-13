use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{DistanceData, DistanceUnit};

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
    if let Ok(v) = s.parse::<f64>() {
        return Some(v);
    }
    if let Some((i, f)) = s.split_once('點').or_else(|| s.split_once('点')) {
        let int_v = parse_zh_integer(i)?;
        let mut frac = 0.0;
        let mut place = 0.1;
        for ch in f.chars() {
            let d = zh_digit(ch)?;
            frac += d * place;
            place /= 10.0;
        }
        return Some(int_v + frac);
    }
    parse_zh_integer(s)
}

fn parse_unit(s: &str) -> Option<DistanceUnit> {
    match s.to_lowercase().as_str() {
        "cm" | "厘米" | "公分" => Some(DistanceUnit::Centimetre),
        "m" | "米" | "公尺" => Some(DistanceUnit::Metre),
        "km" | "千米" | "公里" | "公裏" => Some(DistanceUnit::Kilometre),
        "'" | "foot" | "feet" | "foots" | "feets" | "英尺" | "呎" => Some(DistanceUnit::Foot),
        "\"" | "''" | "inch" | "inches" | "英寸" | "英吋" | "吋" => Some(DistanceUnit::Inch),
        "mile" | "miles" | "英里" | "英裏" => Some(DistanceUnit::Mile),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<number><zh distance unit>".to_string(),
            pattern: vec![regex("([0-9]+(?:\\.[0-9]+)?)\\s*(cm|m|km|foots?|feets?|feet|inch(es)?|miles?|厘米|公分|米|公尺|公里|公裏|英尺|呎|英寸|英吋|吋|英里|英裏|\\'\\'|\\\"|\\')")],
            production: Box::new(|nodes| {
                let (value, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.parse::<f64>().ok()?, parse_unit(m.group(2)?)?),
                    _ => return None,
                };
                Some(TokenData::Distance(DistanceData::new(value, unit)))
            }),
        },
        Rule {
            name: "<zh numeral><zh distance unit>".to_string(),
            pattern: vec![regex("([零一二兩两三四五六七八九十百千點点]+)\\s*(厘米|公分|米|公尺|公里|公裏|英尺|呎|英寸|英吋|吋|英里|英裏)")],
            production: Box::new(|nodes| {
                let (value, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (parse_zh_number(m.group(1)?)?, parse_unit(m.group(2)?)?),
                    _ => return None,
                };
                Some(TokenData::Distance(DistanceData::new(value, unit)))
            }),
        },
        Rule {
            name: "one meter and decimal".to_string(),
            pattern: vec![regex("米([零一二兩两三四五六七八九])")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => zh_digit(m.group(1)?.chars().next()?)?,
                    _ => return None,
                };
                Some(TokenData::Distance(DistanceData::new(
                    1.0 + d / 10.0,
                    DistanceUnit::Metre,
                )))
            }),
        },
        Rule {
            name: "meters and decimal".to_string(),
            pattern: vec![regex("([零一二兩两三四五六七八九十百千]+)米([零一二兩两三四五六七八九])")],
            production: Box::new(|nodes| {
                let (v1, v2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (
                        parse_zh_number(m.group(1)?)?,
                        zh_digit(m.group(2)?.chars().next()?)?,
                    ),
                    _ => return None,
                };
                Some(TokenData::Distance(DistanceData::new(
                    v1 + v2 / 10.0,
                    DistanceUnit::Metre,
                )))
            }),
        },
        Rule {
            name: "between distances".to_string(),
            pattern: vec![regex("([0-9]+(?:\\.[0-9]+)?|[零一二兩两三四五六七八九十百千點点]+|米[零一二兩两三四五六七八九]|[零一二兩两三四五六七八九十百千]+米[零一二兩两三四五六七八九])\\s*(-|~|到|至)\\s*([0-9]+(?:\\.[0-9]+)?|[零一二兩两三四五六七八九十百千點点]+|米[零一二兩两三四五六七八九]|[零一二兩两三四五六七八九十百千]+米[零一二兩两三四五六七八九])\\s*(m|米|公尺)?")],
            production: Box::new(|nodes| {
                let (left, right, unit_text) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)?, m.group(4)),
                    _ => return None,
                };
                let parse_special = |s: &str| -> Option<f64> {
                    if let Some(rest) = s.strip_prefix('米') {
                        return Some(1.0 + zh_digit(rest.chars().next()?)? / 10.0);
                    }
                    if let Some((a, b)) = s.split_once('米') {
                        let v1 = parse_zh_number(a)?;
                        let v2 = zh_digit(b.chars().next()?)?;
                        return Some(v1 + v2 / 10.0);
                    }
                    parse_zh_number(s)
                };
                let from = parse_special(left)?;
                let to = parse_special(right)?;
                if from >= to {
                    return None;
                }
                let unit = parse_unit(unit_text.unwrap_or("米"))?;
                Some(TokenData::Distance(
                    DistanceData::unit_only(unit).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "distance bound max".to_string(),
            pattern: vec![regex("(最多\\s*([0-9]+(?:\\.[0-9]+)?|[零一二兩两三四五六七八九十百千點点]+)\\s*(英里|英裏|米|公尺|公里|公裏|吋|英吋|英寸|\\\")|([0-9]+(?:\\.[0-9]+)?|[零一二兩两三四五六七八九十百千點点]+)\\s*(英里|英裏|米|公尺|公里|公裏|吋|英吋|英寸|\\\")\\s*以下)")],
            production: Box::new(|nodes| {
                let (vtext, utext) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => {
                        if let (Some(v), Some(u)) = (m.group(2), m.group(3)) {
                            (v, u)
                        } else {
                            (m.group(4)?, m.group(5)?)
                        }
                    }
                    _ => return None,
                };
                let value = parse_zh_number(vtext)?;
                let unit = parse_unit(utext)?;
                Some(TokenData::Distance(
                    DistanceData::unit_only(unit).with_max(value),
                ))
            }),
        },
        Rule {
            name: "distance bound min".to_string(),
            pattern: vec![regex("((至少|最少|起碼)\\s*([0-9]+(?:\\.[0-9]+)?|[零一二兩两三四五六七八九十百千點点]+)\\s*(\\\"|吋|英吋|英寸|英里|英裏|米|公尺)|([0-9]+(?:\\.[0-9]+)?|[零一二兩两三四五六七八九十百千點点]+)\\s*(\\\"|吋|英吋|英寸|英里|英裏|米|公尺)\\s*以上)")],
            production: Box::new(|nodes| {
                let (vtext, utext) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => {
                        if let (Some(v), Some(u)) = (m.group(3), m.group(4)) {
                            (v, u)
                        } else {
                            (m.group(5)?, m.group(6)?)
                        }
                    }
                    _ => return None,
                };
                let value = parse_zh_number(vtext)?;
                let unit = parse_unit(utext)?;
                Some(TokenData::Distance(
                    DistanceData::unit_only(unit).with_min(value),
                ))
            }),
        },
        Rule {
            name: "about <distance>".to_string(),
            pattern: vec![regex("(米九|1\\.9米)左右")],
            production: Box::new(|_| {
                Some(TokenData::Distance(DistanceData::new(
                    1.9,
                    DistanceUnit::Metre,
                )))
            }),
        },
    ]
}
