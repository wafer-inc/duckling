use crate::dimensions::numeral::helpers::numeral_data;
use crate::dimensions::time_grain::Grain;
use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::DurationData;

fn is_natural(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value > 0.0 && d.value.fract() == 0.0)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "a day - 하루".to_string(),
            pattern: vec![regex("하루")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(1, Grain::Day)))),
        },
        Rule {
            name: "half an hour".to_string(),
            pattern: vec![regex("시(간)?반")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))
            }),
        },
        Rule {
            name: "<integer> and half hours".to_string(),
            pattern: vec![predicate(is_natural), regex("시간반")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                Some(TokenData::Duration(DurationData::new(
                    60_i64.checked_mul(v)?.checked_add(30)?,
                    Grain::Minute,
                )))
            }),
        },
        Rule {
            name: "thirty minutes".to_string(),
            pattern: vec![regex("서른분")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))
            }),
        },
    ]
}
