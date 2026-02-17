use crate::dimensions::numeral::helpers::{decimals_to_double, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::NumeralData;

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some())
}

fn number_between(low: f64, up: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if d.value >= low && d.value < up)
}

fn units_0_19(s: &str) -> Option<f64> {
    match s {
        "ноль" | "нуля" | "нисколько" => Some(0.0),
        "один" | "одна" | "одну" => Some(1.0),
        "два" | "две" | "двух" | "двое" => Some(2.0),
        "три" => Some(3.0),
        "четыре" => Some(4.0),
        "пять" => Some(5.0),
        "шесть" => Some(6.0),
        "семь" => Some(7.0),
        "восемь" => Some(8.0),
        "девять" => Some(9.0),
        "десять" => Some(10.0),
        "одиннадцать" => Some(11.0),
        "двенадцать" => Some(12.0),
        "тринадцать" => Some(13.0),
        "четырнадцать" => Some(14.0),
        "пятнадцать" => Some(15.0),
        "шестнадцать" => Some(16.0),
        "семнадцать" => Some(17.0),
        "восемнадцать" => Some(18.0),
        "девятнадцать" => Some(19.0),
        // Common genitive forms used by upstream rules.
        "трех" => Some(3.0),
        "четырех" => Some(4.0),
        "пяти" => Some(5.0),
        "шести" => Some(6.0),
        "семи" => Some(7.0),
        "восьми" => Some(8.0),
        "девяти" => Some(9.0),
        "десяти" => Some(10.0),
        "одиннадцати" => Some(11.0),
        "двенадцати" => Some(12.0),
        "тринадцати" => Some(13.0),
        "четырнадцати" => Some(14.0),
        "пятнадцати" => Some(15.0),
        "шестнадцати" => Some(16.0),
        "семнадцати" => Some(17.0),
        "восемнадцати" => Some(18.0),
        "девятнадцати" => Some(19.0),
        _ => None,
    }
}

fn tens(s: &str) -> Option<f64> {
    match s {
        "двадцать" | "двадцати" => Some(20.0),
        "тридцать" | "тридцати" => Some(30.0),
        "сорок" | "сорока" => Some(40.0),
        "пятьдесят" | "пятидесяти" => Some(50.0),
        "шестьдесят" | "шестидесяти" | "шестидесят" => Some(60.0),
        "семьдесят" | "семидесяти" | "семидесят" => Some(70.0),
        "восемьдесят" | "восьмидесяти" | "восьмидесят" | "восемьдесяти" => {
            Some(80.0)
        }
        "девяносто" | "девяноста" => Some(90.0),
        _ => None,
    }
}

fn hundreds(s: &str) -> Option<f64> {
    match s {
        "сто" => Some(100.0),
        "двести" => Some(200.0),
        "триста" => Some(300.0),
        "четыреста" => Some(400.0),
        "пятьсот" => Some(500.0),
        "шестьсот" => Some(600.0),
        "семьсот" => Some(700.0),
        "восемьсот" => Some(800.0),
        "девятьсот" => Some(900.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "integer (0..19)".to_string(),
            pattern: vec![regex("(ноль|нуля|нисколько|один|одна|одну|дв(а|е|ух|ое)|пар(а|у|очк(у|а))|три|четыре|пять|шесть|семь|восемь|девять|десять|одиннадцать|двенадцать|тринадцать|четырнадцать|пятнадцать|шестнадцать|семнадцать|восемнадцать|девятнадцать|трех|четырех|пяти|шести|семи|восьми|девяти|десяти|одиннадцати|двенадцати|тринадцати|четырнадцати|пятнадцати|шестнадцати|семнадцати|восемнадцати|девятнадцати)")],
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
            pattern: vec![regex("(двадцат(ь|и)|тридцат(ь|и)|сорока?|пят(ь|и)десяти?|шест(ь|и)десяти?|сем(ь|и)десяти?|вос(е|ь)м(ь|и)десяти?|девяност(о|а))")],
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
                regex("(сто|двести|триста|четыреста|пятьсот|шестьсот|семьсот|восемьсот|девятьсот)"),
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
            name: "one and a half".to_string(),
            pattern: vec![regex("(полтора|полторы|полутора)")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(1.5)))),
        },
        Rule {
            name: "integer and a half".to_string(),
            pattern: vec![predicate(number_between(1.0, 1000000.0)), regex("с половиной")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v + 0.5)))
            }),
        },
        Rule {
            name: "number dot number".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("точка"), predicate(|td| !has_grain(td))],
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
            pattern: vec![regex("-|минус"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
    ]
}
