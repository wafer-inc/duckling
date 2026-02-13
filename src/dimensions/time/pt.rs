use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (pt)".to_string(),
            pattern: vec![regex("agora|j[áa]|nesse instante|neste instante|nesse momento|neste momento")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (pt)".to_string(),
            pattern: vec![regex("hoje")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (pt)".to_string(),
            pattern: vec![regex("amanh[ãa]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (pt)".to_string(),
            pattern: vec![regex("ontem")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (pt)".to_string(),
            pattern: vec![regex("segunda((\\s|\\-)feira)?|seg\\.?|ter(ç|c)a((\\s|\\-)feira)?|ter\\.|quarta((\\s|\\-)feira)?|qua\\.?|quinta((\\s|\\-)feira)?|qui\\.?|sexta((\\s|\\-)feira)?|sex\\.?|s(á|a)bado|s(á|a)b\\.?|domingo|dom\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("seg") || s.starts_with("segunda") {
                    0
                } else if s.starts_with("ter") {
                    1
                } else if s.starts_with("qua") || s.starts_with("quarta") {
                    2
                } else if s.starts_with("qui") || s.starts_with("quinta") {
                    3
                } else if s.starts_with("sex") || s.starts_with("sexta") {
                    4
                } else if s.starts_with("sá") || s.starts_with("sa") {
                    5
                } else if s.starts_with("dom") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "5 de maio (pt)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+de\\s+maio")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day, year: None })))
            }),
        },
    ]);
    rules
}
