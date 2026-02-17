use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{TemperatureData, TemperatureUnit};

fn temperature_data(td: &TokenData) -> Option<&TemperatureData> {
    match td {
        TokenData::Temperature(d) => Some(d),
        _ => None,
    }
}

fn is_value_only(allow_degree: bool) -> crate::types::PatternItem {
    predicate(move |td| match td {
        TokenData::Temperature(d) => {
            d.value.is_some()
                && d.min_value.is_none()
                && d.max_value.is_none()
                && (d.unit.is_none() || (allow_degree && d.unit == Some(TemperatureUnit::Degree)))
        }
        _ => false,
    })
}

fn is_simple_temperature(td: &TokenData) -> bool {
    matches!(td, TokenData::Temperature(d) if d.value.is_some() && d.unit.is_some() && d.min_value.is_none() && d.max_value.is_none())
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "number as temp".to_string(),
            pattern: vec![dim(DimensionKind::Numeral)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(TemperatureData::new(n.value)))
            }),
        },
        Rule {
            name: "<latent temp> градус".to_string(),
            pattern: vec![is_value_only(false), regex("градус|°|хэм")],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Degree),
                ))
            }),
        },
        Rule {
            name: "<temp> Celsius".to_string(),
            pattern: vec![is_value_only(true), regex("c(el[cs]?(ius)?)?\\.?")],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Celsius),
                ))
            }),
        },
        Rule {
            name: "<temp> °C".to_string(),
            pattern: vec![is_value_only(true), regex("c")],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Celsius),
                ))
            }),
        },
        Rule {
            name: "<temp> Fahrenheit".to_string(),
            pattern: vec![
                is_value_only(true),
                regex("((f(ah?rh?eh?n(h?eit)?)?\\.?)|фарангейт)"),
            ],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Fahrenheit),
                ))
            }),
        },
        Rule {
            name: "<temp> below zero".to_string(),
            pattern: vec![is_value_only(true), regex("тэгээс доош")],
            production: Box::new(|nodes| {
                let mut td = temperature_data(&nodes[0].token_data)?.clone();
                td.value = Some(-td.value?);
                if td.unit.is_none() {
                    td.unit = Some(TemperatureUnit::Degree);
                }
                Some(TokenData::Temperature(td))
            }),
        },
        Rule {
            name: "under/less/lower/no more than <temp>".to_string(),
            pattern: vec![
                regex("доогуур|(бага|ихгүй|их биш)"),
                predicate(is_simple_temperature),
            ],
            production: Box::new(|nodes| {
                let d = temperature_data(&nodes[1].token_data)?;
                Some(TokenData::Temperature(
                    TemperatureData::unit_only(d.unit?).with_max(d.value?),
                ))
            }),
        },
        Rule {
            name: "over/above/at least/more than <temp>".to_string(),
            pattern: vec![
                regex("дээгүүр|их|багадаа"),
                predicate(is_simple_temperature),
            ],
            production: Box::new(|nodes| {
                let d = temperature_data(&nodes[1].token_data)?;
                Some(TokenData::Temperature(
                    TemperatureData::unit_only(d.unit?).with_min(d.value?),
                ))
            }),
        },
    ]
}
