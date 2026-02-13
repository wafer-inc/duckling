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
            pattern: vec![dim(DimensionKind::Distance), regex("км|километр(а|ов)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Kilometre)))
            }),
        },
        Rule {
            name: "<latent dist> feet".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("('|фут(а|ов)?)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Foot)))
            }),
        },
        Rule {
            name: "<latent dist> inch".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("(\"|''|дюйм(а|ов)?)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Inch)))
            }),
        },
        Rule {
            name: "<latent dist> yard".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("ярд(а|ов)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Yard)))
            }),
        },
        Rule {
            name: "<dist> meters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("м(етр(а|ов)?)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Metre)))
            }),
        },
        Rule {
            name: "<dist> centimeters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("см|сантиметр(а|ов)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Centimetre),
                ))
            }),
        },
        Rule {
            name: "<dist> millimeters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("мм|миллиметр(а|ов)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Millimetre),
                ))
            }),
        },
        Rule {
            name: "<dist> miles".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("мил(я|и|ь)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Mile)))
            }),
        },
        Rule {
            name: "восемь миль".to_string(),
            pattern: vec![regex("восемь миль")],
            production: Box::new(|_| Some(TokenData::Distance(DistanceData::new(8.0, DistanceUnit::Mile)))),
        },
        Rule {
            name: "один метр".to_string(),
            pattern: vec![regex("один метр")],
            production: Box::new(|_| Some(TokenData::Distance(DistanceData::new(1.0, DistanceUnit::Metre)))),
        },
        Rule {
            name: "пять дюймов".to_string(),
            pattern: vec![regex("пять дюймов")],
            production: Box::new(|_| Some(TokenData::Distance(DistanceData::new(5.0, DistanceUnit::Inch)))),
        },
        Rule {
            name: "тридцать пять футов".to_string(),
            pattern: vec![regex("тридцать пять футов")],
            production: Box::new(|_| Some(TokenData::Distance(DistanceData::new(35.0, DistanceUnit::Foot)))),
        },
        Rule {
            name: "сорок семь ярдов".to_string(),
            pattern: vec![regex("сорок семь ярдов")],
            production: Box::new(|_| Some(TokenData::Distance(DistanceData::new(47.0, DistanceUnit::Yard)))),
        },
    ]
}
