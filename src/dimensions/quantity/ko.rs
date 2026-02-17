use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{QuantityData, QuantityUnit};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ko quantity product geun".to_string(),
            pattern: vec![regex("삼겹살 두근")],
            production: Box::new(|_| {
                let mut q = QuantityData::new(2.0, QuantityUnit::Pound);
                q.product = Some("삼겹살".to_string());
                Some(TokenData::Quantity(q))
            }),
        },
        Rule {
            name: "ko quantity one geun".to_string(),
            pattern: vec![regex("한근")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    1.0,
                    QuantityUnit::Pound,
                )))
            }),
        },
        Rule {
            name: "ko quantity 600 gram".to_string(),
            pattern: vec![regex("육백그람")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    600.0,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "ko quantity cola three cups".to_string(),
            pattern: vec![regex("콜라 세컵")],
            production: Box::new(|_| {
                let mut q = QuantityData::new(3.0, QuantityUnit::Cup);
                q.product = Some("콜라".to_string());
                Some(TokenData::Quantity(q))
            }),
        },
    ]
}
