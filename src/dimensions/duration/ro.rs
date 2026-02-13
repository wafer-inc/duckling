use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "exact|in jur de <duration>".to_string(),
            pattern: vec![
                regex("(exact|aproximativ|(i|î)n jur de)"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "jumatate de ora".to_string(),
            pattern: vec![regex("(1/2\\s?(h|or(a|ă))|jum(a|ă)tate (de )?or(a|ă))")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(
                    30,
                    crate::dimensions::time_grain::Grain::Minute,
                )))
            }),
        },
        Rule {
            name: "o <unit-of-duration>".to_string(),
            pattern: vec![regex("o|un"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(1, grain)))
            }),
        },
        Rule {
            name: "quarter of an hour".to_string(),
            pattern: vec![regex("(1/4\\s?(h|or(a|ă))|sfert de or(a|ă))")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(
                    15,
                    crate::dimensions::time_grain::Grain::Minute,
                )))
            }),
        },
        Rule {
            name: "trei sferturi de ora".to_string(),
            pattern: vec![regex("(3/4\\s?(h|or(a|ă))|trei sferturi de or(a|ă))")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(
                    45,
                    crate::dimensions::time_grain::Grain::Minute,
                )))
            }),
        },
        Rule {
            name: "<integer> de <unit-of-duration>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value >= 20.0)),
                regex("de"),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(v, grain)))
            }),
        },
    ]
}
