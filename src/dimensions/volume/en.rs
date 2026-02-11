use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{VolumeData, VolumeUnit};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<number> gallons".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"gal(lon)?s?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Volume(VolumeData::new(data.value, VolumeUnit::Gallon)))
            }),
        },
        Rule {
            name: "<number> litres".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"l(i|ı)t(er|re)?s?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Volume(VolumeData::new(data.value, VolumeUnit::Litre)))
            }),
        },
        Rule {
            name: "<number> millilitres".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"ml|millil(i|ı)t(er|re)?s?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Volume(VolumeData::new(data.value, VolumeUnit::Millilitre)))
            }),
        },
        Rule {
            name: "<number> cups".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"cups?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Volume(VolumeData::new(data.value, VolumeUnit::Cup)))
            }),
        },
        Rule {
            name: "<number> pints".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"pints?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Volume(VolumeData::new(data.value, VolumeUnit::Pint)))
            }),
        },
        Rule {
            name: "<number> quarts".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"quarts?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Volume(VolumeData::new(data.value, VolumeUnit::Quart)))
            }),
        },
        Rule {
            name: "<number> tablespoons".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"tbsps?|tablespoons?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Volume(VolumeData::new(data.value, VolumeUnit::Tablespoon)))
            }),
        },
        Rule {
            name: "<number> teaspoons".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"tsps?|teaspoons?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Volume(VolumeData::new(data.value, VolumeUnit::Teaspoon)))
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
                text, &rules, &context, &options,
                &[DimensionKind::Volume],
            );
            let found = entities.iter().any(|e| {
                e.dim == "volume"
                    && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(*expected_val)
                    && e.value.value.get("unit").and_then(|v| v.as_str()) == Some(*expected_unit)
            });
            assert!(found, "Expected {} {} for '{}', got: {:?}", expected_val, expected_unit, text, entities);
        }
    }
}
