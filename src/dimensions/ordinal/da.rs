use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::OrdinalData;

fn ord_lookup(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "første" => Some(1),
        "anden" => Some(2),
        "tredje" => Some(3),
        "fjerde" => Some(4),
        "femte" => Some(5),
        "sjette" => Some(6),
        "syvende" => Some(7),
        "ottende" => Some(8),
        "niende" => Some(9),
        "tiende" => Some(10),
        "elfte" => Some(11),
        "tolvte" => Some(12),
        "trettende" => Some(13),
        "fjortende" => Some(14),
        "femtende" => Some(15),
        "sekstende" => Some(16),
        "syttende" => Some(17),
        "attende" => Some(18),
        "nittende" => Some(19),
        "tyvende" => Some(20),
        "tenogtyvende" => Some(21),
        "toogtyvende" => Some(22),
        "treogtyvende" => Some(23),
        "fireogtyvende" => Some(24),
        "femogtyvende" => Some(25),
        "seksogtyvende" => Some(26),
        "syvogtyvende" => Some(27),
        "otteogtyvende" => Some(28),
        "niogtyvende" => Some(29),
        "tredivte" => Some(30),
        "enogtredivte" => Some(31),
        _ => None,
    }
}

fn tens_ord(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "tyvende" => Some(20),
        "tredivte" => Some(30),
        "fyrrende" | "fyrretyvende" => Some(40),
        "halvtredsende" | "halvtredsindstyvende" => Some(50),
        "tressende" | "tresindstyvende" => Some(60),
        "halvfjerdsende" | "halvfjerdsindstyvende" => Some(70),
        "firsende" | "firsindsstyvende" => Some(80),
        "halvfemsende" | "halvfemsindstyvende" => Some(90),
        _ => None,
    }
}

fn ones_prefix(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "" => Some(0),
        "enog" => Some(1),
        "toog" => Some(2),
        "treog" => Some(3),
        "fireog" => Some(4),
        "femog" => Some(5),
        "seksog" => Some(6),
        "syvog" => Some(7),
        "otteog" => Some(8),
        "niog" => Some(9),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
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
        Rule {
            name: "ordinals (first..19st)".to_string(),
            pattern: vec![regex("(første|anden|tredje|fjerde|femte|sjette|syvende|ottende|niende|tiende|elfte|tolvte|trettende|fjortende|femtende|sekstende|syttende|attende|nittende)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(ord_lookup(s)?)))
            }),
        },
        Rule {
            name: "ordinals, 20 to 99, spelled-out".to_string(),
            pattern: vec![regex("((?:en|to|tre|fire|fem|seks|syv|otte|ni)og)?(tyvende|tredivte|fyrr(?:etyv)?ende|halvtreds(?:indstyv)?ende|tres(?:indstyv|s)?ende|halvfjerds(?:indstyv)?ende|firs(?:indstyv)?ende|halvfems(?:indstyv)?ende)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let ones = ones_prefix(m.group(1).unwrap_or_default())?;
                let tens = tens_ord(m.group(2)?)?;
                Some(TokenData::Ordinal(OrdinalData::new(ones + tens)))
            }),
        },
        Rule {
            name: "ordinals, above 99, spelled out".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(n) if n.value > 99.0 && n.value.fract()==0.0)),
                regex("og"),
                dim(DimensionKind::Ordinal),
            ],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value as i64,
                    _ => return None,
                };
                let o = match &nodes[2].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(n + o)))
            }),
        },
    ]
}
