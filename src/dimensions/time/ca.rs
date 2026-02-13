use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (ca)".to_string(),
            pattern: vec![regex("ara|ja|en aquest moment|en aquests moments")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ca)".to_string(),
            pattern: vec![regex("avui")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ca)".to_string(),
            pattern: vec![regex("dem[àa]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (ca)".to_string(),
            pattern: vec![regex("ahir")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (ca)".to_string(),
            pattern: vec![regex("dilluns|dl\\.?|dimarts|dm\\.?|dimecres|dc\\.?|dijous|dj\\.?|divendres|dv\\.?|dissabte|ds\\.?|diumenge|dg\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("dilluns") || s.starts_with("dl") {
                    0
                } else if s.starts_with("dimarts") || s.starts_with("dm") {
                    1
                } else if s.starts_with("dimecres") || s.starts_with("dc") {
                    2
                } else if s.starts_with("dijous") || s.starts_with("dj") {
                    3
                } else if s.starts_with("divendres") || s.starts_with("dv") {
                    4
                } else if s.starts_with("dissabte") || s.starts_with("ds") {
                    5
                } else if s.starts_with("diumenge") || s.starts_with("dg") {
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
