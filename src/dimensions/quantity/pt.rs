use crate::dimensions::numeral::helpers::is_positive;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{QuantityData, QuantityUnit};

fn quantity_data(td: &TokenData) -> Option<&QuantityData> {
    match td {
        TokenData::Quantity(d) => Some(d),
        _ => None,
    }
}

fn quantity_value_with_unit_transform(unit_text: &str, value: f64) -> f64 {
    match unit_text.to_lowercase().as_str() {
        "miligrama" | "miligramas" | "mg" | "mgs" => value / 1000.0,
        "quilograma" | "quilogramas" | "quilo" | "quilos" | "kg" | "kgs" => value * 1000.0,
        _ => value,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<quantity> copos".to_string(),
            pattern: vec![predicate(is_positive), regex("(copos?)")],
            production: Box::new(|nodes| {
                let v = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(v, QuantityUnit::Cup)))
            }),
        },
        Rule {
            name: "<quantity> gramas".to_string(),
            pattern: vec![
                predicate(is_positive),
                regex("((((mili)|(quilo))?(grama)s?)|(quilos?)|((m|k)?g))"),
            ],
            production: Box::new(|nodes| {
                let v = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value,
                    _ => return None,
                };
                let m = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    quantity_value_with_unit_transform(m, v),
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "<quantity> libras".to_string(),
            pattern: vec![predicate(is_positive), regex("((lb|libra)s?)")],
            production: Box::new(|nodes| {
                let v = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(v, QuantityUnit::Pound)))
            }),
        },
        Rule {
            name: "<quantity> of product".to_string(),
            pattern: vec![dim(DimensionKind::Quantity), regex("de (\\w+)")],
            production: Box::new(|nodes| {
                let mut q = quantity_data(&nodes[0].token_data)?.clone();
                let p = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                q.product = Some(p.to_lowercase());
                Some(TokenData::Quantity(q))
            }),
        },
    ]
}
