use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::helpers::*;
use super::NumeralData;

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some_and(|g| g > 1))
}

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn is_multipliable(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.multipliable)
}

fn number_between(low: f64, up: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if !d.multipliable && d.value >= low && d.value < up)
}

fn one_of(values: &'static [f64]) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if values.contains(&d.value))
}

fn with_grain_multipliable(value: f64, grain: u8) -> NumeralData {
    NumeralData::new(value).with_grain(grain).with_multipliable(true)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*,\\d+)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = text.replace(',', ".").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "decimal with thousands separator".to_string(),
            pattern: vec![regex("(\\d+(([\\. ])\\d\\d\\d)+,\\d+)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let text = m.group(1)?;
                let sep = m.group(3)?;
                let cleaned = text.replace(sep, "").replace(',', ".");
                let v: f64 = cleaned.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer with thousands separator .".to_string(),
            pattern: vec![regex("(\\d{1,3}(([\\. ])\\d\\d\\d){1,5})")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let text = m.group(1)?;
                let sep = m.group(3)?;
                let cleaned = text.replace(sep, "");
                let v: f64 = cleaned.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number (0..16)".to_string(),
            pattern: vec![regex(
                "(z(e|é)ro|une?|deux|trois|quatre|cinq|six|sept|huit|neuf|dix|onze|douze|treize|quatorze|quinze|seize)",
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = match text.to_lowercase().as_str() {
                    "zero" | "zéro" => 0.0,
                    "un" | "une" => 1.0,
                    "deux" => 2.0,
                    "trois" => 3.0,
                    "quatre" => 4.0,
                    "cinq" => 5.0,
                    "six" => 6.0,
                    "sept" => 7.0,
                    "huit" => 8.0,
                    "neuf" => 9.0,
                    "dix" => 10.0,
                    "onze" => 11.0,
                    "douze" => 12.0,
                    "treize" => 13.0,
                    "quatorze" => 14.0,
                    "quinze" => 15.0,
                    "seize" => 16.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number (20..60)".to_string(),
            pattern: vec![regex("(vingt|trente|quarante|cinquante|soixante)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = match text.to_lowercase().as_str() {
                    "vingt" => 20.0,
                    "trente" => 30.0,
                    "quarante" => 40.0,
                    "cinquante" => 50.0,
                    "soixante" => 60.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number (17..19)".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value == 10.0)),
                regex("[\\s\\-]+"),
                predicate(number_between(7.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(10.0 + v)))
            }),
        },
        Rule {
            name: "number 80".to_string(),
            pattern: vec![regex("quatre"), regex("vingts?")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(80.0)))),
        },
        Rule {
            name: "numbers 21 31 41 51".to_string(),
            pattern: vec![
                predicate(one_of(&[20.0, 30.0, 40.0, 50.0])),
                regex("-?et-?"),
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value == 1.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "numbers 22..29 32..39 .. 52..59".to_string(),
            pattern: vec![
                predicate(one_of(&[20.0, 30.0, 40.0, 50.0])),
                regex("[\\s\\-]+"),
                predicate(number_between(2.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "numbers 61 71".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value == 60.0)),
                regex("-?et-?"),
                predicate(one_of(&[1.0, 11.0])),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "numbers 81".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value == 80.0)),
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value == 1.0)),
            ],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(81.0)))),
        },
        Rule {
            name: "numbers 62..69 .. 92..99".to_string(),
            pattern: vec![
                predicate(one_of(&[60.0, 80.0])),
                regex("[\\s\\-]+"),
                predicate(number_between(2.0, 20.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|moins"), dim(DimensionKind::Numeral)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
        Rule {
            name: "numbers suffixes (K, M, G)".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("([kmg])")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let s = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let mult = match s.as_str() {
                    "k" => 1_000.0,
                    "m" => 1_000_000.0,
                    "g" => 1_000_000_000.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v * mult)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(cent|mille|millions?|milliards?)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let data = match text.as_str() {
                    "cent" => with_grain_multipliable(1e2, 2),
                    "mille" => with_grain_multipliable(1e3, 3),
                    "million" | "millions" => with_grain_multipliable(1e6, 6),
                    "milliard" | "milliards" => with_grain_multipliable(1e9, 9),
                    _ => return None,
                };
                Some(TokenData::Numeral(data))
            }),
        },
        Rule {
            name: "intersect 2 numbers".to_string(),
            pattern: vec![
                predicate(has_grain),
                predicate(|td| !is_multipliable(td) && is_positive(td)),
            ],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?;
                let n2 = numeral_data(&nodes[1].token_data)?;
                let g = n1.grain? as i32;
                if 10f64.powi(g) > n2.value {
                    Some(TokenData::Numeral(NumeralData::new(n1.value + n2.value)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "compose by multiplication".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), predicate(is_multipliable)],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?;
                let n2 = numeral_data(&nodes[1].token_data)?;
                match n2.grain {
                    None => Some(TokenData::Numeral(NumeralData::new(n1.value * n2.value))),
                    Some(g) if n2.value > n1.value => Some(TokenData::Numeral(
                        NumeralData::new(n1.value * n2.value).with_grain(g),
                    )),
                    _ => None,
                }
            }),
        },
    ]
}
