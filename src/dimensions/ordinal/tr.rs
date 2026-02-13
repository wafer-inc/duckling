use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn lookup_ordinal(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "birinci" => Some(1),
        "ikinci" => Some(2),
        "üçüncü" => Some(3),
        "dördüncü" => Some(4),
        "beşinci" => Some(5),
        "altıncı" => Some(6),
        "yedinci" => Some(7),
        "sekizinci" => Some(8),
        "dokuzuncu" => Some(9),
        "onuncu" => Some(10),
        "on birinci" => Some(11),
        "on ikinci" => Some(12),
        "on üçüncü" => Some(13),
        "on dördüncü" => Some(14),
        "on beşinci" => Some(15),
        "on altıncı" => Some(16),
        "on yedinci" => Some(17),
        "on sekizinci" => Some(18),
        "on dokuzuncu" => Some(19),
        "yirminci" => Some(20),
        "yirmi birinci" => Some(21),
        "yirmi ikinci" => Some(22),
        "yirmi üçüncü" => Some(23),
        "yirmi dördüncü" => Some(24),
        "yirmi beşinci" => Some(25),
        "yirmi altıncı" => Some(26),
        "yirmi yedinci" => Some(27),
        "yirmi sekizinci" => Some(28),
        "yirmi dokuzuncu" => Some(29),
        "otuzuncu" => Some(30),
        "otuz birinci" => Some(31),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+) ?('?)((inci|nci|ıncı|ncı|uncu|ncu|üncü|ncü|\\.))")],
            production: Box::new(|nodes| {
                let v: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
        Rule {
            name: "ordinals (first..31st)".to_string(),
            pattern: vec![regex("(birinci|ikinci|üçüncü|dördüncü|beşinci|altıncı|yedinci|sekizinci|dokuzuncu|onuncu|on birinci|on ikinci|on üçüncü|on dördüncü|on beşinci|on altıncı|on yedinci|on sekizinci|on dokuzuncu|yirminci|yirmi birinci|yirmi ikinci|yirmi üçüncü|yirmi dördüncü|yirmi beşinci|yirmi altıncı|yirmi yedinci|yirmi sekizinci|yirmi dokuzuncu|otuzuncu|otuz birinci)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(lookup_ordinal(text)?)))
            }),
        },
    ]
}
