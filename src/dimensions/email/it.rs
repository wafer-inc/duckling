use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::EmailData;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "email spelled out".to_string(),
        pattern: vec![regex("([\\w\\._+-]+) chiocciola ([\\w_-]+(\\.[\\w_-]+)+)")],
        production: Box::new(|nodes| {
            let m = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m,
                _ => return None,
            };
            let local = m.group(1)?;
            let domain = m.group(2)?;
            Some(TokenData::Email(EmailData::new(&format!(
                "{}@{}",
                local, domain
            ))))
        }),
    }]
}
