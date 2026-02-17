use crate::dimensions::numeral::helpers::{is_natural, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::dimensions::time_grain::Grain;
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

fn n_plus_one_half(grain: Grain, n: i64) -> Option<DurationData> {
    match grain {
        Grain::Minute => Some(DurationData::new(60_i64.checked_mul(n)?.checked_add(30)?, Grain::Second)),
        Grain::Hour => Some(DurationData::new(60_i64.checked_mul(n)?.checked_add(30)?, Grain::Minute)),
        Grain::Day => Some(DurationData::new(24_i64.checked_mul(n)?.checked_add(12)?, Grain::Hour)),
        Grain::Month => Some(DurationData::new(30_i64.checked_mul(n)?.checked_add(15)?, Grain::Day)),
        Grain::Year => Some(DurationData::new(12_i64.checked_mul(n)?.checked_add(6)?, Grain::Month)),
        _ => None,
    }
}

fn parse_half_grain_ru(s: &str) -> Option<Grain> {
    match s.to_lowercase().as_str() {
        "года" => Some(Grain::Year),
        "месяца" => Some(Grain::Month),
        "дня" => Some(Grain::Day),
        "часа" => Some(Grain::Hour),
        "минуты" => Some(Grain::Minute),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<positive-non-integer> <time-grain>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value > 0.0 && d.value.fract() != 0.0)),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                // Approximate Haskell inCoarsestGrain behavior with seconds conversion.
                let secs = grain.one_in_seconds_f64() * v;
                let target = if secs >= Grain::Month.one_in_seconds_f64() {
                    Grain::Month
                } else if secs >= Grain::Day.one_in_seconds_f64() {
                    Grain::Day
                } else if secs >= Grain::Hour.one_in_seconds_f64() {
                    Grain::Hour
                } else if secs >= Grain::Minute.one_in_seconds_f64() {
                    Grain::Minute
                } else {
                    Grain::Second
                };
                Some(TokenData::Duration(DurationData::new(
                    (secs / target.one_in_seconds_f64()).round() as i64,
                    target,
                )))
            }),
        },
        Rule {
            name: "about|exactly <duration>".to_string(),
            pattern: vec![
                regex("(где-то|приблизительно|примерно|ровно)"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "a <unit-of-duration>".to_string(),
            pattern: vec![dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| match &nodes[0].token_data {
                TokenData::TimeGrain(g) => Some(TokenData::Duration(DurationData::new(1, *g))),
                _ => None,
            }),
        },
        Rule {
            name: "<integer> + '\"".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value > 0.0 && d.value.fract() == 0.0)),
                regex(r#"(['"])"#),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let q = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                match q {
                    "'" => Some(TokenData::Duration(DurationData::new(v, Grain::Minute))),
                    "\"" => Some(TokenData::Duration(DurationData::new(v, Grain::Second))),
                    _ => None,
                }
            }),
        },
        Rule {
            name: "half of a grain".to_string(),
            pattern: vec![regex("пол\\s?(года|месяца|дня|часа|минуты)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = parse_half_grain_ru(s)?;
                Some(TokenData::Duration(n_plus_one_half(grain, 0)?))
            }),
        },
        Rule {
            name: "hour diminutive".to_string(),
            pattern: vec![regex("час(ок|ик|очек)")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(1, Grain::Hour)))),
        },
        Rule {
            name: "hour diminutive 2".to_string(),
            pattern: vec![predicate(is_natural), regex("час(иков|очков)")],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                Some(TokenData::Duration(DurationData::new(n, Grain::Hour)))
            }),
        },
        Rule {
            name: "minute diminutive".to_string(),
            pattern: vec![regex("минутк.|минуточк.")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(1, Grain::Minute)))),
        },
        Rule {
            name: "minute diminutive 2".to_string(),
            pattern: vec![
                predicate(is_natural),
                regex("минутк.|минуток|минуточк.|минуточек"),
            ],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                Some(TokenData::Duration(DurationData::new(n, Grain::Minute)))
            }),
        },
        Rule {
            name: "quarter of an hour".to_string(),
            pattern: vec![regex("((одн(у|а|ой)|1)\\s)?четверт. (часа|ч|ч\\.)")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(15, Grain::Minute)))),
        },
        Rule {
            name: "3 quarters of an hour".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(d) if d.value == 3.0)), regex("четверт(и|ей) (часа|ч|ч\\.)")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(45, Grain::Minute)))),
        },
        Rule {
            name: "сутки".to_string(),
            pattern: vec![regex("сутки")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(24, Grain::Hour)))),
        },
        Rule {
            name: "<integer> суток".to_string(),
            pattern: vec![predicate(is_natural), regex(r"(сутки|суток)")],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                Some(TokenData::Duration(DurationData::new(24_i64.checked_mul(n)?, Grain::Hour)))
            }),
        },
        Rule {
            name: "composite <duration>".to_string(),
            pattern: vec![
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(grain) => *grain,
                    _ => return None,
                };
                let dd = match &nodes[2].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                if g <= dd.grain {
                    return None;
                }
                let left = DurationData::new(n, g);
                Some(TokenData::Duration(left.combine(dd)?))
            }),
        },
    ]
}
