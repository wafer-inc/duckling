use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::EmailData;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "email".to_string(),
        pattern: vec![regex(
            r#"([a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+)"#,
        )],
        production: Box::new(|nodes| {
            let text = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            Some(TokenData::Email(EmailData::new(text)))
        }),
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::types::DimensionKind;

    #[test]
    fn test_email() {
        let rules = rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for email in &[
            "user@example.com",
            "test.name+tag@domain.co.uk",
            "hello@world.org",
        ] {
            let entities = engine::parse_and_resolve(
                email,
                &rules,
                &context,
                &options,
                &[DimensionKind::Email],
            );
            let found = entities.iter().any(|e| {
                e.dim == "email"
                    && e.value.value.get("value").and_then(|v| v.as_str()) == Some(*email)
            });
            assert!(found, "Expected email '{}', got: {:?}", email, entities);
        }
    }

    #[test]
    fn test_email_in_text() {
        let rules = rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        let entities = engine::parse_and_resolve(
            "send it to user@example.com please",
            &rules,
            &context,
            &options,
            &[DimensionKind::Email],
        );
        let found = entities.iter().any(|e| {
            e.dim == "email"
                && e.value
                    .value
                    .get("value")
                    .and_then(|v| v.as_str())
                    == Some("user@example.com")
        });
        assert!(found, "Expected email in text, got: {:?}", entities);
    }
}
