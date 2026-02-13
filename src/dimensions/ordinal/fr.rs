use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn lookup_ordinal(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "première" | "premiere" | "premier" => Some(1),
        "deuxième" | "deuxieme" | "second" | "seconde" => Some(2),
        "troisième" | "troisieme" => Some(3),
        "quatrième" | "quatrieme" => Some(4),
        "cinquième" | "cinquieme" => Some(5),
        "sixième" | "sixieme" => Some(6),
        "septième" | "septieme" => Some(7),
        "huitième" | "huitieme" => Some(8),
        "neuvième" | "neuvieme" => Some(9),
        "dixième" | "dixieme" => Some(10),
        "onzième" | "onzieme" => Some(11),
        "douzième" | "douzieme" => Some(12),
        "treizième" | "treizieme" => Some(13),
        "quatorzième" | "quatorzieme" => Some(14),
        "quinzième" | "quinzieme" => Some(15),
        "seizième" | "seizieme" => Some(16),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+) ?(ere?|ère|ème|eme|e)")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: i64 = n.parse().ok()?;
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
        Rule {
            name: "ordinals (premier..seizieme)".to_string(),
            pattern: vec![regex(
                "(premi(ere?|ère)|(deux|trois|quatr|cinqu|six|sept|huit|neuv|dix|onz|douz|treiz|quatorz|quinz|seiz)i(e|è)me|seconde?)",
            )],
            production: Box::new(|nodes| {
                let txt = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = lookup_ordinal(txt)?;
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
    ]
}
