use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::helpers::*;
use super::NumeralData;

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some())
}

fn is_multipliable(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.multipliable)
}

fn number_between(low: f64, up: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if d.value >= low && d.value < up)
}

fn lookup_simple_number(s: &str) -> Option<f64> {
    match s {
        "null" => Some(0.0),
        "ein" | "eins" | "eine" | "einem" | "einen" | "einer" | "eines" => Some(1.0),
        "zwei" => Some(2.0),
        "drei" => Some(3.0),
        "vier" => Some(4.0),
        "fünf" | "funf" => Some(5.0),
        "sechs" => Some(6.0),
        "sieben" => Some(7.0),
        "acht" => Some(8.0),
        "neun" => Some(9.0),
        "zehn" => Some(10.0),
        "elf" => Some(11.0),
        "zwölf" | "zwolf" => Some(12.0),
        "dreizehn" => Some(13.0),
        "vierzehn" => Some(14.0),
        "fünfzehn" | "funfzehn" => Some(15.0),
        "sechzehn" => Some(16.0),
        "siebzehn" => Some(17.0),
        "achtzehn" => Some(18.0),
        "neunzehn" => Some(19.0),
        "zwanzig" => Some(20.0),
        "dreissig" | "dreißig" => Some(30.0),
        "vierzig" => Some(40.0),
        "fünfzig" | "funfzig" => Some(50.0),
        "sechzig" => Some(60.0),
        "siebzig" => Some(70.0),
        "achtzig" => Some(80.0),
        "neunzig" => Some(90.0),
        _ => None,
    }
}

fn parse_de_compound_number(s: &str) -> Option<f64> {
    let raw = s.to_lowercase();
    let n = raw
        .replace('ä', "a")
        .replace('ö', "o")
        .replace('ü', "u")
        .replace('ß', "ss");

    if let Some(v) = lookup_simple_number(&n) {
        return Some(v);
    }

    if let Some(stem) = n.strip_suffix("hundert") {
        let lead = if stem.is_empty() {
            1.0
        } else {
            lookup_simple_number(stem)?
        };
        return Some(lead * 100.0);
    }
    if let Some(stem) = n.strip_suffix("tausend") {
        let lead = if stem.is_empty() {
            1.0
        } else {
            lookup_simple_number(stem)?
        };
        return Some(lead * 1000.0);
    }

    // einundzwanzig, zweiunddreißig, ...
    if let Some((u, t)) = n.split_once("und") {
        let units = lookup_simple_number(u)?;
        let tens = lookup_simple_number(t)?;
        if (tens % 10.0 == 0.0) && (20.0..=90.0).contains(&tens) && (1.0..10.0).contains(&units) {
            return Some(units + tens);
        }
    }

    // 92 style words with hundreds/thousands prefix: zweiundneunzigtausend unsupported for now
    None
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|minus|negativ"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
        Rule {
            name: "few".to_string(),
            pattern: vec![regex("mehrere")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(3.0)))),
        },
        Rule {
            name: "couple".to_string(),
            pattern: vec![regex("(ein )?paar")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(2.0)))),
        },
        Rule {
            name: "integer 0".to_string(),
            pattern: vec![regex("(keine(m|n|r|s)?|keins?|null|nichts)")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(0.0)))),
        },
        Rule {
            name: "dozen".to_string(),
            pattern: vec![regex("dutzend")],
            production: Box::new(|_| {
                Some(TokenData::Numeral(
                    NumeralData::new(12.0).with_grain(1).with_multipliable(true),
                ))
            }),
        },
        Rule {
            name: "decimal with thousands separator".to_string(),
            pattern: vec![regex("(\\d+(\\.\\d\\d\\d)+\\,\\d+)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = t.replace('.', "").replace(',', ".").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*,\\d+)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = t.replace(',', ".").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer with thousands separator .".to_string(),
            pattern: vec![regex("(\\d{1,3}(\\.\\d\\d\\d){1,5})")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = t.replace('.', "").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "simple and complex numerals written as one word".to_string(),
            pattern: vec![regex("([\\p{L}]+)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = parse_de_compound_number(s)?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "numbers und".to_string(),
            pattern: vec![
                predicate(number_between(1.0, 10.0)),
                regex("und"),
                predicate(number_between(20.0, 100.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex(
                "(hunderte?|tausende?|million(en)?|milliarde(n)?|billion(en)?|billiarde(n)?)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let out = match s.as_str() {
                    "hundert" | "hunderte" => {
                        NumeralData::new(1e2).with_grain(2).with_multipliable(true)
                    }
                    "tausend" | "tausende" => {
                        NumeralData::new(1e3).with_grain(3).with_multipliable(true)
                    }
                    "million" | "millionen" => {
                        NumeralData::new(1e6).with_grain(6).with_multipliable(true)
                    }
                    "milliarde" | "milliarden" => {
                        NumeralData::new(1e9).with_grain(9).with_multipliable(true)
                    }
                    "billion" | "billionen" => NumeralData::new(1e12)
                        .with_grain(12)
                        .with_multipliable(true),
                    "billiarde" | "billiarden" => NumeralData::new(1e15)
                        .with_grain(15)
                        .with_multipliable(true),
                    _ => return None,
                };
                Some(TokenData::Numeral(out))
            }),
        },
        Rule {
            name: "compose by multiplication".to_string(),
            pattern: vec![predicate(is_positive), predicate(is_multipliable)],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[1].token_data)?;
                match b.grain {
                    Some(g) if b.value > a.value => Some(TokenData::Numeral(
                        NumeralData::new(a.value * b.value).with_grain(g),
                    )),
                    Some(_) => None,
                    None => Some(TokenData::Numeral(NumeralData::new(a.value * b.value))),
                }
            }),
        },
        Rule {
            name: "intersect".to_string(),
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
            name: "numbers suffixes (K, M, G)".to_string(),
            pattern: vec![predicate(is_positive), regex("([kmg])")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let s = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let out = match s.as_str() {
                    "k" => v * 1e3,
                    "m" => v * 1e6,
                    "g" => v * 1e9,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(out)))
            }),
        },
    ]
}
