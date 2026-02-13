use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (ga)".to_string(),
            pattern: vec![regex("anois")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ga)".to_string(),
            pattern: vec![regex("inniu")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ga)".to_string(),
            pattern: vec![regex("am[áa]rach")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (ga)".to_string(),
            pattern: vec![regex("inn[ée]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (ga)".to_string(),
            pattern: vec![regex("d[ée] luain|an luan|d[ée] m[áa]irt|d[ée] c[ée]adaoin|d[ée]ardaoin|d[ée] haoine|d[ée] sathairn|d[ée] domhnaigh")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("luain") || s.contains("luan") {
                    0
                } else if s.contains("márt") || s.contains("mairt") {
                    1
                } else if s.contains("céadaoin") || s.contains("ceadaoin") {
                    2
                } else if s.contains("ardaoin") {
                    3
                } else if s.contains("haoine") {
                    4
                } else if s.contains("sathairn") {
                    5
                } else if s.contains("domhnaigh") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
    ]);
    rules
}
