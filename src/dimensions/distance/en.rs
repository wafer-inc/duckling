use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{DistanceData, DistanceUnit};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<number> miles".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"mi(le)?s?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(DistanceData::new(
                    data.value,
                    DistanceUnit::Mile,
                )))
            }),
        },
        Rule {
            name: "<number> yards".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"y(ar)?ds?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(DistanceData::new(
                    data.value,
                    DistanceUnit::Yard,
                )))
            }),
        },
        Rule {
            name: "<number> feet".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"(f(oo|ee)?t|')"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(DistanceData::new(
                    data.value,
                    DistanceUnit::Foot,
                )))
            }),
        },
        Rule {
            name: "<number> inches".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"(in(ch(es)?)?|")"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(DistanceData::new(
                    data.value,
                    DistanceUnit::Inch,
                )))
            }),
        },
        Rule {
            name: "<number> kilometres".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"k(ilo)?m(eter|etre)?s?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(DistanceData::new(
                    data.value,
                    DistanceUnit::Kilometre,
                )))
            }),
        },
        Rule {
            name: "<number> metres".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"m(eter|etre)?s?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(DistanceData::new(
                    data.value,
                    DistanceUnit::Metre,
                )))
            }),
        },
        Rule {
            name: "<number> centimetres".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"c(enti)?m(eter|etre)?s?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(DistanceData::new(
                    data.value,
                    DistanceUnit::Centimetre,
                )))
            }),
        },
        Rule {
            name: "<number> millimetres".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"mm|millim(eter|etre)?s?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Distance(DistanceData::new(
                    data.value,
                    DistanceUnit::Millimetre,
                )))
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
                e.dim == "distance"
                    && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(*expected_val)
                    && e.value.value.get("unit").and_then(|v| v.as_str()) == Some(*expected_unit)
            });
            assert!(found, "Expected {} {} for '{}', got: {:?}", expected_val, expected_unit, text, entities);
        }
    }
}
