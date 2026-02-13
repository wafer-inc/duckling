use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{DistanceData, DistanceUnit};

fn distance_data(td: &TokenData) -> Option<&DistanceData> {
    match td {
        TokenData::Distance(d) => Some(d),
        _ => None,
    }
}

fn lookup_unit(s: &str) -> Option<DistanceUnit> {
    match s.to_lowercase().as_str() {
        "cm" | "centimetri" | "centimetru" => Some(DistanceUnit::Centimetre),
        "picior" | "picioare" => Some(DistanceUnit::Foot),
        "inch" | "inchi" | "inci" => Some(DistanceUnit::Inch),
        "km" | "kilometri" | "kilometru" => Some(DistanceUnit::Kilometre),
        "m" | "metri" | "metru" => Some(DistanceUnit::Metre),
        "mila" | "milă" | "mile" => Some(DistanceUnit::Mile),
        "y" | "yar" | "yard" | "yarzi" | "yd" | "yzi" => Some(DistanceUnit::Yard),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<latent dist> foot/inch/yard/meter/kilometer/centimeter".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex("(inc(hi?|i)|(centi|kilo)?metr[iu]|mil[eaă]|[ck]?m|picio(are|r)|y(ar)?(zi|d)?)"),
            ],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                let matched = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let u = lookup_unit(matched)?;
                Some(TokenData::Distance(d.clone().with_unit(u)))
            }),
        },
        Rule {
            name: "<latent dist> de foot/inch/yard/meter/kilometer/centimeter".to_string(),
            pattern: vec![
                dim(DimensionKind::Distance),
                regex("de (inc(hi?|i)|(centi|kilo)?metr[iu]|mil[eaă]|[ck]?m|picio(are|r)|y(ar)?(zi|d)?)"),
            ],
            production: Box::new(|nodes| {
                let d = distance_data(&nodes[0].token_data)?;
                let matched = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let u = lookup_unit(matched)?;
                Some(TokenData::Distance(d.clone().with_unit(u)))
            }),
        },
    ]
}
