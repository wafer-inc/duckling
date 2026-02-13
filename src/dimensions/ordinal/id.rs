use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn lookup_ordinal(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "pertama" => Some(1),
        "kedua" => Some(2),
        "ketiga" => Some(3),
        "keempat" => Some(4),
        "kelima" => Some(5),
        "keenam" => Some(6),
        "ketujuh" => Some(7),
        "kedelapan" => Some(8),
        "kesembilan" => Some(9),
        "kesepuluh" => Some(10),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinals".to_string(),
            pattern: vec![regex("(pertama|kedua|ketiga|keempat|kelima|keenam|ketujuh|kedelapan|kesembilan|kesepuluh)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(lookup_ordinal(s)?)))
            }),
        },
        Rule {
            name: "ordinals (digits)".to_string(),
            pattern: vec![regex("ke-0*(\\d+)")],
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
