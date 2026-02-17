use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{VolumeData, VolumeUnit};

fn volume_data(td: &TokenData) -> Option<&VolumeData> {
    match td {
        TokenData::Volume(d) => Some(d),
        _ => None,
    }
}

fn is_unit_only() -> crate::types::PatternItem {
    predicate(|td| {
        matches!(
            td,
            TokenData::Volume(d)
                if d.value.is_none()
                    && d.unit.is_some()
                    && d.min_value.is_none()
                    && d.max_value.is_none()
        )
    })
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "number as volume".to_string(),
            pattern: vec![dim(DimensionKind::Numeral)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?;
                if n.value <= 0.0 {
                    return None;
                }
                Some(TokenData::Volume(VolumeData::value_only(n.value)))
            }),
        },
        Rule {
            name: "<number> <volume>".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), is_unit_only()],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?;
                let d = volume_data(&nodes[1].token_data)?;
                if n.value <= 0.0 {
                    return None;
                }
                Some(TokenData::Volume(VolumeData::new(n.value, d.unit?)))
            }),
        },
        Rule {
            name: "<latent vol> ml".to_string(),
            pattern: vec![regex("мл|миллилитр(а|ов)?")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Millilitre,
                )))
            }),
        },
        Rule {
            name: "<vol> hectoliters".to_string(),
            pattern: vec![regex("гл|гектолитр(а|ов)?")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Hectolitre,
                )))
            }),
        },
        Rule {
            name: "<vol> liters".to_string(),
            pattern: vec![regex("л(итр(а|ов)?)?")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Litre)))
            }),
        },
        Rule {
            name: "<latent vol> gallon".to_string(),
            pattern: vec![regex("галлон(а|ов)?")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Gallon)))
            }),
        },
        Rule {
            name: "half liter".to_string(),
            pattern: vec![regex("поллитра|пол-литра|пол литра")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(0.5, VolumeUnit::Litre)))
            }),
        },
        Rule {
            name: "one liter word".to_string(),
            pattern: vec![regex("один литр")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(1.0, VolumeUnit::Litre)))
            }),
        },
        Rule {
            name: "two liters word".to_string(),
            pattern: vec![regex("два литра")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(2.0, VolumeUnit::Litre)))
            }),
        },
        Rule {
            name: "one and a half liter".to_string(),
            pattern: vec![regex("полтора литра")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(1.5, VolumeUnit::Litre)))
            }),
        },
    ]
}
