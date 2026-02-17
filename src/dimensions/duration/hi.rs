use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "pandrah minute".to_string(),
            pattern: vec![regex("पंद्रह मिनट")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(15, Grain::Minute)))
            }),
        },
        Rule {
            name: "a day".to_string(),
            pattern: vec![regex("((एक दिन)|(एक दिवस)|(दिन|दिवस))")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(1, Grain::Day)))),
        },
        Rule {
            name: "fortnight".to_string(),
            pattern: vec![regex("((एक पखवाड़ा)|(पखवाड़ा)|(पखवाड़ा))")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(14, Grain::Day)))),
        },
        Rule {
            name: "ढाई साल".to_string(),
            pattern: vec![regex("(डेढ़|ढाई)\\s*(साल|वर्ष|बरस)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let n = match m.group(1)? {
                    "डेढ़" => 1,
                    "ढाई" => 2,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(
                    12_i64.checked_mul(n)?.checked_add(6)?,
                    Grain::Month,
                )))
            }),
        },
        Rule {
            name: "डेढ़ घंटा".to_string(),
            pattern: vec![regex("(डेढ़|ढाई)\\s*(घंटा)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let n = match m.group(1)? {
                    "डेढ़" => 1,
                    "ढाई" => 2,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(
                    60_i64.checked_mul(n)?.checked_add(30)?,
                    Grain::Minute,
                )))
            }),
        },
        Rule {
            name: "one year".to_string(),
            pattern: vec![regex("एक (साल|वर्ष|बरस)")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(1, Grain::Year)))),
        },
        Rule {
            name: "half <grain>".to_string(),
            pattern: vec![regex("आधा"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = match g {
                    Grain::Minute => DurationData::new(30, Grain::Second),
                    Grain::Hour => DurationData::new(30, Grain::Minute),
                    Grain::Day => DurationData::new(12, Grain::Hour),
                    Grain::Month => DurationData::new(15, Grain::Day),
                    Grain::Year => DurationData::new(6, Grain::Month),
                    _ => return None,
                };
                Some(TokenData::Duration(dd))
            }),
        },
    ]
}
