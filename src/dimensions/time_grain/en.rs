use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::Grain;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "grain (second)".to_string(),
            pattern: vec![regex(r#"seconds?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Second))),
        },
        Rule {
            name: "grain (minute)".to_string(),
            pattern: vec![regex(r#"minutes?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Minute))),
        },
        Rule {
            name: "grain (hour)".to_string(),
            pattern: vec![regex(r#"hours?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Hour))),
        },
        Rule {
            name: "grain (day)".to_string(),
            pattern: vec![regex(r#"days?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Day))),
        },
        Rule {
            name: "grain (week)".to_string(),
            pattern: vec![regex(r#"weeks?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Week))),
        },
        Rule {
            name: "grain (month)".to_string(),
            pattern: vec![regex(r#"months?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Month))),
        },
        Rule {
            name: "grain (quarter)".to_string(),
            pattern: vec![regex(r#"quarters?"#)],
            production: Box::new(|_nodes| Some(TokenData::TimeGrain(Grain::Quarter))),
        },
        Rule {
            name: "grain (year)".to_string(),
            pattern: vec![regex(r#"years?"#)],
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
                e.dim == "time-grain"
                    && e.value.value.get("value").and_then(|v| v.as_str()) == Some(*expected_grain)
            });
            assert!(found, "Expected grain '{}' for '{}', got: {:?}", expected_grain, text, entities);
        }
    }
}
