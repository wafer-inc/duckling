use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{distance_sum, DistanceData, DistanceUnit};

fn distance_data(token_data: &TokenData) -> Option<&DistanceData> {
    match token_data {
        TokenData::Distance(data) => Some(data),
        _ => None,
    }
}

/// Matches simple Distance tokens (has value and unit, no interval).
fn is_simple_distance() -> crate::types::PatternItem {
    predicate(|td| {
        matches!(td, TokenData::Distance(data)
            if data.value.is_some()
                && data.unit.is_some()
                && data.min_value.is_none()
                && data.max_value.is_none())
    })
}

pub fn rules() -> Vec<Rule> {
    vec![
        // === Common rule: numeral → distance (value only, no unit) ===
        Rule {
            name: "number as distance".to_string(),
            pattern: vec![dim(DimensionKind::Numeral)],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(DistanceData::value_only(data.value)))
            }),
        },
        // === Unit rules (ruleDistances): distance + unit regex → distance with unit ===
        // Imperial units
        Rule {
            name: "miles".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex(r"mi(le(s)?)?")],
            production: Box::new(|nodes| {
                let dd = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(dd.clone().with_unit(DistanceUnit::Mile)))
            }),
        },
        Rule {
            name: "yard".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex(r"y(ar)?ds?")],
            production: Box::new(|nodes| {
                let dd = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(dd.clone().with_unit(DistanceUnit::Yard)))
            }),
        },
        Rule {
            name: "feet".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex(r"('|f(oo|ee)?ts?)")],
            production: Box::new(|nodes| {
                let dd = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(dd.clone().with_unit(DistanceUnit::Foot)))
            }),
        },
        Rule {
            name: "inch".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex(r#"("|''|in(ch(es)?)?)"#),
            ],
            production: Box::new(|nodes| {
                let dd = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(dd.clone().with_unit(DistanceUnit::Inch)))
            }),
        },
        // Metric units
        Rule {
            name: "km".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex(r"k(ilo)?m?(et(er|re))?s?"),
            ],
            production: Box::new(|nodes| {
                let dd = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(dd.clone().with_unit(DistanceUnit::Kilometre)))
            }),
        },
        Rule {
            name: "meters".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex(r"met(er|re)s?"),
            ],
            production: Box::new(|nodes| {
                let dd = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(dd.clone().with_unit(DistanceUnit::Metre)))
            }),
        },
        Rule {
            name: "centimeters".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex(r"cm|centimet(er|re)s?"),
            ],
            production: Box::new(|nodes| {
                let dd = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(dd.clone().with_unit(DistanceUnit::Centimetre)))
            }),
        },
        Rule {
            name: "millimeters".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex(r"mm|millimet(er|re)s?"),
            ],
            production: Box::new(|nodes| {
                let dd = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(dd.clone().with_unit(DistanceUnit::Millimetre)))
            }),
        },
        // Ambiguous "m" (miles or metres)
        Rule {
            name: "m (miles or meters)".to_string(),
            pattern: vec![dim(DimensionKind::Distance), regex(r"m")],
            production: Box::new(|nodes| {
                let dd = distance_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(dd.clone().with_unit(DistanceUnit::M)))
            }),
        },
        // === Composite distance rules ===
        // <distance> ,|and <distance>
        Rule {
            name: "composite <distance> (with ,/and)".to_string(),
            pattern: vec![
                is_simple_distance(),
                regex(r",|and"),
                is_simple_distance(),
            ],
            production: Box::new(|nodes| {
                let d1 = distance_data(&nodes[0].token_data)?;
                let d2 = distance_data(&nodes[2].token_data)?;
                let v1 = d1.value?;
                let v2 = d2.value?;
                let u1 = d1.unit?;
                let u2 = d2.unit?;
                if u1 == u2 || v1 <= 0.0 || v2 <= 0.0 {
                    return None;
                }
                let (value, unit) = distance_sum(v1, u1, v2, u2)?;
                Some(TokenData::Distance(DistanceData::new(value, unit)))
            }),
        },
        // <distance> <distance> (no separator)
        Rule {
            name: "composite <distance>".to_string(),
            pattern: vec![is_simple_distance(), is_simple_distance()],
            production: Box::new(|nodes| {
                let d1 = distance_data(&nodes[0].token_data)?;
                let d2 = distance_data(&nodes[1].token_data)?;
                let v1 = d1.value?;
                let v2 = d2.value?;
                let u1 = d1.unit?;
                let u2 = d2.unit?;
                if u1 == u2 || v1 <= 0.0 || v2 <= 0.0 {
                    return None;
                }
                let (value, unit) = distance_sum(v1, u1, v2, u2)?;
                Some(TokenData::Distance(DistanceData::new(value, unit)))
            }),
        },
        // === Precision ===
        Rule {
            name: "about <distance>".to_string(),
            pattern: vec![
                regex(r"exactly|precisely|about|approx(\.?|imately)?|close to|near( to)?|around|almost"),
                dim(DimensionKind::Distance),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        // === Interval rules ===
        // between|from <numeral> and|to <distance>
        Rule {
            name: "between|from <numeral> and|to <distance>".to_string(),
            pattern: vec![
                regex(r"between|from"),
                dim(DimensionKind::Numeral),
                regex(r"to|and"),
                is_simple_distance(),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[1].token_data)?;
                let dd = distance_data(&nodes[3].token_data)?;
                let from = num.value;
                let to = dd.value?;
                let unit = dd.unit?;
                if from >= to {
                    return None;
                }
                Some(TokenData::Distance(
                    DistanceData::unit_only(unit).with_interval(from, to),
                ))
            }),
        },
        // between|from <distance> and|to <distance>
        Rule {
            name: "between|from <distance> and|to <distance>".to_string(),
            pattern: vec![
                regex(r"between|from"),
                is_simple_distance(),
                regex(r"to|and"),
                is_simple_distance(),
            ],
            production: Box::new(|nodes| {
                let from_d = distance_data(&nodes[1].token_data)?;
                let to_d = distance_data(&nodes[3].token_data)?;
                let from = from_d.value?;
                let to = to_d.value?;
                let u1 = from_d.unit?;
                let u2 = to_d.unit?;
                if from >= to || u1 != u2 {
                    return None;
                }
                Some(TokenData::Distance(
                    DistanceData::unit_only(u1).with_interval(from, to),
                ))
            }),
        },
        // <numeral> - <distance>
        Rule {
            name: "<numeral> - <distance>".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r"\-"),
                is_simple_distance(),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let dd = distance_data(&nodes[2].token_data)?;
                let from = num.value;
                let to = dd.value?;
                let unit = dd.unit?;
                if from >= to || from <= 0.0 {
                    return None;
                }
                Some(TokenData::Distance(
                    DistanceData::unit_only(unit).with_interval(from, to),
                ))
            }),
        },
        // <distance> - <distance>
        Rule {
            name: "<distance> - <distance>".to_string(),
            pattern: vec![
                is_simple_distance(),
                regex(r"\-"),
                is_simple_distance(),
            ],
            production: Box::new(|nodes| {
                let from_d = distance_data(&nodes[0].token_data)?;
                let to_d = distance_data(&nodes[2].token_data)?;
                let from = from_d.value?;
                let to = to_d.value?;
                let u1 = from_d.unit?;
                let u2 = to_d.unit?;
                if from >= to || u1 != u2 {
                    return None;
                }
                Some(TokenData::Distance(
                    DistanceData::unit_only(u1).with_interval(from, to),
                ))
            }),
        },
        // under/less/lower than <distance>
        Rule {
            name: "under <distance>".to_string(),
            pattern: vec![
                regex(r"under|(less|lower|not? more) than"),
                is_simple_distance(),
            ],
            production: Box::new(|nodes| {
                let dd = distance_data(&nodes[1].token_data)?;
                let to = dd.value?;
                let unit = dd.unit?;
                Some(TokenData::Distance(
                    DistanceData::unit_only(unit).with_max(to),
                ))
            }),
        },
        // over/above/at least/more than <distance>
        Rule {
            name: "over <distance>".to_string(),
            pattern: vec![
                regex(r"over|above|at least|more than"),
                is_simple_distance(),
            ],
            production: Box::new(|nodes| {
                let dd = distance_data(&nodes[1].token_data)?;
                let from = dd.value?;
                let unit = dd.unit?;
                Some(TokenData::Distance(
                    DistanceData::unit_only(unit).with_min(from),
                ))
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use crate::dimensions::numeral;
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::types::DimensionKind;

    #[test]
    fn test_distance() {
        let mut rules = numeral::en::rules();
        rules.extend(super::rules());
        let options = Options { with_latent: false };
        let context = Context::default();

        for (text, expected_val, expected_unit) in &[
            ("3 miles", 3.0, "mile"),
            ("5 km", 5.0, "kilometre"),
            ("10 feet", 10.0, "foot"),
            ("2 inches", 2.0, "inch"),
        ] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::Distance],
            );
            let found = entities.iter().any(|e| {
                match &e.value {
                    crate::types::DimensionValue::Distance(crate::types::MeasurementValue::Value { value, unit }) => {
                        (*value - *expected_val).abs() < 0.01 && unit == *expected_unit
                    }
                    _ => false,
                }
            });
            assert!(
                found,
                "Expected {} {} for '{}', got: {:?}",
                expected_val, expected_unit, text, entities
            );
        }
    }
}
