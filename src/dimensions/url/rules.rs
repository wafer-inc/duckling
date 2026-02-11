use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::UrlData;

pub fn rules() -> Vec<Rule> {
    vec![
        // Main URL rule: matches URLs with domain.tld pattern
        // Ported from Haskell ruleURL
        Rule {
            name: "url".to_string(),
            pattern: vec![regex(
                r"((([a-zA-Z]+)://)?(w{2,3}[0-9]*\.)?(([a-zA-Z0-9_-]+\.)+[a-z]{2,4})(:\d+)?(/[^?\s#]*)?(\?[^\s#]+)?(#[-,*=&a-zA-Z0-9]+)?)",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let full = m.group(1)?;
                let domain = m.group(5)?;
                Some(TokenData::Url(UrlData::new(full, &domain.to_lowercase())))
            }),
        },
        // Localhost rule
        // Ported from Haskell ruleLocalhost
        Rule {
            name: "localhost".to_string(),
            pattern: vec![regex(
                r"((([a-zA-Z]+)://)?localhost(:\d+)?(/[^?\s#]*)?(\?[^\s#]+)?)",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let full = m.group(1)?;
                Some(TokenData::Url(UrlData::new(full, "localhost")))
            }),
        },
        // Local URL rule: protocol://hostname (no TLD required)
        // Ported from Haskell ruleLocalURL
        Rule {
            name: "local url".to_string(),
            pattern: vec![regex(
                r"(([a-zA-Z]+)://([a-zA-Z0-9_-]+)(:\d+)?(/[^?\s#]*)?(\?[^\s#]+)?)",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let full = m.group(1)?;
                let domain = m.group(3)?;
                Some(TokenData::Url(UrlData::new(full, &domain.to_lowercase())))
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
    fn test_urls() {
        let rules = rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for (text, expected_domain) in &[
            ("https://www.example.com", "example.com"),
            ("http://google.com/search?q=test", "google.com"),
            ("www.github.com", "github.com"),
            ("cnn.com/info", "cnn.com"),
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
                    && e.value
                        .value
                        .get("domain")
                        .and_then(|v| v.as_str())
                        == Some(*expected_domain)
            });
            assert!(
                found,
                "Expected URL with domain '{}' for '{}', got: {:?}",
                expected_domain, text, entities
            );
        }
    }
}
