use crate::dimensions::numeral::helpers::{is_multipliable, is_positive, numeral_data};
use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn number_between(low: f64, high: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if d.value >= low && d.value < high)
}

fn one_of(values: &'static [f64]) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if values.contains(&d.value))
}

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some())
}

fn af_basic(s: &str) -> Option<f64> {
    match s {
        "nul" | "geen" | "niks" | "zero" => Some(0.0),
        "een" => Some(1.0),
        "twee" => Some(2.0),
        "drie" => Some(3.0),
        "vier" => Some(4.0),
        "vyf" => Some(5.0),
        "ses" => Some(6.0),
        "sewe" => Some(7.0),
        "agt" | "ag" => Some(8.0),
        "nege" => Some(9.0),
        "tien" => Some(10.0),
        _ => None,
    }
}

fn af_11_19(s: &str) -> Option<f64> {
    match s {
        "elf" => Some(11.0),
        "twaalf" => Some(12.0),
        "dertien" => Some(13.0),
        "veertien" => Some(14.0),
        "vyftien" => Some(15.0),
        "sestien" => Some(16.0),
        "sewentien" => Some(17.0),
        "agtien" => Some(18.0),
        "negentien" | "neentien" => Some(19.0),
        _ => None,
    }
}

fn af_tens_prefix(s: &str) -> Option<f64> {
    match s {
        "twin" => Some(20.0),
        "der" => Some(30.0),
        "veer" => Some(40.0),
        "vyf" => Some(50.0),
        "ses" => Some(60.0),
        "sewen" => Some(70.0),
        "tag" => Some(80.0),
        "negen" | "neen" => Some(90.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "number (0..10)".to_string(),
            pattern: vec![regex(r"(nul|geen|niks|zero|tien|een|twee|drie|vier|vyf|ses|sewe|agt?|nege)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(af_basic(&s)?)))
            }),
        },
        Rule {
            name: "a dozen".to_string(),
            pattern: vec![regex("dosyn")],
            production: Box::new(|_| {
                Some(TokenData::Numeral(
                    NumeralData::new(12.0).with_multipliable(true).with_quantifier(),
                ))
            }),
        },
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex(r"(\d*\,\d+)")],
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
            name: "powers of ten".to_string(),
            pattern: vec![regex("(honderd|duisend)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                match s.as_str() {
                    "honderd" => Some(TokenData::Numeral(
                        NumeralData::new(100.0).with_grain(2).with_multipliable(true),
                    )),
                    "duisend" => Some(TokenData::Numeral(
                        NumeralData::new(1000.0).with_grain(3).with_multipliable(true),
                    )),
                    _ => None,
                }
            }),
        },
        Rule {
            name: "number (11..19)".to_string(),
            pattern: vec![regex(r"(elf|twaalf|dertien|veertien|vyftien|sestien|sewentien|agtien|neg?entien)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(af_11_19(&s)?)))
            }),
        },
        Rule {
            name: "integer (20,30..90)".to_string(),
            pattern: vec![regex(r"(twin|der|veer|vyf|ses|sewen|tag|neg?en)tig")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(af_tens_prefix(&s)?)))
            }),
        },
        Rule {
            name: "integer ([2-9][1-9])".to_string(),
            pattern: vec![predicate(number_between(1.0, 10.0)), regex("en"), predicate(one_of(&[20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0]))],
            production: Box::new(|nodes| {
                let units = numeral_data(&nodes[0].token_data)?.value;
                let tens = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(tens + units)))
            }),
        },
        Rule {
            name: "intersect 2 numbers".to_string(),
            pattern: vec![predicate(|td| has_grain(td) && is_positive(td)), predicate(|td| !is_multipliable(td) && is_positive(td))],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?;
                let n2 = numeral_data(&nodes[1].token_data)?;
                let g = n1.grain?;
                if 10f64.powi(g as i32) > n2.value {
                    Some(TokenData::Numeral(NumeralData::new(n1.value + n2.value)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "compose by multiplication".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(_))), predicate(is_multipliable)],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[1].token_data)?;
                if b.grain.is_none() || (b.grain.is_some() && b.value > a.value) {
                    let mut out = NumeralData::new(a.value * b.value);
                    if let Some(g) = b.grain {
                        out = out.with_grain(g);
                    }
                    Some(TokenData::Numeral(out))
                } else {
                    None
                }
            }),
        },
    ]
}
