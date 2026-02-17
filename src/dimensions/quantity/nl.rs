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
        "milligram" | "mg" => value / 1000.0,
        "kilo" | "kilogram" | "kg" => value * 1000.0,
        "pond" => value * 500.0,
        "ons" => value * 100.0,
        _ => value,
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

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<quantity> kopje".to_string(),
            pattern: vec![predicate(is_positive), regex("(kopjes?)")],
            production: Box::new(|nodes| {
                let v = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(v, QuantityUnit::Cup)))
            }),
        },
        Rule {
            name: "<quantity> grams".to_string(),
            pattern: vec![predicate(is_positive), regex("(g((r)?(am)?)?)")],
            production: Box::new(|nodes| {
                let v = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value,
                    _ => return None,
                };
                let matched = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    quantity_value_with_unit_transform(matched, v),
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "<quantity> milligrams".to_string(),
            pattern: vec![predicate(is_positive), regex("((m(illi)?)(g(ram)?))")],
            production: Box::new(|nodes| {
                let v = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value,
                    _ => return None,
                };
                let matched = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    quantity_value_with_unit_transform(matched, v),
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "<quantity> kilograms".to_string(),
            pattern: vec![predicate(is_positive), regex("((k(ilo)?)(g(ram)?)?)")],
            production: Box::new(|nodes| {
                let v = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value,
                    _ => return None,
                };
                let matched = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    quantity_value_with_unit_transform(matched, v),
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "<quantity> pond".to_string(),
            pattern: vec![predicate(is_positive), regex("(pond(je(s)?)?)")],
            production: Box::new(|nodes| {
                let v = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    v * 500.0,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "<quantity> ons".to_string(),
            pattern: vec![predicate(is_positive), regex("(ons(je(s)?)?)")],
            production: Box::new(|nodes| {
                let v = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    v * 100.0,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "<quantity> product".to_string(),
            pattern: vec![dim(DimensionKind::Quantity), regex("(\\w+)")],
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
        Rule {
            name: "ongeveer|plm|plusminus <quantity>".to_string(),
            pattern: vec![
                regex("\\~|precies|exact|ongeveer|bijna|ongeveer"),
                dim(DimensionKind::Quantity),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "tussen|van <numeral> en|tot <quantity>".to_string(),
            pattern: vec![
                regex("tussen|van"),
                predicate(is_positive),
                regex("tot|en"),
                predicate(is_simple_quantity),
            ],
            production: Box::new(|nodes| {
                let from = match &nodes[1].token_data {
                    TokenData::Numeral(n) => n.value,
                    _ => return None,
                };
                let d = quantity_data(&nodes[3].token_data)?;
                let to = d.value?;
                let u = d.unit?;
                if from >= to {
                    return None;
                }
                Some(TokenData::Quantity(
                    QuantityData::unit_only(u).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "tussen|van <quantity> en|tot <quantity>".to_string(),
            pattern: vec![
                regex("tussen|van"),
                predicate(is_simple_quantity),
                regex("en|tot"),
                predicate(is_simple_quantity),
            ],
            production: Box::new(|nodes| {
                let d1 = quantity_data(&nodes[1].token_data)?;
                let d2 = quantity_data(&nodes[3].token_data)?;
                let from = d1.value?;
                let to = d2.value?;
                if from >= to || d1.unit != d2.unit {
                    return None;
                }
                Some(TokenData::Quantity(
                    QuantityData::unit_only(d1.unit?).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "<numeral> - <quantity>".to_string(),
            pattern: vec![
                predicate(is_positive),
                regex("\\-"),
                predicate(is_simple_quantity),
            ],
            production: Box::new(|nodes| {
                let from = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value,
                    _ => return None,
                };
                let d = quantity_data(&nodes[2].token_data)?;
                let to = d.value?;
                if from >= to {
                    return None;
                }
                Some(TokenData::Quantity(
                    QuantityData::unit_only(d.unit?).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "<quantity> - <quantity>".to_string(),
            pattern: vec![
                predicate(is_simple_quantity),
                regex("\\-"),
                predicate(is_simple_quantity),
            ],
            production: Box::new(|nodes| {
                let d1 = quantity_data(&nodes[0].token_data)?;
                let d2 = quantity_data(&nodes[2].token_data)?;
                let from = d1.value?;
                let to = d2.value?;
                if from >= to || d1.unit != d2.unit {
                    return None;
                }
                Some(TokenData::Quantity(
                    QuantityData::unit_only(d1.unit?).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "minder dan/hoogstens/op zijn hoogst/maximaal/hooguit <quantity>".to_string(),
            pattern: vec![
                regex("minder dan|hoogstens|hooguit|maximaal|op zijn hoogst"),
                predicate(is_simple_quantity),
            ],
            production: Box::new(|nodes| {
                let d = quantity_data(&nodes[1].token_data)?;
                Some(TokenData::Quantity(
                    QuantityData::unit_only(d.unit?).with_max(d.value?),
                ))
            }),
        },
        Rule {
            name: "meer dan/minstens/op zijn minst <quantity>".to_string(),
            pattern: vec![
                regex("meer dan|minstens|minimaal|op zijn minst|minder dan"),
                predicate(is_simple_quantity),
            ],
            production: Box::new(|nodes| {
                let d = quantity_data(&nodes[1].token_data)?;
                Some(TokenData::Quantity(
                    QuantityData::unit_only(d.unit?).with_min(d.value?),
                ))
            }),
        },
        Rule {
            name: "een <quantity> kopje".to_string(),
            pattern: vec![regex("een? (kopjes?)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    1.0,
                    QuantityUnit::Cup,
                )))
            }),
        },
        Rule {
            name: "een <quantity> grams".to_string(),
            pattern: vec![regex("een? (g((r)?(am)?)?)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    quantity_value_with_unit_transform(m, 1.0),
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "een <quantity> milligrams".to_string(),
            pattern: vec![regex("een? ((m(illi)?)(g(ram)?))")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    quantity_value_with_unit_transform(m, 1.0),
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "een <quantity> kilograms".to_string(),
            pattern: vec![regex("een? ((k(ilo)?)(g(ram)?)?)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    quantity_value_with_unit_transform(m, 1.0),
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "een <quantity> pond".to_string(),
            pattern: vec![regex("een? (pond(je(s)?)?)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    500.0,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "een <quantity> ons".to_string(),
            pattern: vec![regex("een? (ons(je(s)?)?)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    100.0,
                    QuantityUnit::Gram,
                )))
            }),
        },
    ]
}
