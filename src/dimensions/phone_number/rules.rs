use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::PhoneNumberData;

pub fn rules() -> Vec<Rule> {
    vec![
        // Comprehensive phone number rule ported from Haskell.
        // Matches: optional country code (+N or (+N)), number body with separators,
        // optional extension.
        // Since Rust regex doesn't support lookaheads, we validate digit count
        // in the production function (7-15 digits in the body).
        Rule {
            name: "phone number".to_string(),
            pattern: vec![regex(
                r"(?:\(?\+(\d{1,2})\)?[\s\-\.]*)?([\d(][\d()\s\-\.]{4,14}[\d)])(?:\s*e?xt?\.?\s*(\d{1,20}))?",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let prefix = m.group(1);
                let body = m.group(2)?;
                let extension = m.group(3);

                // Count digits in the body
                let body_digits: String = body.chars().filter(|c| c.is_ascii_digit()).collect();
                if body_digits.len() < 7 || body_digits.len() > 15 {
                    return None;
                }

                // Build the cleaned phone number value (matching Haskell's cleanup)
                let mut value = String::new();
                if let Some(code) = prefix {
                    value.push_str(&format!("(+{}) ", code));
                }
                let cleaned: String = body
                    .chars()
                    .filter(|c| !matches!(c, '.' | ' ' | '-' | '\t' | '(' | ')'))
                    .collect();
                value.push_str(&cleaned);
                if let Some(ext) = extension {
                    value.push_str(&format!(" ext {}", ext));
                }

                Some(TokenData::PhoneNumber(PhoneNumberData::new(&value)))
            }),
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
    fn test_phone_numbers() {
        let rules = rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for text in &[
            "650-701-8887",
            "(+1)650-701-8887",
            "+1 6507018887",
            "+33 1 46647998",
            "06 2070 2220",
            "4.8.6.6.8.2.7",
            "06354640807",
        ] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::PhoneNumber],
            );
            let found = entities
                .iter()
                .any(|e| matches!(&e.value, crate::types::DimensionValue::PhoneNumber(_)));
            assert!(
                found,
                "Expected phone number for '{}', got: {:?}",
                text, entities
            );
        }
    }

    #[test]
    fn test_no_phone_numbers() {
        let rules = rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for text in &["12345", "1234567890123456777777", "12345678901234567"] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::PhoneNumber],
            );
            let found = entities
                .iter()
                .any(|e| matches!(&e.value, crate::types::DimensionValue::PhoneNumber(_)));
            assert!(
                !found,
                "Expected NO phone number for '{}', got: {:?}",
                text, entities
            );
        }
    }
}
