use crate::dimensions::numeral::helpers::{is_positive, numeral_data};
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
            name: "kiloliter".to_string(),
            pattern: vec![predicate(is_positive), regex("(kl\\.?|g?ch?illil(í|i)t(ea|i)r)")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Volume(VolumeData::new(
                    10.0 * v,
                    VolumeUnit::Hectolitre,
                )))
            }),
        },
        Rule {
            name: "<latent vol> ml".to_string(),
            pattern: vec![regex("m(l\\.?|h?illil(í|i)t(ea|i)r)")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Millilitre,
                )))
            }),
        },
        Rule {
            name: "<vol> hectoliters".to_string(),
            pattern: vec![regex("heictil(í|i)t(ea|i)r")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Hectolitre,
                )))
            }),
        },
        Rule {
            name: "<vol> liters".to_string(),
            pattern: vec![regex("(l(í|i)t(ea|i)r|l\\.?)")],
            production: Box::new(|_| Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Litre)))),
        },
        Rule {
            name: "<latent vol> gallon".to_string(),
            pattern: vec![regex("n?gh?al(ú|u)i?n")],
            production: Box::new(|_| Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Gallon)))),
        },
    ]
}
