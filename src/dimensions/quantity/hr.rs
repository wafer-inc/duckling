use crate::dimensions::numeral::helpers::numeral_data;
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
                regex("k(il(o|e|a))?(g(rama?)?)?"),
            ],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Quantity(QuantityData::new(
                    1000.0 * n.value,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "<quantity> product".to_string(),
            pattern: vec![dim(DimensionKind::Quantity), regex("(mes(o|a)|soli?)")],
            production: Box::new(|nodes| {
                let mut q = quantity_data(&nodes[0].token_data)?.clone();
                let p = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                q.product = Some(match p.as_str() {
                    "meso" | "mesa" => "meso".to_string(),
                    "sol" | "soli" => "sol".to_string(),
                    _ => return None,
                });
                Some(TokenData::Quantity(q))
            }),
        },
    ]
}
