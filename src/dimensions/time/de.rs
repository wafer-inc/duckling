use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (de)".to_string(),
            pattern: vec![regex("jetzt|sofort|gerade eben|zu dieser zeit")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (de)".to_string(),
            pattern: vec![regex("heute")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (de)".to_string(),
            pattern: vec![regex("morgen")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (de)".to_string(),
            pattern: vec![regex("gestern")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (de)".to_string(),
            pattern: vec![regex("montags?|mo\\.?|die?nstags?|di\\.?|mittwochs?|mi\\.?|donn?erstags?|do\\.?|freitags?|fr\\.?|samstags?|sonnabends?|sa\\.?|sonntags?|so\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("montag") || s == "mo" || s == "mo." {
                    0
                } else if s.starts_with("dienstag") || s.starts_with("dienstags") || s == "di" || s == "di." {
                    1
                } else if s.starts_with("mittwoch") || s == "mi" || s == "mi." {
                    2
                } else if s.starts_with("donnerstag") || s.starts_with("donnerstags") || s == "do" || s == "do." {
                    3
                } else if s.starts_with("freitag") || s.starts_with("freitags") || s == "fr" || s == "fr." {
                    4
                } else if s.starts_with("samstag") || s.starts_with("samstags") || s.starts_with("sonnabend") || s == "sa" || s == "sa." {
                    5
                } else if s.starts_with("sonntag") || s.starts_with("sonntags") || s == "so" || s == "so." {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "1 märz (de)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*m(ä|ae)rz")],
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
