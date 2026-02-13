use crate::dimensions::numeral::helpers::{is_positive, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::NumeralData;

fn numeral_0_10(s: &str) -> Option<f64> {
    match s {
        "零" | "ゼロ" => Some(0.0),
        "一" => Some(1.0),
        "二" => Some(2.0),
        "三" => Some(3.0),
        "四" => Some(4.0),
        "五" => Some(5.0),
        "六" => Some(6.0),
        "七" => Some(7.0),
        "八" => Some(8.0),
        "九" => Some(9.0),
        "十" => Some(10.0),
        _ => None,
    }
}

fn one_to_nine(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if (1.0..10.0).contains(&d.value))
}

fn is_val(td: &TokenData, candidates: &[f64]) -> bool {
    matches!(td, TokenData::Numeral(d) if candidates.contains(&d.value))
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*\\.\\d+)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(s.parse().ok()?)))
            }),
        },
        Rule {
            name: "decimal with thousands separator".to_string(),
            pattern: vec![regex("(\\d+(,\\d\\d\\d)+\\.\\d+)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(
                    s.replace(',', "").parse().ok()?,
                )))
            }),
        },
        Rule {
            name: "integer with thousands separator ,".to_string(),
            pattern: vec![regex("(\\d{1,3}(,\\d\\d\\d){1,5})")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(
                    s.replace(',', "").parse().ok()?,
                )))
            }),
        },
        Rule {
            name: "integer (0..10)".to_string(),
            pattern: vec![regex("(ゼロ|零|一|二|三|四|五|六|七|八|九|十)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(numeral_0_10(s)?)))
            }),
        },
        Rule {
            name: "integer (11..19)".to_string(),
            pattern: vec![regex("十"), predicate(one_to_nine)],
            production: Box::new(|nodes| {
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(10.0 + b)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![predicate(one_to_nine), regex("十")],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a * 10.0)))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![
                predicate(|td| {
                    is_val(td, &[20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0])
                }),
                predicate(one_to_nine),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "integer (100)".to_string(),
            pattern: vec![regex("百")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(100.0)))),
        },
        Rule {
            name: "integer (100..199)".to_string(),
            pattern: vec![regex("百"), predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..100.0).contains(&d.value)))],
            production: Box::new(|nodes| {
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(100.0 + b)))
            }),
        },
        Rule {
            name: "integer (200..900)".to_string(),
            pattern: vec![predicate(one_to_nine), regex("百")],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                if a >= 2.0 {
                    Some(TokenData::Numeral(NumeralData::new(a * 100.0)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "integer 201..999".to_string(),
            pattern: vec![
                predicate(|td| {
                    is_val(td, &[200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0])
                }),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..100.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "integer (1000)".to_string(),
            pattern: vec![regex("千")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(1000.0)))),
        },
        Rule {
            name: "integer (1000..1999)".to_string(),
            pattern: vec![regex("千"), predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..1000.0).contains(&d.value)))],
            production: Box::new(|nodes| {
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(1000.0 + b)))
            }),
        },
        Rule {
            name: "integer (2000..9000)".to_string(),
            pattern: vec![predicate(one_to_nine), regex("千")],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                if a >= 2.0 {
                    Some(TokenData::Numeral(NumeralData::new(a * 1000.0)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "integer 2001..9999".to_string(),
            pattern: vec![
                predicate(|td| {
                    is_val(
                        td,
                        &[2000.0, 3000.0, 4000.0, 5000.0, 6000.0, 7000.0, 8000.0, 9000.0],
                    )
                }),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..1000.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "integer (10000)".to_string(),
            pattern: vec![regex("万")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(10000.0)))),
        },
        Rule {
            name: "integer (10000..19999)".to_string(),
            pattern: vec![regex("万"), predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..10000.0).contains(&d.value)))],
            production: Box::new(|nodes| {
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(10000.0 + b)))
            }),
        },
        Rule {
            name: "integer (20000..90000)".to_string(),
            pattern: vec![predicate(one_to_nine), regex("万")],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                if a >= 2.0 {
                    Some(TokenData::Numeral(NumeralData::new(a * 10000.0)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "integer 20001..99999".to_string(),
            pattern: vec![
                predicate(|td| {
                    is_val(
                        td,
                        &[
                            20000.0, 30000.0, 40000.0, 50000.0, 60000.0, 70000.0, 80000.0,
                            90000.0,
                        ],
                    )
                }),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..10000.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "<number>个".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("(个|個)")],
            production: Box::new(|nodes| Some(nodes[0].token_data.clone())),
        },
        Rule {
            name: "numbers suffixes (K, M, G, 千, 万)".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("(k|m|g|千|万)")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let s = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let out = match s.as_str() {
                    "k" | "千" => v * 1e3,
                    "万" => v * 1e4,
                    "m" => v * 1e6,
                    "g" => v * 1e9,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(out)))
            }),
        },
        Rule {
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|マイナス\\s?|負\\s?"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
    ]
}
