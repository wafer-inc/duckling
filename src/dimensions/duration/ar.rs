use crate::dimensions::numeral::helpers::numeral_data;
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

fn is_natural(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value > 0.0 && d.value.fract() == 0.0)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "single <unit-of-duration>".to_string(),
            pattern: vec![dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| match &nodes[0].token_data {
                TokenData::TimeGrain(g) => Some(TokenData::Duration(DurationData::new(1, *g))),
                _ => None,
            }),
        },
        Rule {
            name: "half an hour".to_string(),
            pattern: vec![regex(r"(1/2\s?ساع[ةه]?|نصف? ساع[ةه])")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))),
        },
        Rule {
            name: "quarter of an hour".to_string(),
            pattern: vec![regex(r"(ربع ساعة)")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(15, Grain::Minute)))),
        },
        Rule {
            name: "<integer> and half hour".to_string(),
            pattern: vec![predicate(is_natural), regex(r"و ?نصف? ساع[ةه]")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                Some(TokenData::Duration(DurationData::new(30 + 60 * v, Grain::Minute)))
            }),
        },
        Rule {
            name: "two seconds".to_string(),
            pattern: vec![regex(r"ثانيتين|ثانيتان|لحظتين|لحظتان")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(2, Grain::Second)))),
        },
        Rule {
            name: "two minutes".to_string(),
            pattern: vec![regex(r"دقيقتين|دقيقتان")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(2, Grain::Minute)))),
        },
        Rule {
            name: "two hours".to_string(),
            pattern: vec![regex(r"ساعتين|ساعتان")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(2, Grain::Hour)))),
        },
        Rule {
            name: "two years".to_string(),
            pattern: vec![regex(r"سنتين|سنتان")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(2, Grain::Year)))),
        },
    ]
}
