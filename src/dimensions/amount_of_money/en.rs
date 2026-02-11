use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

pub fn rules() -> Vec<Rule> {
    vec![
        // $<number>
        Rule {
            name: "$<number>".to_string(),
            pattern: vec![regex(r#"\$\s?(\d+(?:\.\d+)?)"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let val: f64 = text.parse().ok()?;
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::new(
                    val,
                    Currency::Dollar,
                )))
            }),
        },
        // <number> dollars
        Rule {
            name: "<number> dollars".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"dollars?|bucks?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::new(
                    data.value,
                    Currency::Dollar,
                )))
            }),
        },
        // <number> cents
        Rule {
            name: "<number> cents".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"cents?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::new(
                    data.value,
                    Currency::Cent,
                )))
            }),
        },
        // EUR: €<number>
        Rule {
            name: "€<number>".to_string(),
            pattern: vec![regex(r#"€\s?(\d+(?:\.\d+)?)"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let val: f64 = text.parse().ok()?;
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::new(
                    val,
                    Currency::Euro,
                )))
            }),
        },
        // <number> euros
        Rule {
            name: "<number> euros".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"euros?"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::new(
                    data.value,
                    Currency::Euro,
                )))
            }),
        },
        // GBP: £<number>
        Rule {
            name: "£<number>".to_string(),
            pattern: vec![regex(r#"£\s?(\d+(?:\.\d+)?)"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let val: f64 = text.parse().ok()?;
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::new(
                    val,
                    Currency::Pound,
                )))
            }),
        },
        // <number> pounds (money)
        Rule {
            name: "<number> pounds (money)".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"pounds?\s*sterling|GBP"#),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::new(
                    data.value,
                    Currency::Pound,
                )))
            }),
        },
        // ¥<number>
        Rule {
            name: "¥<number>".to_string(),
            pattern: vec![regex(r#"¥\s?(\d+(?:\.\d+)?)"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let val: f64 = text.parse().ok()?;
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::new(
                    val,
                    Currency::Yen,
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
    fn test_money() {
        let mut rules = numeral::en::rules();
        rules.extend(super::rules());
        let options = Options { with_latent: false };
        let context = Context::default();

        for (text, expected_val, expected_unit) in &[
            ("$10", 10.0, "USD"),
            ("$3.50", 3.5, "USD"),
            ("€20", 20.0, "EUR"),
            ("£100", 100.0, "GBP"),
        ] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::AmountOfMoney],
            );
            let found = entities.iter().any(|e| {
                e.dim == "amount-of-money"
                    && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(*expected_val)
                    && e.value.value.get("unit").and_then(|v| v.as_str()) == Some(*expected_unit)
            });
            assert!(found, "Expected {} {} for '{}', got: {:?}", expected_val, expected_unit, text, entities);
        }
    }
}
