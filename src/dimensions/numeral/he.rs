use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn lex(s: &str) -> Option<f64> {
    match s {
        "אפס" => Some(0.0),
        "אחד" | "אחת" | "יחיד" => Some(1.0),
        "שתיים" | "שניים" | "זוג" => Some(2.0),
        "שלוש" | "שלושה" => Some(3.0),
        "ארבע" | "ארבעה" => Some(4.0),
        "חמש" | "חמישה" => Some(5.0),
        "שש" | "ששה" => Some(6.0),
        "שבע" | "שבעה" => Some(7.0),
        "שמונה" => Some(8.0),
        "תשע" | "תשעה" => Some(9.0),
        "עשר" | "עשרה" => Some(10.0),
        "עשרים" => Some(20.0),
        "שלושים" => Some(30.0),
        "ארבעים" => Some(40.0),
        "חמישים" => Some(50.0),
        "שישים" => Some(60.0),
        "שבעים" => Some(70.0),
        "שמונים" => Some(80.0),
        "תשעים" => Some(90.0),
        "ארבעה עשר" | "ארבע עשרה" => Some(14.0),
        "ששה עשר" | "שש עשרה" => Some(16.0),
        "שבעה עשר" | "שבע עשרה" => Some(17.0),
        "שמונה עשר" | "שמונה עשרה" => Some(18.0),
        "חצי" => Some(0.5),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "hebrew lexical numerals".to_string(),
            pattern: vec![regex("(אפס|אחד|אחת|יחיד|שתיים|שניים|זוג|שלוש|שלושה|ארבע|ארבעה|חמש|חמישה|שש|ששה|שבע|שבעה|שמונה|תשע|תשעה|עשר|עשרה|עשרים|שלושים|ארבעים|חמישים|שישים|שבעים|שמונים|תשעים|ארבעה עשר|ארבע עשרה|ששה עשר|שש עשרה|שבעה עשר|שבע עשרה|שמונה עשר|שמונה עשרה|חצי)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(lex(s)?)))
            }),
        },
        Rule {
            name: "composite tens".to_string(),
            pattern: vec![regex("(שלושים|עשרים|ארבעים|חמישים|שישים|שבעים|שמונים|תשעים)\\s+ו?(אחד|אחת|שתיים|שניים|שלוש|שלושה|ארבע|ארבעה|חמש|חמישה|שש|ששה|שבע|שבעה|שמונה|תשע|תשעה)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let t = m.group(1)?;
                let u = m.group(2)?;
                Some(TokenData::Numeral(NumeralData::new(lex(t)? + lex(u)?)))
            }),
        },
        Rule {
            name: "dot decimal".to_string(),
            pattern: vec![regex("(\\d*\\.\\d+)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(m.parse::<f64>().ok()?)))
            }),
        },
        Rule {
            name: "comma thousands".to_string(),
            pattern: vec![regex("(\\d{1,3}(,\\d\\d\\d){1,5})")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(
                    m.replace(',', "").parse::<f64>().ok()?,
                )))
            }),
        },
        Rule {
            name: "negative prefix".to_string(),
            pattern: vec![regex("(-|מינוס)\\s*(\\d{1,3}(,\\d\\d\\d){0,5})")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let v: f64 = m.replace(',', "").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
    ]
}
