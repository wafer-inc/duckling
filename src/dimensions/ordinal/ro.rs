use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn lookup_spelled_out_ordinal(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "doi" => Some(2),
        "trei" => Some(3),
        "patru" => Some(4),
        "cinci" => Some(5),
        "sase" | "șase" => Some(6),
        "sapte" | "șapte" => Some(7),
        "opt" => Some(8),
        "noua" | "nouă" => Some(9),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinals (digits)".to_string(),
            pattern: vec![regex("al?\\s0*(\\d+)[ -]?(le)?a")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: i64 = m.parse().ok()?;
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
        Rule {
            name: "special ordinals".to_string(),
            pattern: vec![regex("(prim(a|ul)|a (patra|[dn]oua))")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match t.as_str() {
                    "prima" | "primul" => 1,
                    "a doua" => 2,
                    "a patra" => 4,
                    "a noua" => 9,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
        Rule {
            name: "spelled out ordinals".to_string(),
            pattern: vec![regex("al?\\s(doi|trei|patru|cinci|(s|ș)a(s|pt)e|opt|nou(a|ă))[ -]?(le)?a")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = lookup_spelled_out_ordinal(m)?;
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
    ]
}
