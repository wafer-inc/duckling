use crate::dimensions::numeral::helpers::{is_natural, numeral_data};
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

fn duration_data(td: &TokenData) -> Option<&DurationData> {
    match td {
        TokenData::Duration(d) => Some(d),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "quarter of an hour".to_string(),
            pattern: vec![regex("1/4\\s?(h|u(ur)?)|kwartier")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(15, Grain::Minute)))),
        },
        Rule {
            name: "<integer> kwartier".to_string(),
            pattern: vec![predicate(is_natural), regex("kwartier")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                Some(TokenData::Duration(DurationData::new(15_i64.checked_mul(v)?, Grain::Minute)))
            }),
        },
        Rule {
            name: "composite <duration> (with ,/and)".to_string(),
            pattern: vec![
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
                regex(",|en|plus"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = duration_data(&nodes[3].token_data)?;
                if g <= dd.grain {
                    return None;
                }
                Some(TokenData::Duration(DurationData::new(v, g).combine(dd)?))
            }),
        },
        Rule {
            name: "composite <duration>".to_string(),
            pattern: vec![
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = duration_data(&nodes[2].token_data)?;
                if g <= dd.grain {
                    return None;
                }
                Some(TokenData::Duration(DurationData::new(v, g).combine(dd)?))
            }),
        },
        Rule {
            name: "half an hour".to_string(),
            pattern: vec![regex("(1/2\\s?uur|half uur)")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))),
        },
        Rule {
            name: "three-quarters of an hour".to_string(),
            pattern: vec![regex("3/4\\s?uur")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(45, Grain::Minute)))),
        },
        Rule {
            name: "number,number uur".to_string(),
            pattern: vec![regex("(\\d+)\\,(\\d+) *(uur|uren)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let h: i64 = m.group(1)?.parse().ok()?;
                let frac_str = m.group(2)?;
                let frac_num: i64 = frac_str.parse().ok()?;
                let denom: i64 = 10_i64.pow(frac_str.len() as u32);
                let total_minutes = 60_i64.checked_mul(h)?.checked_add(frac_num.checked_mul(60)?.checked_div(denom)?)?;
                Some(TokenData::Duration(DurationData::new(
                    total_minutes,
                    Grain::Minute,
                )))
            }),
        },
        Rule {
            name: "<integer> and an half hour".to_string(),
            pattern: vec![predicate(is_natural), regex("en een half (uur|uren)")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                Some(TokenData::Duration(DurationData::new(60_i64.checked_mul(v)?.checked_add(30)?, Grain::Minute)))
            }),
        },
        Rule {
            name: "1 and an half hour".to_string(),
            pattern: vec![regex("anderhalf uur")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(90, Grain::Minute)))),
        },
        Rule {
            name: "about|exactly <duration>".to_string(),
            pattern: vec![regex("(ongeveer|precies|plusminus|exact)"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "<integer> + '\"".to_string(),
            pattern: vec![predicate(is_natural), regex("(['\"])")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let q = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let g = match q {
                    "'" => Grain::Minute,
                    "\"" => Grain::Second,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(v, g)))
            }),
        },
    ]
}
