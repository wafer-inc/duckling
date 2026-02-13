use crate::dimensions::time_grain::Grain;
use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{Direction, TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (ko)".to_string(),
            pattern: vec![regex("방금|지금")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ko)".to_string(),
            pattern: vec![regex("오늘")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ko)".to_string(),
            pattern: vec![regex("내일")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (ko)".to_string(),
            pattern: vec![regex("어제")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (ko)".to_string(),
            pattern: vec![regex("월(요일|욜)|화(요일|욜)|수(요일|욜)|목(요일|욜)|금(요일|욜)|토(요일|욜)|일(요일|욜)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_string(),
                    _ => return None,
                };
                let dow = if s.starts_with('월') {
                    0
                } else if s.starts_with('화') {
                    1
                } else if s.starts_with('수') {
                    2
                } else if s.starts_with('목') {
                    3
                } else if s.starts_with('금') {
                    4
                } else if s.starts_with('토') {
                    5
                } else if s.starts_with('일') {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "month/day (ko)".to_string(),
            pattern: vec![regex("(\\d{1,2})월\\s*(\\d{1,2})일")],
            production: Box::new(|nodes| {
                let (m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
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
            name: "year/month (ko)".to_string(),
            pattern: vec![regex("(\\d{4})년\\s*(\\d{1,2})월")],
            production: Box::new(|nodes| {
                let (y, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day: 1,
                    year: Some(year),
                })))
            }),
        },
        Rule {
            name: "full spoken ymd (ko)".to_string(),
            pattern: vec![regex("이천십오년\\s*삼월\\s*삼일")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 3,
                    year: Some(2015),
                })))
            }),
        },
        Rule {
            name: "dom with suffix (ko)".to_string(),
            pattern: vec![regex("(\\d{1,2})일에")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "yy/mm/dd (ko)".to_string(),
            pattern: vec![regex("(\\d{2})/(\\d{1,2})/(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (yy, m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let year2: i32 = yy.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: Some(1900 + year2),
                })))
            }),
        },
        Rule {
            name: "next month name (ko)".to_string(),
            pattern: vec![regex("다음\\s*(\\d{1,2})월")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::Month(month));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "this week (ko)".to_string(),
            pattern: vec![regex("이번(주)?|금주")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "last week (ko)".to_string(),
            pattern: vec![regex("저번(주)?|지난(주)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
    ]);
    rules
}
