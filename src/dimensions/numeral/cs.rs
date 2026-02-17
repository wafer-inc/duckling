use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn cs_number_value(s: &str) -> Option<f64> {
    match s {
        "nula" => Some(0.0),
        "jeden" | "jedna" | "jedno" => Some(1.0),
        "dva" | "dvě" | "dvĕ" => Some(2.0),
        "tři" => Some(3.0),
        "čtyři" => Some(4.0),
        "pět" => Some(5.0),
        "šest" => Some(6.0),
        "sedm" => Some(7.0),
        "osm" => Some(8.0),
        "devět" => Some(9.0),
        "deset" => Some(10.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "number (0..10)".to_string(),
        pattern: vec![regex(
            r"(nula|jed(en|n[ao])|dv(a|ě|ĕ)|t(ř)i|(č)ty(ř)i|p(ě)t|(š)est|sedm|osm|dev(ě)t|deset)",
        )],
        production: Box::new(|nodes| {
            let s = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                _ => return None,
            };
            Some(TokenData::Numeral(NumeralData::new(cs_number_value(&s)?)))
        }),
    }]
}
