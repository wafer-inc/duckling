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
            name: "<latent dist> km".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("km|kilometr(y|ů)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Kilometre)))
            }),
        },
        Rule {
            name: "<dist> meters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("metr(y|ů)?|m")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Metre)))
            }),
        },
        Rule {
            name: "<dist> centimeters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("cm|centimetr(y|ů)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Centimetre),
                ))
            }),
        },
        Rule {
            name: "<dist> miles".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("mil")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Mile)))
            }),
        },
        Rule {
            name: "osm mil".to_string(),
            pattern: vec![regex("osm mil")],
            production: Box::new(|_| Some(TokenData::Distance(DistanceData::new(8.0, DistanceUnit::Mile)))),
        },
    ]
}
