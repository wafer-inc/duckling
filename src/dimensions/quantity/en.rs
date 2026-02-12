use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{QuantityData, QuantityUnit};

fn quantity_data(token_data: &TokenData) -> Option<&QuantityData> {
    match token_data {
        TokenData::Quantity(data) => Some(data),
        _ => None,
    }
}

/// Get the gram multiplier based on the matched unit text.
/// "kg"/"kilogram" → *1000, "mg"/"milligram" → /1000, "g"/"gram" → *1
fn gram_multiplier(matched: &str) -> f64 {
    match matched.chars().next().unwrap_or('g') {
        'm' | 'M' => 0.001,
        'k' | 'K' => 1000.0,
        _ => 1.0,
    }
}

/// Matches simple Quantity tokens (has value and unit, no interval).
fn is_simple_quantity() -> crate::types::PatternItem {
    predicate(|td| {
        matches!(td, TokenData::Quantity(data)
        if data.value.is_some()
            && data.unit.is_some()
            && data.min_value.is_none()
            && data.max_value.is_none())
    })
}

pub fn rules() -> Vec<Rule> {
    vec![
        // === Numeral + unit rules (ruleNumeralQuantities) ===

        // <number> cups
        Rule {
            name: "<quantity> cups".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex(r"(cups?)")],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                if data.value <= 0.0 {
                    return None;
                }
                Some(TokenData::Quantity(QuantityData::new(
                    data.value,
                    QuantityUnit::Cup,
                )))
            }),
        },
        // <number> grams/kg/mg
        Rule {
            name: "<quantity> grams".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r"(((m(illi)?[.]?)|(k(ilo)?)[.]?)?g(ram)?s?[.]?)[.]?"),
            ],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                if data.value <= 0.0 {
                    return None;
                }
                let matched = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let value = data.value * gram_multiplier(matched);
                Some(TokenData::Quantity(QuantityData::new(
                    value,
                    QuantityUnit::Gram,
                )))
            }),
        },
        // <number> pounds
        Rule {
            name: "<quantity> lb".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex(r"((lb|pound)s?)")],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                if data.value <= 0.0 {
                    return None;
                }
                Some(TokenData::Quantity(QuantityData::new(
                    data.value,
                    QuantityUnit::Pound,
                )))
            }),
        },
        // <number> ounces
        Rule {
            name: "<quantity> oz".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex(r"((ounces?)|oz)")],
            production: Box::new(|nodes| {
                let data = numeral_data(&nodes[0].token_data)?;
                if data.value <= 0.0 {
                    return None;
                }
                Some(TokenData::Quantity(QuantityData::new(
                    data.value,
                    QuantityUnit::Ounce,
                )))
            }),
        },
        // === "a/an" + unit rules (ruleAQuantity) ===
        Rule {
            name: "a <quantity> cups".to_string(),
            pattern: vec![regex(r"an? (cups?)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    1.0,
                    QuantityUnit::Cup,
                )))
            }),
        },
        Rule {
            name: "a <quantity> grams".to_string(),
            pattern: vec![regex(
                r"an? (((m(illi)?[.]?)|(k(ilo)?)[.]?)?g(ram)?s?[.]?)[.]?",
            )],
            production: Box::new(|nodes| {
                let matched = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let value = gram_multiplier(matched);
                Some(TokenData::Quantity(QuantityData::new(
                    value,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "a <quantity> lb".to_string(),
            pattern: vec![regex(r"an? ((lb|pound)s?)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    1.0,
                    QuantityUnit::Pound,
                )))
            }),
        },
        Rule {
            name: "a <quantity> oz".to_string(),
            pattern: vec![regex(r"an? ((ounces?)|oz)")],
            production: Box::new(|_| {
                Some(TokenData::Quantity(QuantityData::new(
                    1.0,
                    QuantityUnit::Ounce,
                )))
            }),
        },
        // === <quantity> of product ===
        Rule {
            name: "<quantity> of product".to_string(),
            pattern: vec![dim(DimensionKind::Quantity), regex(r"of (\w+)")],
            production: Box::new(|nodes| {
                let qd = quantity_data(&nodes[0].token_data)?;
                let product = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let mut result = qd.clone();
                result.product = Some(product.to_lowercase());
                Some(TokenData::Quantity(result))
            }),
        },
        // === Precision ===
        Rule {
            name: "about <quantity>".to_string(),
            pattern: vec![
                regex(
                    r"~|exactly|precisely|about|approx(\.?|imately)?|close to|near( to)?|around|almost",
                ),
                dim(DimensionKind::Quantity),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        // === Interval rules ===

        // between|from <numeral> and|to <quantity>
        Rule {
            name: "between|from <numeral> and|to <quantity>".to_string(),
            pattern: vec![
                regex(r"between|from"),
                dim(DimensionKind::Numeral),
                regex(r"to|and"),
                is_simple_quantity(),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[1].token_data)?;
                let qd = quantity_data(&nodes[3].token_data)?;
                let from = num.value;
                let to = qd.value?;
                let unit = qd.unit?;
                if from >= to {
                    return None;
                }
                Some(TokenData::Quantity(
                    QuantityData::unit_only(unit).with_interval(from, to),
                ))
            }),
        },
        // between|from <quantity> and|to <quantity>
        Rule {
            name: "between|from <quantity> and|to <quantity>".to_string(),
            pattern: vec![
                regex(r"between|from"),
                is_simple_quantity(),
                regex(r"and|to"),
                is_simple_quantity(),
            ],
            production: Box::new(|nodes| {
                let from_data = quantity_data(&nodes[1].token_data)?;
                let to_data = quantity_data(&nodes[3].token_data)?;
                let from = from_data.value?;
                let to = to_data.value?;
                let u1 = from_data.unit?;
                let u2 = to_data.unit?;
                if from >= to || u1 != u2 {
                    return None;
                }
                Some(TokenData::Quantity(
                    QuantityData::unit_only(u1).with_interval(from, to),
                ))
            }),
        },
        // <numeral> - <quantity>
        Rule {
            name: "<numeral> - <quantity>".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r"\-"),
                is_simple_quantity(),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let qd = quantity_data(&nodes[2].token_data)?;
                let from = num.value;
                let to = qd.value?;
                let unit = qd.unit?;
                if from >= to || from <= 0.0 {
                    return None;
                }
                Some(TokenData::Quantity(
                    QuantityData::unit_only(unit).with_interval(from, to),
                ))
            }),
        },
        // <quantity> - <quantity>
        Rule {
            name: "<quantity> - <quantity>".to_string(),
            pattern: vec![is_simple_quantity(), regex(r"\-"), is_simple_quantity()],
            production: Box::new(|nodes| {
                let from_data = quantity_data(&nodes[0].token_data)?;
                let to_data = quantity_data(&nodes[2].token_data)?;
                let from = from_data.value?;
                let to = to_data.value?;
                let u1 = from_data.unit?;
                let u2 = to_data.unit?;
                if from >= to || u1 != u2 {
                    return None;
                }
                Some(TokenData::Quantity(
                    QuantityData::unit_only(u1).with_interval(from, to),
                ))
            }),
        },
        // at most / under / below / less than <quantity>
        Rule {
            name: "at most <quantity>".to_string(),
            pattern: vec![
                regex(r"under|below|at most|(less|lower|not? more) than"),
                is_simple_quantity(),
            ],
            production: Box::new(|nodes| {
                let data = quantity_data(&nodes[1].token_data)?;
                let to = data.value?;
                let unit = data.unit?;
                Some(TokenData::Quantity(
                    QuantityData::unit_only(unit).with_max(to),
                ))
            }),
        },
        // over / above / at least / more than <quantity>
        Rule {
            name: "more than <quantity>".to_string(),
            pattern: vec![
                regex(r"over|above|exceeding|beyond|at least|(more|larger|bigger|heavier) than"),
                is_simple_quantity(),
            ],
            production: Box::new(|nodes| {
                let data = quantity_data(&nodes[1].token_data)?;
                let from = data.value?;
                let unit = data.unit?;
                Some(TokenData::Quantity(
                    QuantityData::unit_only(unit).with_min(from),
                ))
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
    fn test_quantity() {
        let mut rules = numeral::en::rules();
        rules.extend(super::rules());
        let options = Options { with_latent: false };
        let context = Context::default();

        let entities = engine::parse_and_resolve(
            "5 pounds",
            &rules,
            &context,
            &options,
            &[DimensionKind::Quantity],
        );
        let found = entities.iter().any(|e| match &e.value {
            crate::types::DimensionValue::Quantity {
                measurement: crate::types::MeasurementValue::Value { value, unit },
                ..
            } => (*value - 5.0).abs() < 0.01 && unit == "pound",
            _ => false,
        });
        assert!(found, "Expected 5 pounds, got: {:?}", entities);
    }
}
