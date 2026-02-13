use crate::dimensions::numeral::helpers::{decimals_to_double, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::NumeralData;

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn number_between(low: f64, up: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if d.value >= low && d.value < up)
}

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some())
}

fn units_0_19(s: &str) -> Option<f64> {
    match s {
        "нуль" => Some(0.0),
        "один" | "одна" | "одну" | "одне" | "одного" => Some(1.0),
        "два" | "дві" | "двоє" | "пара" | "пару" | "парочку" | "парочка" => Some(2.0),
        "три" => Some(3.0),
        "чотири" => Some(4.0),
        "п‘ять" => Some(5.0),
        "шість" => Some(6.0),
        "сім" => Some(7.0),
        "вісім" => Some(8.0),
        "дев‘ять" => Some(9.0),
        "десять" => Some(10.0),
        "одинадцять" => Some(11.0),
        "дванадцять" => Some(12.0),
        "тринадцять" => Some(13.0),
        "чотирнадцять" => Some(14.0),
        "п‘ятнадцять" => Some(15.0),
        "шістнадцять" => Some(16.0),
        "сімнадцять" => Some(17.0),
        "вісімнадцять" => Some(18.0),
        "дев‘ятнадцять" => Some(19.0),
        _ => None,
    }
}

fn tens(s: &str) -> Option<f64> {
    match s {
        "двадцять" => Some(20.0),
        "тридцять" => Some(30.0),
        "сорок" => Some(40.0),
        "п‘ятдесят" => Some(50.0),
        "шістдесят" => Some(60.0),
        "сімдесят" => Some(70.0),
        "вісімдесят" => Some(80.0),
        "дев‘яносто" => Some(90.0),
        _ => None,
    }
}

fn hundreds(s: &str) -> Option<f64> {
    match s {
        "сто" => Some(100.0),
        "двісті" => Some(200.0),
        "триста" => Some(300.0),
        "чотириста" => Some(400.0),
        "п‘ятсот" => Some(500.0),
        "шістсот" => Some(600.0),
        "сімсот" => Some(700.0),
        "вісімсот" => Some(800.0),
        "дев‘ятсот" => Some(900.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "integer (0..19)".to_string(),
            pattern: vec![regex("(нуль|один|одна|одну|одне|одного|два|дві|двоє|пара|пару|парочку|парочка|три|чотири|п‘ять|шість|сім|вісім|дев‘ять|десять|одинадцять|дванадцять|тринадцять|чотирнадцять|п‘ятнадцять|шістнадцять|сімнадцять|вісімнадцять|дев‘ятнадцять)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(units_0_19(&s)?)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(двадцять|тридцять|сорок|п‘ятдесят|шістдесят|сімдесят|вісімдесят|дев‘яносто)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(tens(&s)?)))
            }),
        },
        Rule {
            name: "integer (100..900)".to_string(),
            pattern: vec![
                regex("(сто|двісті|триста|чотириста|п‘ятсот|шістсот|сімсот|вісімсот|дев‘ятсот)"),
            ],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(
                    NumeralData::new(hundreds(&s)?).with_grain(2),
                ))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![
                predicate(|td| {
                    matches!(td, TokenData::Numeral(d) if [20.0,30.0,40.0,50.0,60.0,70.0,80.0,90.0].contains(&d.value))
                }),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "integer 101..999".to_string(),
            pattern: vec![
                predicate(|td| {
                    matches!(td, TokenData::Numeral(d) if [100.0,200.0,300.0,400.0,500.0,600.0,700.0,800.0,900.0].contains(&d.value))
                }),
                predicate(number_between(1.0, 100.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "number dot number".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("крапка"), predicate(|td| !has_grain(td))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + decimals_to_double(b))))
            }),
        },
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*\\.\\d+)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(m.parse::<f64>().ok()?)))
            }),
        },
        Rule {
            name: "decimal with thousands separator".to_string(),
            pattern: vec![regex("(\\d+(,\\d\\d\\d)+(\\.\\d+)?)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = m.replace(',', "").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "numbers suffixes (K, M, G)".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("([кКмМгГ])")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let s = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let out = match s {
                    "к" | "К" => v * 1e3,
                    "м" | "М" => v * 1e6,
                    "г" | "Г" => v * 1e9,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(out)))
            }),
        },
        Rule {
            name: "numbers prefix with -, minus".to_string(),
            pattern: vec![regex("-|мінус\\s?"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
    ]
}
