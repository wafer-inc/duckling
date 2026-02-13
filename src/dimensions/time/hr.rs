use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (hr)".to_string(),
            pattern: vec![regex("sad|ovaj tren")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (hr)".to_string(),
            pattern: vec![regex("danas")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (hr)".to_string(),
            pattern: vec![regex("sutra")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (hr)".to_string(),
            pattern: vec![regex("ju[čc]er")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (hr)".to_string(),
            pattern: vec![regex("ponedjelja?ka?|pon\\.?|utora?ka?|uto?\\.?|srijed(a|e|u)|sri\\.?|(č|c)etvrta?ka?|(č|c)et\\.?|peta?ka?|pet\\.?|subot(a|e|u)|sub?\\.?|nedjelj(a|e|u)|ned\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("pon") {
                    0
                } else if s.starts_with("uto") {
                    1
                } else if s.starts_with("sri") || s.starts_with("srijed") {
                    2
                } else if s.starts_with("čet") || s.starts_with("cet") {
                    3
                } else if s.starts_with("pet") {
                    4
                } else if s.starts_with("sub") {
                    5
                } else if s.starts_with("ned") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "1. ozujak (hr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.?\\s*o(z|ž)uja?k")],
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
