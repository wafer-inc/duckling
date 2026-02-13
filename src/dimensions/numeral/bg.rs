use crate::dimensions::numeral::helpers::{decimals_to_double, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::NumeralData;

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some())
}

fn number_between(low: f64, high: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if d.value >= low && d.value < high)
}

fn zero_nineteen(s: &str) -> Option<f64> {
    match s {
        "нула" => Some(0.0),
        "един" | "една" | "едно" => Some(1.0),
        "два" | "две" => Some(2.0),
        "три" => Some(3.0),
        "четири" => Some(4.0),
        "пет" => Some(5.0),
        "шест" => Some(6.0),
        "седем" => Some(7.0),
        "осем" => Some(8.0),
        "девет" => Some(9.0),
        "десет" => Some(10.0),
        "единадесет" | "единайсет" => Some(11.0),
        "дванадесет" | "дванайсет" => Some(12.0),
        "тринадесет" | "тринайсет" => Some(13.0),
        "четиринадесет" | "четиринайсет" => Some(14.0),
        "петнадесет" | "петнайсет" => Some(15.0),
        "шестнадесет" | "шестнайсет" => Some(16.0),
        "седемнадесет" | "седемнайсет" => Some(17.0),
        "осемнадесет" | "осемнайсет" => Some(18.0),
        "деветнадесет" | "деветнайсет" => Some(19.0),
        _ => None,
    }
}

fn tens_word(s: &str) -> Option<f64> {
    match s {
        "двадесет" => Some(20.0),
        "тридесет" => Some(30.0),
        "четирдесет" => Some(40.0),
        "петдесет" => Some(50.0),
        "шестдесет" => Some(60.0),
        "седемдесет" => Some(70.0),
        "осемдесет" => Some(80.0),
        "деветдесет" => Some(90.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "number (0..19)".to_string(),
            pattern: vec![regex("(нула|едина(де|й)сет|двана(де|й)сет|трина(де|й)сет|четирина(де|й)сет|петна(де|й)сет|шестна(де|й)сет|седемна(де|й)сет|осемна(де|й)сет|деветна(де|й)сет|един|една|едно|два|две|три|четири|пет|шест|седем|осем|девет|десет)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(zero_nineteen(&s)?)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("((два|три|четири|пет|шест|седем|осем|девет)десет)")],
            production: Box::new(|nodes| {
                let tens = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(tens_word(&tens)?)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(хиляд(а|и)|милион(а|и)?|милиард(а|и)?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let out = match s.as_str() {
                    "хиляда" | "хиляди" => {
                        NumeralData::new(1e3).with_grain(3).with_multipliable(true)
                    }
                    "милион" | "милиона" | "милиони" => {
                        NumeralData::new(1e6).with_grain(6).with_multipliable(true)
                    }
                    "милиард" | "милиарда" | "милиарди" => {
                        NumeralData::new(1e9).with_grain(9).with_multipliable(true)
                    }
                    _ => return None,
                };
                Some(TokenData::Numeral(out))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![
                predicate(|td| {
                    matches!(td, TokenData::Numeral(n) if [20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0].contains(&n.value))
                }),
                regex("и"),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let tens = numeral_data(&nodes[0].token_data)?.value;
                let units = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(tens + units)))
            }),
        },
        Rule {
            name: "integer (100..900)".to_string(),
            pattern: vec![regex("(сто|двеста|триста|(четири|пет|шест|седем|осем|девет)стотин)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match s.as_str() {
                    "сто" => 100.0,
                    "двеста" => 200.0,
                    "триста" => 300.0,
                    "четиристотин" => 400.0,
                    "петстотин" => 500.0,
                    "шестстотин" => 600.0,
                    "седемстотин" => 700.0,
                    "осемстотин" => 800.0,
                    "деветстотин" => 900.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v).with_grain(2)))
            }),
        },
        Rule {
            name: "integer 101..999".to_string(),
            pattern: vec![
                predicate(|td| {
                    matches!(td, TokenData::Numeral(n) if [200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0].contains(&n.value))
                }),
                predicate(number_between(1.0, 100.0)),
            ],
            production: Box::new(|nodes| {
                let hundreds = numeral_data(&nodes[0].token_data)?.value;
                let rest = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(hundreds + rest)))
            }),
        },
        Rule {
            name: "one point 2".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex("цяло и"),
                predicate(|td| !has_grain(td)),
            ],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?;
                let n2 = numeral_data(&nodes[2].token_data)?;
                Some(TokenData::Numeral(NumeralData::new(
                    n1.value + decimals_to_double(n2.value),
                )))
            }),
        },
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*\\.\\d+)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(s.parse().ok()?)))
            }),
        },
        Rule {
            name: "comma-separated numbers".to_string(),
            pattern: vec![regex("(\\d+(,\\d\\d\\d)+(\\.\\d+)?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = s.replace(',', "").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "suffixes (K,M,G))".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("([кКмМгГ])")],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?;
                let s = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let factor = match s {
                    "к" | "К" => 1e3,
                    "м" | "М" => 1e6,
                    "г" | "Г" => 1e9,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(n.value * factor)))
            }),
        },
        Rule {
            name: "negative numbers".to_string(),
            pattern: vec![regex("-|минус\\s?"), dim(DimensionKind::Numeral)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?;
                Some(TokenData::Numeral(NumeralData::new(-n.value)))
            }),
        },
    ]
}
