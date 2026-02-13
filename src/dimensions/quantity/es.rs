use crate::dimensions::numeral::helpers::{is_positive, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{QuantityData, QuantityUnit};

fn quantity_data(td: &TokenData) -> Option<&QuantityData> {
    match td {
        TokenData::Quantity(d) => Some(d),
        _ => None,
    }
}

fn is_simple_quantity(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Quantity(d)
            if d.value.is_some()
                && d.unit.is_some()
                && d.min_value.is_none()
                && d.max_value.is_none()
                && d.product.is_none()
    )
}

fn gram_value(matched: &str, value: f64) -> f64 {
    let m = matched.to_lowercase();
    if m.contains("mili") || m.contains("mg") {
        value / 1000.0
    } else if m.contains("kilo") || m.contains("kg") {
        value * 1000.0
    } else {
        value
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<quantity> cups".to_string(),
            pattern: vec![predicate(is_positive), regex("(tazas?)")],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Quantity(QuantityData::new(n.value, QuantityUnit::Cup)))
            }),
        },
        Rule {
            name: "<quantity> grams".to_string(),
            pattern: vec![predicate(is_positive), regex("(((m(ili)?)|(k(ilo)?))?g(ramo)?s?)")],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?;
                let unit_text = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    gram_value(unit_text, n.value),
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "<quantity> pounds".to_string(),
            pattern: vec![predicate(is_positive), regex("((lb|libra)s?)")],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Quantity(QuantityData::new(
                    n.value,
                    QuantityUnit::Pound,
                )))
            }),
        },
        Rule {
            name: "<quantity> ounces".to_string(),
            pattern: vec![predicate(is_positive), regex("((onzas?)|oz)")],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Quantity(QuantityData::new(
                    n.value,
                    QuantityUnit::Ounce,
                )))
            }),
        },
        Rule {
            name: "a <quantity> cups".to_string(),
            pattern: vec![regex("una? (tazas?)")],
            production: Box::new(|_| Some(TokenData::Quantity(QuantityData::new(1.0, QuantityUnit::Cup)))),
        },
        Rule {
            name: "a <quantity> grams".to_string(),
            pattern: vec![regex("una? (((m(ili)?)|(k(ilo)?))?g(ramo)?s?)")],
            production: Box::new(|nodes| {
                let unit_text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    gram_value(unit_text, 1.0),
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "a <quantity> pounds".to_string(),
            pattern: vec![regex("una? ((lb|libra)s?)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    1.0,
                    QuantityUnit::Pound,
                )))
            }),
        },
        Rule {
            name: "<quantity> de producto".to_string(),
            pattern: vec![dim(DimensionKind::Quantity), regex("de (\\w+)")],
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
        Rule {
            name: "under <quantity>".to_string(),
            pattern: vec![
                regex("no m(á|a)s que|menos de|por debajo de|como mucho|como m(á|a)xim(o|a)|a lo sumo|menos (que|de)"),
                predicate(is_simple_quantity),
            ],
            production: Box::new(|nodes| {
                let q = quantity_data(&nodes[1].token_data)?;
                Some(TokenData::Quantity(
                    QuantityData::unit_only(q.unit?).with_max(q.value?),
                ))
            }),
        },
        Rule {
            name: "over <quantity>".to_string(),
            pattern: vec![
                regex("m(á|a)s( grande| pesado)? (de|que)|mayor de|por encima de|excesivo|fuera de|por lo menos|como m(í|i)nim(o|a)|al menos"),
                predicate(is_simple_quantity),
            ],
            production: Box::new(|nodes| {
                let q = quantity_data(&nodes[1].token_data)?;
                Some(TokenData::Quantity(
                    QuantityData::unit_only(q.unit?).with_min(q.value?),
                ))
            }),
        },
    ]
}
