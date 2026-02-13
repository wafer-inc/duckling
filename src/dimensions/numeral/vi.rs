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

fn integer_0_19(s: &str) -> Option<f64> {
    match s {
        "không" => Some(0.0),
        "một" | "linh một" | "lẻ một" => Some(1.0),
        "hai" | "linh hai" | "lẻ hai" => Some(2.0),
        "ba" | "linh ba" | "lẻ ba" => Some(3.0),
        "bốn" | "linh bốn" | "lẻ bốn" => Some(4.0),
        "năm" | "linh năm" | "lẻ năm" => Some(5.0),
        "sáu" | "linh sáu" | "lẻ sáu" => Some(6.0),
        "bảy" | "linh bảy" | "lẻ bảy" => Some(7.0),
        "tám" | "linh tám" | "lẻ tám" => Some(8.0),
        "chín" | "linh chín" | "lẻ chín" => Some(9.0),
        "mười" | "linh mười" | "lẻ mười" => Some(10.0),
        "mười một" => Some(11.0),
        "mười hai" => Some(12.0),
        "mười ba" => Some(13.0),
        "mười bốn" => Some(14.0),
        "mười lăm" => Some(15.0),
        "mười sáu" => Some(16.0),
        "mười bảy" => Some(17.0),
        "mười tám" => Some(18.0),
        "mười chín" => Some(19.0),
        _ => None,
    }
}

fn tens(s: &str) -> Option<f64> {
    match s {
        "hai mươi" => Some(20.0),
        "ba mươi" => Some(30.0),
        "bốn mươi" => Some(40.0),
        "năm mươi" => Some(50.0),
        "sáu mươi" => Some(60.0),
        "bảy mươi" => Some(70.0),
        "tám mươi" => Some(80.0),
        "chín mươi" => Some(90.0),
        _ => None,
    }
}

fn powers(s: &str) -> Option<NumeralData> {
    match s {
        "chục" => Some(NumeralData::new(1e1).with_grain(1).with_multipliable(true)),
        "trăm" => Some(NumeralData::new(1e2).with_grain(2).with_multipliable(true)),
        "nghìn" | "ngàn" => Some(NumeralData::new(1e3).with_grain(3).with_multipliable(true)),
        "triệu" => Some(NumeralData::new(1e6).with_grain(6).with_multipliable(true)),
        "tỷ" | "tỉ" => Some(NumeralData::new(1e9).with_grain(9).with_multipliable(true)),
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
            name: "integer (0..19)".to_string(),
            pattern: vec![regex("(không|một|linh một|lẻ một|hai|linh hai|lẻ hai|ba|linh ba|lẻ ba|bốn|linh bốn|lẻ bốn|năm|linh năm|lẻ năm|sáu|linh sáu|lẻ sáu|bảy|linh bảy|lẻ bảy|tám|linh tám|lẻ tám|chín|linh chín|lẻ chín|mười một|mười hai|mười ba|mười bốn|mười lăm|mười sáu|mười bảy|mười tám|mười chín|mười|linh mười|lẻ mười)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(integer_0_19(&s)?)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(hai mươi|ba mươi|bốn mươi|năm mươi|sáu mươi|bảy mươi|tám mươi|chín mươi)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(tens(&s)?)))
            }),
        },
        Rule {
            name: "numbers 21..99".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(d) if [20.0,30.0,40.0,50.0,60.0,70.0,80.0,90.0].contains(&d.value))), predicate(number_between(1.0, 10.0))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "numbers xx1 with mốt".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(d) if [20.0,30.0,40.0,50.0,60.0,70.0,80.0,90.0].contains(&d.value))), regex("mốt")],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + 1.0)))
            }),
        },
        Rule {
            name: "numbers xx5 with lăm".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(d) if [20.0,30.0,40.0,50.0,60.0,70.0,80.0,90.0].contains(&d.value))), regex("lăm")],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + 5.0)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(chục|trăm|nghìn|ngàn|triệu|t(ỷ|ỉ))")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let s = m.group(1)?.to_lowercase();
                Some(TokenData::Numeral(powers(&s)?))
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
            name: "dot word decimal".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("chấm|phẩy"), predicate(|td| !has_grain(td))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + decimals_to_double(b))))
            }),
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
            name: "negative prefix".to_string(),
            pattern: vec![regex("\\-|âm"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?;
                Some(TokenData::Numeral(NumeralData::new(-n.value)))
            }),
        },
        Rule {
            name: "dozen".to_string(),
            pattern: vec![regex("tá")],
            production: Box::new(|_| {
                Some(TokenData::Numeral(
                    NumeralData::new(12.0)
                        .with_grain(1)
                        .with_multipliable(true),
                ))
            }),
        },
    ]
}
