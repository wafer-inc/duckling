use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{VolumeData, VolumeUnit};

fn volume_data(token_data: &TokenData) -> Option<&VolumeData> {
    match token_data {
        TokenData::Volume(data) => Some(data),
        _ => None,
    }
}

/// Matches Volume tokens with unit only (no value, no interval).
fn is_unit_only() -> crate::types::PatternItem {
    predicate(|td| {
        matches!(td, TokenData::Volume(data)
        if data.value.is_none()
            && data.unit.is_some()
            && data.min_value.is_none()
            && data.max_value.is_none())
    })
}

/// Matches Volume tokens with a simple value (value + unit, no interval).
fn is_simple_volume() -> crate::types::PatternItem {
    predicate(|td| {
        matches!(td, TokenData::Volume(data)
        if data.value.is_some()
            && data.unit.is_some()
            && data.min_value.is_none()
            && data.max_value.is_none())
    })
}

pub fn rules() -> Vec<Rule> {
    vec![
        // === Common rules (from Volume/Rules.hs) ===

        // number as volume (latent, value only, no unit)
        Rule {
            name: "number as volume".to_string(),
            pattern: vec![dim(DimensionKind::Numeral)],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                if data.value <= 0.0 {
                    return None;
                }
                Some(TokenData::Volume(VolumeData::value_only(data.value)))
            }),
        },
        // <number> <volume> (numeral + unit-only → volume with value and unit)
        Rule {
            name: "<number> <volume>".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), is_unit_only()],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                let vol = volume_data(&nodes[1].token_data)?;
                let unit = vol.unit?;
                if data.value <= 0.0 {
                    return None;
                }
                Some(TokenData::Volume(VolumeData::new(data.value, unit)))
            }),
        },
        // <numeral> - <volume> (common interval dash with numeral)
        Rule {
            name: "<numeral> - <volume>".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"\-"#),
                is_simple_volume(),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let vol = volume_data(&nodes[2].token_data)?;
                let to = vol.value?;
                let unit = vol.unit?;
                let from = num.value;
                if from >= to || from <= 0.0 {
                    return None;
                }
                Some(TokenData::Volume(
                    VolumeData::unit_only(unit).with_interval(from, to),
                ))
            }),
        },
        // <volume> - <volume> (common interval dash)
        Rule {
            name: "<volume> - <volume>".to_string(),
            pattern: vec![is_simple_volume(), regex(r#"\-"#), is_simple_volume()],
            production: Box::new(|nodes| {
                let from_data = volume_data(&nodes[0].token_data)?;
                let to_data = volume_data(&nodes[2].token_data)?;
                let from = from_data.value?;
                let to = to_data.value?;
                let u1 = from_data.unit?;
                let u2 = to_data.unit?;
                if from >= to || u1 != u2 {
                    return None;
                }
                Some(TokenData::Volume(
                    VolumeData::unit_only(u1).with_interval(from, to),
                ))
            }),
        },
        // === Unit-only rules (from Volume/EN/Rules.hs rulesVolumes) ===
        Rule {
            name: "<latent vol> ml".to_string(),
            pattern: vec![regex(r#"m(l(s?)|illilit(er|re)s?)"#)],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Millilitre,
                )))
            }),
        },
        Rule {
            name: "<vol> hectoliters".to_string(),
            pattern: vec![regex(r#"hectolit(er|re)s?"#)],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Hectolitre,
                )))
            }),
        },
        Rule {
            name: "<vol> liters".to_string(),
            pattern: vec![regex(r#"l(it(er|re)s?)?"#)],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Litre)))
            }),
        },
        Rule {
            name: "<latent vol> gallon".to_string(),
            pattern: vec![regex(r#"gal((l?ons?)|s)?"#)],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Gallon)))
            }),
        },
        Rule {
            name: "<vol> cups".to_string(),
            pattern: vec![regex(r#"cups?"#)],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Cup)))
            }),
        },
        Rule {
            name: "<vol> pints".to_string(),
            pattern: vec![regex(r#"pints?"#)],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Pint)))
            }),
        },
        Rule {
            name: "<vol> quarts".to_string(),
            pattern: vec![regex(r#"quarts?"#)],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(VolumeUnit::Quart)))
            }),
        },
        Rule {
            name: "<vol> tablespoons".to_string(),
            pattern: vec![regex(r#"tbsps?|tablespoons?"#)],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Tablespoon,
                )))
            }),
        },
        Rule {
            name: "<vol> teaspoons".to_string(),
            pattern: vec![regex(r#"tsps?|teaspoons?"#)],
            production: Box::new(|_| {
                Some(TokenData::Volume(VolumeData::unit_only(
                    VolumeUnit::Teaspoon,
                )))
            }),
        },
        // === Fractional volume rules (from Volume/EN/Rules.hs rulesFractionalVolume) ===

        // "a/an <unit>" → 1.0
        Rule {
            name: "one <volume>".to_string(),
            pattern: vec![regex(r#"an? "#), is_unit_only()],
            production: Box::new(|nodes| {
                let vol = volume_data(&nodes[1].token_data)?;
                let unit = vol.unit?;
                Some(TokenData::Volume(VolumeData::new(1.0, unit)))
            }),
        },
        // "half <unit>" → 0.5
        Rule {
            name: "half <volume>".to_string(),
            pattern: vec![regex(r#"half(-|(( of)?( a(n?))?))"#), is_unit_only()],
            production: Box::new(|nodes| {
                let vol = volume_data(&nodes[1].token_data)?;
                let unit = vol.unit?;
                Some(TokenData::Volume(VolumeData::new(0.5, unit)))
            }),
        },
        // "third <unit>" → 1/3
        Rule {
            name: "third <volume>".to_string(),
            pattern: vec![regex(r#"third(-|(( of)?( a(n?))?))"#), is_unit_only()],
            production: Box::new(|nodes| {
                let vol = volume_data(&nodes[1].token_data)?;
                let unit = vol.unit?;
                Some(TokenData::Volume(VolumeData::new(1.0 / 3.0, unit)))
            }),
        },
        // "quarter/fourth <unit>" → 0.25
        Rule {
            name: "fourth <volume>".to_string(),
            pattern: vec![
                regex(r#"(quarter|fourth)(-|(( of)?( a(n?))?))"#),
                is_unit_only(),
            ],
            production: Box::new(|nodes| {
                let vol = volume_data(&nodes[1].token_data)?;
                let unit = vol.unit?;
                Some(TokenData::Volume(VolumeData::new(0.25, unit)))
            }),
        },
        // "fifth <unit>" → 0.2
        Rule {
            name: "fifth <volume>".to_string(),
            pattern: vec![regex(r#"fifth(-|(( of)?( a(n?))?))"#), is_unit_only()],
            production: Box::new(|nodes| {
                let vol = volume_data(&nodes[1].token_data)?;
                let unit = vol.unit?;
                Some(TokenData::Volume(VolumeData::new(0.2, unit)))
            }),
        },
        // "tenth <unit>" → 0.1
        Rule {
            name: "tenth <volume>".to_string(),
            pattern: vec![regex(r#"tenth(-|(( of)?( a(n?))?))"#), is_unit_only()],
            production: Box::new(|nodes| {
                let vol = volume_data(&nodes[1].token_data)?;
                let unit = vol.unit?;
                Some(TokenData::Volume(VolumeData::new(0.1, unit)))
            }),
        },
        // === EN-specific rules (from Volume/EN/Rules.hs) ===

        // about/approximately <volume>
        Rule {
            name: "about <volume>".to_string(),
            pattern: vec![
                regex(
                    r#"~|exactly|precisely|about|approx(\.?|imately)?|close to|near( to)?|around|almost"#,
                ),
                predicate(
                    |td| matches!(td, TokenData::Volume(data) if data.value.is_some() || data.min_value.is_some() || data.max_value.is_some()),
                ),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        // between|from <numeral> and|to <volume>
        Rule {
            name: "between|from <numeral> and|to <volume>".to_string(),
            pattern: vec![
                regex(r#"between|from"#),
                dim(DimensionKind::Numeral),
                regex(r#"to|and"#),
                is_simple_volume(),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[1].token_data)?;
                let vol = volume_data(&nodes[3].token_data)?;
                let from = num.value;
                let to = vol.value?;
                let unit = vol.unit?;
                if from >= to {
                    return None;
                }
                Some(TokenData::Volume(
                    VolumeData::unit_only(unit).with_interval(from, to),
                ))
            }),
        },
        // between|from <volume> and|to <volume>
        Rule {
            name: "between|from <volume> and|to <volume>".to_string(),
            pattern: vec![
                regex(r#"between|from"#),
                is_simple_volume(),
                regex(r#"to|and"#),
                is_simple_volume(),
            ],
            production: Box::new(|nodes| {
                let from_data = volume_data(&nodes[1].token_data)?;
                let to_data = volume_data(&nodes[3].token_data)?;
                let from = from_data.value?;
                let to = to_data.value?;
                let u1 = from_data.unit?;
                let u2 = to_data.unit?;
                if from >= to || u1 != u2 {
                    return None;
                }
                Some(TokenData::Volume(
                    VolumeData::unit_only(u1).with_interval(from, to),
                ))
            }),
        },
        // at most / under / below / less than <volume>
        Rule {
            name: "at most <volume>".to_string(),
            pattern: vec![
                regex(r#"under|below|at most|(less|lower|not? more) than"#),
                is_simple_volume(),
            ],
            production: Box::new(|nodes| {
                let data = volume_data(&nodes[1].token_data)?;
                let to = data.value?;
                let unit = data.unit?;
                Some(TokenData::Volume(VolumeData::unit_only(unit).with_max(to)))
            }),
        },
        // over / above / at least / more than <volume>
        Rule {
            name: "more than <volume>".to_string(),
            pattern: vec![
                regex(r#"over|above|exceeding|beyond|at least|(more|larger|bigger|heavier) than"#),
                is_simple_volume(),
            ],
            production: Box::new(|nodes| {
                let data = volume_data(&nodes[1].token_data)?;
                let from = data.value?;
                let unit = data.unit?;
                Some(TokenData::Volume(
                    VolumeData::unit_only(unit).with_min(from),
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
    fn test_volume() {
        let mut rules = numeral::en::rules();
        rules.extend(super::rules());
        let options = Options { with_latent: false };
        let context = Context::default();

        for (text, expected_val, expected_unit) in &[
            ("2 gallons", 2.0, "gallon"),
            ("3 litres", 3.0, "litre"),
            ("500 ml", 500.0, "millilitre"),
            ("1 cup", 1.0, "cup"),
        ] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::Volume],
            );
            let found = entities.iter().any(|e| match &e.value {
                crate::types::DimensionValue::Volume(crate::types::MeasurementValue::Value {
                    value,
                    unit,
                }) => (*value - *expected_val).abs() < 0.01 && unit == *expected_unit,
                _ => false,
            });
            assert!(
                found,
                "Expected {} {} for '{}', got: {:?}",
                expected_val, expected_unit, text, entities
            );
        }
    }
}
