use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

fn is_dom_token(td: &TokenData) -> bool {
    match td {
        TokenData::Numeral(n) => {
            let v = n.value as i64;
            (1..=31).contains(&v) && (n.value - v as f64).abs() < f64::EPSILON
        }
        TokenData::Ordinal(o) => (1..=31).contains(&o.value),
        _ => false,
    }
}

fn dom_value(td: &TokenData) -> Option<u32> {
    match td {
        TokenData::Numeral(n) if (n.value - n.value.floor()).abs() < f64::EPSILON => {
            let v = n.value as i64;
            if (1..=31).contains(&v) { Some(v as u32) } else { None }
        }
        TokenData::Ordinal(o) if (1..=31).contains(&o.value) => Some(o.value as u32),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule { name: "now (ka)".to_string(), pattern: vec![regex("ახლა|ეხლა")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))) },
        Rule { name: "today (ka)".to_string(), pattern: vec![regex("დღეს")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))) },
        Rule { name: "tomorrow (ka)".to_string(), pattern: vec![regex("ხვალ")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))) },
        Rule { name: "yesterday (ka)".to_string(), pattern: vec![regex("გუშინ")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))) },
        Rule { name: "month may (ka)".to_string(), pattern: vec![regex("მაის(ი|ში)?")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(5))))) },
        Rule { name: "month feb (ka)".to_string(), pattern: vec![regex("თებერვალ(ი|ს)")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(2))))) },
        Rule {
            name: "day of week (ka)".to_string(),
            pattern: vec![regex("ორშაბათი?ს?|სამშაბათი?ს?|ოთხშაბათი?ს?|ხუთშაბათი?ს?|პარასკევი?ს?|შაბათი?ს?|კვირას?|კვირის")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.contains("ორშაბათ") {
                    0
                } else if s.contains("სამშაბათ") {
                    1
                } else if s.contains("ოთხშაბათ") {
                    2
                } else if s.contains("ხუთშაბათ") {
                    3
                } else if s.contains("პარასკევ") {
                    4
                } else if s.contains("შაბათ") {
                    5
                } else if s.contains("კვირა") || s.contains("კვირის") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "year with -ში (ka)".to_string(),
            pattern: vec![regex("(\\d{4})-?ში")],
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
            name: "<dom> feb (ka)".to_string(),
            pattern: vec![predicate(is_dom_token), regex("თებერვალ(ი|ს)")],
            production: Box::new(|nodes| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: dom_value(&nodes[0].token_data)?, year: None })))
            }),
        },
        Rule {
            name: "1-ლი მარტი (ka)".to_string(),
            pattern: vec![regex("(\\d{1,2})-?ლი\\s*მარტი?ს?")],
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
