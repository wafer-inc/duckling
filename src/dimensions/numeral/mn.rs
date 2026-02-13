use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

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

fn unit_val(s: &str) -> Option<f64> {
    match s {
        "нуль" | "тэг" | "нойл" => Some(0.0),
        "нэг" | "ганц" => Some(1.0),
        "хоёр" => Some(2.0),
        "гурав" => Some(3.0),
        "дөрөв" => Some(4.0),
        "тав" => Some(5.0),
        "зургаа" => Some(6.0),
        "долоо" => Some(7.0),
        "найм" => Some(8.0),
        "ес" => Some(9.0),
        _ => None,
    }
}

fn ten_to_nineteen(s: &str) -> Option<f64> {
    match s {
        "арван нэг" => Some(11.0),
        "арван хоёр" => Some(12.0),
        "арван гурав" => Some(13.0),
        "арван дөрөв" => Some(14.0),
        "арван тав" => Some(15.0),
        "арван зургаа" => Some(16.0),
        "арван долоо" => Some(17.0),
        "арван найм" => Some(18.0),
        "арван ес" => Some(19.0),
        _ => None,
    }
}

fn tens_val(s: &str) -> Option<f64> {
    match s {
        "хорь" | "хорин" => Some(20.0),
        "гуч" | "гучин" => Some(30.0),
        "дөч" | "дөчин" => Some(40.0),
        "тавь" | "тавин" => Some(50.0),
        "жар" | "жаран" => Some(60.0),
        "дал" | "далан" => Some(70.0),
        "ная" | "наян" => Some(80.0),
        "ер" | "ерэн" => Some(90.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|хасах|сөрөг"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
        Rule {
            name: "few".to_string(),
            pattern: vec![regex("хэдхэн")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(3.0)))),
        },
        Rule {
            name: "ten".to_string(),
            pattern: vec![regex("арав")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(10.0).with_grain(1)))),
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
            name: "integer ([2-9][1-9])".to_string(),
            pattern: vec![regex("(хорин|гучин|дөчин|тавин|жаран|далан|наян|ерэн) ?(нэг|хоёр|гурав|дөрөв|тав|зургаа|долоо|найм|ес)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let t = tens_val(m.group(1)?)?;
                let u = unit_val(m.group(2)?)?;
                Some(TokenData::Numeral(NumeralData::new(t + u)))
            }),
        },
        Rule {
            name: "numbers und".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(n) if [20.0,30.0,40.0,50.0,60.0,70.0,80.0,90.0].contains(&n.value))),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
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
        Rule {
            name: "intersect".to_string(),
            pattern: vec![predicate(has_grain), predicate(|td| !is_multipliable(td) && is_positive(td))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[1].token_data)?;
                let g = a.grain?;
                if 10f64.powi(g as i32) > b.value {
                    Some(TokenData::Numeral(NumeralData::new(a.value + b.value)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "numbers suffixes (K, M, G)".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(_))), regex("([kmg])")],
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
            name: "couple".to_string(),
            pattern: vec![regex("хос")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(2.0)))),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(зуу?|мянга?|сая?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let out = match s.as_str() {
                    "зуу" | "зуун" => NumeralData::new(1e2).with_grain(2).with_multipliable(true),
                    "мянга" | "мянган" => NumeralData::new(1e3).with_grain(3).with_multipliable(true),
                    "сая" | "саяын" => NumeralData::new(1e6).with_grain(6).with_multipliable(true),
                    _ => return None,
                };
                Some(TokenData::Numeral(out))
            }),
        },
        Rule {
            name: "integer (0..19)".to_string(),
            pattern: vec![regex("(нуль|тэг|нойл|нэг|ганц|хоёр|гурав|дөрөв|тав|зургаа|долоо|найм|ес|арван нэг|арван хоёр|арван гурав|арван дөрөв|арван тав|арван зургаа|арван долоо|арван найм|арван ес|арав|арван)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let sl = s.to_lowercase();
                let v = if let Some(v) = unit_val(&sl) {
                    v
                } else if sl == "арав" || sl == "арван" {
                    10.0
                } else {
                    ten_to_nineteen(&sl)?
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(хорь|гуч|дөч|тавь|жар|дал|ная|ер)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(tens_val(&s.to_lowercase())?)))
            }),
        },
        Rule {
            name: "number dot number".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(_))),
                regex("цэг"),
                predicate(|td| matches!(td, TokenData::Numeral(n) if n.grain.is_none())),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                let mut m = 1.0;
                while v2 >= m {
                    m *= 10.0;
                }
                Some(TokenData::Numeral(NumeralData::new(v1 + (v2 / m))))
            }),
        },
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*\\.\\d+)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(t.parse().ok()?)))
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
                Some(TokenData::Numeral(NumeralData::new(t.replace('.', "").parse().ok()?)))
            }),
        },
    ]
}
