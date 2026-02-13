use crate::dimensions::numeral::helpers::is_natural;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

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
            pattern: vec![regex("(mitja hora|dos quarts)")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(
                    30,
                    crate::dimensions::time_grain::Grain::Minute,
                )))
            }),
        },
        Rule {
            name: "quarter of an hour".to_string(),
            pattern: vec![regex("((1 |un )?quarts?|1/4) d'hora")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(
                    15,
                    crate::dimensions::time_grain::Grain::Minute,
                )))
            }),
        },
        Rule {
            name: "three-quarters of an hour".to_string(),
            pattern: vec![regex("((tres|3) quarts|3/4)( d'hor([a]|(es)))?")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(
                    45,
                    crate::dimensions::time_grain::Grain::Minute,
                )))
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
                let v = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value as i64,
                    _ => return None,
                };
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = duration_data(&nodes[2].token_data)?;
                if g <= dd.grain {
                    return None;
                }
                Some(TokenData::Duration(DurationData::new(v, g).combine(dd)))
            }),
        },
        Rule {
            name: "composite <duration> (with and)".to_string(),
            pattern: vec![
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
                regex("i"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let v = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value as i64,
                    _ => return None,
                };
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = duration_data(&nodes[3].token_data)?;
                if g <= dd.grain {
                    return None;
                }
                Some(TokenData::Duration(DurationData::new(v, g).combine(dd)))
            }),
        },
    ]
}
