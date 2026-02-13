use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn unit_or_ten(s: &str) -> Option<f64> {
    match s {
        "sufuri" | "zero" => Some(0.0),
        "moja" => Some(1.0),
        "mbili" => Some(2.0),
        "tatu" => Some(3.0),
        "nne" => Some(4.0),
        "tano" => Some(5.0),
        "sita" => Some(6.0),
        "saba" => Some(7.0),
        "nane" => Some(8.0),
        "tisa" => Some(9.0),
        "kumi" => Some(10.0),
        _ => None,
    }
}

fn tens(s: &str) -> Option<f64> {
    match s {
        "ishirini" => Some(20.0),
        "thelathini" => Some(30.0),
        "arubaini" | "arobaini" => Some(40.0),
        "hamsini" => Some(50.0),
        "sitini" => Some(60.0),
        "sabini" => Some(70.0),
        "themanini" => Some(80.0),
        "tisini" => Some(90.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "number (0..10)".to_string(),
            pattern: vec![regex("(sufuri|zero|moja|mbili|tatu|nne|tano|sita|saba|nane|tisa|kumi)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(unit_or_ten(&s)?)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(ishirini|thelathini|arubaini|arobaini|hamsini|sitini|sabini|themanini|tisini)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(tens(&s)?)))
            }),
        },
        Rule {
            name: "integer 11..99".to_string(),
            pattern: vec![
                predicate(|td| {
                    matches!(td, TokenData::Numeral(d) if [20.0,30.0,40.0,50.0,60.0,70.0,80.0,90.0].contains(&d.value))
                }),
                regex("-?na-?"),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..10.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let t = numeral_data(&nodes[0].token_data)?.value;
                let u = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(t + u)))
            }),
        },
    ]
}
