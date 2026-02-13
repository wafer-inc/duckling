use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::Grain;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "seconde (grain)".to_string(),
            pattern: vec![regex("sec(onde)?s?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Second))),
        },
        Rule {
            name: "minute (grain)".to_string(),
            pattern: vec![regex("min(ute)?s?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Minute))),
        },
        Rule {
            name: "heure (grain)".to_string(),
            pattern: vec![regex("heures?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Hour))),
        },
        Rule {
            name: "jour (grain)".to_string(),
            pattern: vec![regex("jour(n(e|é)e?)?s?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Day))),
        },
        Rule {
            name: "semaine (grain)".to_string(),
            pattern: vec![regex("semaines?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Week))),
        },
        Rule {
            name: "mois (grain)".to_string(),
            pattern: vec![regex("mois")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Month))),
        },
        Rule {
            name: "trimestre (grain)".to_string(),
            pattern: vec![regex("trimestres?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Quarter))),
        },
        Rule {
            name: "année (grain)".to_string(),
            pattern: vec![regex("an(n(e|é)e?)?s?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Year))),
        },
    ]
}
