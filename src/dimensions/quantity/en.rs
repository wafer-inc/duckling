use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::QuantityData;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<number> pounds (weight)".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"(lb|pound)s?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Quantity(QuantityData::new(data.value, "pound")))
            }),
        },
        Rule {
            name: "<number> ounces".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"(oz|ounce)s?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Quantity(QuantityData::new(data.value, "ounce")))
            }),
        },
        Rule {
            name: "<number> grams".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"g(ram)?s?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Quantity(QuantityData::new(data.value, "gram")))
            }),
        },
        Rule {
            name: "<number> kilograms".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"k(ilo)?g(ram)?s?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Quantity(QuantityData::new(data.value, "kilogram")))
            }),
        },
        Rule {
            name: "<number> cups of product".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"cups?\s+of\s+(\w+)"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                let product = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_string(),
                    _ => return None,
                };
                Some(TokenData::Quantity(
                    QuantityData::new(data.value, "cup").with_product(&product),
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
    fn test_quantity() {
        let mut rules = numeral::en::rules();
        rules.extend(super::rules());
        let options = Options { with_latent: false };
        let context = Context::default();

        let entities = engine::parse_and_resolve(
            "5 pounds",
            &rules,
            &context,
            &options,
            &[DimensionKind::Quantity],
        );
        let found = entities.iter().any(|e| {
            e.dim == "quantity"
                && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(5.0)
                && e.value.value.get("unit").and_then(|v| v.as_str()) == Some("pound")
        });
        assert!(found, "Expected 5 pounds, got: {:?}", entities);
    }
}
