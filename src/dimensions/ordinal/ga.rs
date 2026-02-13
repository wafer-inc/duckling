use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn normalize(s: &str) -> String {
    s.to_lowercase()
        .replace('á', "a")
        .replace('é', "e")
        .replace('í', "i")
        .replace('ó', "o")
        .replace('ú', "u")
}

fn lookup_ordinal(s: &str) -> Option<i64> {
    match normalize(s).as_str() {
        "t-aonu" | "aonu" | "chead" => Some(1),
        "dara" => Some(2),
        "triu" => Some(3),
        "ceathru" => Some(4),
        "cuigiu" => Some(5),
        "seu" => Some(6),
        "seachtu" => Some(7),
        "t-ochtu" | "ochtu" => Some(8),
        "naou" => Some(9),
        "deichiu" => Some(10),
        "fichiu" => Some(20),
        "triochadu" => Some(30),
        "daicheadu" => Some(40),
        "caogadu" => Some(50),
        "seascadu" => Some(60),
        "seachtodu" => Some(70),
        "ochtodu" | "t-ochtodu" => Some(80),
        "nochadu" => Some(90),
        "ceadu" => Some(100),
        "miliu" => Some(1000),
        "milliunu" => Some(1_000_000),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+) ?(adh|a|d|ú|u)")],
            production: Box::new(|nodes| {
                let v: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
        Rule {
            name: "ordinals (chéad, dara, etc.)".to_string(),
            pattern: vec![regex("(ch(é|e)ad|aon(ú|u)|t-aon(ú|u)|dara|tr(í|i)(ú|u)|ceathr(ú|u)|c(ú|u)igi(ú|u)|s(é|e)(ú|u)|seacht(ú|u)|ocht(ú|u)|t-ocht(ú|u)|nao(ú|u)|deichi(ú|u)|fichi(ú|u)|tr(í|i)ochad(ú|u)|daichead(ú|u)|caogad(ú|u)|seascad(ú|u)|seacht(ó|o)d(ú|u)|ocht(ó|o)d(ú|u)|t-ocht(ó|o)d(ú|u)|n(ó|o)chad(ú|u)|c(é|e)ad(ú|u)|mili(ú|u)|milli(ú|u)n(ú|u))")],
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
