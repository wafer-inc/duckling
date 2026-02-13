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

fn units_0_19(s: &str) -> Option<f64> {
    match s {
        "zero" | "nic" => Some(0.0),
        "jeden" | "jedna" | "jedno" => Some(1.0),
        "dwa" | "dwie" | "dwom" | "dwóm" => Some(2.0),
        "trzy" => Some(3.0),
        "cztery" | "czterem" => Some(4.0),
        "pięć" => Some(5.0),
        "sześć" => Some(6.0),
        "siedem" => Some(7.0),
        "osiem" => Some(8.0),
        "dziewięć" => Some(9.0),
        "dziesięć" => Some(10.0),
        "jedenaście" => Some(11.0),
        "dwanaście" => Some(12.0),
        "trzynaście" => Some(13.0),
        "czternaście" => Some(14.0),
        "piętnaście" => Some(15.0),
        "szesnaście" => Some(16.0),
        "siedemnaście" => Some(17.0),
        "osiemnaście" => Some(18.0),
        "dziewiętnaście" => Some(19.0),
        "dwadzieścia" => Some(20.0),
        _ => None,
    }
}

fn tens(s: &str) -> Option<f64> {
    match s {
        "dwadzieścia" | "dwudziestu" => Some(20.0),
        "trzydzieści" | "trzydziestu" => Some(30.0),
        "siedemdziesiąt" | "siedemdziesięciu" => Some(70.0),
        "osiemdziesiąt" | "osiemdziesiat" | "osiemdziesięciu" | "osiemdziesieciu" => Some(80.0),
        "dziewięćdziesiąt" | "dziewiecdziesiat" | "dziewięćdziesięciu" | "dziewiecdziesieciu" => {
            Some(90.0)
        }
        _ => None,
    }
}

fn hundreds(s: &str) -> Option<f64> {
    match s {
        "sto" => Some(100.0),
        "dwieście" => Some(200.0),
        "siedemset" => Some(700.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "two".to_string(),
            pattern: vec![regex("para|paru|parę")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(2.0)))),
        },
        Rule {
            name: "single".to_string(),
            pattern: vec![regex("pojedynczy")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(1.0)))),
        },
        Rule {
            name: "fifteenth as numeral".to_string(),
            pattern: vec![regex("piętnasta")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(15.0)))),
        },
        Rule {
            name: "integer (0..19)".to_string(),
            pattern: vec![regex("(zero|nic|jeden|jedna|jedno|dwa|dwie|dwom|dwóm|trzy|cztery|czterem|pięć|sześć|siedem|osiem|dziewięć|dziesięć|jedenaście|dwanaście|trzynaście|czternaście|piętnaście|szesnaście|siedemnaście|osiemnaście|dziewiętnaście)")],
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
            pattern: vec![regex("(dwadzieścia|dwudziestu|trzydzieści|trzydziestu|siedemdziesiąt|siedemdziesięciu|osiemdziesiąt|osiemdziesiat|osiemdziesięciu|osiemdziesieciu|dziewięćdziesiąt|dziewiecdziesiat|dziewięćdziesięciu|dziewiecdziesieciu)")],
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
            pattern: vec![regex("(sto|dwieście|siedemset)")],
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
            name: "powers of tens".to_string(),
            pattern: vec![regex("(tysiąc|tysięcy|milion|milionów)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let n = match s.as_str() {
                    "tysiąc" | "tysięcy" => {
                        NumeralData::new(1e3).with_grain(3).with_multipliable(true)
                    }
                    "milion" | "milionów" => {
                        NumeralData::new(1e6).with_grain(6).with_multipliable(true)
                    }
                    _ => return None,
                };
                Some(TokenData::Numeral(n))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![
                predicate(|td| {
                    matches!(td, TokenData::Numeral(d) if [20.0,30.0,70.0,80.0,90.0].contains(&d.value))
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
            name: "integer 101..999".to_string(),
            pattern: vec![predicate(has_grain), predicate(number_between(1.0, 100.0))],
            production: Box::new(|nodes| {
                let h = numeral_data(&nodes[0].token_data)?;
                let r = numeral_data(&nodes[1].token_data)?;
                let g = h.grain? as i32;
                if 10f64.powi(g) > r.value {
                    Some(TokenData::Numeral(NumeralData::new(h.value + r.value)))
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
                if n2.grain.is_none() || n2.value > n1.value {
                    let mut out = NumeralData::new(n1.value * n2.value);
                    if let Some(g) = n2.grain {
                        out = out.with_grain(g);
                    }
                    Some(TokenData::Numeral(out))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "intersect (with and)".to_string(),
            pattern: vec![
                predicate(has_grain),
                regex("i|a"),
                predicate(|td| !is_multipliable(td) && is_positive(td)),
            ],
            production: Box::new(|nodes| {
                let h = numeral_data(&nodes[0].token_data)?;
                let r = numeral_data(&nodes[2].token_data)?;
                let g = h.grain? as i32;
                if 10f64.powi(g) > r.value {
                    Some(TokenData::Numeral(NumeralData::new(h.value + r.value)))
                } else {
                    None
                }
            }),
        },
    ]
}
