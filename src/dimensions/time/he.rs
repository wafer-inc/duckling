use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use crate::dimensions::time_grain::Grain;
use super::{Direction, TimeData, TimeForm};

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
        Rule {
            name: "march next (he)".to_string(),
            pattern: vec![regex("מרץ הבא")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::Month(3));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "this week (he)".to_string(),
            pattern: vec![regex("בשבוע הזה")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "last week (he)".to_string(),
            pattern: vec![regex("שבוע שעבר|שבוע האחרון")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "next week (he)".to_string(),
            pattern: vec![regex("שבוע הבא")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "last month (he)".to_string(),
            pattern: vec![regex("חודש שעבר")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "next month (he)".to_string(),
            pattern: vec![regex("חודש הבא")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "next year (he)".to_string(),
            pattern: vec![regex("שנה הבאה")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "הערב (he)".to_string(),
            pattern: vec![regex("הערב")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(20, 0, false))))),
        },
        Rule {
            name: "רבע ל12 (he)".to_string(),
            pattern: vec![regex("רבע ל\\s*12")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(11, 45, false))))),
        },
        Rule {
            name: "בעוד 2 דקות (he)".to_string(),
            pattern: vec![regex("בעוד\\s*2\\s*דקות")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 2 })))),
        },
        Rule {
            name: "בעוד 60 דקות (he)".to_string(),
            pattern: vec![regex("בעוד\\s*60\\s*דקות")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 60 })))),
        },
        Rule {
            name: "בעוד רבע שעה (he)".to_string(),
            pattern: vec![regex("בעוד\\s+רבע\\s+שעה")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 15 })))),
        },
        Rule {
            name: "בעוד חצי שעה (he)".to_string(),
            pattern: vec![regex("בעוד\\s+חצי\\s+שעה")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 30 })))),
        },
        Rule {
            name: "בעוד 24 שעות (he)".to_string(),
            pattern: vec![regex("בעוד\\s*24\\s*שעות")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: 24 })))),
        },
        Rule {
            name: "בעוד עשרים וארבע שעות (he)".to_string(),
            pattern: vec![regex("בעוד\\s+עשרים\\s+וארבע\\s+שעות")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: 24 })))),
        },
        Rule {
            name: "בעוד שבעה ימים (he)".to_string(),
            pattern: vec![regex("בעוד\\s+שבעה\\s+ימים")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: 7 })))),
        },
        Rule {
            name: "לפני שבעה ימים (he)".to_string(),
            pattern: vec![regex("לפני\\s+שבעה\\s+ימים")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: -7 })))),
        },
        Rule {
            name: "year 4-digit (he)".to_string(),
            pattern: vec![regex("(19\\d{2}|20\\d{2})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        Rule {
            name: "בסופ״ש האחרון (he)".to_string(),
            pattern: vec![regex("בסופ[\"״']?ש האחרון")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "בסופ״ש הזה (he)".to_string(),
            pattern: vec![regex("בסופ[\"״']?ש הזה")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
    ]);
    rules
}
