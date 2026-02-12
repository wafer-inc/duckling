use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{units_are_compatible, TemperatureData, TemperatureUnit};

fn temperature_data(token_data: &TokenData) -> Option<&TemperatureData> {
    match token_data {
        TokenData::Temperature(data) => Some(data),
        _ => None,
    }
}

/// Matches Temperature tokens with a value, no min/max.
/// If allow_degree is true, also accepts tokens with unit=Degree.
/// If allow_degree is false, only accepts tokens with unit=None (latent).
fn is_value_only(allow_degree: bool) -> crate::types::PatternItem {
    predicate(move |td| match td {
        TokenData::Temperature(data) => {
            data.value.is_some()
                && data.min_value.is_none()
                && data.max_value.is_none()
                && (data.unit.is_none()
                    || (allow_degree && data.unit == Some(TemperatureUnit::Degree)))
        }
        _ => false,
    })
}

/// Matches any Temperature token with a value.
fn is_simple_temperature() -> crate::types::PatternItem {
    predicate(|td| matches!(td, TokenData::Temperature(data) if data.value.is_some()))
}

pub fn rules() -> Vec<Rule> {
    vec![
        // number as temp (latent, no unit)
        Rule {
            name: "number as temp".to_string(),
            pattern: vec![dim(DimensionKind::Numeral)],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(TemperatureData::new(data.value)))
            }),
        },
        // <latent temp> degrees
        Rule {
            name: "<latent temp> degrees".to_string(),
            pattern: vec![is_value_only(false), regex(r#"(deg(ree?)?s?\.?)|°"#)],
            production: Box::new(|nodes| {
                let data = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    data.clone().with_unit(TemperatureUnit::Degree),
                ))
            }),
        },
        // <temp> Celsius
        Rule {
            name: "<temp> celsius".to_string(),
            pattern: vec![is_value_only(true), regex(r#"c(el[cs]?(ius)?)?\.?"#)],
            production: Box::new(|nodes| {
                let data = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    data.clone().with_unit(TemperatureUnit::Celsius),
                ))
            }),
        },
        // <temp> Fahrenheit
        Rule {
            name: "<temp> fahrenheit".to_string(),
            pattern: vec![is_value_only(true), regex(r#"f(ah?rh?eh?n(h?eit)?)?\.?"#)],
            production: Box::new(|nodes| {
                let data = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    data.clone().with_unit(TemperatureUnit::Fahrenheit),
                ))
            }),
        },
        // °F / °C
        Rule {
            name: "°F / °C".to_string(),
            pattern: vec![is_value_only(false), regex(r#"°\s*(f|c)"#)],
            production: Box::new(|nodes| {
                let data = temperature_data(&nodes[0].token_data)?;
                let unit_text = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let unit = match unit_text.to_lowercase().as_str() {
                    "f" => TemperatureUnit::Fahrenheit,
                    "c" => TemperatureUnit::Celsius,
                    _ => return None,
                };
                Some(TokenData::Temperature(data.clone().with_unit(unit)))
            }),
        },
        // <temp> below zero
        Rule {
            name: "<temp> below zero".to_string(),
            pattern: vec![is_value_only(true), regex(r#"below zero"#)],
            production: Box::new(|nodes| {
                let data = temperature_data(&nodes[0].token_data)?;
                let v = data.value?;
                let mut result = data.clone();
                result.value = Some(-v);
                if result.unit.is_none() {
                    result.unit = Some(TemperatureUnit::Degree);
                }
                Some(TokenData::Temperature(result))
            }),
        },
        // between|from <temp> and|to <temp>
        Rule {
            name: "between|from <temp> and|to <temp>".to_string(),
            pattern: vec![
                regex(r#"between|from"#),
                is_simple_temperature(),
                regex(r#"to|and"#),
                is_simple_temperature(),
            ],
            production: Box::new(|nodes| {
                let from_data = temperature_data(&nodes[1].token_data)?;
                let to_data = temperature_data(&nodes[3].token_data)?;
                let from = from_data.value?;
                let to = to_data.value?;
                let u2 = to_data.unit?;
                if from >= to || !units_are_compatible(from_data.unit, u2) {
                    return None;
                }
                Some(TokenData::Temperature(
                    TemperatureData::unit_only(u2).with_interval(from, to),
                ))
            }),
        },
        // <temp> - <temp>
        Rule {
            name: "<temp> - <temp>".to_string(),
            pattern: vec![
                is_simple_temperature(),
                regex(r#"\-"#),
                is_simple_temperature(),
            ],
            production: Box::new(|nodes| {
                let from_data = temperature_data(&nodes[0].token_data)?;
                let to_data = temperature_data(&nodes[2].token_data)?;
                let from = from_data.value?;
                let to = to_data.value?;
                let u2 = to_data.unit?;
                if from >= to || !units_are_compatible(from_data.unit, u2) {
                    return None;
                }
                Some(TokenData::Temperature(
                    TemperatureData::unit_only(u2).with_interval(from, to),
                ))
            }),
        },
        // over/above/at least/more than <temp>
        Rule {
            name: "over/above/at least/more than <temp>".to_string(),
            pattern: vec![
                regex(r#"over|above|at least|more than"#),
                is_simple_temperature(),
            ],
            production: Box::new(|nodes| {
                let data = temperature_data(&nodes[1].token_data)?;
                let from = data.value?;
                let u = data.unit?;
                Some(TokenData::Temperature(
                    TemperatureData::unit_only(u).with_min(from),
                ))
            }),
        },
        // under/less/lower/no more than <temp>
        Rule {
            name: "under/less/lower/no more than <temp>".to_string(),
            pattern: vec![
                regex(r#"under|(less|lower|not? more) than"#),
                is_simple_temperature(),
            ],
            production: Box::new(|nodes| {
                let data = temperature_data(&nodes[1].token_data)?;
                let to = data.value?;
                let u = data.unit?;
                Some(TokenData::Temperature(
                    TemperatureData::unit_only(u).with_max(to),
                ))
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimensions::numeral;
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::types::DimensionKind;

    fn all_rules() -> Vec<Rule> {
        let mut r = numeral::en::rules();
        r.extend(rules());
        r
    }

    #[test]
    fn test_temperature() {
        let rules = all_rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        let entities = engine::parse_and_resolve(
            "80 degrees fahrenheit",
            &rules,
            &context,
            &options,
            &[DimensionKind::Temperature],
        );
        let found = entities.iter().any(|e| match &e.value {
            crate::types::DimensionValue::Temperature(crate::types::MeasurementValue::Value {
                value,
                unit,
            }) => (*value - 80.0).abs() < 0.01 && unit == "fahrenheit",
            _ => false,
        });
        assert!(found, "Expected 80F, got: {:?}", entities);
    }

    #[test]
    fn test_temperature_celsius() {
        let rules = all_rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        let entities = engine::parse_and_resolve(
            "3 degrees celsius",
            &rules,
            &context,
            &options,
            &[DimensionKind::Temperature],
        );
        let found = entities.iter().any(|e| match &e.value {
            crate::types::DimensionValue::Temperature(crate::types::MeasurementValue::Value {
                value,
                unit,
            }) => (*value - 3.0).abs() < 0.01 && unit == "celsius",
            _ => false,
        });
        assert!(found, "Expected 3C, got: {:?}", entities);
    }

    #[test]
    fn test_3_degrees() {
        let rules = all_rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        let entities = engine::parse_and_resolve(
            "it's 3 degrees outside",
            &rules,
            &context,
            &options,
            &[DimensionKind::Temperature],
        );
        let found = entities.iter().any(|e| match &e.value {
            crate::types::DimensionValue::Temperature(crate::types::MeasurementValue::Value {
                value,
                ..
            }) => (*value - 3.0).abs() < 0.01,
            _ => false,
        });
        assert!(found, "Expected temperature 3, got: {:?}", entities);
    }

    #[test]
    fn test_below_zero() {
        let rules = all_rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        let entities = engine::parse_and_resolve(
            "2 degrees below zero",
            &rules,
            &context,
            &options,
            &[DimensionKind::Temperature],
        );
        let found = entities.iter().any(|e| match &e.value {
            crate::types::DimensionValue::Temperature(crate::types::MeasurementValue::Value {
                value,
                unit,
            }) => (*value - (-2.0)).abs() < 0.01 && unit == "degree",
            _ => false,
        });
        assert!(found, "Expected -2 degree, got: {:?}", entities);
    }
}
