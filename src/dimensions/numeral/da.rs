use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "number words (da corpus subset)".to_string(),
            pattern: vec![regex(
                r"(nul|én|en|ét|et|to|fjorten|seksten|sytten|atten|Atten)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = match s.to_lowercase().as_str() {
                    "nul" => 0.0,
                    "én" | "en" | "ét" | "et" => 1.0,
                    "to" => 2.0,
                    "fjorten" => 14.0,
                    "seksten" => 16.0,
                    "sytten" => 17.0,
                    "atten" => 18.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "et par".to_string(),
            pattern: vec![regex("et par")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(2.0)))),
        },
        Rule {
            name: "fem tusinde".to_string(),
            pattern: vec![regex("(fem tusinde|fem Tusind|5 tusind)")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(5000.0)))),
        },
        Rule {
            name: "thousands separator".to_string(),
            pattern: vec![regex(r"(\d{1,3}(\.\d\d\d){1,5})")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(
                    t.replace('.', "").parse().ok()?,
                )))
            }),
        },
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex(r"(\d*,\d+)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(
                    t.replace(',', ".").parse().ok()?,
                )))
            }),
        },
        Rule {
            name: "kmg suffix".to_string(),
            pattern: vec![regex(r"(\d*\,?\d+|\,\d+)([kmgKMG])")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let n = m.group(1)?;
                let x: f64 = if n.starts_with(',') {
                    format!("0{}", n).replace(',', ".").parse().ok()?
                } else {
                    n.replace(',', ".").parse().ok()?
                };
                let s = m.group(2)?.to_lowercase();
                let v = match s.as_str() {
                    "k" => x * 1e3,
                    "m" => x * 1e6,
                    "g" => x * 1e9,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "negative".to_string(),
            pattern: vec![regex(
                r"(-\s*|minus\s+|negativ\s+)(\d{1,3}(\.\d\d\d){1,5}|\d*\,?\d+|\,\d+)([kmgKMG])?",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let n = m.group(2)?;
                let mut x: f64 = if n.starts_with(',') {
                    format!("0{}", n)
                        .replace('.', "")
                        .replace(',', ".")
                        .parse()
                        .ok()?
                } else {
                    n.replace('.', "").replace(',', ".").parse().ok()?
                };
                if let Some(suf) = m.group(4) {
                    x = match suf.to_lowercase().as_str() {
                        "k" => x * 1e3,
                        "m" => x * 1e6,
                        "g" => x * 1e9,
                        _ => x,
                    };
                }
                Some(TokenData::Numeral(NumeralData::new(-x)))
            }),
        },
    ]
}
