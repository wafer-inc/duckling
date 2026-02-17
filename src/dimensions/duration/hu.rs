use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "egy <unit-of-duration>".to_string(),
            pattern: vec![regex("egy"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::TimeGrain(g) => Some(TokenData::Duration(DurationData::new(1, *g))),
                _ => None,
            }),
        },
        Rule {
            name: "quarter of an hour".to_string(),
            pattern: vec![regex(r"(negyed[\s-]?óra)")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(15, Grain::Minute)))
            }),
        },
        Rule {
            name: "half an hour".to_string(),
            pattern: vec![regex(r"(fél[\s-]?óra)")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))
            }),
        },
        Rule {
            name: "kettő perc".to_string(),
            pattern: vec![regex(r"kettő perc")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(2, Grain::Minute)))
            }),
        },
        Rule {
            name: "hét hét".to_string(),
            pattern: vec![regex(r"hét hét")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(7, Grain::Week)))),
        },
    ]
}
