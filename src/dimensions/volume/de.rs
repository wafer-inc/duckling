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

fn is_simple_volume(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Volume(d)
            if d.value.is_some()
                && d.unit.is_some()
                && d.min_value.is_none()
                && d.max_value.is_none()
    )
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
            pattern: vec![regex("m(l|illiliter[ns]?)")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Millilitre,
                )))
            }),
        },
        Rule {
            name: "<vol> hectoliters".to_string(),
            pattern: vec![regex("hektoliter[ns]?")],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Hectolitre,
                )))
            }),
        },
        Rule {
            name: "<vol> liters".to_string(),
            pattern: vec![regex("l(iter[ns]?)?")],
            production: Box::new(|_| Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Litre)))),
        },
        Rule {
            name: "one <volume>".to_string(),
            pattern: vec![regex("ein(e[ns])?"), is_unit_only()],
            production: Box::new(|nodes| {
                let d = volume_data(&nodes[1].token_data)?;
                Some(TokenData::Volume(VolumeData::new(1.0, d.unit?)))
            }),
        },
        Rule {
            name: "half <volume>".to_string(),
            pattern: vec![regex("(ein(e[ns])? )?halb(e[rn])? "), is_unit_only()],
            production: Box::new(|nodes| {
                let d = volume_data(&nodes[1].token_data)?;
                Some(TokenData::Volume(VolumeData::new(0.5, d.unit?)))
            }),
        },
        Rule {
            name: "third <volume>".to_string(),
            pattern: vec![regex("(ein(e[ns])? )?dritttel "), is_unit_only()],
            production: Box::new(|nodes| {
                let d = volume_data(&nodes[1].token_data)?;
                Some(TokenData::Volume(VolumeData::new(1.0 / 3.0, d.unit?)))
            }),
        },
        Rule {
            name: "fourth <volume>".to_string(),
            pattern: vec![regex("(ein(e[ns])? )?viertel "), is_unit_only()],
            production: Box::new(|nodes| {
                let d = volume_data(&nodes[1].token_data)?;
                Some(TokenData::Volume(VolumeData::new(0.25, d.unit?)))
            }),
        },
        Rule {
            name: "fifth <volume>".to_string(),
            pattern: vec![regex("(ein(e[ns])? )?fünftel "), is_unit_only()],
            production: Box::new(|nodes| {
                let d = volume_data(&nodes[1].token_data)?;
                Some(TokenData::Volume(VolumeData::new(0.2, d.unit?)))
            }),
        },
        Rule {
            name: "tenth <volume>".to_string(),
            pattern: vec![regex("(ein(e[ns])? )?zehntel "), is_unit_only()],
            production: Box::new(|nodes| {
                let d = volume_data(&nodes[1].token_data)?;
                Some(TokenData::Volume(VolumeData::new(0.1, d.unit?)))
            }),
        },
        Rule {
            name: "about <volume>".to_string(),
            pattern: vec![
                regex("\\~|(ganz )?genau|präzise|(in )?etwa|ungefähr|um( die)?|fast"),
                dim(DimensionKind::Volume),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "between|from <numeral> and|to <volume>".to_string(),
            pattern: vec![
                regex("zwischen|von"),
                predicate(is_positive),
                regex("und|bis( zu)?"),
                predicate(is_simple_volume),
            ],
            production: Box::new(|nodes| {
                let from = numeral_data(&nodes[1].token_data)?.value;
                let d = volume_data(&nodes[3].token_data)?;
                let to = d.value?;
                if from >= to {
                    return None;
                }
                Some(TokenData::Volume(
                    VolumeData::unit_only(d.unit?).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "between|from <volume> to|and <volume>".to_string(),
            pattern: vec![
                regex("zwischen|von"),
                predicate(is_simple_volume),
                regex("bis( zu)?|und"),
                predicate(is_simple_volume),
            ],
            production: Box::new(|nodes| {
                let d1 = volume_data(&nodes[1].token_data)?;
                let d2 = volume_data(&nodes[3].token_data)?;
                let from = d1.value?;
                let to = d2.value?;
                if from >= to || d1.unit != d2.unit {
                    return None;
                }
                Some(TokenData::Volume(
                    VolumeData::unit_only(d1.unit?).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "at most <volume>".to_string(),
            pattern: vec![
                regex("unter|weniger( als)?|höchstens|nicht mehr als"),
                predicate(is_simple_volume),
            ],
            production: Box::new(|nodes| {
                let d = volume_data(&nodes[1].token_data)?;
                Some(TokenData::Volume(
                    VolumeData::unit_only(d.unit?).with_max(d.value?),
                ))
            }),
        },
        Rule {
            name: "more than <volume>".to_string(),
            pattern: vec![
                regex("über|mindestens|wenigstens|mehr als|größer( als)?"),
                predicate(is_simple_volume),
            ],
            production: Box::new(|nodes| {
                let d = volume_data(&nodes[1].token_data)?;
                Some(TokenData::Volume(
                    VolumeData::unit_only(d.unit?).with_min(d.value?),
                ))
            }),
        },
    ]
}
