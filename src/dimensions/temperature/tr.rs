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

fn negate_temp(mut td: TemperatureData) -> TemperatureData {
    td.value = td.value.map(|v| -v);
    if td.unit.is_none() {
        td.unit = Some(TemperatureUnit::Degree);
    }
    td
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
            pattern: vec![is_value_only(false), regex("derece|°")],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Degree),
                ))
            }),
        },
        Rule {
            name: "<temp> Celsius".to_string(),
            pattern: vec![is_value_only(true), regex("c|santigrat")],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Celsius),
                ))
            }),
        },
        Rule {
            name: "<temp> Fahrenheit".to_string(),
            pattern: vec![is_value_only(true), regex("f(ahrenh?ayt)?")],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Fahrenheit),
                ))
            }),
        },
        Rule {
            name: "below zero <temp>".to_string(),
            pattern: vec![regex("sıfırın altında"), is_value_only(true)],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Temperature(negate_temp(td)))
            }),
        },
        Rule {
            name: "<temp> below zero".to_string(),
            pattern: vec![is_value_only(true), regex("sıfırın altında")],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?.clone();
                Some(TokenData::Temperature(negate_temp(td)))
            }),
        },
    ]
}
