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
            name: "<latent temp> temp".to_string(),
            pattern: vec![is_value_only(false), regex("(grados?)|°")],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Degree),
                ))
            }),
        },
        Rule {
            name: "<temp> Celsius".to_string(),
            pattern: vec![
                is_value_only(true),
                regex("(cent(i|í)grados?|c(el[cs]?(ius)?)?\\.?)"),
            ],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Celsius),
                ))
            }),
        },
        Rule {
            name: "<temp> Fahrenheit".to_string(),
            pattern: vec![is_value_only(true), regex("f(ah?reh?n(h?eit)?)?\\.?")],
            production: Box::new(|nodes| {
                let td = temperature_data(&nodes[0].token_data)?;
                Some(TokenData::Temperature(
                    td.clone().with_unit(TemperatureUnit::Fahrenheit),
                ))
            }),
        },
        Rule {
            name: "<latent temp> temp bajo cero".to_string(),
            pattern: vec![is_value_only(true), regex("bajo cero")],
            production: Box::new(|nodes| {
                let mut td = temperature_data(&nodes[0].token_data)?.clone();
                let v = td.value?;
                td.value = Some(-v);
                if td.unit.is_none() {
                    td.unit = Some(TemperatureUnit::Degree);
                }
                Some(TokenData::Temperature(td))
            }),
        },
    ]
}
