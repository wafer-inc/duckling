use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::helpers::*;
use super::NumeralData;

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn is_multipliable(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.multipliable)
}

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some_and(|g| g > 1))
}

fn number_between(low: f64, up: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if !d.multipliable && d.value >= low && d.value < up)
}

fn one_of(values: &'static [f64]) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if values.contains(&d.value))
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "couple".to_string(),
            pattern: vec![regex("(een )?paar")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(2.0)))),
        },
        Rule {
            name: "dozen".to_string(),
            pattern: vec![regex("dozijn")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(12.0).with_grain(1)))),
        },
        Rule {
            name: "gros".to_string(),
            pattern: vec![regex("gros")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(144.0).with_grain(1)))),
        },
        Rule {
            name: "few".to_string(),
            pattern: vec![regex("meerdere")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(3.0)))),
        },
        Rule {
            name: "ten".to_string(),
            pattern: vec![regex("tien")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(10.0).with_grain(1)))),
        },
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
            pattern: vec![regex("(\\d+(\\.\\d\\d\\d)+(,\\d+)?)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let cleaned = text.replace('.', "").replace(',', ".");
                let v: f64 = cleaned.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer (0..19)".to_string(),
            pattern: vec![regex(
                "(geen|nul|niks|een|één|twee|drie|vier|vijftien|vijf|zestien|zes|zeventien|zeven|achttien|acht|negentien|negen|tien|elf|twaalf|dertien|veertien)",
            )],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match t.as_str() {
                    "geen" | "nul" | "niks" => 0.0,
                    "een" | "één" => 1.0,
                    "twee" => 2.0,
                    "drie" => 3.0,
                    "vier" => 4.0,
                    "vijf" => 5.0,
                    "zes" => 6.0,
                    "zeven" => 7.0,
                    "acht" => 8.0,
                    "negen" => 9.0,
                    "tien" => 10.0,
                    "elf" => 11.0,
                    "twaalf" => 12.0,
                    "dertien" => 13.0,
                    "veertien" => 14.0,
                    "vijftien" => 15.0,
                    "zestien" => 16.0,
                    "zeventien" => 17.0,
                    "achttien" => 18.0,
                    "negentien" => 19.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(twintig|dertig|veertig|vijftig|zestig|zeventig|tachtig|negentig)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match t.as_str() {
                    "twintig" => 20.0,
                    "dertig" => 30.0,
                    "veertig" => 40.0,
                    "vijftig" => 50.0,
                    "zestig" => 60.0,
                    "zeventig" => 70.0,
                    "tachtig" => 80.0,
                    "negentig" => 90.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer ([2-9][1-9])".to_string(),
            pattern: vec![regex(
                "(een|twee|drie|vier|vijf|zes|zeven|acht|negen)(?:e|ë)n(twintig|dertig|veertig|vijftig|zestig|zeventig|tachtig|negentig)",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let u = m.group(1)?.to_lowercase();
                let d = m.group(2)?.to_lowercase();
                let uv = match u.as_str() {
                    "een" => 1.0,
                    "twee" => 2.0,
                    "drie" => 3.0,
                    "vier" => 4.0,
                    "vijf" => 5.0,
                    "zes" => 6.0,
                    "zeven" => 7.0,
                    "acht" => 8.0,
                    "negen" => 9.0,
                    _ => return None,
                };
                let dv = match d.as_str() {
                    "twintig" => 20.0,
                    "dertig" => 30.0,
                    "veertig" => 40.0,
                    "vijftig" => 50.0,
                    "zestig" => 60.0,
                    "zeventig" => 70.0,
                    "tachtig" => 80.0,
                    "negentig" => 90.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(uv + dv)))
            }),
        },
        Rule {
            name: "numbers en".to_string(),
            pattern: vec![
                predicate(number_between(1.0, 10.0)),
                regex("-?en-?"),
                predicate(one_of(&[20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0])),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(honderd|duizend|miljoen)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let d = match t.as_str() {
                    "honderd" => NumeralData::new(1e2).with_grain(2).with_multipliable(true),
                    "duizend" => NumeralData::new(1e3).with_grain(3).with_multipliable(true),
                    "miljoen" => NumeralData::new(1e6).with_grain(6).with_multipliable(true),
                    _ => return None,
                };
                Some(TokenData::Numeral(d))
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
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|min|minus|negatief"), predicate(is_positive)],
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
    ]
}
