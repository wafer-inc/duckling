use crate::dimensions::numeral::helpers::{decimals_to_double, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::NumeralData;

fn one_two_digits(s: &str) -> Option<f64> {
    match s {
        "μηδέν" => Some(0.0),
        "ένα" | "ένας" | "ενός" | "μία" | "μια" => Some(1.0),
        "δύο" | "δυο" => Some(2.0),
        "τρία" | "τρεις" => Some(3.0),
        "τέσσερα" | "τέσσερις" => Some(4.0),
        "πέντε" => Some(5.0),
        "έξι" => Some(6.0),
        "επτά" | "εφτά" => Some(7.0),
        "οκτώ" | "οχτώ" => Some(8.0),
        "εννιά" | "εννέα" => Some(9.0),
        "δέκα" | "δεκαριά" => Some(10.0),
        "έντεκα" | "ένδεκα" => Some(11.0),
        "δώδεκα" | "ντουζίνα" | "ντουζίνες" => Some(12.0),
        "δεκατρία" => Some(13.0),
        "δεκατέσσερα" => Some(14.0),
        "δεκαπέντε" => Some(15.0),
        "δεκαέξι" => Some(16.0),
        "δεκαεπτά" => Some(17.0),
        "δεκαοκτώ" => Some(18.0),
        "δεκαεννέα" | "δεκαεννιά" => Some(19.0),
        "είκοσι" => Some(20.0),
        "τριάντα" => Some(30.0),
        "σαράντα" => Some(40.0),
        "πενήντα" => Some(50.0),
        "εξήντα" => Some(60.0),
        "εβδομήντα" => Some(70.0),
        "ογδόντα" => Some(80.0),
        "ενενήντα" => Some(90.0),
        _ => None,
    }
}

fn hundreds_prefix(s: &str) -> Option<f64> {
    match s {
        "δι" => Some(200.0),
        "τρι" => Some(300.0),
        "τετρ" => Some(400.0),
        "πεντ" => Some(500.0),
        "εξ" => Some(600.0),
        "επτ" | "εφτ" => Some(700.0),
        "οκτ" | "οχτ" => Some(800.0),
        "εννι" => Some(900.0),
        _ => None,
    }
}

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some())
}

fn is_multipliable(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.multipliable)
}

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "few".to_string(),
            pattern: vec![regex("μερικ(ά|ές|οί)")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(3.0)))),
        },
        Rule {
            name: "number (0..19, 20, 30..90)".to_string(),
            pattern: vec![regex("(μηδέν|[εέ]ν[αοό]ς?|μ[ιί]ας?|δ[υύ]ο|τρ(ία|εις)|τέσσερ(α|ις)|πέντε|έξι|ε[πφ]τά|ο[κχ]τώ|ενν(ιά|έα)|δέκα|δεκαριά|έν[τδ]εκα|δώδεκα|ντουζίν(α|ες)|δεκα(τρία|τέσσερα|πέντε|έξι|ε[πφ]τά|ο[χκ]τώ|ενν(έα|ιά))|είκοσι|(τριά|σαρά|πενή|εξή|εβδομή|ογδό|ενενή)ντα)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(one_two_digits(&s)?)))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(d) if [20.0,30.0,40.0,50.0,60.0,70.0,80.0,90.0].contains(&d.value))), predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..10.0).contains(&d.value)))],
            production: Box::new(|nodes| {
                let t = numeral_data(&nodes[0].token_data)?.value;
                let u = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(t + u)))
            }),
        },
        Rule {
            name: "number (100)".to_string(),
            pattern: vec![regex("(εκατόν?)")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(100.0).with_grain(2)))),
        },
        Rule {
            name: "number (200..900)".to_string(),
            pattern: vec![regex("((δι|τρι|τετρ|πεντ|εξ|ε(π|φ)τ|ο(χ|κ)τ|εννι)ακόσι(α|ες|οι))")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(
                    NumeralData::new(hundreds_prefix(&s)?).with_grain(2),
                ))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(χίλι(α|οι|ες)|χιλιάδες|εκατομμύρι(ο|α)|δις|δισεκατομμύρι(ο|α))")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let out = match s.as_str() {
                    "χίλια" | "χίλιοι" | "χίλιες" | "χιλιάδες" => {
                        NumeralData::new(1e3).with_grain(3).with_multipliable(true)
                    }
                    "εκατομμύριο" | "εκατομμύρια" => {
                        NumeralData::new(1e6).with_grain(6).with_multipliable(true)
                    }
                    "δις" | "δισεκατομμύριο" | "δισεκατομμύρια" => {
                        NumeralData::new(1e9).with_grain(9).with_multipliable(true)
                    }
                    _ => return None,
                };
                Some(TokenData::Numeral(out))
            }),
        },
        Rule {
            name: "intersect 2 numbers".to_string(),
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
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d+,\\d+)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(
                    s.replace(',', ".").parse().ok()?,
                )))
            }),
        },
        Rule {
            name: "one point two".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("κόμμα"), predicate(|td| !has_grain(td))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + decimals_to_double(b))))
            }),
        },
        Rule {
            name: "dot-separated numbers".to_string(),
            pattern: vec![regex("(\\d+(\\.\\d\\d\\d)+(,\\d+)?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let normalized = s.replace('.', "").replace(',', ".");
                Some(TokenData::Numeral(NumeralData::new(normalized.parse().ok()?)))
            }),
        },
        Rule {
            name: "negative numbers".to_string(),
            pattern: vec![regex("-|μείον"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?;
                Some(TokenData::Numeral(NumeralData::new(-n.value)))
            }),
        },
    ]
}
