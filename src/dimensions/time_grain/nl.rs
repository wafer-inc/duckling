use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::Grain;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "second (grain)".to_string(),
            pattern: vec![regex("(seconde(n|s)?|sec|s)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Second))),
        },
        Rule {
            name: "minute (grain)".to_string(),
            pattern: vec![regex("(minuut|minuten|min|m)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Minute))),
        },
        Rule {
            name: "hour (grain)".to_string(),
            pattern: vec![regex("(uur|uren|u|h)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Hour))),
        },
        Rule {
            name: "day (grain)".to_string(),
            pattern: vec![regex("dagen|dag|d")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Day))),
        },
        Rule {
            name: "week (grain)".to_string(),
            pattern: vec![regex("(weken|week|w)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Week))),
        },
        Rule {
            name: "month (grain)".to_string(),
            pattern: vec![regex("(maanden|maand|mnd)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Month))),
        },
        Rule {
            name: "quarter (grain)".to_string(),
            pattern: vec![regex("(kwartaal|kwartalen)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Quarter))),
        },
        Rule {
            name: "year (grain)".to_string(),
            pattern: vec![regex("(jaren|jaar|j)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Year))),
        },
    ]
}
