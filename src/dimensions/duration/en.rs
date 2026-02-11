use crate::dimensions::numeral::helpers::numeral_data;
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

pub fn rules() -> Vec<Rule> {
    vec![
        // <integer> <grain>: "3 days", "2 hours"
        Rule {
            name: "<integer> <grain>".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                if num.value == num.value.floor() {
                    Some(TokenData::Duration(DurationData::new(
                        num.value as i64,
                        grain,
                    )))
                } else {
                    None
                }
            }),
        },
        // "a <grain>": "a day", "an hour"
        Rule {
            name: "a <grain>".to_string(),
            pattern: vec![
                regex(r#"an?"#),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(1, grain)))
            }),
        },
        // "half an hour", "half a day"
        Rule {
            name: "half a <grain>".to_string(),
            pattern: vec![
                regex(r#"half\s+an?"#),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                // Represent as sub-grain: half an hour = 30 minutes
                let (val, new_grain) = match grain {
                    Grain::Hour => (30, Grain::Minute),
                    Grain::Day => (12, Grain::Hour),
                    Grain::Year => (6, Grain::Month),
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(val, new_grain)))
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimensions::{numeral, time_grain};
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::types::DimensionKind;

    fn all_rules() -> Vec<Rule> {
        let mut r = numeral::en::rules();
        r.extend(time_grain::en::rules());
        r.extend(rules());
        r
    }

    #[test]
    fn test_duration() {
        let rules = all_rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for (text, expected_val, expected_unit) in &[
            ("3 days", 3, "day"),
            ("2 hours", 2, "hour"),
            ("1 week", 1, "week"),
            ("5 minutes", 5, "minute"),
        ] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::Duration],
            );
            let found = entities.iter().any(|e| {
                e.dim == "duration"
                    && e.value.value.get("value").and_then(|v| v.as_i64()) == Some(*expected_val as i64)
                    && e.value.value.get("unit").and_then(|v| v.as_str()) == Some(*expected_unit)
            });
            assert!(found, "Expected {} {} for '{}', got: {:?}", expected_val, expected_unit, text, entities);
        }
    }
}
