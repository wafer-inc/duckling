use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{QuantityData, QuantityUnit};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<quantity> milligrams".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("(мг|миллиграмм)")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value / 1000.0;
                Some(TokenData::Quantity(QuantityData::new(
                    v,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "<quantity> grams".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("г(рамм?)?")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Quantity(QuantityData::new(
                    v,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "<quantity> kilograms".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("(кг|килограмм?)")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value * 1000.0;
                Some(TokenData::Quantity(QuantityData::new(
                    v,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "<quantity> lb".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("фунт")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Quantity(QuantityData::new(
                    v,
                    QuantityUnit::Pound,
                )))
            }),
        },
        Rule {
            name: "<quantity> oz".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("унц")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Quantity(QuantityData::new(
                    v,
                    QuantityUnit::Ounce,
                )))
            }),
        },
        Rule {
            name: "a milligram".to_string(),
            pattern: vec![regex("(мг|миллиграмм)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    0.001,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "a gram".to_string(),
            pattern: vec![regex("г(рамм?)?")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    1.0,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "a kilogram".to_string(),
            pattern: vec![regex("(кг|килограмм?)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    1000.0,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "a pound".to_string(),
            pattern: vec![regex("фунт")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    1.0,
                    QuantityUnit::Pound,
                )))
            }),
        },
        Rule {
            name: "an ounce".to_string(),
            pattern: vec![regex("унц")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    1.0,
                    QuantityUnit::Ounce,
                )))
            }),
        },
    ]
}
