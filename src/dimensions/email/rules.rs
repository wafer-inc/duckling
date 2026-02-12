use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::EmailData;

pub fn rules() -> Vec<Rule> {
    vec![
        // Standard email: user@example.com
        Rule {
            name: "email".to_string(),
            pattern: vec![regex(
                r"([\w._+-]+@[\w_-]+(\.[\w_-]+)+)",
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Email(EmailData::new(text)))
            }),
        },
        // Spelled-out email: "alice at example dot com" (EN-specific)
        Rule {
            name: "email spelled out".to_string(),
            pattern: vec![regex(
                r"([\w_+-]+(?:(?:\s+dot\s+|\.)+[\w_+-]+){0,10})(?:\s+at\s+|@)((?:[a-zA-Z][\w_-]*)(?:(?:\.|\s+dot\s+)[\w_-]+){1,10})",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let local = m.group(1)?;
                let domain = m.group(2)?;

                // Replace " dot " (with surrounding whitespace) with "."
                let replace_dots = |s: &str| -> String {
                    let mut result = s.to_string();
                    // Replace all whitespace-dot-whitespace patterns
                    while let Some(pos) = result.to_lowercase().find(" dot ") {
                        let end = pos + 5;
                        // Find actual boundaries (could have multiple spaces)
                        result = format!("{}.{}", &result[..pos], &result[end..]);
                    }
                    result
                };

                let clean_local = replace_dots(local);
                let clean_domain = replace_dots(domain);
                let email = format!("{}@{}", clean_local, clean_domain);

                Some(TokenData::Email(EmailData::new(&email)))
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
                matches!(&e.value, crate::types::DimensionValue::Email(v) if v == *email)
            });
            assert!(found, "Expected email '{}', got: {:?}", email, entities);
        }
    }
}
