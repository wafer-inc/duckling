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
            pattern: vec![regex("مي?لي?( ?لي?تي?ر)?|مل")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Millilitre,
                )))
            }),
        },
        Rule {
            name: "<vol> hectoliters".to_string(),
            pattern: vec![regex("هي?كتو ?لي?تر")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Hectolitre,
                )))
            }),
        },
        Rule {
            name: "<vol> liters".to_string(),
            pattern: vec![regex("لي?تي?ر(ات)?")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Litre)))
            }),
        },
        Rule {
            name: "<latent vol> gallon".to_string(),
            pattern: vec![regex("[جغق]الون(ين|ان|ات)?")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Gallon)))
            }),
        },
        Rule {
            name: "quarter liter".to_string(),
            pattern: vec![regex("ربع لي?تي?ر")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(0.25, VolumeUnit::Litre)))
            }),
        },
        Rule {
            name: "half liter".to_string(),
            pattern: vec![regex("نصف? لي?تي?ر")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(0.5, VolumeUnit::Litre)))
            }),
        },
        Rule {
            name: "two liters".to_string(),
            pattern: vec![regex("لي?تي?ر(ان|ين)")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(2.0, VolumeUnit::Litre)))
            }),
        },
        Rule {
            name: "liter and quarter".to_string(),
            pattern: vec![regex("لي?تي?ر و ?ربع")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(1.25, VolumeUnit::Litre)))
            }),
        },
        Rule {
            name: "liter and half".to_string(),
            pattern: vec![regex("لي?تي?ر و ?نصف?")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(1.5, VolumeUnit::Litre)))
            }),
        },
    ]
}
