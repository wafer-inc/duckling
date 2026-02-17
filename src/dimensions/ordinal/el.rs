use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn normalize_greek(s: &str) -> String {
    s.to_lowercase()
        .replace('ά', "α")
        .replace('έ', "ε")
        .replace('ή', "η")
        .replace('ί', "ι")
        .replace('ό', "ο")
        .replace('ύ', "υ")
        .replace('ώ', "ω")
        .replace(['ϊ', 'ΐ'], "ι")
        .replace(['ϋ', 'ΰ'], "υ")
}

fn ordinal_stem_value(stem: &str) -> Option<i64> {
    match normalize_greek(stem).as_str() {
        "πρωτ" => Some(1),
        "δευτερ" => Some(2),
        "τριτ" => Some(3),
        "τεταρτ" => Some(4),
        "πεμπτ" => Some(5),
        "εκτ" => Some(6),
        "εβδομ" => Some(7),
        "ογδο" => Some(8),
        "ενατ" => Some(9),
        "δεκατ" => Some(10),
        "ενδεκατ" | "εντεκατ" => Some(11),
        "δωδεκατ" => Some(12),
        "εικοστ" => Some(20),
        "τριακοστ" => Some(30),
        "τεσσαρακοστ" => Some(40),
        "πεντηκοστ" => Some(50),
        "εξηκοστ" => Some(60),
        "εβδομηκοστ" => Some(70),
        "ογδοηκοστ" => Some(80),
        "ενενηκοστ" => Some(90),
        _ => None,
    }
}

fn is_between(lo: i64, hi: i64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Ordinal(o) if o.value >= lo && o.value <= hi)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinals (1st..12th, 20th, 30th..90th)".to_string(),
            pattern: vec![regex("(πρώτ|δε[υύ]τ[εέ]ρ|τρίτ|τ[εέ]τ[αά]ρτ|πέμπτ|έκτ|[εέ]βδ[οό]μ(ηκοστ)?|[οό]γδ[οό](ηκοστ)?|[εέ]ν[αά]τ|δ[εέ]κ[αά]τ|εν[δτ][εέ]κ[αά]τ|δωδ[εέ]κ[αά]τ|εικοστ|τριακοστ|τεσσαρακοστ|πεντηκοστ|εξηκοστ|ενενηκοστ)([οό][υύιί]?ς?|[ηή]ς?|[εέ]ς|ων)")],
            production: Box::new(|nodes| {
                let stem = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(ordinal_stem_value(stem)?)))
            }),
        },
        Rule {
            name: "ordinals (composite: 11th..19th, 21st..29th, ..., 91st..99th)"
                .to_string(),
            pattern: vec![predicate(is_between(10, 90)), predicate(is_between(1, 9))],
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
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+) ?(ο[ςυι]?|ης?|ες)")],
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
