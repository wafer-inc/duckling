use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::Grain;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "second (grain)".to_string(),
            pattern: vec![regex(r"sekund(y|zie|(e|ę)|om|ami|ach|o|a)?|s")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Second))),
        },
        Rule {
            name: "minute (grain)".to_string(),
            pattern: vec![regex(r"minut(y|cie|(e|ę)|om|o|ami|ach|(a|ą))?|m")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Minute))),
        },
        Rule {
            name: "hour (grain)".to_string(),
            pattern: vec![regex(r"h|godzin(y|(e|ę)|ie|om|o|ami|ach|(a|ą))?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Hour))),
        },
        Rule {
            name: "day (grain)".to_string(),
            pattern: vec![regex(r"dzie(n|ń|ni(a|ą))|dni(owi|ach|a|ą)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Day))),
        },
        Rule {
            name: "week (grain)".to_string(),
            pattern: vec![regex(r"tydzie(n|ń|)|tygod(ni(owi|u|a|em))|tygodn(iach|iami|iom|ie|i)|tyg\.?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Week))),
        },
        Rule {
            name: "month (grain)".to_string(),
            pattern: vec![regex(r"miesi(a|ą)c(owi|em|u|e|om|ami|ach|a)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Month))),
        },
        Rule {
            name: "quarter (grain)".to_string(),
            pattern: vec![regex(r"kwarta(l|ł)(u|owi|em|e|(o|ó)w|om|ach|ami|y)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Quarter))),
        },
        Rule {
            name: "year (grain)".to_string(),
            pattern: vec![regex(r"rok(u|owi|iem)?|lat(ami|ach|a|om)?")],
            production: Box::new(|_| Some(TokenData::TimeGrain(Grain::Year))),
        },
    ]
}
