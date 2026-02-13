use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (he)".to_string(),
            pattern: vec![regex("עכשיו|מייד")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (he)".to_string(),
            pattern: vec![regex("היום")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (he)".to_string(),
            pattern: vec![regex("מחר")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (he)".to_string(),
            pattern: vec![regex("אתמול")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (he)".to_string(),
            pattern: vec![regex("ראשון|שני|שלישי|רביעי|חמישי|שישי|שבת")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = match s {
                    "שני" => 0,
                    "שלישי" => 1,
                    "רביעי" => 2,
                    "חמישי" => 3,
                    "שישי" => 4,
                    "שבת" => 5,
                    "ראשון" => 6,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "date with march (he)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+למרץ")],
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
            name: "march day reversed (he)".to_string(),
            pattern: vec![regex("במרץ\\s*(\\d{1,2})")],
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
            name: "mid march (he)".to_string(),
            pattern: vec![regex("באמצע מרץ")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 15,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "hebrew feb date (he)".to_string(),
            pattern: vec![regex("ה?(\\d{1,2})\\s+בפברואר|ה?(\\d{1,2})\\s+לפברואר")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
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
            name: "february day reversed (he)".to_string(),
            pattern: vec![regex("פברואר\\s*(\\d{1,2})")],
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
            name: "named month day reversed (he)".to_string(),
            pattern: vec![regex("(ינואר|פברואר|מרץ|אפריל|מאי|יוני|יולי|אוגוסט|ספטמבר|אוקטובר|נובמבר|דצמבר)\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (mname, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let month = match mname {
                    "ינואר" => 1,
                    "פברואר" => 2,
                    "מרץ" => 3,
                    "אפריל" => 4,
                    "מאי" => 5,
                    "יוני" => 6,
                    "יולי" => 7,
                    "אוגוסט" => 8,
                    "ספטמבר" => 9,
                    "אוקטובר" => 10,
                    "נובמבר" => 11,
                    "דצמבר" => 12,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
    ]);
    rules
}
