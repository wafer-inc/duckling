use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::Grain;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "segon (grain)".to_string(),
            pattern: vec![regex("seg(on)?s?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Second))),
        },
        Rule {
            name: "minuts (grain)".to_string(),
            pattern: vec![regex("min(ut)?s?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Minute))),
        },
        Rule {
            name: "hora (grain)".to_string(),
            pattern: vec![regex("h(or(a|es))?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Hour))),
        },
        Rule {
            name: "dia (grain)".to_string(),
            pattern: vec![regex("d(í|i)(a|es)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Day))),
        },
        Rule {
            name: "setmana (grain)".to_string(),
            pattern: vec![regex("setman(a|es)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Week))),
        },
        Rule {
            name: "mes (grain)".to_string(),
            pattern: vec![regex("mes(os)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Month))),
        },
        Rule {
            name: "trimestre (grain)".to_string(),
            pattern: vec![regex("trimestres?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Quarter))),
        },
        Rule {
            name: "any (grain)".to_string(),
            pattern: vec![regex("anys?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Year))),
        },
    ]
}
