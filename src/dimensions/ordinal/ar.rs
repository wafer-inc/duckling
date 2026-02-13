use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn ordinals_map(s: &str) -> Option<i64> {
    match s {
        "اول" | "أول" | "حاد" | "حادي" | "واحد" => Some(1),
        "ثان" | "ثاني" => Some(2),
        "ثالث" => Some(3),
        "رابع" => Some(4),
        "خامس" => Some(5),
        "سادس" => Some(6),
        "سابع" => Some(7),
        "ثامن" => Some(8),
        "تاسع" => Some(9),
        "عاشر" => Some(10),
        _ => None,
    }
}

fn cardinals_map(s: &str) -> Option<i64> {
    match s {
        "عشر" => Some(20),
        "ثلاث" => Some(30),
        "اربع" => Some(40),
        "خمس" => Some(50),
        "ست" => Some(60),
        "سبع" => Some(70),
        "ثمان" => Some(80),
        "تسع" => Some(90),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinals (composite, e.g., eighty-seven)".to_string(),
            pattern: vec![regex("ال(واحد|حادي?|ثاني?|ثالث|رابع|خامس|سادس|سابع|ثامن|تاسع|عاشر) و ?ال(عشر|ثلاث|اربع|خمس|ست|سبع|ثمان|تسع)(ون|ين)")],
            production: Box::new(|nodes| {
                let unit = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let tens = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(
                    ordinals_map(unit)? + cardinals_map(tens)?,
                )))
            }),
        },
        Rule {
            name: "ordinals (first..tenth)".to_string(),
            pattern: vec![regex("(?:ال)?([أا]ول|ثاني?|ثالث|رابع|خامس|سادس|سابع|ثامن|تاسع|عاشر)[ةهى]?")],
            production: Box::new(|nodes| {
                let value = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => ordinals_map(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(value)))
            }),
        },
        Rule {
            name: "ordinals (eleventh)".to_string(),
            pattern: vec![regex("ال([اأإ]حد[يى]?|حاد(ي[ةه]?)?) ?عشر[ةه]?")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(11)))),
        },
        Rule {
            name: "ordinals (twelveth)".to_string(),
            pattern: vec![regex("ال([اأإ]ثن[يى]?|ثان(ي[ةه]?)?) ?عشر[ةه]?")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(12)))),
        },
        Rule {
            name: "ordinals (thirtieth..nineteenth)".to_string(),
            pattern: vec![regex("ال(ثالث|رابع|خامس|سادس|سابع|ثامن|تاسع)[ةه]? ?عشرة?")],
            production: Box::new(|nodes| {
                let unit = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(10 + ordinals_map(unit)?)))
            }),
        },
        Rule {
            name: "ordinals (twenty, thirty..ninety)".to_string(),
            pattern: vec![regex("ال(عشر|ثلاث|اربع|خمس|ست|سبع|ثمان|تسع)(ون|ين)")],
            production: Box::new(|nodes| {
                let tens = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(cardinals_map(tens)?)))
            }),
        },
    ]
}
