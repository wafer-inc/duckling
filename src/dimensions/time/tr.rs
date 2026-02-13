use crate::dimensions::time_grain::Grain;
use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (tr)".to_string(),
            pattern: vec![regex("[şs]imdi|şu an|su an")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (tr)".to_string(),
            pattern: vec![regex("bug[üu]n")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (tr)".to_string(),
            pattern: vec![regex("yar[ıi]n")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (tr)".to_string(),
            pattern: vec![regex("d[üu]n")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week monday (tr)".to_string(),
            pattern: vec![regex("pazartesi'?(si|den|ye)?|pzts?|salı?'?(sı|dan|ya)?|çar(şamba)?'?(sı|dan|ya)?|per(şembe)?'?(si|den|ye)?|cuma?'?(sı|dan|ya)?|cumartesi'?(si|den|ye)?|cmt|paz(ar)?'?(ı|dan|a)?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("paz") || s.starts_with("pzt") {
                    0
                } else if s.starts_with("sal") {
                    1
                } else if s.contains("çarşamba") || s.contains("carsamba") {
                    2
                } else if s.contains("perşembe") || s.contains("persembe") || s.starts_with("per") {
                    3
                } else if s.starts_with("cuma") {
                    4
                } else if s.starts_with("cumartesi") || s == "cmt" {
                    5
                } else if s.starts_with("pazar") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "year (tr)".to_string(),
            pattern: vec![regex("(\\d{4})")],
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
            name: "ay sonu (tr)".to_string(),
            pattern: vec![regex("ay sonu|y[ıi]l sonu")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::AllGrain(Grain::Month)),
                })))
            }),
        },
    ]);
    rules
}
