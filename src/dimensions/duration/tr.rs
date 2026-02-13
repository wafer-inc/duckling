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
            name: "quarter of an hour".to_string(),
            pattern: vec![regex("(1/4\\s?sa(at)?|çeyrek saat)")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(15, Grain::Minute)))),
        },
        Rule {
            name: "half an hour".to_string(),
            pattern: vec![regex("(1/2\\s?sa(at)?|yarım saat)")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))),
        },
        Rule {
            name: "three-quarters of an hour".to_string(),
            pattern: vec![regex("(3/4\\s?sa(at)?|üç çeyrek sa(at)?)")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(45, Grain::Minute)))),
        },
        Rule {
            name: "fortnight".to_string(),
            pattern: vec![regex("iki hafta")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(14, Grain::Day)))),
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
            name: "number.number hours".to_string(),
            pattern: vec![regex("(\\d+)(\\.|,)(\\d+) *sa(at)?")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let hh: i64 = m.group(1)?.parse().ok()?;
                let frac = m.group(3)?;
                let num: i64 = frac.parse().ok()?;
                let den: i64 = 10_i64.pow(frac.len() as u32);
                let minutes = 60 * hh + (num * 60) / den;
                Some(TokenData::Duration(DurationData::new(minutes, Grain::Minute)))
            }),
        },
        Rule {
            name: "<integer> and an half hour".to_string(),
            pattern: vec![predicate(is_natural), regex("buçuk sa(at)?")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                Some(TokenData::Duration(DurationData::new(30 + 60 * v, Grain::Minute)))
            }),
        },
        Rule {
            name: "<duration> about|exactly".to_string(),
            pattern: vec![regex("(yaklaşık|tam(\\solarak)?)"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "<duration> about|exactly 2".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex("gibi|civarında")],
            production: Box::new(|nodes| Some(nodes[0].token_data.clone())),
        },
    ]
}
