use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule { name: "now (zh)".to_string(), pattern: vec![regex("现在|現在|此时|此時|当前|當前|宜家|而家|依家")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))) },
        Rule { name: "today (zh)".to_string(), pattern: vec![regex("今天|今日|此刻")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))) },
        Rule { name: "tomorrow (zh)".to_string(), pattern: vec![regex("明天")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))) },
        Rule { name: "yesterday (zh)".to_string(), pattern: vec![regex("昨天")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))) },
        Rule {
            name: "guoqing (zh)".to_string(),
            pattern: vec![regex("国庆|國慶")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 1, year: None })))),
        },
    ]);
    rules
}
