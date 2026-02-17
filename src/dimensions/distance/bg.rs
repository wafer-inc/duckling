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
            pattern: vec![dim(DimensionKind::Distance), regex("км|километ(ра|ри|ър)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Kilometre),
                ))
            }),
        },
        Rule {
            name: "<latent dist> feet".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("('|фут(а|ове)?)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Foot)))
            }),
        },
        Rule {
            name: "<latent dist> inch".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("(\"|''|инч(а|ове)?)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Inch)))
            }),
        },
        Rule {
            name: "<latent dist> yard".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("ярд(а|ове)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Yard)))
            }),
        },
        Rule {
            name: "<dist> meters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("м(етър|етр(а|и))?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Metre),
                ))
            }),
        },
        Rule {
            name: "<dist> centimeters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("см|сантимет(ри|ра|ър)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Centimetre),
                ))
            }),
        },
        Rule {
            name: "<dist> millimeters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("мм|милимет(ра|ри|ър)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Millimetre),
                ))
            }),
        },
        Rule {
            name: "<dist> miles".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("мил(я|и)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Mile)))
            }),
        },
    ]
}
