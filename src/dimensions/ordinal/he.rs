use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::OrdinalData;

fn literal_ordinal(s: &str) -> Option<i64> {
    match s {
        "אחד" | "ראשון" => Some(1),
        "שתיים" | "שניים" | "שני" => Some(2),
        "שלושה" | "שלישי" => Some(3),
        "ארבעה" | "רביעי" => Some(4),
        "חמישי" | "חמישה" => Some(5),
        "ששה" | "שישי" => Some(6),
        "שבעה" | "שביעי" => Some(7),
        "שמונה" | "שמיני" => Some(8),
        "תשעה" | "תשיעי" => Some(9),
        "עשרה" | "עשירי" => Some(10),
        "אחד עשר" | "אחד עשרה" => Some(11),
        "שניים עשר" | "תרי עשר" => Some(12),
        "שלוש עשר" | "שלושה עשר" | "שלוש עשרה" | "שלושה עשרה" => Some(13),
        "ארבע עשר" | "ארבעה עשר" | "ארבע עשרה" | "ארבעה עשרה" => Some(14),
        "חמישה עשר" | "חמש עשרה" => Some(15),
        "שש עשר" | "ששה עשר" | "שש עשרה" | "ששה עשרה" => Some(16),
        "שבע עשר" | "שבעה עשר" | "שבע עשרה" | "שבעה עשרה" => Some(17),
        "שמונה עשר" | "שמונה עשרה" => Some(18),
        "תשע עשר" | "תשעה עשר" | "תשע עשרה" | "תשעה עשרה" => Some(19),
        "עשרים" => Some(20),
        "שלושים" => Some(30),
        "ארבעים" => Some(40),
        "חמישים" => Some(50),
        "שישים" => Some(60),
        "שבעים" => Some(70),
        "שמונים" => Some(80),
        "תשעים" => Some(90),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal composition (with and)".to_string(),
            pattern: vec![dim(DimensionKind::Ordinal), regex("ו"), dim(DimensionKind::Ordinal)],
            production: Box::new(|nodes| {
                let v1 = match &nodes[0].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                let v2 = match &nodes[2].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "ordinals literals (he)".to_string(),
            pattern: vec![regex("(אחד|ראשון|שתיים|שניים|שני|שלושה|שלישי|ארבעה|רביעי|חמישי|חמישה|ששה|שישי|שבעה|שביעי|שמונה|שמיני|תשעה|תשיעי|עשרה|עשירי|אחד עשר(ה)?|שניים עשר|תרי עשר|שלוש(ה)? עשר(ה)?|ארבע(ה)? עשר(ה)?|חמישה עשר|חמש עשרה?|שש(ה)? עשר(ה)?|שבע(ה)? עשר(ה)?|שמונה עשר(ה)?|תשע(ה)? עשר(ה)?|עשרים|שלושים|ארבעים|חמישים|שישים|שבעים|שמונים|תשעים)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(literal_ordinal(s)?)))
            }),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+) ")],
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
