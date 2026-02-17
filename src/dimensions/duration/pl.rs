use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

pub fn rules() -> Vec<Rule> {
    vec![
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
            pattern: vec![regex("p(o|ó)(l|ł) godziny")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))
            }),
        },
    ]
}
