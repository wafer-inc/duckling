use crate::dimensions::numeral::helpers::{is_natural, numeral_data};
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

fn n_plus_one_half(grain: Grain, n: i64) -> Option<DurationData> {
    match grain {
        Grain::Minute => Some(DurationData::new(30 + 60 * n, Grain::Second)),
        Grain::Hour => Some(DurationData::new(30 + 60 * n, Grain::Minute)),
        Grain::Day => Some(DurationData::new(12 + 24 * n, Grain::Hour)),
        Grain::Month => Some(DurationData::new(15 + 30 * n, Grain::Day)),
        Grain::Year => Some(DurationData::new(6 + 12 * n, Grain::Month)),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<integer> <unit-of-duration>".to_string(),
            pattern: vec![predicate(is_natural), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(n, grain)))
            }),
        },
        Rule {
            name: "<unit-of-duration> as duration".to_string(),
            pattern: vec![dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| match &nodes[0].token_data {
                TokenData::TimeGrain(g) => Some(TokenData::Duration(DurationData::new(1, *g))),
                _ => None,
            }),
        },
        Rule {
            name: "half an hour".to_string(),
            pattern: vec![regex("(1/2\\s?|пів\\s?)години?")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))),
        },
        Rule {
            name: "number.number hours".to_string(),
            pattern: vec![regex("(\\d+)\\.(\\d+)\\s*години?")],
            production: Box::new(|nodes| {
                let rm = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let h: i64 = rm.group(1)?.parse().ok()?;
                let frac = rm.group(2)?;
                let num: i64 = frac.parse().ok()?;
                let den: i64 = 10_i64.pow(frac.len() as u32);
                let total_minutes = 60 * h + (num * 60) / den;
                Some(TokenData::Duration(DurationData::new(
                    total_minutes,
                    Grain::Minute,
                )))
            }),
        },
        Rule {
            name: "<integer> and a half hour".to_string(),
            pattern: vec![predicate(is_natural), regex("з\\s+половиною\\s+години")],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                Some(TokenData::Duration(DurationData::new(
                    30 + 60 * n,
                    Grain::Minute,
                )))
            }),
        },
        Rule {
            name: "about <duration>".to_string(),
            pattern: vec![regex("близько"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "exactly <duration>".to_string(),
            pattern: vec![regex("рівно"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "half a <time-grain>".to_string(),
            pattern: vec![regex("(1/2|пів)"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(n_plus_one_half(grain, 0)?))
            }),
        },
        Rule {
            name: "one and a half <unit-of-duration>".to_string(),
            pattern: vec![regex("півтори"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(n_plus_one_half(grain, 1)?))
            }),
        },
        Rule {
            name: "composite <duration> (with ,/і)".to_string(),
            pattern: vec![
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
                regex(",|і"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(grain) => *grain,
                    _ => return None,
                };
                let dd = match &nodes[3].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                if g <= dd.grain {
                    return None;
                }
                let left = DurationData::new(n, g);
                Some(TokenData::Duration(left.combine(dd)))
            }),
        },
    ]
}
