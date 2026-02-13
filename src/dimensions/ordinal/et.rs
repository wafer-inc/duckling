use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn lookup_ordinal_word(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "esimene" => Some(1),
        "teine" => Some(2),
        "kolmas" => Some(3),
        "neljas" => Some(4),
        "viies" => Some(5),
        "kuues" => Some(6),
        "seitsmes" => Some(7),
        "kaheksas" => Some(8),
        "üheksas" => Some(9),
        "kümnes" => Some(10),
        "üheteistkümnes" => Some(11),
        "kaheteistkümnes" => Some(12),
        "kolmeteistkümnes" => Some(13),
        "neljateistkümnes" => Some(14),
        "viieteistkümnes" => Some(15),
        "kuueteistkümnes" => Some(16),
        "seitsmeteistkümnes" => Some(17),
        "kaheksateistkümnes" => Some(18),
        "üheksateistkümnes" => Some(19),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+)\\.")],
            production: Box::new(|nodes| {
                let v: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
        Rule {
            name: "ordinals (first..19th)".to_string(),
            pattern: vec![regex("(esimene|teine|kolmas|neljas|viies|kuues|seitsmes|kaheksas|üheksas|kümnes|üheteistkümnes|kaheteistkümnes|kolmeteistkümnes|neljateistkümnes|viieteistkümnes|kuueteistkümnes|seitsmeteistkümnes|kaheksateistkümnes|üheksateistkümnes)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(lookup_ordinal_word(s)?)))
            }),
        },
    ]
}
