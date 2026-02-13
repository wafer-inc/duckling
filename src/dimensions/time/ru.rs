use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (ru)".to_string(),
            pattern: vec![regex("сейчас")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ru)".to_string(),
            pattern: vec![regex("сегодня")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ru)".to_string(),
            pattern: vec![regex("завтра")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (ru)".to_string(),
            pattern: vec![regex("вчера")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (ru)".to_string(),
            pattern: vec![regex("понедельник(а)?|пн|вторник(а)?|вт|сред(а|у)|ср|четверг(а)?|чт|пятниц(а|у)|пт|суббот(а|у)|сб|воскресенье|вс|в\\s+пятницу")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.trim(),
                    _ => return None,
                };
                let dow = match s {
                    "понедельник" | "понедельника" | "пн" => 0,
                    "вторник" | "вторника" | "вт" => 1,
                    "среда" | "среду" | "ср" => 2,
                    "четверг" | "четверга" | "чт" => 3,
                    "пятница" | "пятницу" | "пт" | "в пятницу" => 4,
                    "суббота" | "субботу" | "сб" => 5,
                    "воскресенье" | "вс" => 6,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "18 февраля (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})(-го|-е)?\\s+феврал[яь]")],
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
            name: "восемнадцатое февраля (ru)".to_string(),
            pattern: vec![regex("восемнадцат(ое|ого)\\s+феврал[яь]")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 2,
                    day: 18,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "1 марта (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+март[а]?")],
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
            name: "первое марта (ru)".to_string(),
            pattern: vec![regex("перв(ое|ого)\\s+март[а]?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
    ]);
    rules
}
