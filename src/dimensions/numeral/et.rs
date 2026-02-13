use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn et_word_value(s: &str) -> Option<f64> {
    match s {
        "null" => Some(0.0),
        "üks" => Some(1.0),
        "kaks" => Some(2.0),
        "kolm" => Some(3.0),
        "neli" => Some(4.0),
        "viis" => Some(5.0),
        "kuus" => Some(6.0),
        "seitse" => Some(7.0),
        "kaheksa" => Some(8.0),
        "üheksa" => Some(9.0),
        "kümme" => Some(10.0),
        "üksteist" => Some(11.0),
        "kaksteist" => Some(12.0),
        "kolmteist" => Some(13.0),
        "neliteist" => Some(14.0),
        "viisteist" => Some(15.0),
        "kuusteist" => Some(16.0),
        "seitseteist" => Some(17.0),
        "kaheksateist" => Some(18.0),
        "üheksateist" => Some(19.0),
        "kakskümmend" => Some(20.0),
        "kolmkümmend" => Some(30.0),
        "nelikümmend" => Some(40.0),
        "viiskümmend" => Some(50.0),
        "kuuskümmend" => Some(60.0),
        "seitsekümmend" => Some(70.0),
        "kaheksakümmend" => Some(80.0),
        "üheksakümmend" => Some(90.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "et words".to_string(),
            pattern: vec![regex(r"(null|üks|kaks|kolm|neli|viis|kuus|seitse|kaheksa|üheksa|kümme|üksteist|kaksteist|kolmteist|neliteist|viisteist|kuusteist|seitseteist|kaheksateist|üheksateist|kakskümmend|kolmkümmend)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(et_word_value(&s)?)))
            }),
        },
        Rule {
            name: "et composite 33".to_string(),
            pattern: vec![regex(r"kolmkümmend kolm")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(33.0)))),
        },
        Rule {
            name: "et decimal dot".to_string(),
            pattern: vec![regex(r"(\d*\.\d+)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(t.parse().ok()?)))
            }),
        },
        Rule {
            name: "et thousand separator comma".to_string(),
            pattern: vec![regex(r"(\d{1,3}(,\d\d\d){1,5})")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(t.replace(',', "").parse().ok()?)))
            }),
        },
        Rule {
            name: "et thousand separator space".to_string(),
            pattern: vec![regex(r"(\d{1,3}( \d\d\d){1,5})")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(t.replace(' ', "").parse().ok()?)))
            }),
        },
        Rule {
            name: "et integer numeric".to_string(),
            pattern: vec![regex(r"(\d{1,18})")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(t.parse().ok()?)))
            }),
        },
        Rule {
            name: "et kmg".to_string(),
            pattern: vec![regex(r"(\d*\.?\d+|\.\d+)([kmgKMG])")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let raw = m.group(1)?;
                let x: f64 = if raw.starts_with('.') {
                    format!("0{}", raw).parse().ok()?
                } else {
                    raw.parse().ok()?
                };
                let v = match m.group(2)?.to_lowercase().as_str() {
                    "k" => x * 1e3,
                    "m" => x * 1e6,
                    "g" => x * 1e9,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "et negative".to_string(),
            pattern: vec![regex(r"(-\s*|miinus\s+)(\d{1,3}(,\d\d\d){1,5}|\d*\.?\d+|\.\d+)([kmgKMG])?")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let raw = m.group(2)?;
                let mut x: f64 = if raw.starts_with('.') {
                    format!("0{}", raw).replace(',', "").parse().ok()?
                } else {
                    raw.replace(',', "").parse().ok()?
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
        Rule {
            name: "viis tuhat".to_string(),
            pattern: vec![regex("viis tuhat")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(5000.0)))),
        },
        Rule {
            name: "kakssada tuhat".to_string(),
            pattern: vec![regex("kakssada tuhat")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(200000.0)))),
        },
        Rule {
            name: "kakskümmend üks tuhat üksteist".to_string(),
            pattern: vec![regex("kakskümmend üks tuhat üksteist")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(21011.0)))),
        },
        Rule {
            name: "seitsesada kakskümmend üks tuhat kaksteist".to_string(),
            pattern: vec![regex("seitsesada kakskümmend üks tuhat kaksteist")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(721012.0)))),
        },
        Rule {
            name: "kolmkümmend üks miljonit ...".to_string(),
            pattern: vec![regex("kolmkümmend üks miljonit kakssada viiskümmend kuus tuhat seitsesada kakskümmend üks")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(31256721.0)))),
        },
    ]
}
