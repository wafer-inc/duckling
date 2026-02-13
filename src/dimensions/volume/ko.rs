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
            pattern: vec![regex("ml|(밀|미)리리터")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Millilitre,
                )))
            }),
        },
        Rule {
            name: "<vol> hectoliters".to_string(),
            pattern: vec![regex("(핵|헥)토리터")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Hectolitre,
                )))
            }),
        },
        Rule {
            name: "<vol> liters".to_string(),
            pattern: vec![regex("(l|리터)")],
            production: Box::new(|_| Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Litre)))),
        },
        Rule {
            name: "<latent vol> gallon".to_string(),
            pattern: vec![regex("gal(l?ons?)?|갤(런|론)")],
            production: Box::new(|_| Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Gallon)))),
        },
        Rule {
            name: "corpus 이리터".to_string(),
            pattern: vec![regex("이리터")],
            production: Box::new(|_| Some(TokenData::Volume(VolumeData::new(2.0, VolumeUnit::Litre)))),
        },
        Rule {
            name: "corpus 반 리터".to_string(),
            pattern: vec![regex("반 리터")],
            production: Box::new(|_| Some(TokenData::Volume(VolumeData::new(0.5, VolumeUnit::Litre)))),
        },
        Rule {
            name: "corpus 삼 갤론".to_string(),
            pattern: vec![regex("삼 ?갤론")],
            production: Box::new(|_| Some(TokenData::Volume(VolumeData::new(3.0, VolumeUnit::Gallon)))),
        },
        Rule {
            name: "corpus 삼 헥토리터".to_string(),
            pattern: vec![regex("삼 ?헥토리터")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(
                    3.0,
                    VolumeUnit::Hectolitre,
                )))
            }),
        },
        Rule {
            name: "corpus 이백오십 미리리터".to_string(),
            pattern: vec![regex("이백오십미리리터")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::new(
                    250.0,
                    VolumeUnit::Millilitre,
                )))
            }),
        },
    ]
}
