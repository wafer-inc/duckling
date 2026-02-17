use crate::dimensions::numeral::helpers::numeral_data;
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

fn is_natural(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value > 0.0 && d.value.fract() == 0.0)
}

fn is_positive_non_integer(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value > 0.0 && d.value.fract() != 0.0)
}

// Equivalent to Haskell's nPlusOneHalf for BG duration rules.
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

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<positive-numeral> <time-grain> and a half".to_string(),
            pattern: vec![predicate(is_natural), dim(DimensionKind::TimeGrain), regex("и половина")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(n_plus_one_half(grain, v)?))
            }),
        },
        Rule {
            name: "<time-grain> and a half".to_string(),
            pattern: vec![dim(DimensionKind::TimeGrain), regex("и половина")],
            production: Box::new(|nodes| {
                let grain = match &nodes[0].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(n_plus_one_half(grain, 1)?))
            }),
        },
        Rule {
            name: "<positive-numeral> <time-grain>".to_string(),
            pattern: vec![predicate(is_positive_non_integer), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let seconds = grain.one_in_seconds_f64() * v;
                Some(TokenData::Duration(DurationData::new(
                    seconds.floor() as i64,
                    Grain::Second,
                )))
            }),
        },
        Rule {
            name: "about <duration>".to_string(),
            pattern: vec![regex("(към|приблизително|примерно|някъде)"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "<integer> + '\"'".to_string(),
            pattern: vec![predicate(is_natural), regex("(['\"])")],
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
            name: "a <unit-of-duration>".to_string(),
            pattern: vec![dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let grain = match &nodes[0].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(1, grain)))
            }),
        },
        Rule {
            name: "half of a <time-grain>".to_string(),
            pattern: vec![regex("половин"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(n_plus_one_half(grain, 0)?))
            }),
        },
    ]
}
