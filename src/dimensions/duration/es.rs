use crate::dimensions::numeral::helpers::{is_natural, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;
use crate::dimensions::time_grain::Grain;

fn duration_data(td: &TokenData) -> Option<&DurationData> {
    match td {
        TokenData::Duration(d) => Some(d),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "half of an hour".to_string(),
            pattern: vec![regex("media horas?")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))),
        },
        Rule {
            name: "quarter of an hour".to_string(),
            pattern: vec![regex("cuartos? de hora")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(15, Grain::Minute)))),
        },
        Rule {
            name: "three-quarters of an hour".to_string(),
            pattern: vec![regex("tres cuartos? de horas?")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(45, Grain::Minute)))),
        },
        Rule {
            name: "composite <duration>".to_string(),
            pattern: vec![
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = duration_data(&nodes[2].token_data)?;
                if g <= dd.grain {
                    return None;
                }
                Some(TokenData::Duration(DurationData::new(v, g).combine(dd)?))
            }),
        },
        Rule {
            name: "composite <duration> (with and)".to_string(),
            pattern: vec![
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
                regex("y"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = duration_data(&nodes[3].token_data)?;
                if g <= dd.grain {
                    return None;
                }
                Some(TokenData::Duration(DurationData::new(v, g).combine(dd)?))
            }),
        },
    ]
}
