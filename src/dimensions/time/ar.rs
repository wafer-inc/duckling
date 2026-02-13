use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (ar)".to_string(),
            pattern: vec![regex("حالا|ال[آا]ن|في هذه اللحظة")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ar)".to_string(),
            pattern: vec![regex("اليوم")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ar)".to_string(),
            pattern: vec![regex("غد[اً]?|بكرا|بكرة")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (ar)".to_string(),
            pattern: vec![regex("[أا]مس|البارحة")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "first of feb (ar)".to_string(),
            pattern: vec![regex("في اول شباط|الاول من شباط|الأول من شباط|الاول من شهر شباط|الأول من شهر شباط")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 2,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "day of week (ar)".to_string(),
            pattern: vec![regex("(ال)?[اإ]ثنين|(ال)?ثلاثاء?|(ال)?[اأ]ربعاء?|(ال)?خميس|(ال)?جمع[ةه]|(ال)?سبت|(ال)?[اأ]حد")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.contains("اثنين") {
                    0
                } else if s.contains("ثلاثاء") {
                    1
                } else if s.contains("اربعاء") || s.contains("أربعاء") {
                    2
                } else if s.contains("خميس") {
                    3
                } else if s.contains("جمع") {
                    4
                } else if s.contains("سبت") {
                    5
                } else if s.contains("احد") || s.contains("أحد") {
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
