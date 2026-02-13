use crate::dimensions::time_grain::Grain;
use crate::pattern::regex;
use crate::types::{Rule, TokenData};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "segundo (grain)".to_string(),
            pattern: vec![regex("seg(undo)?s?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Second))),
        },
        Rule {
            name: "minutos (grain)".to_string(),
            pattern: vec![regex("min(uto)?s?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Minute))),
        },
        Rule {
            name: "hora (grain)".to_string(),
            pattern: vec![regex("h(ora)?s?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Hour))),
        },
        Rule {
            name: "dia (grain)".to_string(),
            pattern: vec![regex("d(í|i)as?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Day))),
        },
        Rule {
            name: "semana (grain)".to_string(),
            pattern: vec![regex("semanas?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Week))),
        },
        Rule {
            name: "mes (grain)".to_string(),
            pattern: vec![regex("mes(es)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Month))),
        },
        Rule {
            name: "trimestre (grain)".to_string(),
            pattern: vec![regex("trimestres?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Quarter))),
        },
        Rule {
            name: "año (grain)".to_string(),
            pattern: vec![regex("a(n|ñ)os?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Year))),
        },
    ]
}
