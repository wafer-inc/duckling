use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn lookup_unit_ordinal_stem(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "primeir" => Some(1),
        "segund" => Some(2),
        "terceir" => Some(3),
        "quart" => Some(4),
        "quint" => Some(5),
        "sext" => Some(6),
        "setim" | "sétim" => Some(7),
        "oitav" => Some(8),
        "non" => Some(9),
        "decim" | "décim" => Some(10),
        _ => None,
    }
}

fn lookup_tens_ordinal_stem(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "vi" => Some(20),
        "tri" => Some(30),
        "quadra" => Some(40),
        "qüinqua" | "quinqua" => Some(50),
        "sexa" => Some(60),
        "septua" => Some(70),
        "octo" => Some(80),
        "nona" => Some(90),
        _ => None,
    }
}

fn one_of_ordinals(values: &'static [i64]) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Ordinal(o) if values.contains(&o.value))
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinals (1..10)".to_string(),
            pattern: vec![regex("(primeir|segund|terceir|quart|quint|sext|s[ée]tim|oitav|non|d[ée]cim)[ao]s?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = lookup_unit_ordinal_stem(s)?;
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
        Rule {
            name: "cardinals (20 .. 90)".to_string(),
            pattern: vec![regex("(vi|tri|quadra|q[üu]inqua|sexa|septua|octo|nona)g[ée]sim[ao]s?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = lookup_tens_ordinal_stem(s)?;
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
        Rule {
            name: "ordinals (11..19)".to_string(),
            pattern: vec![
                predicate(one_of_ordinals(&[10, 20, 30, 40, 50, 60, 70, 80, 90])),
                predicate(one_of_ordinals(&[1, 2, 3, 4, 5, 6, 7, 8, 9])),
            ],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                let u = match &nodes[1].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(t.checked_add(u)?)))
            }),
        },
    ]
}
