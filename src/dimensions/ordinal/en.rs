use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinals (first..nineteenth)".to_string(),
            pattern: vec![regex(
                r#"(first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth|eleventh|twelfth|thirteenth|fourteenth|fifteenth|sixteenth|seventeenth|eighteenth|nineteenth)"#,
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let val = match text.to_lowercase().as_str() {
                    "first" => 1,
                    "second" => 2,
                    "third" => 3,
                    "fourth" => 4,
                    "fifth" => 5,
                    "sixth" => 6,
                    "seventh" => 7,
                    "eighth" => 8,
                    "ninth" => 9,
                    "tenth" => 10,
                    "eleventh" => 11,
                    "twelfth" => 12,
                    "thirteenth" => 13,
                    "fourteenth" => 14,
                    "fifteenth" => 15,
                    "sixteenth" => 16,
                    "seventeenth" => 17,
                    "eighteenth" => 18,
                    "nineteenth" => 19,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(val)))
            }),
        },
        Rule {
            name: "ordinals (twentieth..ninetieth)".to_string(),
            pattern: vec![regex(
                r#"(twentieth|thirtieth|fortieth|fiftieth|sixtieth|seventieth|eightieth|ninetieth)"#,
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let val = match text.to_lowercase().as_str() {
                    "twentieth" => 20,
                    "thirtieth" => 30,
                    "fortieth" => 40,
                    "fiftieth" => 50,
                    "sixtieth" => 60,
                    "seventieth" => 70,
                    "eightieth" => 80,
                    "ninetieth" => 90,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(val)))
            }),
        },
        // Composite ordinals: twenty-fifth, thirty first, fortysecond, etc.
        Rule {
            name: "ordinals (composite, e.g. twenty-fifth)".to_string(),
            pattern: vec![regex(
                r#"(twenty|thirty|forty|fifty|sixty|seventy|eighty|ninety)[\s\-\u{2014}]?(first|second|third|fourth|fifth|sixth|seventh|eighth|ninth)"#,
            )],
            production: Box::new(|nodes| {
                let tens_text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let units_text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let tens = match tens_text.to_lowercase().as_str() {
                    "twenty" => 20,
                    "thirty" => 30,
                    "forty" => 40,
                    "fifty" => 50,
                    "sixty" => 60,
                    "seventy" => 70,
                    "eighty" => 80,
                    "ninety" => 90,
                    _ => return None,
                };
                let units = match units_text.to_lowercase().as_str() {
                    "first" => 1,
                    "second" => 2,
                    "third" => 3,
                    "fourth" => 4,
                    "fifth" => 5,
                    "sixth" => 6,
                    "seventh" => 7,
                    "eighth" => 8,
                    "ninth" => 9,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(tens + units)))
            }),
        },
        // Numeric ordinals: 1st, 2nd, 3rd, 4th, 21st, etc.
        Rule {
            name: "ordinal (numeric)".to_string(),
            pattern: vec![regex(r#"(\d+)\s*(st|nd|rd|th)"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let val: i64 = text.parse().ok()?;
                Some(TokenData::Ordinal(OrdinalData::new(val)))
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::types::{DimensionKind, DimensionValue};

    #[test]
    fn test_ordinals() {
        let rules = rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for (text, expected) in &[
            ("first", 1i64),
            ("second", 2),
            ("third", 3),
            ("tenth", 10),
            ("1st", 1),
            ("2nd", 2),
            ("3rd", 3),
            ("21st", 21),
        ] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::Ordinal],
            );
            let found = entities
                .iter()
                .any(|e| matches!(&e.value, DimensionValue::Ordinal(v) if *v == *expected));
            assert!(
                found,
                "Expected ordinal {} for '{}', got: {:?}",
                expected, text, entities
            );
        }
    }
}
