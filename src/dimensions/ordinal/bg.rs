use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn ord_lookup(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "първ" => Some(1),
        "втор" => Some(2),
        "трет" => Some(3),
        "четвърт" => Some(4),
        "пет" => Some(5),
        "шест" => Some(6),
        "седм" => Some(7),
        "осм" => Some(8),
        "девет" => Some(9),
        "десет" => Some(10),
        "единадесет" => Some(11),
        "дванадесет" => Some(12),
        "тринадесет" => Some(13),
        "четиринадесет" => Some(14),
        "петнадесет" => Some(15),
        "шестнадесет" => Some(16),
        "седемнадесет" => Some(17),
        "осемнадесет" => Some(18),
        "деветнадесет" => Some(19),
        "двадесет" => Some(20),
        _ => None,
    }
}

fn dozens_lookup(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "двадесет" => Some(20),
        "тридесет" => Some(30),
        "четирдесет" => Some(40),
        "петдесет" => Some(50),
        "шестдесет" => Some(60),
        "седемдесет" => Some(70),
        "осемдесет" => Some(80),
        "деветдесет" => Some(90),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal 21..99".to_string(),
            pattern: vec![
                regex("(двадесет|тридесет|четирдесет|петдесет|шестдесет|седемдесет|осемдесет|деветдесет)"),
                regex("и (първ|втор|трет|четвърт|пет|шест|седм|осм|девет)(и(ят?|те)?|а(та)?|о(то)?)"),
            ],
            production: Box::new(|nodes| {
                let m1 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let m2 = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(
                    dozens_lookup(m1)?.checked_add(ord_lookup(m2)?)?,
                )))
            }),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+)-?((в|р|м|т)(и(я(т)?|те)?|а(та)?|о(то)?))")],
            production: Box::new(|nodes| {
                let v: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
        Rule {
            name: "ordinals (first..19th)".to_string(),
            pattern: vec![regex("(първ|втор|трет|четвърт|пет|шест|седм|осм|девет|десет|единадесет|дванадесет|тринадесет|четиринадесет|петнадесет|шестнадесет|седемнадесет|осемнадесет|деветнадесет|двадесет)(и(я(т)?|те)?|а(та)?|о(то)?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(ord_lookup(s)?)))
            }),
        },
    ]
}
