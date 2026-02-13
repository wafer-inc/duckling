use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn unit_ordinal(s: &str) -> Option<i64> {
    match s {
        "нэг" => Some(1),
        "хоёр" => Some(2),
        "гурав" => Some(3),
        "дөрөв" => Some(4),
        "тав" => Some(5),
        "зургаа" => Some(6),
        "долоо" => Some(7),
        "найм" => Some(8),
        "ес" => Some(9),
        "арав" => Some(10),
        _ => None,
    }
}

fn tens_cardinal(s: &str) -> Option<i64> {
    match s {
        "арван" => Some(10),
        "хорин" | "хорь" => Some(20),
        "гучин" | "гуч" => Some(30),
        "дөчин" | "дөч" => Some(40),
        "тавин" | "тавь" => Some(50),
        "жаран" | "жар" => Some(60),
        "далан" | "дал" => Some(70),
        "наян" | "ная" => Some(80),
        "ерэн" | "ер" => Some(90),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinals (first..19th)".to_string(),
            pattern: vec![regex("(нэг|хоёр|гурав|дөрөв|тав|зургаа|долоо|найм|ес|арав) ?(дугаар|дүгээр|дахь|дэх)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(unit_ordinal(s)?)))
            }),
        },
        Rule {
            name: "ordinal 10..99".to_string(),
            pattern: vec![regex("(арван|хорин|гучин|дөчин|тавин|жаран|далан|наян|ерэн) ?(нэг|хоёр|гурав|дөрөв|тав|зургаа|долоо|найм|ес) ?(дугаар|дүгээр|дахь|дэх)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let t = tens_cardinal(m.group(1)?)?;
                let u = unit_ordinal(m.group(2)?)?;
                Some(TokenData::Ordinal(OrdinalData::new(t + u)))
            }),
        },
        Rule {
            name: "integer (20..90) ordinal".to_string(),
            pattern: vec![regex("(хорь|гуч|дөч|тавь|жар|дал|ная|ер) ?(дугаар|дүгээр|дахь|дэх)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(tens_cardinal(s)?)))
            }),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+)-?(ын|ийн|р|с|)")],
            production: Box::new(|nodes| {
                let v: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
        Rule {
            name: "ordinal (digits) 2".to_string(),
            pattern: vec![regex("0*(\\d+)(\\.| ?(дугаар|дүгээр|дахь|дэх))")],
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
