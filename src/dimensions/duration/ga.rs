use crate::dimensions::numeral::helpers::numeral_data;
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

fn numeral_is(v: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(n) if (n.value - v).abs() < 1e-9)
}

fn numeral_lt(v: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(n) if n.value < v)
}

fn numeral_in_tens(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(n) if matches!(n.value as i64, 10 | 20 | 30 | 40 | 50) && n.value.fract() == 0.0)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "coicís".to_string(),
            pattern: vec![regex("coic(í|i)s(í|i|e)?")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(14, Grain::Day)))),
        },
        Rule {
            name: "leathuair".to_string(),
            pattern: vec![regex("leathuair(e|eanta)?")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))
            }),
        },
        Rule {
            name: "aon X amhain".to_string(),
            pattern: vec![predicate(numeral_is(1.0)), dim(DimensionKind::TimeGrain), predicate(numeral_is(1.0))],
            production: Box::new(|nodes| {
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(1, g)))
            }),
        },
        Rule {
            name: "<unit-integer> <unit-of-duration> <tens-integer>".to_string(),
            pattern: vec![predicate(numeral_lt(10.0)), dim(DimensionKind::TimeGrain), predicate(numeral_in_tens)],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Duration(DurationData::new((v1 + v2).floor() as i64, g)))
            }),
        },
        Rule {
            name: "composite <duration>".to_string(),
            pattern: vec![predicate(crate::dimensions::numeral::helpers::is_natural), dim(DimensionKind::TimeGrain), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let d2 = duration_data(&nodes[2].token_data)?;
                if g <= d2.grain {
                    return None;
                }
                Some(TokenData::Duration(DurationData::new(v, g).combine(d2)))
            }),
        },
        Rule {
            name: "composite <duration> (with and)".to_string(),
            pattern: vec![predicate(crate::dimensions::numeral::helpers::is_natural), dim(DimensionKind::TimeGrain), regex("agus|is"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let d2 = duration_data(&nodes[3].token_data)?;
                if g <= d2.grain {
                    return None;
                }
                Some(TokenData::Duration(DurationData::new(v, g).combine(d2)))
            }),
        },
    ]
}
