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

fn is_multipliable(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.multipliable)
}

fn number_between(low: f64, up: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if d.value >= low && d.value < up)
}

fn unit_or_11(s: &str) -> Option<f64> {
    match s {
        "kosong" | "nol" => Some(0.0),
        "satu" => Some(1.0),
        "dua" => Some(2.0),
        "tiga" => Some(3.0),
        "empat" => Some(4.0),
        "lima" => Some(5.0),
        "enam" => Some(6.0),
        "tujuh" => Some(7.0),
        "delapan" => Some(8.0),
        "sembilan" => Some(9.0),
        "sebelas" => Some(11.0),
        _ => None,
    }
}

fn powers(s: &str) -> Option<NumeralData> {
    match s {
        "ratus" => Some(NumeralData::new(1e2).with_grain(2).with_multipliable(true)),
        "ribu" => Some(NumeralData::new(1e3).with_grain(3).with_multipliable(true)),
        "juta" => Some(NumeralData::new(1e6).with_grain(6).with_multipliable(true)),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*,\\d+)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(
                    m.replace(',', ".").parse().ok()?,
                )))
            }),
        },
        Rule {
            name: "decimal with thousands separator".to_string(),
            pattern: vec![regex("(\\d+(\\.\\d\\d\\d)+,\\d+)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let fmt = m.replace('.', "").replace(',', ".");
                Some(TokenData::Numeral(NumeralData::new(fmt.parse().ok()?)))
            }),
        },
        Rule {
            name: "integer with thousands separator .".to_string(),
            pattern: vec![regex("(\\d{1,3}(\\.\\d\\d\\d){1,5})")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(
                    m.replace('.', "").parse().ok()?,
                )))
            }),
        },
        Rule {
            name: "dozen".to_string(),
            pattern: vec![regex("(se)?lusin")],
            production: Box::new(|_| {
                Some(TokenData::Numeral(
                    NumeralData::new(12.0)
                        .with_grain(1)
                        .with_multipliable(true),
                ))
            }),
        },
        Rule {
            name: "integer (0..9 11)".to_string(),
            pattern: vec![regex("(kosong|nol|satu|dua|tiga|empat|lima|enam|tujuh|delapan|sembilan|sebelas)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(unit_or_11(&s)?)))
            }),
        },
        Rule {
            name: "ten".to_string(),
            pattern: vec![regex("(se)?puluh")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(10.0).with_grain(1)))),
        },
        Rule {
            name: "teen".to_string(),
            pattern: vec![predicate(number_between(2.0, 10.0)), regex("belas")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v + 10.0)))
            }),
        },
        Rule {
            name: "integer 20..90".to_string(),
            pattern: vec![predicate(number_between(2.0, 10.0)), predicate(|td| matches!(td, TokenData::Numeral(d) if d.value == 10.0))],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?;
                let v2 = numeral_data(&nodes[1].token_data)?;
                let g = v2.grain?;
                Some(TokenData::Numeral(
                    NumeralData::new(v1.value * v2.value).with_grain(g),
                ))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if [20.0,30.0,40.0,50.0,60.0,70.0,80.0,90.0].contains(&d.value))),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(se)?(ratus|ribu|juta)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let p = m.group(2)?;
                Some(TokenData::Numeral(powers(&p.to_lowercase())?))
            }),
        },
        Rule {
            name: "compose by multiplication".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), predicate(is_multipliable)],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[1].token_data)?;
                if b.grain.is_none() || b.value > a.value {
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
        Rule {
            name: "intersect".to_string(),
            pattern: vec![predicate(has_grain), predicate(|td| !is_multipliable(td) && is_positive(td))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[1].token_data)?;
                let g = a.grain? as i32;
                if 10f64.powi(g) > b.value {
                    Some(TokenData::Numeral(NumeralData::new(a.value + b.value)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "number comma number".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("koma"), predicate(|td| !has_grain(td))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + decimals_to_double(b))))
            }),
        },
        Rule {
            name: "some/few/couple".to_string(),
            pattern: vec![regex("beberapa")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(3.0)))),
        },
        Rule {
            name: "numbers suffixes (K, M, G)".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("([kmgKMG])")],
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
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|minus|negatif"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?;
                Some(TokenData::Numeral(NumeralData::new(-n.value)))
            }),
        },
    ]
}
