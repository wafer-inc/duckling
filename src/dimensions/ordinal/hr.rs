use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn lookup_ordinal(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "trece" | "treći" | "treća" | "treće" | "trećeg" => Some(3),
        "cetvrti" | "četvrti" | "cetvrta" | "četvrta" | "cetvrto" | "četvrto" => Some(4),
        "sesti" | "šesti" | "sesta" | "šesta" | "sesto" | "šesto" | "sestog" | "šestog" | "sestoga" | "šestoga" => Some(6),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+)(\\.| ?(t(i|a)(n|r|s)?)|(ste(n|r|s)?))")],
            production: Box::new(|nodes| {
                let v: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
        Rule {
            name: "ordinals (first..19th subset)".to_string(),
            pattern: vec![regex("(tre(c|ć)(e|i|a|eg)|(č|c)etvrt(i|a|o)|([sš])est(i|a|o(g|ga)?))")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(lookup_ordinal(s)?)))
            }),
        },
    ]
}
