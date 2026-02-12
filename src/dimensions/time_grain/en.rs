use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::Grain;

pub fn rules() -> Vec<Rule> {
    // Regex patterns match Haskell Duckling's TimeGrain/EN/Rules.hs
    vec![
        Rule {
            name: "second (grain)".to_string(),
            pattern: vec![regex(r#"sec(ond)?s?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Second))),
        },
        Rule {
            name: "minute (grain)".to_string(),
            pattern: vec![regex(r#"m(in(ute)?s?)?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Minute))),
        },
        Rule {
            name: "hour (grain)".to_string(),
            pattern: vec![regex(r#"h(((ou)?rs?)|r)?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Hour))),
        },
        Rule {
            name: "day (grain)".to_string(),
            pattern: vec![regex(r#"days?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Day))),
        },
        Rule {
            name: "week (grain)".to_string(),
            pattern: vec![regex(r#"weeks?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Week))),
        },
        Rule {
            name: "month (grain)".to_string(),
            pattern: vec![regex(r#"months?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Month))),
        },
        Rule {
            name: "quarter (grain)".to_string(),
            pattern: vec![regex(r#"(quarter|qtr)s?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Quarter))),
        },
        Rule {
            name: "year (grain)".to_string(),
            pattern: vec![regex(r#"y(ea)?rs?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Year))),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::types::DimensionKind;

    #[test]
    fn test_time_grains() {
        let rules = rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for (text, expected_grain) in &[
            ("second", "second"),
            ("minutes", "minute"),
            ("hour", "hour"),
            ("days", "day"),
            ("week", "week"),
            ("months", "month"),
            ("year", "year"),
        ] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::TimeGrain],
            );
            let found = entities.iter().any(|e| {
                matches!(&e.value, crate::types::DimensionValue::TimeGrain(g) if g.as_str() == *expected_grain)
            });
            assert!(
                found,
                "Expected grain '{}' for '{}', got: {:?}",
                expected_grain, text, entities
            );
        }
    }
}
