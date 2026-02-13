use crate::dimensions::numeral::helpers::{decimals_to_double, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::NumeralData;

fn digit_value(c: char) -> Option<f64> {
    match c {
        '零' | '〇' => Some(0.0),
        '一' | '壹' => Some(1.0),
        '二' | '兩' | '两' | '貳' | '贰' => Some(2.0),
        '三' | '參' | '叁' => Some(3.0),
        '四' | '肆' => Some(4.0),
        '五' | '伍' => Some(5.0),
        '六' | '陸' => Some(6.0),
        '七' | '柒' => Some(7.0),
        '八' | '捌' => Some(8.0),
        '九' | '玖' => Some(9.0),
        _ => None,
    }
}

fn normalize_zh(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '萬' => '万',
            '億' => '亿',
            '點' => '点',
            '個' => '个',
            '負' => '负',
            '拾' => '十',
            '佰' => '百',
            '仟' => '千',
            '貳' => '二',
            '參' => '三',
            '陸' => '六',
            '柒' => '七',
            '肆' => '四',
            '伍' => '五',
            '壹' => '一',
            '玖' => '九',
            '捌' => '八',
            '廿' => '#',
            '卅' => '$',
            '卌' => '%',
            _ => c,
        })
        .collect::<String>()
        .replace('#', "二十")
        .replace('$', "三十")
        .replace('%', "四十")
}

fn parse_chinese_integer(raw: &str) -> Option<f64> {
    let s = normalize_zh(raw);

    if let Some(rest) = s.strip_prefix("千") {
        if rest.chars().count() == 1 {
            return Some(1000.0 + digit_value(rest.chars().next()?)? * 100.0);
        }
    }
    if let Some(rest) = s.strip_prefix("万") {
        if rest.chars().count() == 1 {
            return Some(10000.0 + digit_value(rest.chars().next()?)? * 1000.0);
        }
    }
    if let Some(stem) = s.strip_suffix("万") {
        if stem.chars().count() == 2 {
            let mut it = stem.chars();
            if let (Some(h), Some(t)) = (it.next(), it.next()) {
                if h == '百' {
                    return Some((100.0 + digit_value(t)? * 10.0) * 10000.0);
                }
            }
        }
    }

    if s.chars().all(|c| digit_value(c).is_some()) {
        let mut n = String::new();
        for c in s.chars() {
            n.push(char::from(b'0' + digit_value(c)? as u8));
        }
        return n.parse::<f64>().ok();
    }

    let mut total = 0.0;
    let mut section = 0.0;
    let mut number = 0.0;

    for c in s.chars() {
        if let Some(d) = digit_value(c) {
            number = d;
            continue;
        }
        let unit = match c {
            '十' => Some(10.0),
            '百' => Some(100.0),
            '千' => Some(1000.0),
            _ => None,
        };
        if let Some(u) = unit {
            if number == 0.0 {
                number = 1.0;
            }
            section += number * u;
            number = 0.0;
            continue;
        }
        let big = match c {
            '万' => Some(1e4),
            '亿' => Some(1e8),
            _ => None,
        };
        if let Some(u) = big {
            section += number;
            if section == 0.0 {
                section = 1.0;
            }
            total += section * u;
            section = 0.0;
            number = 0.0;
            continue;
        }
        return None;
    }

    Some(total + section + number)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "zh integer".to_string(),
            pattern: vec![regex("([〇零一二兩两三四五六七八九十百千万萬亿億壹貳參肆伍陸柒捌玖拾佰仟廿卅卌]+)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(parse_chinese_integer(s)?)))
            }),
        },
        Rule {
            name: "<number>个".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("(个|個)")],
            production: Box::new(|nodes| Some(nodes[0].token_data.clone())),
        },
        Rule {
            name: "pair".to_string(),
            pattern: vec![regex("([一壹])?(对|對|双|雙)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let v = if m.group(1).is_some() { 2.0 } else { 2.0 };
                Some(TokenData::Numeral(NumeralData::new(v).with_quantifier()))
            }),
        },
        Rule {
            name: "dozen".to_string(),
            pattern: vec![regex("([一二三四五六七八九])?打")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let mult = match m.group(1) {
                    Some(s) => parse_chinese_integer(s)?,
                    None => 1.0,
                };
                Some(TokenData::Numeral(NumeralData::new(mult * 12.0).with_quantifier()))
            }),
        },
        Rule {
            name: "half".to_string(),
            pattern: vec![regex("(一半半|一半|半个|半個|1半)")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(0.5)))),
        },
        Rule {
            name: "fraction n/d".to_string(),
            pattern: vec![regex("([一二三四五六七八九十百千兩两壹貳參肆伍陸柒捌玖0-9]+)/([一二三四五六七八九十百千兩两壹貳參肆伍陸柒捌玖0-9]+)")],
            production: Box::new(|nodes| {
                let (n, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let nn = parse_chinese_integer(n).or_else(|| n.parse::<f64>().ok())?;
                let dd = parse_chinese_integer(d).or_else(|| d.parse::<f64>().ok())?;
                if dd == 0.0 {
                    None
                } else {
                    Some(TokenData::Numeral(NumeralData::new(nn / dd)))
                }
            }),
        },
        Rule {
            name: "fraction 分之".to_string(),
            pattern: vec![regex("([一二三四五六七八九十百千兩两壹貳參肆伍陸柒捌玖0-9]+)分之([一二三四五六七八九十百千兩两壹貳參肆伍陸柒捌玖0-9]+)")],
            production: Box::new(|nodes| {
                let (d, n) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let nn = parse_chinese_integer(n).or_else(|| n.parse::<f64>().ok())?;
                let dd = parse_chinese_integer(d).or_else(|| d.parse::<f64>().ok())?;
                if dd == 0.0 {
                    None
                } else {
                    Some(TokenData::Numeral(NumeralData::new(nn / dd)))
                }
            }),
        },
        Rule {
            name: "fraction 份".to_string(),
            pattern: vec![regex("([一二三四五六七八九十百千兩两壹貳參肆伍陸柒捌玖0-9]+)份([一二三四五六七八九十百千兩两壹貳參肆伍陸柒捌玖0-9]+)")],
            production: Box::new(|nodes| {
                let (d, n) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let nn = parse_chinese_integer(n).or_else(|| n.parse::<f64>().ok())?;
                let dd = parse_chinese_integer(d).or_else(|| d.parse::<f64>().ok())?;
                if dd == 0.0 {
                    None
                } else {
                    Some(TokenData::Numeral(NumeralData::new(nn / dd)))
                }
            }),
        },
        Rule {
            name: "x and fraction".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("又"), predicate(|td| matches!(td, TokenData::Numeral(d) if d.value < 1.0))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "dot decimal".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("点|點"), predicate(|td| matches!(td, TokenData::Numeral(d) if d.grain.is_none()))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + decimals_to_double(b))))
            }),
        },
        Rule {
            name: "negative".to_string(),
            pattern: vec![regex("-|负|負"), predicate(|td| matches!(td, TokenData::Numeral(d) if d.value >= 0.0))],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
    ]
}
