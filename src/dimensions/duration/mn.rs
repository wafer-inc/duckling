use crate::dimensions::numeral::helpers::numeral_data;
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

fn is_natural(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value > 0.0 && d.value.fract() == 0.0)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "half of a grain".to_string(),
            pattern: vec![regex("хагас\\s?(жил|сар|өдөр|цаг|минут)")],
            production: Box::new(|nodes| {
                let g = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => match m.group(1)? {
                        "жил" => Grain::Year,
                        "сар" => Grain::Month,
                        "өдөр" => Grain::Day,
                        "цаг" => Grain::Hour,
                        "минут" => Grain::Minute,
                        _ => return None,
                    },
                    _ => return None,
                };
                let out = match g {
                    Grain::Year => DurationData::new(6, Grain::Month),
                    Grain::Month => DurationData::new(15, Grain::Day),
                    Grain::Day => DurationData::new(12, Grain::Hour),
                    Grain::Hour => DurationData::new(30, Grain::Minute),
                    Grain::Minute => DurationData::new(30, Grain::Second),
                    _ => return None,
                };
                Some(TokenData::Duration(out))
            }),
        },
        Rule {
            name: "<integer> + '\"'".to_string(),
            pattern: vec![predicate(is_natural), regex("(['\\\"])")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let q = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                match q {
                    "'" => Some(TokenData::Duration(DurationData::new(v, Grain::Minute))),
                    "\"" => Some(TokenData::Duration(DurationData::new(v, Grain::Second))),
                    _ => None,
                }
            }),
        },
        Rule {
            name: "about|exactly <duration>".to_string(),
            pattern: vec![regex("(ойролцоогоор|яг)"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "a <unit-of-duration>".to_string(),
            pattern: vec![dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let g = match &nodes[0].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(1, g)))
            }),
        },
        Rule {
            name: "<positive-numeral> <time-grain>".to_string(),
            pattern: vec![
                predicate(
                    |td| matches!(td, TokenData::Numeral(n) if n.value > 0.0 && n.value.fract() != 0.0),
                ),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let seconds = match g {
                    Grain::NoGrain | Grain::Second => v,
                    Grain::Minute => v * 60.0,
                    Grain::Hour => v * 3600.0,
                    Grain::Day => v * 86400.0,
                    Grain::Week => v * 604800.0,
                    Grain::Month => v * 2592000.0,
                    Grain::Quarter => v * 7776000.0,
                    Grain::Year => v * 31536000.0,
                };
                Some(TokenData::Duration(DurationData::new(
                    seconds.floor() as i64,
                    Grain::Second,
                )))
            }),
        },
    ]
}
