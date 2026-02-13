use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, predicate, regex};
use crate::dimensions::time_grain::Grain;
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

pub fn rules() -> Vec<Rule> {
    vec![
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
            name: "сутки".to_string(),
            pattern: vec![regex("сутки")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(24, Grain::Hour)))),
        },
        Rule {
            name: "<integer> суток".to_string(),
            pattern: vec![regex(r"(\d+)\s*(сутки|суток)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let n: i64 = m.group(1)?.parse().ok()?;
                Some(TokenData::Duration(DurationData::new(24 * n, Grain::Hour)))
            }),
        },
    ]
}
