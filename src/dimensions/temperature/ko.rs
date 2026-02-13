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
            name: "<latent temp> degrees".to_string(),
            pattern: vec![is_value_only(false), regex("도|°")],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Degree),
                ))
            }),
        },
        Rule {
            name: "섭씨 <temp>".to_string(),
            pattern: vec![regex("섭씨"), is_value_only(true)],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[1].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Celsius),
                ))
            }),
        },
        Rule {
            name: "화씨 <temp>".to_string(),
            pattern: vec![regex("화씨"), is_value_only(true)],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[1].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Fahrenheit),
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
            name: "<temp> °F".to_string(),
            pattern: vec![is_value_only(true), regex("f")],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Fahrenheit),
                ))
            }),
        },
    ]
}
