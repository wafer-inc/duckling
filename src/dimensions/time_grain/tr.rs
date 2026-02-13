use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::Grain;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "saniye (grain)".to_string(),
            pattern: vec![regex("sa?n(iye)?(nin)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Second))),
        },
        Rule {
            name: "dakika (grain)".to_string(),
            pattern: vec![regex("da?k(ika)?(nın)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Minute))),
        },
        Rule {
            name: "saat (grain)".to_string(),
            pattern: vec![regex("sa(at)?(in)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Hour))),
        },
        Rule {
            name: "gün (grain)".to_string(),
            pattern: vec![regex("gün(ün)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Day))),
        },
        Rule {
            name: "hafta (grain)".to_string(),
            pattern: vec![regex("hafta(nın)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Week))),
        },
        Rule {
            name: "ay (grain)".to_string(),
            pattern: vec![regex("ay(ın)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Month))),
        },
        Rule {
            name: "çeyrek yıl (grain)".to_string(),
            pattern: vec![regex("çeyrek yıl(ın)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Quarter))),
        },
        Rule {
            name: "yıl (grain)".to_string(),
            pattern: vec![regex("yıl(ın)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Year))),
        },
    ]
}
