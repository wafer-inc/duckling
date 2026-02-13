use crate::dimensions::numeral::helpers::{decimals_to_double, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::NumeralData;

fn thai_digit(tok: &str) -> Option<f64> {
    match tok {
        "ศูนย์" => Some(0.0),
        "หนึ่ง" | "เอ็ด" => Some(1.0),
        "สอง" | "ยี่" => Some(2.0),
        "สาม" => Some(3.0),
        "สี่" => Some(4.0),
        "ห้า" => Some(5.0),
        "หก" => Some(6.0),
        "เจ็ด" => Some(7.0),
        "แปด" => Some(8.0),
        "เก้า" => Some(9.0),
        _ => None,
    }
}

fn tokenize_thai_number(mut s: &str) -> Option<Vec<&str>> {
    let mut out = Vec::new();
    while !s.is_empty() {
        let candidates = [
            "ศูนย์", "หนึ่ง", "เอ็ด", "สอง", "ยี่", "สาม", "สี่", "ห้า", "หก", "เจ็ด", "แปด",
            "เก้า", "สิบ", "ร้อย", "พัน", "หมื่น", "แสน", "ล้าน",
        ];
        let mut matched = None;
        for c in candidates {
            if s.starts_with(c) {
                matched = Some(c);
                break;
            }
        }
        let m = matched?;
        out.push(m);
        s = &s[m.len()..];
    }
    Some(out)
}

fn parse_thai_integer(s: &str) -> Option<f64> {
    let tokens = tokenize_thai_number(s)?;
    let mut total = 0.0;
    let mut section = 0.0;
    let mut number = 0.0;

    for t in tokens {
        if let Some(d) = thai_digit(t) {
            number = d;
            continue;
        }
        let unit = match t {
            "สิบ" => Some(10.0),
            "ร้อย" => Some(100.0),
            "พัน" => Some(1000.0),
            "หมื่น" => Some(10000.0),
            "แสน" => Some(100000.0),
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
        if t == "ล้าน" {
            section += number;
            if section == 0.0 {
                section = 1.0;
            }
            total += section * 1e6;
            section = 0.0;
            number = 0.0;
            continue;
        }
        return None;
    }

    Some(total + section + number)
}

fn power_word(s: &str) -> Option<NumeralData> {
    match s {
        "สิบ" => Some(NumeralData::new(1e1).with_grain(1).with_multipliable(true)),
        "ร้อย" => Some(NumeralData::new(1e2).with_grain(2).with_multipliable(true)),
        "พัน" => Some(NumeralData::new(1e3).with_grain(3).with_multipliable(true)),
        "หมื่น" => Some(NumeralData::new(1e4).with_grain(4).with_multipliable(true)),
        "แสน" => Some(NumeralData::new(1e5).with_grain(5).with_multipliable(true)),
        "ล้าน" => Some(NumeralData::new(1e6).with_grain(6).with_multipliable(true)),
        "โหล" => Some(
            NumeralData::new(12.0)
                .with_grain(1)
                .with_multipliable(true)
                .with_quantifier(),
        ),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "thai none".to_string(),
            pattern: vec![regex("ไม่มี")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(0.0)))),
        },
        Rule {
            name: "thai word number".to_string(),
            pattern: vec![regex("([ศูนย์หนึ่งเอ็ดสองยี่สามสี่ห้าหกเจ็ดแปดเก้าสิบร้อยพันหมื่นแสนล้าน]+)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(parse_thai_integer(s)?)))
            }),
        },
        Rule {
            name: "powers".to_string(),
            pattern: vec![regex("(สิบ|ร้อย|พัน|หมื่น|แสน|ล้าน|โหล)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(power_word(s)?))
            }),
        },
        Rule {
            name: "compose multiply".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), predicate(|td| matches!(td, TokenData::Numeral(d) if d.multipliable))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[1].token_data)?;
                if b.value > a.value || b.value == 12.0 {
                    let mut out = NumeralData::new(a.value * b.value);
                    if let Some(g) = b.grain {
                        out = out.with_grain(g);
                    }
                    if b.quantifier {
                        out = out.with_quantifier();
                    }
                    Some(TokenData::Numeral(out))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "intersect".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.grain.is_some())),
                predicate(|td| matches!(td, TokenData::Numeral(d) if !d.multipliable && d.value >= 0.0)),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[1].token_data)?;
                let g = a.grain? as i32;
                if 10f64.powi(g) > b.value {
                    Some(TokenData::Numeral(NumeralData::new(a.value + b.value)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "dot word decimal".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("จุด"), predicate(|td| matches!(td, TokenData::Numeral(d) if d.grain.is_none()))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + decimals_to_double(b))))
            }),
        },
        Rule {
            name: "dot word decimal compact".to_string(),
            pattern: vec![regex("([ศูนย์หนึ่งเอ็ดสองยี่สามสี่ห้าหกเจ็ดแปดเก้า]+)จุด([ศูนย์หนึ่งเอ็ดสองยี่สามสี่ห้าหกเจ็ดแปดเก้า]+)")],
            production: Box::new(|nodes| {
                let (a, b) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let lhs = parse_thai_integer(a)?;
                let rhs = parse_thai_integer(b)?;
                Some(TokenData::Numeral(NumeralData::new(lhs + decimals_to_double(rhs))))
            }),
        },
        Rule {
            name: "negative".to_string(),
            pattern: vec![regex("-|ลบ"), predicate(|td| matches!(td, TokenData::Numeral(d) if d.value >= 0.0))],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
    ]
}
