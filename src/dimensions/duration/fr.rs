use crate::dimensions::numeral::helpers::{is_natural, numeral_data};
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;
use crate::dimensions::time_grain::Grain;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "une <unit-of-duration>".to_string(),
            pattern: vec![regex("une|la|le?"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(1, grain)))
            }),
        },
        Rule {
            name: "un quart d'heure".to_string(),
            pattern: vec![regex("(1/4\\s?h(eure)?|(un|1) quart d'heure)")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(15, Grain::Minute)))
            }),
        },
        Rule {
            name: "une demi heure".to_string(),
            pattern: vec![regex("(1/2\\s?h(eure)?|(1|une) demi(e)?(\\s|-)heure)")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))
            }),
        },
        Rule {
            name: "trois quarts d'heure".to_string(),
            pattern: vec![regex("(3/4\\s?h(eure)?|(3|trois) quart(s)? d'heure)")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(45, Grain::Minute)))
            }),
        },
        Rule {
            name: "environ <duration>".to_string(),
            pattern: vec![regex("environ"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "<integer> + '\"".to_string(),
            pattern: vec![crate::pattern::predicate(is_natural), regex("(['\"])")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let q = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = match q {
                    "'" => Grain::Minute,
                    "\"" => Grain::Second,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(v, grain)))
            }),
        },
    ]
}
