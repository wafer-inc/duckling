use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::Grain;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "soicind (grain)".to_string(),
            pattern: vec![regex("t?sh?oicind(í|i)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Second))),
        },
        Rule {
            name: "nóiméad (grain)".to_string(),
            pattern: vec![regex("n[óo]im(é|e)[ai]da?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Minute))),
        },
        Rule {
            name: "uair (grain)".to_string(),
            pattern: vec![regex("([thn]-?)?uair(e|eanta)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Hour))),
        },
        Rule {
            name: "lá (grain)".to_string(),
            pattern: vec![regex("l(ae(thanta)?|(á|a))")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Day))),
        },
        Rule {
            name: "seachtain (grain)".to_string(),
            pattern: vec![regex("t?sh?eachtain(e|(í|i))?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Week))),
        },
        Rule {
            name: "mí (grain)".to_string(),
            pattern: vec![regex("mh?(í|i)(sa|nna)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Month))),
        },
        Rule {
            name: "ráithe (grain)".to_string(),
            pattern: vec![regex("r(á|a)ith(e|(í|i))")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Quarter))),
        },
        Rule {
            name: "bliain (grain)".to_string(),
            pattern: vec![regex("m?bh?lia(in|na|nta)")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Year))),
        },
    ]
}
