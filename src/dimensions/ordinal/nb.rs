use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn lookup_ordinal(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "første" => Some(1),
        "andre" => Some(2),
        "tredje" => Some(3),
        "fjerde" => Some(4),
        "femte" => Some(5),
        "sjette" => Some(6),
        "syvende" => Some(7),
        "åttende" => Some(8),
        "niende" => Some(9),
        "tiende" => Some(10),
        "ellevte" => Some(11),
        "tolvte" => Some(12),
        "trettende" => Some(13),
        "fjortende" => Some(14),
        "femtende" => Some(15),
        "sekstende" => Some(16),
        "syttende" => Some(17),
        "attende" => Some(18),
        "nittende" => Some(19),
        "tyvende" | "tjuende" => Some(20),
        "enogtjuende" | "enogtyvende" => Some(21),
        "toogtyvende" | "toogtjuende" => Some(22),
        "treogtyvende" | "treogtjuende" => Some(23),
        "fireogtjuende" | "fireogtyvende" => Some(24),
        "femogtyvende" | "femogtjuende" => Some(25),
        "seksogtjuende" | "seksogtyvende" => Some(26),
        "syvogtyvende" | "syvogtjuende" => Some(27),
        "åtteogtyvende" | "åtteogtjuende" => Some(28),
        "niogtyvende" | "niogtjuende" => Some(29),
        "tredefte" => Some(30),
        "enogtredefte" => Some(31),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinals (first..31st)".to_string(),
            pattern: vec![regex("(første|andre|tredje|fjerde|femtende|femte|sjette|syvende|åttende|niende|tiende|ellevte|tolvte|trettende|fjortende|sekstende|syttende|attende|nittende|tyvende|tjuende|enogtyvende|toogtyvende|treogtyvende|fireogtyvende|femogtyvende|seksogtyvende|syvogtyvende|åtteogtyvende|niogtyvende|enogtjuende|toogtjuende|treogtjuende|fireogtjuende|femogtjuende|seksogtjuende|syvogtjuende|åtteogtyvend|niogtjuende|tredefte|enogtredefte)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(lookup_ordinal(s)?)))
            }),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+)(\\.|ste?)")],
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
