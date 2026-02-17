use crate::dimensions::numeral::helpers::{is_positive, numeral_data};
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
                crate::pattern::predicate(is_positive),
                regex("(de )?livr(a|e|ă)"),
            ],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Quantity(QuantityData::new(
                    n.value,
                    QuantityUnit::Pound,
                )))
            }),
        },
        Rule {
            name: "<quantity> of product".to_string(),
            pattern: vec![
                dim(DimensionKind::Quantity),
                regex("de (carne|can[aă]|zah[aă]r|mamaliga)"),
            ],
            production: Box::new(|nodes| {
                let mut q = quantity_data(&nodes[0].token_data)?.clone();
                let product = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                q.product = Some(product.to_lowercase());
                Some(TokenData::Quantity(q))
            }),
        },
    ]
}
