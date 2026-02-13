use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn khmer_digit(ch: char) -> Option<i64> {
    match ch {
        '០' => Some(0),
        '១' => Some(1),
        '២' => Some(2),
        '៣' => Some(3),
        '៤' => Some(4),
        '៥' => Some(5),
        '៦' => Some(6),
        '៧' => Some(7),
        '៨' => Some(8),
        '៩' => Some(9),
        _ => None,
    }
}

fn parse_km_number(s: &str) -> Option<i64> {
    if let Ok(v) = s.parse::<i64>() {
        return Some(v);
    }
    if s.chars().all(|c| khmer_digit(c).is_some()) {
        let mut v = 0i64;
        for ch in s.chars() {
            v = v * 10 + khmer_digit(ch)?;
        }
        return Some(v);
    }
    match s {
        "មួយ" => Some(1),
        "បួនរយ" => Some(400),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "ordinal (km ទី...)".to_string(),
        pattern: vec![regex("ទី([០-៩0-9]+|មួយ|បួនរយ)")],
        production: Box::new(|nodes| {
            let s = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            Some(TokenData::Ordinal(OrdinalData::new(parse_km_number(s)?)))
        }),
    }]
}
