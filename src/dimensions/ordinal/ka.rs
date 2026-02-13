use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn ordinals_map(s: &str) -> Option<i64> {
    match s {
        "პირველი" | "პირველ" | "მეერთე" => Some(1),
        "მეორე" => Some(2),
        "მესამე" => Some(3),
        "მეოთხე" => Some(4),
        "მეხუთე" => Some(5),
        "მეექვსე" => Some(6),
        "მეშვიდე" => Some(7),
        "მერვე" => Some(8),
        "მეცხრე" => Some(9),
        "მეათე" => Some(10),
        "მეთერთმეტე" => Some(11),
        "მეთორმეტე" => Some(12),
        "მეცამეტე" => Some(13),
        "მეთოთხმეტე" => Some(14),
        "მეთხუთმეტე" => Some(15),
        "მეთქვსმეტე" | "მეთექვსმეტე" => Some(16),
        "მეჩვიდმეტე" => Some(17),
        "მეთვრამეტე" => Some(18),
        "მეცხრამეტე" => Some(19),
        "მეოცე" => Some(20),
        "ოცდამეათე" => Some(30),
        "მეორმოცე" => Some(40),
        "ორმოცდამეათე" => Some(50),
        "მესამოცე" => Some(60),
        "სამოცდამეათე" => Some(70),
        "მეოთხმოცე" => Some(80),
        "ოთხმოცდამეათე" => Some(90),
        _ => None,
    }
}

fn cardinals_map(s: &str) -> Option<i64> {
    match s {
        "ოცი" | "ოცდა" => Some(20),
        "ოცდაათი" => Some(30),
        "ორმოცი" | "ორმოცდა" => Some(40),
        "ორმოცდაათი" => Some(50),
        "სამოცი" | "სამოცდა" => Some(60),
        "სამოცდაათი" => Some(70),
        "ოთხმოცი" | "ოთხმოცდა" => Some(80),
        "ოთხმოცდაათი" => Some(90),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinals (first..twentieth,thirtieth,...)".to_string(),
            pattern: vec![regex("(პირველი?|მეორე|მესამე|მეოთხე|მეხუთე|მეექვსე|მეშვიდე|მერვე|მეცხრე|მეათე|მეთერთმეტე|მეთოთხმეტე|მეცამეტე|მეთოთხმეტე|მეთხუთმეტე|მეთექვსმეტე|მეჩვიდმეტე|მეთვრამეტე|მეცხრამეტე|მეოცე|ოცდამეათე|მეორმოცე|ორმოცდამეათე|მესამოცე|სამოცდამეათე|მეოთხმოცე|ოთხმოცდამეათე)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(ordinals_map(text)?)))
            }),
        },
        Rule {
            name: "ordinals (composite, e.g. eighty-seven, forty—seventh, twenty ninth, thirtythird)".to_string(),
            pattern: vec![regex("(ოცდა|ორმოცდა|სამოცდა|ოთხმოცდა)(\\s|-|—)?(მეერთე|მეორე|მესამე|მეოთხე|მეხუთე|მეექვსე|მეშვიდე|მერვე|მეცხრე|მეათე|მეთერთმეტე|მეთოთხმეტე|მეცამეტე|მეთოთხმეტე|მეთხუთმეტე|მეთექვსმეტე|მეჩვიდმეტე|მეთვრამეტე|მეცხრამეტე)")],
            production: Box::new(|nodes| {
                let tens = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let units = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(3)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(
                    cardinals_map(tens)? + ordinals_map(units)?,
                )))
            }),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+) ?(-ლი|-ე)")],
            production: Box::new(|nodes| {
                let value: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(value)))
            }),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("მე-? ?0*(\\d+)")],
            production: Box::new(|nodes| {
                let value: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(value)))
            }),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("მე-? ?0*(\\d+) ?(-ლი|-ე)")],
            production: Box::new(|nodes| {
                let value: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(value)))
            }),
        },
    ]
}
