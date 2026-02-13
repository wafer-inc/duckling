use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::Grain;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "second (grain)".to_string(),
            pattern: vec![regex("сек(унд(а|и))?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Second))),
        },
        Rule {
            name: "minute (grain)".to_string(),
            pattern: vec![regex("мин(ут(а|и))?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Minute))),
        },
        Rule {
            name: "hour (grain)".to_string(),
            pattern: vec![regex("ч(ас(ове(те)?|а|ът)?)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Hour))),
        },
        Rule {
            name: "day (grain)".to_string(),
            pattern: vec![regex("ден(а|я(т)?)?|дни(те)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Day))),
        },
        Rule {
            name: "week (grain)".to_string(),
            pattern: vec![regex("седмица(та)?|седмици(те)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Week))),
        },
        Rule {
            name: "month (grain)".to_string(),
            pattern: vec![regex("месец(и(те)?|а|ът)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Month))),
        },
        Rule {
            name: "quarter (grain)".to_string(),
            pattern: vec![regex("тримесечи(я|е(то|та)?)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Quarter))),
        },
        Rule {
            name: "year (grain)".to_string(),
            pattern: vec![regex("г(од(ин(а(та)?|и(те)?))?)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Year))),
        },
    ]
}
