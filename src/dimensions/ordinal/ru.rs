use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn ordinals_first_map(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "перв" => Some(1),
        "втор" => Some(2),
        "трет" => Some(3),
        "четверт" | "четвёрт" => Some(4),
        "пят" => Some(5),
        "шест" => Some(6),
        "седьм" => Some(7),
        "восьм" => Some(8),
        "девят" => Some(9),
        "десят" => Some(10),
        "одиннадцат" => Some(11),
        "двенадцат" => Some(12),
        "тринадцат" => Some(13),
        "четырнадцат" => Some(14),
        "пятнадцат" => Some(15),
        "шестнадцат" => Some(16),
        "семнадцат" => Some(17),
        "восемнадцат" => Some(18),
        "девятнадцат" => Some(19),
        "двадцат" => Some(20),
        "тридцат" => Some(30),
        "сороков" => Some(40),
        "пятидесят" => Some(50),
        "шестидесят" => Some(60),
        "семидесят" => Some(70),
        "восьмидесят" => Some(80),
        "девяност" => Some(90),
        "сот" => Some(100),
        _ => None,
    }
}

fn cardinals_map(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "двадцать" => Some(20),
        "тридцать" => Some(30),
        "сорок" => Some(40),
        "пятьдесят" => Some(50),
        "шестьдесят" => Some(60),
        "семьдесят" => Some(70),
        "восемьдесят" => Some(80),
        "девяносто" => Some(90),
        "сто" => Some(100),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal 21..99".to_string(),
            pattern: vec![
                regex("(двадцать|тридцать|сорок|пятьдесят|шестьдесят|семьдесят|восемьдесят|девяносто)"),
                regex("(перв|втор|трет|четв[её]рт|пят|шест|седьм|восьм|девят)(ье(го|й)?|ого|ому|ый|ой|ий|ая|ое|ья|ые|ым|ых)"),
            ],
            production: Box::new(|nodes| {
                let dozen = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => cardinals_map(m.group(1)?)?,
                    _ => return None,
                };
                let unit = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => ordinals_first_map(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(dozen + unit)))
            }),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+)-?((ы|о|и|а|е|ь)?(ее|й|я|е|го|му?))")],
            production: Box::new(|nodes| {
                let value: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(value)))
            }),
        },
        Rule {
            name: "ordinals (first..20th, then 30th, 40th, ..., 100th)".to_string(),
            pattern: vec![regex("(перв|втор|трет|четв[её]рт|пят|шест|седьм|восьм|девят|десят|одиннадцат|двенадцат|тринадцат|четырнадцат|пятнадцат|шестнадцат|семнадцат|восемнадцат|девятнадцат|двадцат|тридцат|сороков|пятидесят|шестидесят|семидесят|восьмидесят|девяност|сот)(ь(его|ему|ей|ем|им|их|и|е)|ого|ому|ый|ой|ий|ая|ое|ья|ом|ые|ым|ых)")],
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
