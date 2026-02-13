use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (da)".to_string(),
            pattern: vec![regex("nu")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (da)".to_string(),
            pattern: vec![regex("i dag|idag")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (da)".to_string(),
            pattern: vec![regex("i morgen|imorgen")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (da)".to_string(),
            pattern: vec![regex("i g[åa]r|ig[åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (da)".to_string(),
            pattern: vec![regex("mandag|tirsdag|onsdag|torsdag|tors\\.?|fredag|fre\\.?|l[øo]rdag|l[øo]r\\.?|s[øo]ndag|s[øo]n\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("mandag") {
                    0
                } else if s.contains("tirsdag") {
                    1
                } else if s.contains("onsdag") {
                    2
                } else if s.contains("torsdag") || s.starts_with("tors") {
                    3
                } else if s.contains("fredag") || s.starts_with("fre") {
                    4
                } else if s.contains("rdag") || s.contains("lørdag") || s.starts_with("lør") || s.starts_with("lor") {
                    5
                } else if s.contains("ndag") || s.starts_with("søn") || s.starts_with("son") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "den første marts (da)".to_string(),
            pattern: vec![regex("den f[øo]rste marts|1\\.\\s*marts|den\\s*1\\.\\s*marts")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "3 marts (da)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.?\\s*marts")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day, year: None })))
            }),
        },
    ]);
    rules
}
