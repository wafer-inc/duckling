use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::UrlData;

pub fn rules() -> Vec<Rule> {
    vec![
        // URLs with protocol
        Rule {
            name: "url (with protocol)".to_string(),
            pattern: vec![regex(
                r#"(https?://(?:www\.)?[-a-zA-Z0-9@:%._\+~#=]+\.[a-zA-Z]{2,}[-a-zA-Z0-9@:%_\+.~#?&/=]*)"#,
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let domain = extract_domain(text)?;
                Some(TokenData::Url(UrlData::new(text, &domain)))
            }),
        },
        // URLs without protocol: www.example.com
        Rule {
            name: "url (www.)".to_string(),
            pattern: vec![regex(
                r#"(www\.[-a-zA-Z0-9@:%._\+~#=]+\.[a-zA-Z]{2,}[-a-zA-Z0-9@:%_\+.~#?&/=]*)"#,
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let domain = extract_domain(text)?;
                Some(TokenData::Url(UrlData::new(text, &domain)))
            }),
        },
    ]
}

fn extract_domain(url: &str) -> Option<String> {
    let without_protocol = url
        .strip_prefix("https://")
        .or_else(|| url.strip_prefix("http://"))
        .unwrap_or(url);
    let without_www = without_protocol
        .strip_prefix("www.")
        .unwrap_or(without_protocol);
    let domain = without_www.split('/').next()?;
    let domain = domain.split('?').next()?;
    let domain = domain.split('#').next()?;
    Some(domain.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::types::DimensionKind;

    #[test]
    fn test_urls() {
        let rules = rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for (text, expected_domain) in &[
            ("https://www.example.com", "example.com"),
            ("http://google.com/search?q=test", "google.com"),
            ("www.github.com", "github.com"),
        ] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::Url],
            );
            let found = entities.iter().any(|e| {
                e.dim == "url"
                    && e.value.value.get("domain").and_then(|v| v.as_str())
                        == Some(*expected_domain)
            });
            assert!(found, "Expected URL with domain '{}' for '{}', got: {:?}", expected_domain, text, entities);
        }
    }
}
