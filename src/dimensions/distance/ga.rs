use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{DistanceData, DistanceUnit};

fn distance_data(td: &TokenData) -> Option<&DistanceData> {
    match td {
        TokenData::Distance(d) => Some(d),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<dist> meters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("mh?(e|é)adai?r")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Metre)))
            }),
        },
        Rule {
            name: "<dist> centimeters".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex("(c\\.?m\\.?|g?ch?eintimh?(e|é)adai?r)"),
            ],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Centimetre),
                ))
            }),
        },
        Rule {
            name: "<dist> miles".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("mh?(í|i)lt?e")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Mile)))
            }),
        },
        Rule {
            name: "<latent dist> km".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex("(k\\.?(m\\.?)?|g?ch?ilim(e|é)adai?r)"),
            ],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Kilometre),
                ))
            }),
        },
        Rule {
            name: "<latent dist> troigh".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("('|d?th?roi[tg]he?|tr\\.?)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Foot)))
            }),
        },
        Rule {
            name: "<latent dist> orlach".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex("(''|([nth]-?)?orl(ach|aigh|a(í|i)|\\.))"),
            ],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Inch)))
            }),
        },
        Rule {
            name: "<dist> m (ambiguous miles or meters)".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("m")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::M)))
            }),
        },
    ]
}
