use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::helpers::*;
use super::NumeralData;

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn has_no_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_none())
}

fn number_between(low: f64, up: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if d.value >= low && d.value < up)
}

fn one_of(values: &'static [f64]) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if values.contains(&d.value))
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "integer (0-9) with two digits".to_string(),
            pattern: vec![regex("zero|0"), predicate(number_between(1.0, 10.0))],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number (0..15)".to_string(),
            pattern: vec![regex(
                "(zero|u(na|n)?|d(o|ue)s|tres|quatre|cinc|sis|set|vuit|nou|deu|onze|dotze|tretze|catorze|quinze)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match s.as_str() {
                    "zero" => 0.0,
                    "u" | "un" | "una" => 1.0,
                    "dos" | "dues" => 2.0,
                    "tres" => 3.0,
                    "quatre" => 4.0,
                    "cinc" => 5.0,
                    "sis" => 6.0,
                    "set" => 7.0,
                    "vuit" => 8.0,
                    "nou" => 9.0,
                    "deu" => 10.0,
                    "onze" => 11.0,
                    "dotze" => 12.0,
                    "tretze" => 13.0,
                    "catorze" => 14.0,
                    "quinze" => 15.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number (20..90)".to_string(),
            pattern: vec![regex("(vint|(tre|quara|cinqua|seixa|seta|vuita|nora)nta)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match s.as_str() {
                    "vint" => 20.0,
                    "trenta" => 30.0,
                    "quaranta" => 40.0,
                    "cinquanta" => 50.0,
                    "seixanta" => 60.0,
                    "setanta" => 70.0,
                    "vuitanta" => 80.0,
                    "noranta" => 90.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number (21..29)".to_string(),
            pattern: vec![
                predicate(one_of(&[20.0])),
                regex("(-i-| i )"),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "number (16..19 21..29)".to_string(),
            pattern: vec![regex(
                "(setze|d(i|e|è)sset|d(e|i)(v|h)uit|d(i|e|è)nou|vint-i-u(na)?|vint-i-dos|vint-i-tres|vint-i-quatre|vint-i-cinc|vint-i-sis|vint-i-set|vint-i-vuit|vint-i-nou)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match s.as_str() {
                    "setze" => 16.0,
                    "disset" | "dèsset" | "desset" => 17.0,
                    "devuit" | "divuit" | "dihuit" => 18.0,
                    "dinou" | "dènou" | "denou" => 19.0,
                    "vint-i-u" | "vint-i-una" => 21.0,
                    "vint-i-dos" => 22.0,
                    "vint-i-tres" => 23.0,
                    "vint-i-quatre" => 24.0,
                    "vint-i-cinc" => 25.0,
                    "vint-i-sis" => 26.0,
                    "vint-i-set" => 27.0,
                    "vint-i-vuit" => 28.0,
                    "vint-i-nou" => 29.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number (31..39 41..49 51..59 61..69 71..79 81..89 91..99)".to_string(),
            pattern: vec![
                predicate(one_of(&[30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0])),
                regex("-"),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "number 100..1000".to_string(),
            pattern: vec![regex(
                "(cent(s)?|dos-cents|tres-cents|quatre-cents|cinc-cents|sis-cents|set-cents|vuit-cents|nou-cents|mil)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match s.as_str() {
                    "cent" | "cents" => 100.0,
                    "dos-cents" => 200.0,
                    "tres-cents" => 300.0,
                    "quatre-cents" => 400.0,
                    "cinc-cents" => 500.0,
                    "sis-cents" => 600.0,
                    "set-cents" => 700.0,
                    "vuit-cents" => 800.0,
                    "nou-cents" => 900.0,
                    "mil" => 1000.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "numbers 200..999".to_string(),
            pattern: vec![
                predicate(number_between(2.0, 10.0)),
                regex("-"),
                predicate(one_of(&[100.0])),
                predicate(number_between(0.0, 100.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v4 = numeral_data(&nodes[3].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(100.0 * v1 + v4)))
            }),
        },
        Rule {
            name: "number dot number".to_string(),
            pattern: vec![predicate(is_positive), regex("coma"), predicate(has_no_grain)],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + decimals_to_double(v2))))
            }),
        },
        Rule {
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|menys"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
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
        Rule {
            name: "decimal number ,".to_string(),
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
            name: "decimal with thousands separator .".to_string(),
            pattern: vec![regex("(\\d+(\\.\\d\\d\\d)+,\\d+)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let fmt = t.replace('.', "").replace(',', ".");
                let v: f64 = fmt.parse().ok()?;
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
    ]
}
