use crate::dimensions::time_grain::Grain;
use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::DurationData;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<integer> counter months".to_string(),
            pattern: vec![regex(r"(\d+)\s*(ケ|ヶ|カ|ヵ|か|箇)(月|げつ|つき)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let v: i64 = m.group(1)?.parse().ok()?;
                Some(TokenData::Duration(DurationData::new(v, Grain::Month)))
            }),
        },
        Rule {
            name: "one unit".to_string(),
            pattern: vec![regex(
                r"一\s*(秒(毎|間)?|分(毎|間)?|時(毎|間)?|曜?日(毎|間)?|週(毎|間)?|月(毎|間)?|年(毎|間)?)",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let u = m.group(1)?;
                let g = if u.contains('秒') {
                    Grain::Second
                } else if u.contains('分') {
                    Grain::Minute
                } else if u.contains('時') {
                    Grain::Hour
                } else if u.contains('日') {
                    Grain::Day
                } else if u.contains('週') {
                    Grain::Week
                } else if u.contains('月') {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Duration(DurationData::new(1, g)))
            }),
        },
        Rule {
            name: "four minutes".to_string(),
            pattern: vec![regex(r"四\s*分(毎|間)?")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(4, Grain::Minute)))
            }),
        },
        Rule {
            name: "one hundred days".to_string(),
            pattern: vec![regex(r"百\s*日(毎|間)?")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(100, Grain::Day)))),
        },
    ]
}
