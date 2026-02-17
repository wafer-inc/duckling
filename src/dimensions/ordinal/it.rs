use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn lookup_ordinal(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "primo" | "prima" | "primi" | "prime" => Some(1),
        "secondo" | "seconda" | "secondi" | "seconde" => Some(2),
        "terzo" | "terza" | "terzi" | "terze" => Some(3),
        "quarto" | "quarta" | "quarti" | "quarte" => Some(4),
        "quinto" | "quinta" | "quinti" | "quinte" => Some(5),
        "sesto" | "sesta" | "sesti" | "seste" => Some(6),
        "settimo" | "settima" | "settimi" | "settime" => Some(7),
        "ottavo" | "ottava" | "ottavi" | "ottave" => Some(8),
        "nono" | "nona" | "noni" | "none" => Some(9),
        "decimo" | "decima" | "decimi" | "decime" => Some(10),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+) ?(ª|°|°)")],
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
            name: "ordinals (primo..10)".to_string(),
            pattern: vec![regex(
                "((prim|second|terz|quart|quint|sest|settim|ottav|non|decim)(o|a|i|e))",
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
