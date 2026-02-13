use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "a <unit-of-duration>".to_string(),
            pattern: vec![regex("en|ett|et?|e"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::TimeGrain(g) => Some(TokenData::Duration(DurationData::new(1, *g))),
                _ => None,
            }),
        },
        Rule {
            name: "half an hour".to_string(),
            pattern: vec![regex("(1/2|en halv) time")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(30, crate::dimensions::time_grain::Grain::Minute)))),
        },
    ]
}
