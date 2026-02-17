use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn ordinals_first_map(s: &str) -> Option<i64> {
    match s {
        "перш" => Some(1),
        "друг" => Some(2),
        "трет" => Some(3),
        "четверт" => Some(4),
        "п‘ят" | "п'ят" => Some(5),
        "шост" => Some(6),
        "сьом" => Some(7),
        "восьм" => Some(8),
        "дев‘ят" | "дев'ят" => Some(9),
        "десят" => Some(10),
        "одинадцят" => Some(11),
        "дванадцят" => Some(12),
        "тринадцят" => Some(13),
        "чотирнадцят" => Some(14),
        "п‘ятнадцят" | "п'ятнадцят" => Some(15),
        "шістнадцят" => Some(16),
        "сімнадцят" => Some(17),
        "вісімнадцят" => Some(18),
        "дев‘ятнадцят" | "дев'ятнадцят" => Some(19),
        "двадцят" => Some(20),
        _ => None,
    }
}

fn tens_map(s: &str) -> Option<i64> {
    match s {
        "двадцять" => Some(20),
        "тридцять" => Some(30),
        "сорок" => Some(40),
        "п‘ятдесят" | "п'ятдесят" => Some(50),
        "шістдесят" | "шістьдесят" => Some(60),
        "сімдесят" => Some(70),
        "вісімдесят" => Some(80),
        "дев‘яносто" | "дев'яносто" => Some(90),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal 21..99".to_string(),
            pattern: vec![
                regex("(двадцять|тридцять|сорок|п[‘']ятдесят|шістьдесят|сімдесят|вісімдесят|дев[‘']яносто)"),
                regex("(перш|друг|трет|четверт|п[‘']ят|шост|сьом|восьм|дев[‘']ят)(ий|ій|а|я|е|є)"),
            ],
            production: Box::new(|nodes| {
                let tens = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => tens_map(m.group(1)?)?,
                    _ => return None,
                };
                let units = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => ordinals_first_map(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(tens.checked_add(units)?)))
            }),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+)-?((и|і)?й|а|я|е|є)")],
            production: Box::new(|nodes| {
                let value: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(value)))
            }),
        },
        Rule {
            name: "ordinals (first..19th)".to_string(),
            pattern: vec![regex("(перш|друг|трет|четверт|п[‘']ят|шост|сьом|восьм|дев[‘']ят|десят|одинадцят|дванадцят|тринадцят|чотирнадцят|п[‘']ятнадцят|шістнадцят|сімнадцят|вісімнадцят|дев[‘']ятнадцят|двадцят)(ий|ій|а|я|е|є)")],
            production: Box::new(|nodes| {
                let value = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => ordinals_first_map(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(value)))
            }),
        },
    ]
}
