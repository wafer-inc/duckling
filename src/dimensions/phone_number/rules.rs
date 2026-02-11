use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::PhoneNumberData;

pub fn rules() -> Vec<Rule> {
    vec![
        // US phone numbers: (123) 456-7890, 123-456-7890, etc.
        Rule {
            name: "phone number (US)".to_string(),
            pattern: vec![regex(
                r#"(\+?1?\s*\(?[2-9]\d{2}\)?\s*[-.\s]?\d{3}\s*[-.\s]?\d{4})"#,
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                // Must have at least 10 digits
                let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
                if digits.len() >= 10 {
                    Some(TokenData::PhoneNumber(PhoneNumberData::new(text)))
                } else {
                    None
                }
            }),
        },
        // International format: +44 20 1234 5678
        Rule {
            name: "phone number (international)".to_string(),
            pattern: vec![regex(
                r#"(\+\d{1,3}\s*[-.]?\s*\d[\d\s\-\.]{7,15}\d)"#,
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
                if digits.len() >= 10 {
                    Some(TokenData::PhoneNumber(PhoneNumberData::new(text)))
                } else {
                    None
                }
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

        for text in &["(650) 123-4567", "650-123-4567", "+1 650 123 4567"] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::PhoneNumber],
            );
            let found = entities.iter().any(|e| e.dim == "phone-number");
            assert!(found, "Expected phone number for '{}', got: {:?}", text, entities);
        }
    }
}
