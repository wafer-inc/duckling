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
            name: "<dist> centimeters".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex("(cm|cent(í|i)m(etres?))"),
            ],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Centimetre),
                ))
            }),
        },
        Rule {
            name: "<dist> meters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("m(etres?)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Metre),
                ))
            }),
        },
        Rule {
            name: "<dist> miles".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("milles?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Mile)))
            }),
        },
        Rule {
            name: "<latent dist> km".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex("(k|qu)(il(o|ò|ó))?m?(etre)?s?"),
            ],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Kilometre),
                ))
            }),
        },
    ]
}
