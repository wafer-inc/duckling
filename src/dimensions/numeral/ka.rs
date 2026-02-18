use crate::dimensions::numeral::helpers::{decimals_to_double, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::NumeralData;

fn lex_0_19(s: &str) -> Option<NumeralData> {
    let value = match s {
        "ნოლ" | "ნულ" | "ნული" | "ნოლი" => 0.0,
        "ერთი" => 1.0,
        "ორი" | "ორ" => 2.0,
        "სამი" | "სამ" => 3.0,
        "ოთხი" | "ოთხ" => 4.0,
        "ხუთი" | "ხუთ" => 5.0,
        "ექვსი" | "ექვს" => 6.0,
        "შვიდი" | "შვიდ" => 7.0,
        "რვა" | "რვ" => 8.0,
        "ცხრა" | "ცხრ" => 9.0,
        "ათი" | "აათი" => 10.0,
        "თერთმეტი" | "თერთმეტ" => 11.0,
        "თორმეტი" | "თორმეტ" => 12.0,
        "ცამეტი" | "ცამეტ" => 13.0,
        "თოთხმეტი" | "თოთხმეტ" => 14.0,
        "თხუთმეტი" | "თხუთმეტ" => 15.0,
        "თექვსმეტი" | "თექვსმეტ" => 16.0,
        "ჩვიდმეტი" | "ჩვიდმეტ" => 17.0,
        "თვრამეტი" | "თვრამეტ" => 18.0,
        "ცხრამეტი" | "ცხრამეტ" => 19.0,
        "წყვილი" | "წყვილები" => {
            return Some(NumeralData::new(2.0).not_ok_for_any_time())
        }
        "ცოტა" | "რამდენიმე" | "რამოდენიმე" => {
            return Some(NumeralData::new(3.0).not_ok_for_any_time())
        }
        _ => return None,
    };
    Some(NumeralData::new(value))
}

fn tens(s: &str) -> Option<f64> {
    match s {
        "ოცი" | "ოცდა" | "ოც" => Some(20.0),
        "ოცდაათ" | "ოცდაათი" => Some(30.0),
        "ორმოც" | "ორმოცი" | "ორმოცდა" => Some(40.0),
        "ორმოცდაათ" | "ორმოცდაათი" => Some(50.0),
        "სამოც" | "სამოცი" | "სამოცდა" => Some(60.0),
        "სამოცდაათ" | "სამოცდაათი" => Some(70.0),
        "ოთხმოც" | "ოთხმოცი" | "ოთხმოცდა" => Some(80.0),
        "ოთხმოცდაათ" | "ოთხმოცდაათი" => Some(90.0),
        _ => None,
    }
}

fn hundreds(s: &str) -> Option<f64> {
    match s {
        "ასი" | "ას" => Some(100.0),
        "ორასი" | "ორას" | "ორ ას" | "ორ ასი" => Some(200.0),
        "სამასი" | "სამას" | "სამ ას" | "სამ ასი" => {
            Some(300.0)
        }
        "ოთხასი" | "ოთხას" | "ოთხ ას" | "ოთხ ასი" => {
            Some(400.0)
        }
        "ხუთასი" | "ხუთას" | "ხუთ ას" | "ხუთ ასი" => {
            Some(500.0)
        }
        "ექვსასი" | "ექვსას" | "ექვს ას" | "ექვს ასი" => {
            Some(600.0)
        }
        "შვიდასი" | "შვიდას" | "შვიდ ას" | "შვიდ ასი" => {
            Some(700.0)
        }
        "რვაასი" | "რვაას" | "რვა ას" | "რვა ასი" => {
            Some(800.0)
        }
        "ცხრაასი" | "ცხრაას" | "ცხრა ას" | "ცხრა ასი" => {
            Some(900.0)
        }
        _ => None,
    }
}

fn power(s: &str) -> Option<NumeralData> {
    match s {
        "ათასი" | "ათას" => {
            Some(NumeralData::new(1e3).with_grain(3).with_multipliable(true))
        }
        "მილიონი" | "მილიონ" => {
            Some(NumeralData::new(1e6).with_grain(6).with_multipliable(true))
        }
        "მილიარდი" | "მილიარდ" => {
            Some(NumeralData::new(1e9).with_grain(9).with_multipliable(true))
        }
        _ => None,
    }
}

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some())
}

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn is_multipliable(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.multipliable)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "integer (0..19)".to_string(),
            pattern: vec![regex("(წყვილ(ებ)?ი|ცოტა|რამდენიმე|რამოდენიმე|ნოლი?|ნული?|ერთი|ორი?|სამი?|ოთხი?|ხუთი?|ექვსი?|შვიდი?|რვა|თერთმეტი?|თორმეტი?|ცამეტი?|თოთხმეტი?|თხუთმეტი?|თექვსმეტი?|ჩვიდმეტი?|თვრამეტი?|ცხრამეტი?|ცხრა|ა?ათი)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(lex_0_19(&s)?))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(ოცდაათი?|ორმოცდაათი?|სამოცდაათი?|ოთხმოცდაათი?|ოცდა|ორმოცდა|სამოცდა|ოთხმოცდა|ოცი?|ორმოცი?|სამოცი?|ოთხმოცი?)")],
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
            pattern: vec![regex("(ასი?|ორ ?ასი?|სამ ?ასი?|ოთხ ?ასი?|ხუთ ?ასი?|ექვს ?ასი?|შვიდ ?ასი?|რვა ?ასი?|ცხრა ?ასი?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(hundreds(&s)?).with_grain(2)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(ათასი?|მილიონი?|მილიარდი?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(power(&s)?))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if [20.0,40.0,50.0,60.0,70.0,80.0,90.0].contains(&d.value))),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..20.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "integer 100..999 (3 parts)".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if [100.0,200.0,300.0,400.0,500.0,600.0,700.0,800.0,900.0].contains(&d.value))),
                predicate(|td| matches!(td, TokenData::Numeral(d) if [20.0,40.0,50.0,60.0,70.0,80.0,90.0].contains(&d.value))),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..20.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[1].token_data)?.value;
                let c = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b + c)))
            }),
        },
        Rule {
            name: "integer 100..999 (2 parts)".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if [100.0,200.0,300.0,400.0,500.0,600.0,700.0,800.0,900.0].contains(&d.value))),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..100.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
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
            name: "intersect 2 numbers".to_string(),
            pattern: vec![predicate(|td| has_grain(td) && is_positive(td)), predicate(|td| !is_multipliable(td) && is_positive(td))],
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
            name: "comma-separated numbers".to_string(),
            pattern: vec![regex("(\\d+(,\\d\\d\\d)+(\\.\\d+)?)")],
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
            name: "one point two".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("წერტილი|მთელი"), predicate(|td| !has_grain(td))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + decimals_to_double(b))))
            }),
        },
        Rule {
            name: "point 77".to_string(),
            pattern: vec![regex("წერტილი|მთელი"), predicate(|td| !has_grain(td))],
            production: Box::new(|nodes| {
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(decimals_to_double(b))))
            }),
        },
        Rule {
            name: "suffixes (K,M,G))".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("(k|m|g)")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let s = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let mul = match s.as_str() {
                    "k" => 1e3,
                    "m" => 1e6,
                    "g" => 1e9,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v * mul)))
            }),
        },
        Rule {
            name: "negative numbers".to_string(),
            pattern: vec![regex("(-|მინუს|მინ)"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
    ]
}
