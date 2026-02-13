use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::Grain;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "secunde (grain)".to_string(),
            pattern: vec![regex("sec(und(a|e|ă))?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Second))),
        },
        Rule {
            name: "minute (grain)".to_string(),
            pattern: vec![regex("min(ut(e|ul)?)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Minute))),
        },
        Rule {
            name: "ore (grain)".to_string(),
            pattern: vec![regex("h|or(a|e(le)?|ă)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Hour))),
        },
        Rule {
            name: "zile (grain)".to_string(),
            pattern: vec![regex("zi(le(le)?|u(a|ă))?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Day))),
        },
        Rule {
            name: "saptamani (grain)".to_string(),
            pattern: vec![regex("sapt(a|ă)m(a|â)n(ile|a|ă|i)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Week))),
        },
        Rule {
            name: "luni (grain)".to_string(),
            pattern: vec![regex("lun(i(le)?|a|ă)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Month))),
        },
        Rule {
            name: "trimestru (grain)".to_string(),
            pattern: vec![regex("trimestr(e(le)?|ul?)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Quarter))),
        },
        Rule {
            name: "ani (grain)".to_string(),
            pattern: vec![regex("an(ul|ii?)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Year))),
        },
    ]
}
