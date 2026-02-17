use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::OrdinalData;

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "fifth ordinal".to_string(),
            pattern: vec![regex("pi(a|ą)t(y|ego|emu|m|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(5)))),
        },
        Rule {
            name: "first ordinal".to_string(),
            pattern: vec![regex("pierw?sz(y|ego|emu|m|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(1)))),
        },
        Rule {
            name: "fourth ordinal".to_string(),
            pattern: vec![regex("czwart(y|ego|emu|ym|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(4)))),
        },
        Rule {
            name: "22nd ordinal no space".to_string(),
            pattern: vec![regex("dwudziest(ym|y|ego|emu|(a|ą)|ej)drugi?(ego|emu|m|(a|ą)|ej)?")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(22)))),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex(
                "0*(\\d+)( |-)?(szy|sza|szym|ego|go|szego|gi(ego|ej)?|st(a|y|ej)|t(ej|y|ego)|ci(ego)?)",
            )],
            production: Box::new(|nodes| {
                let value: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(value)))
            }),
        },
        Rule {
            name: "23rd ordinal no space".to_string(),
            pattern: vec![regex("dwudziest(ym|y|ego|emu|(a|ą)|ej)trzeci(ego|ch|emu|m|mi|ej|(a|ą))?")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(23)))),
        },
        Rule {
            name: "second ordinal".to_string(),
            pattern: vec![regex("drugi?(ego|emu|m|(a|ą)|ej)?")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(2)))),
        },
        Rule {
            name: "seventh ordinal".to_string(),
            pattern: vec![regex("si(o|ó)dm(y|ego|emu|m|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(7)))),
        },
        Rule {
            name: "21st ordinal no space".to_string(),
            pattern: vec![regex("dwudziest(ym|y|ego|emu|(a|ą)|ej)pierw?sz(y|ego|emu|m|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(21)))),
        },
        Rule {
            name: "8th ordinal".to_string(),
            pattern: vec![regex("(o|ó|Ó)sm(y|ego|emu|m|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(8)))),
        },
        Rule {
            name: "17th ordinal".to_string(),
            pattern: vec![regex("siedemnast(ym|y|ego|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(17)))),
        },
        Rule {
            name: "18th ordinal".to_string(),
            pattern: vec![regex("osiemnast(ym|y|ego|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(18)))),
        },
        Rule {
            name: "19th ordinal".to_string(),
            pattern: vec![regex("dziewi(ę|e)tnast(ym|y|ego|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(19)))),
        },
        Rule {
            name: "20th ordinal".to_string(),
            pattern: vec![regex("dwudziest(ym|y|ego|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(20)))),
        },
        Rule {
            name: "21-29th ordinal".to_string(),
            pattern: vec![
                regex("dwudziest(ym|y|ego|emu|(a|ą)|ej)( |-)?"),
                dim(DimensionKind::Ordinal),
            ],
            production: Box::new(|nodes| {
                let value = match &nodes[1].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(20_i64.checked_add(value)?)))
            }),
        },
        Rule {
            name: "30th ordinal".to_string(),
            pattern: vec![regex("trzydziest(ym|y|ego|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(30)))),
        },
        Rule {
            name: "31-39th ordinal".to_string(),
            pattern: vec![
                regex("trzydziest(ym|y|ego|emu|(a|ą)|ej)( |-)?"),
                dim(DimensionKind::Ordinal),
            ],
            production: Box::new(|nodes| {
                let value = match &nodes[1].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(30_i64.checked_add(value)?)))
            }),
        },
        Rule {
            name: "9th ordinal".to_string(),
            pattern: vec![regex("dziewi(a|ą)t(ym|y|ego|em|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(9)))),
        },
        Rule {
            name: "10th ordinal".to_string(),
            pattern: vec![regex("dziesi(a|ą)t(ym|y|ego|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(10)))),
        },
        Rule {
            name: "11th ordinal".to_string(),
            pattern: vec![regex("jedenast(ym|y|ego|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(11)))),
        },
        Rule {
            name: "12th ordinal".to_string(),
            pattern: vec![regex("dwunast(ym|y|ego|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(12)))),
        },
        Rule {
            name: "13th ordinal".to_string(),
            pattern: vec![regex("trzynast(ym|y|ego|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(13)))),
        },
        Rule {
            name: "14th ordinal".to_string(),
            pattern: vec![regex("czternast(ym|y|ego|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(14)))),
        },
        Rule {
            name: "15th ordinal".to_string(),
            pattern: vec![regex("pi(e|ę)tnast(ym|y|ego|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(15)))),
        },
        Rule {
            name: "16th ordinal".to_string(),
            pattern: vec![regex("szesnast(ym|y|ego|emu|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(16)))),
        },
        Rule {
            name: "24th ordinal no space".to_string(),
            pattern: vec![regex("dwudziest(ym|y|ego|emu|(a|ą)|ej)czwart(y|ego|emu|ym|(a|ą)|ej)")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(24)))),
        },
        Rule {
            name: "third ordinal".to_string(),
            pattern: vec![regex("trzeci(ego|ch|emu|m|mi|ej|(a|ą))?")],
            production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(3)))),
        },
    ]
}
