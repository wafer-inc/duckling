use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::Grain;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "second (grain)".to_string(),
            pattern: vec![regex("सेकंड")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Second))),
        },
        Rule {
            name: "minute (grain)".to_string(),
            pattern: vec![regex("मिनट")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Minute))),
        },
        Rule {
            name: "hour (grain)".to_string(),
            pattern: vec![regex("घंटा")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Hour))),
        },
        Rule {
            name: "day (grain)".to_string(),
            pattern: vec![regex("(दिन|दिवस)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Day))),
        },
        Rule {
            name: "week (grain)".to_string(),
            pattern: vec![regex("(सप्ताह|हफ़्ता)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Week))),
        },
        Rule {
            name: "month (grain)".to_string(),
            pattern: vec![regex("महीना")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Month))),
        },
        Rule {
            name: "quarter (grain)".to_string(),
            pattern: vec![regex("चौथाई")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Quarter))),
        },
        Rule {
            name: "year (grain)".to_string(),
            pattern: vec![regex("(साल|वर्ष)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Year))),
        },
    ]
}
