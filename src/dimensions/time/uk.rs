use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (uk)".to_string(),
            pattern: vec![regex("зараз")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (uk)".to_string(),
            pattern: vec![regex("сьогодні")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (uk)".to_string(),
            pattern: vec![regex("завтра")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (uk)".to_string(),
            pattern: vec![regex("вчора")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (uk)".to_string(),
            pattern: vec![regex("понеділ(ок|ка)|пн|вівтор(ок|ка)|вт|серед(а|у)|ср|четвер(га)?|чт|п'ятниц(я|і|ю)|пт|субот(а|и|у)|сб|неділ(я|і|ю)|нд")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("пн") || s.starts_with("понеділ") {
                    0
                } else if s.starts_with("вт") || s.starts_with("вівтор") {
                    1
                } else if s.starts_with("ср") || s.starts_with("серед") {
                    2
                } else if s.starts_with("чт") || s.starts_with("четвер") {
                    3
                } else if s.starts_with("пт") || s.starts_with("п'ятниц") {
                    4
                } else if s.starts_with("сб") || s.starts_with("субот") {
                    5
                } else if s.starts_with("нд") || s.starts_with("неділ") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "date with lutogo (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+лютого")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 2,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "date with bereznya (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+березня")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "pershe bereznya (uk)".to_string(),
            pattern: vec![regex("перше\\s+березня")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "1 бер. (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+бер\\.?")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "15.2 (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (d, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "15 Лют (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Лл]ют\\.?")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 2,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "8 Сер (uk)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Сс]ер\\.?")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 8,
                    day,
                    year: None,
                })))
            }),
        },
    ]);
    rules
}
