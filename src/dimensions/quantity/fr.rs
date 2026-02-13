use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{QuantityData, QuantityUnit};

fn quantity_data(td: &TokenData) -> Option<&QuantityData> {
    match td {
        TokenData::Quantity(d) => Some(d),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<number> <units>".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex("(tasses?|cuill?(e|è)res? (a|à) soupe?)"),
            ],
            production: Box::new(|nodes| {
                let v = match &nodes[0].token_data {
                    TokenData::Numeral(d) => d.value,
                    _ => return None,
                };
                let unit_match = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let unit = match unit_match.as_str() {
                    "tasse" | "tasses" => QuantityUnit::Cup,
                    _ => QuantityUnit::Tablespoon,
                };
                Some(TokenData::Quantity(QuantityData::new(v, unit)))
            }),
        },
        Rule {
            name: "<quantity> of product".to_string(),
            pattern: vec![dim(DimensionKind::Quantity), regex("de (caf(e|é)|sucre)")],
            production: Box::new(|nodes| {
                let mut q = quantity_data(&nodes[0].token_data)?.clone();
                let product = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                q.product = Some(product.to_string());
                Some(TokenData::Quantity(q))
            }),
        },
    ]
}
