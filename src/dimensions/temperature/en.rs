use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{TemperatureData, TemperatureUnit};

pub fn rules() -> Vec<Rule> {
    vec![
        // <number> degrees
        Rule {
            name: "<number> degrees".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"(deg(ree)?s?\.?|°)"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    TemperatureData::new(data.value).with_unit(TemperatureUnit::Degree),
                ))
            }),
        },
        // <number> degrees fahrenheit
        Rule {
            name: "<number> degrees fahrenheit".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"(deg(ree)?s?\.?\s*)?f(ahrenheit)?\.?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    TemperatureData::new(data.value).with_unit(TemperatureUnit::Fahrenheit),
                ))
            }),
        },
        // <number> degrees celsius
        Rule {
            name: "<number> degrees celsius".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"(deg(ree)?s?\.?\s*)?c(elsius|entigrade)?\.?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    TemperatureData::new(data.value).with_unit(TemperatureUnit::Celsius),
                ))
            }),
        },
        // <number> degrees (with °F or °C)
        Rule {
            name: "°F / °C".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"°\s*(f|c)"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                let unit_text = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let unit = match unit_text.to_lowercase().as_str() {
                    "f" => TemperatureUnit::Fahrenheit,
                    "c" => TemperatureUnit::Celsius,
                    _ => return None,
                };
                Some(TokenData::Temperature(
                    TemperatureData::new(data.value).with_unit(unit),
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
        let found = entities.iter().any(|e| {
            e.dim == "temperature"
                && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(80.0)
                && e.value.value.get("unit").and_then(|v| v.as_str()) == Some("fahrenheit")
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
        let found = entities.iter().any(|e| {
            e.dim == "temperature"
                && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(3.0)
                && e.value.value.get("unit").and_then(|v| v.as_str()) == Some("celsius")
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
        let found = entities.iter().any(|e| {
            e.dim == "temperature"
                && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(3.0)
        });
        assert!(found, "Expected temperature 3, got: {:?}", entities);
    }
}
