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
                regex("cm|센(티|치)((미|메)터)?"),
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
            pattern: vec![dim(DimensionKind::Distance), regex("m|(미|메|매)터")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Metre),
                ))
            }),
        },
        Rule {
            name: "<dist> miles".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("miles?|마일(즈)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Mile)))
            }),
        },
        Rule {
            name: "<latent dist> km".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex("km|(킬|키)로((미|메)터)?"),
            ],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Kilometre),
                ))
            }),
        },
        Rule {
            name: "구메터".to_string(),
            pattern: vec![regex("구메터")],
            production: Box::new(|_| {
                Some(TokenData::Distance(DistanceData::new(
                    9.0,
                    DistanceUnit::Metre,
                )))
            }),
        },
        Rule {
            name: "이센치".to_string(),
            pattern: vec![regex("이센치")],
            production: Box::new(|_| {
                Some(TokenData::Distance(DistanceData::new(
                    2.0,
                    DistanceUnit::Centimetre,
                )))
            }),
        },
    ]
}
