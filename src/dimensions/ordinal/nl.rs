use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn lookup_ordinal(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "eerste" => Some(1),
        "tweede" => Some(2),
        "derde" => Some(3),
        "vierde" => Some(4),
        "vijfde" => Some(5),
        "zesde" | "zeste" => Some(6),
        "zevende" => Some(7),
        "achtste" | "achste" => Some(8),
        "negende" => Some(9),
        "tiende" => Some(10),
        "elfde" => Some(11),
        "twaalfde" => Some(12),
        "dertiende" => Some(13),
        "veertiende" => Some(14),
        "vijftiende" => Some(15),
        "zestiende" => Some(16),
        "zeventiende" => Some(17),
        "achttiende" => Some(18),
        "negentiende" => Some(19),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+)(\\.| ?(ste|de))")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: i64 = s.parse().ok()?;
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
        Rule {
            name: "ordinals (first..19th)".to_string(),
            pattern: vec![regex(
                "(eerste|tweede|derde|vierde|vijfde|zeste|zevende|achtste|negende|tiende|elfde|twaalfde|veertiende|vijftiende|zestiende|zeventiende|achttiende|negentiende)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = lookup_ordinal(s)?;
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
    ]
}
