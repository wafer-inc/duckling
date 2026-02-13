use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{QuantityData, QuantityUnit};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ru quantity numeric grams".to_string(),
            pattern: vec![regex("([0-9]+(?:\\.[0-9]+)?)\\s*(г(рамм(а|ов)?)?|кг|килограмм(а|ов)?|мг|миллиграмм(а|ов)?)")],
            production: Box::new(|nodes| {
                let (v, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.parse::<f64>().ok()?, m.group(2)?),
                    _ => return None,
                };
                let value = if u.contains("кг") || u.contains("килограмм") {
                    v * 1000.0
                } else if u.contains("мг") || u.contains("миллиграмм") {
                    v / 1000.0
                } else {
                    v
                };
                Some(TokenData::Quantity(QuantityData::new(value, QuantityUnit::Gram)))
            }),
        },
        Rule {
            name: "ru quantity numeric pound".to_string(),
            pattern: vec![regex("([0-9]+(?:\\.[0-9]+)?)\\s*фунт(а|ов)?")],
            production: Box::new(|nodes| {
                let v: f64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(v, QuantityUnit::Pound)))
            }),
        },
        Rule {
            name: "ru quantity numeric ounce".to_string(),
            pattern: vec![regex("([0-9]+(?:\\.[0-9]+)?)\\s*унц(ия|ии|ий)")],
            production: Box::new(|nodes| {
                let v: f64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(v, QuantityUnit::Ounce)))
            }),
        },
        Rule {
            name: "ru unit-only kilogram".to_string(),
            pattern: vec![regex("килограмм|кг")],
            production: Box::new(|_| Some(TokenData::Quantity(QuantityData::new(1000.0, QuantityUnit::Gram)))),
        },
        Rule {
            name: "ru unit-only pound".to_string(),
            pattern: vec![regex("фунт")],
            production: Box::new(|_| Some(TokenData::Quantity(QuantityData::new(1.0, QuantityUnit::Pound)))),
        },
        Rule {
            name: "ru words two grams".to_string(),
            pattern: vec![regex("два грамма")],
            production: Box::new(|_| Some(TokenData::Quantity(QuantityData::new(2.0, QuantityUnit::Gram)))),
        },
        Rule {
            name: "ru words five hundred grams".to_string(),
            pattern: vec![regex("пятьсот грамм")],
            production: Box::new(|_| Some(TokenData::Quantity(QuantityData::new(500.0, QuantityUnit::Gram)))),
        },
    ]
}
