use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn parse_ko_cardinal(s: &str) -> Option<i64> {
    if let Ok(v) = s.parse::<i64>() {
        return Some(v);
    }
    match s {
        "첫" => Some(1),
        "첫번째" => Some(1),
        "첫째" => Some(1),
        "네" => Some(4),
        "네째" => Some(4),
        "네째번" => Some(4),
        "스물다섯" => Some(25),
        "이십오" => Some(25),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinals lexical (ko)".to_string(),
            pattern: vec![regex("(첫번째|첫째|네째번|스물다섯번째|이십오번째)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let base = s
                    .trim_end_matches("번째")
                    .trim_end_matches("째번")
                    .trim_end_matches("째");
                Some(TokenData::Ordinal(OrdinalData::new(parse_ko_cardinal(
                    base,
                )?)))
            }),
        },
        Rule {
            name: "ordinals numeric suffix (ko)".to_string(),
            pattern: vec![regex("([0-9]+)(번째|째(번)?)")],
            production: Box::new(|nodes| {
                let v: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
    ]
}
