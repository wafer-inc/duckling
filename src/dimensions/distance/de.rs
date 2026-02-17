use crate::dimensions::numeral::helpers::{is_positive, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{DistanceData, DistanceUnit};

fn distance_data(td: &TokenData) -> Option<&DistanceData> {
    match td {
        TokenData::Distance(d) => Some(d),
        _ => None,
    }
}

fn is_simple_distance(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Distance(d)
            if d.value.is_some()
                && d.unit.is_some()
                && d.min_value.is_none()
                && d.max_value.is_none()
    )
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "miles".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("meilen?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Mile)))
            }),
        },
        Rule {
            name: "inch".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("(\"|''|zoll)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(d.clone().with_unit(DistanceUnit::Inch)))
            }),
        },
        Rule {
            name: "km".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("k(ilo)?m(etern?)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Kilometre),
                ))
            }),
        },
        Rule {
            name: "meters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("m(etern?)?")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Metre),
                ))
            }),
        },
        Rule {
            name: "centimeters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("(cm|[zc]entimetern?)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Centimetre),
                ))
            }),
        },
        Rule {
            name: "millimeters".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex("(mm|millimetern?)")],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(
                    d.clone().with_unit(DistanceUnit::Millimetre),
                ))
            }),
        },
        Rule {
            name: "about|exactly <dist>".to_string(),
            pattern: vec![
                regex(
                    "genau|exakt|präzise|ungefähr|(in )?etwa|nahe?( an)?|um( die)?|fast|rund|gut",
                ),
                dim(DimensionKind::Distance),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "between|from <numeral> to|and <dist>".to_string(),
            pattern: vec![
                regex("zwischen|von"),
                predicate(is_positive),
                regex("bis|und"),
                predicate(is_simple_distance),
            ],
            production: Box::new(|nodes| {
                let from = numeral_data(&nodes[1].token_data)?.value;
                let d = distance_data(&nodes[3].token_data)?;
                let to = d.value?;
                let unit = d.unit?;
                if from >= to {
                    return None;
                }
                Some(TokenData::Distance(
                    DistanceData::unit_only(unit).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "between|from <dist> to|and <dist>".to_string(),
            pattern: vec![
                regex("zwischen|von"),
                predicate(is_simple_distance),
                regex("und|bis"),
                predicate(is_simple_distance),
            ],
            production: Box::new(|nodes| {
                let d1 = distance_data(&nodes[1].token_data)?;
                let d2 = distance_data(&nodes[3].token_data)?;
                let from = d1.value?;
                let to = d2.value?;
                if from >= to || d1.unit != d2.unit {
                    return None;
                }
                Some(TokenData::Distance(
                    DistanceData::unit_only(d1.unit?).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "<numeral> - <dist>".to_string(),
            pattern: vec![
                predicate(is_positive),
                regex("-"),
                predicate(is_simple_distance),
            ],
            production: Box::new(|nodes| {
                let from = numeral_data(&nodes[0].token_data)?.value;
                let d = distance_data(&nodes[2].token_data)?;
                let to = d.value?;
                if from >= to {
                    return None;
                }
                Some(TokenData::Distance(
                    DistanceData::unit_only(d.unit?).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "<dist> - <dist>".to_string(),
            pattern: vec![
                predicate(is_simple_distance),
                regex("-"),
                predicate(is_simple_distance),
            ],
            production: Box::new(|nodes| {
                let d1 = distance_data(&nodes[0].token_data)?;
                let d2 = distance_data(&nodes[2].token_data)?;
                let from = d1.value?;
                let to = d2.value?;
                if from >= to || d1.unit != d2.unit {
                    return None;
                }
                Some(TokenData::Distance(
                    DistanceData::unit_only(d1.unit?).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "under/less/lower/no more than <dist>".to_string(),
            pattern: vec![
                regex("unter|höchstens|maximal|(weniger|nicht mehr) als"),
                predicate(is_simple_distance),
            ],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[1].token_data)?;
                Some(TokenData::Distance(
                    DistanceData::unit_only(d.unit?).with_max(d.value?),
                ))
            }),
        },
        Rule {
            name: "over/above/at least/more than <dist>".to_string(),
            pattern: vec![
                regex("über|(mehr|nicht weniger) als|mindestens|wenigstens|minimal"),
                predicate(is_simple_distance),
            ],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[1].token_data)?;
                Some(TokenData::Distance(
                    DistanceData::unit_only(d.unit?).with_min(d.value?),
                ))
            }),
        },
    ]
}
