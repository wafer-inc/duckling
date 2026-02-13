use crate::dimensions::numeral::helpers::numeral_data;
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

fn lookup_zero_to_nineteen(s: &str) -> Option<f64> {
    match s {
        "nula" => Some(0.0),
        "jeden" | "jedna" | "jedno" => Some(1.0),
        "dva" | "dve" => Some(2.0),
        "tri" => Some(3.0),
        "štyri" => Some(4.0),
        "päť" => Some(5.0),
        "šesť" => Some(6.0),
        "sedem" => Some(7.0),
        "osem" => Some(8.0),
        "deväť" => Some(9.0),
        "desať" => Some(10.0),
        "jedenásť" => Some(11.0),
        "dvanásť" => Some(12.0),
        "trinásť" => Some(13.0),
        "štrnásť" => Some(14.0),
        "pätnásť" => Some(15.0),
        "šestnásť" => Some(16.0),
        "sedemnásť" => Some(17.0),
        "osemnásť" => Some(18.0),
        "devätnásť" => Some(19.0),
        _ => None,
    }
}

fn lookup_tens(s: &str) -> Option<f64> {
    match s {
        "dvadsať" => Some(20.0),
        "tridsať" => Some(30.0),
        "štyridsať" => Some(40.0),
        "päťdesiat" => Some(50.0),
        "šesťdesiat" => Some(60.0),
        "sedemdesiat" => Some(70.0),
        "osemdesiat" => Some(80.0),
        "devätdesiat" => Some(90.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "few".to_string(),
            pattern: vec![regex("(zo)?pár")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(3.0)))),
        },
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*,\\d+)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = m.replace(',', ".").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "decimal with thousands separator".to_string(),
            pattern: vec![regex("(\\d+(\\.\\d\\d\\d)+\\,\\d+)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = m.replace('.', "").replace(',', ".").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
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
                let v: f64 = m.replace('.', "").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer (0..19)".to_string(),
            pattern: vec![regex("(nula|jed(enásť|en|na|no)|dv(anásť|a|e)|trinásť|tri|štrnásť|štyri|pätnásť|päť|šestnásť|šesť|sedemnásť|sedem|osemnásť|osem|devätnásť|deväť|desať)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(lookup_zero_to_nineteen(&s)?)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("((dva|tri|štyri)dsať|(päť|šesť|sedem|osem|devät)desiat)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(lookup_tens(&s)?)))
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
                let t = numeral_data(&nodes[0].token_data)?.value;
                let u = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(t + u)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(sto(vky)?|tisíce?|milióny?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let n = match s.as_str() {
                    "sto" | "stovky" => NumeralData::new(1e2).with_grain(2).with_multipliable(true),
                    "tisíc" | "tisíce" => {
                        NumeralData::new(1e3).with_grain(3).with_multipliable(true)
                    }
                    "milión" | "milióny" => {
                        NumeralData::new(1e6).with_grain(6).with_multipliable(true)
                    }
                    _ => return None,
                };
                Some(TokenData::Numeral(n))
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
            name: "intersect (with and)".to_string(),
            pattern: vec![predicate(has_grain), regex("a"), predicate(|td| !is_multipliable(td) && is_positive(td))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[2].token_data)?;
                let g = a.grain? as i32;
                if 10f64.powi(g) > b.value {
                    Some(TokenData::Numeral(NumeralData::new(a.value + b.value)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "number dot number".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex("celá|celých|celé"),
                predicate(|td| !has_grain(td)),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                let mut mul = 1.0;
                while b >= mul {
                    mul *= 10.0;
                }
                Some(TokenData::Numeral(NumeralData::new(a + b / mul)))
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
            pattern: vec![regex("-|mínus|záporné"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
    ]
}
