use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::helpers::*;
use super::NumeralData;

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some_and(|g| g > 1))
}

fn is_multipliable(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.multipliable)
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
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*,\\d+)")],
            production: Box::new(|nodes| {
                let txt = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = txt.replace(',', ".").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "decimal with thousands separator".to_string(),
            pattern: vec![regex("(\\d+(\\.\\d\\d\\d)+,\\d+)")],
            production: Box::new(|nodes| {
                let txt = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = txt.replace('.', "").replace(',', ".").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer cu separator de mii dot".to_string(),
            pattern: vec![regex("(\\d{1,3}(\\.\\d\\d\\d){1,5})")],
            production: Box::new(|nodes| {
                let txt = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = txt.replace('.', "").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer (0..10)".to_string(),
            pattern: vec![regex(
                "(zero|nimic|nici(\\s?o|\\sun(a|ul?))|una|unul?|doi|dou(a|ă)|trei|patru|cinci|(s|ș)ase|(s|ș)apte|opt|nou(a|ă)|zec[ei]|(i|î)nt(a|â)i|un|o)",
            )],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match t.as_str() {
                    "zero" | "nimic" | "nicio" | "nici o" | "nici una" | "nici unu"
                    | "nici unul" => 0.0,
                    "un" | "una" | "unu" | "unul" | "o" | "intai" | "întai" | "intâi"
                    | "întâi" => 1.0,
                    "doi" | "doua" | "două" => 2.0,
                    "trei" => 3.0,
                    "patru" => 4.0,
                    "cinci" => 5.0,
                    "sase" | "șase" => 6.0,
                    "sapte" | "șapte" => 7.0,
                    "opt" => 8.0,
                    "noua" | "nouă" => 9.0,
                    "zece" | "zeci" => 10.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer (11..19)".to_string(),
            pattern: vec![regex(
                "((cin|sapti|opti)(s|ș)pe|(cinci|(s|ș)apte|opt)sprezece|(un|doi|trei|pai|(s|ș)ai|nou(a|ă))((s|ș)pe|sprezece))",
            )],
            production: Box::new(|nodes| {
                let txt = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let normalized = txt
                    .replace('ș', "s")
                    .replace('ă', "a")
                    .replace('â', "a")
                    .replace('î', "i");
                let v = match normalized.as_str() {
                    s if s.starts_with("un") => 11.0,
                    s if s.starts_with("doi") => 12.0,
                    s if s.starts_with("trei") => 13.0,
                    s if s.starts_with("pai") => 14.0,
                    s if s.starts_with("cin") => 15.0,
                    s if s.starts_with("sai") => 16.0,
                    s if s.starts_with("sapti") || s.starts_with("sapte") => 17.0,
                    s if s.starts_with("opti") || s.starts_with("opt") => 18.0,
                    s if s.starts_with("noua") => 19.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(dou[aă]|trei|patru|cinci|[sș]ai|[sș]apte|opt|nou[aă])\\s?zeci")],
            production: Box::new(|nodes| {
                let txt = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let unit = match txt.as_str() {
                    "doua" | "două" => 2.0,
                    "trei" => 3.0,
                    "patru" => 4.0,
                    "cinci" => 5.0,
                    "sai" | "șai" => 6.0,
                    "sapte" | "șapte" => 7.0,
                    "opt" => 8.0,
                    "noua" | "nouă" => 9.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(
                    NumeralData::new(unit * 10.0).with_grain(2).with_multipliable(true),
                ))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![
                predicate(one_of(&[20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0])),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(sut(a|e|ă)?|milio(n|ane)?|miliar(de?)?|mi[ei]?)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let d = match t.as_str() {
                    "suta" | "sute" | "sută" => {
                        NumeralData::new(1e2).with_grain(2).with_multipliable(true)
                    }
                    "mi" | "mie" | "mii" => NumeralData::new(1e3).with_grain(3).with_multipliable(true),
                    "milio" | "milion" | "milioane" => {
                        NumeralData::new(1e6).with_grain(6).with_multipliable(true)
                    }
                    "miliar" | "miliard" | "miliarde" => {
                        NumeralData::new(1e9).with_grain(9).with_multipliable(true)
                    }
                    _ => return None,
                };
                Some(TokenData::Numeral(d))
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
            name: "intersect (cu și)".to_string(),
            pattern: vec![
                predicate(has_grain),
                regex("[sș]i"),
                predicate(|td| !is_multipliable(td) && is_positive(td)),
            ],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?;
                let n2 = numeral_data(&nodes[2].token_data)?;
                let g = n1.grain? as i32;
                if 10f64.powi(g) > n2.value {
                    Some(TokenData::Numeral(NumeralData::new(n1.value + n2.value)))
                } else {
                    None
                }
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
            name: "compose by multiplication".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value >= 20.0)),
                regex("de"),
                predicate(is_multipliable),
            ],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?;
                let n2 = numeral_data(&nodes[2].token_data)?;
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
            name: "numbers prefix with - or minus".to_string(),
            pattern: vec![regex("-|minus"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
        Rule {
            name: "numbers suffixes with (negativ)".to_string(),
            pattern: vec![predicate(is_positive), regex("neg(ativ)?")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
    ]
}
