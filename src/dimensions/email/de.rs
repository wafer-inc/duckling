use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::EmailData;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "email spelled out".to_string(),
        pattern: vec![regex(
            "([\\w_+-]+(?:(?: punkt |\\.)[\\w_+-]+){0,10})(?: at |@)([a-zA-Z]+(?:(?:\\.| punkt )[\\w_-]+){1,10})",
        )],
        production: Box::new(|nodes| {
            let m = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m,
                _ => return None,
            };
            let local = m.group(1)?.replace(" punkt ", ".");
            let domain = m.group(2)?.replace(" punkt ", ".");
            let email = format!("{}@{}", local, domain);
            Some(TokenData::Email(EmailData::new(&email)))
        }),
    }]
}
