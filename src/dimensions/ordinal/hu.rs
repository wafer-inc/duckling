use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn ordinals_map(s: &str) -> Option<i64> {
    match s {
        "első" => Some(1),
        "második" => Some(2),
        "harmadik" => Some(3),
        "negyedik" => Some(4),
        "ötödik" => Some(5),
        "hatodik" => Some(6),
        "hetedik" => Some(7),
        "nyolcadik" => Some(8),
        "kilencedik" => Some(9),
        "tizedik" => Some(10),
        "huszadik" => Some(20),
        "harmincadik" => Some(30),
        "negyvenedik" => Some(40),
        "ötvenedik" => Some(50),
        "hatvanadik" => Some(60),
        "hetvenedik" => Some(70),
        "nyolcvanadik" => Some(80),
        "kilencvenedik" => Some(90),
        _ => None,
    }
}

fn ordinals_map2(s: &str) -> Option<i64> {
    match s {
        "egyedik" => Some(1),
        "kettedik" => Some(2),
        "harmadik" => Some(3),
        "negyedik" => Some(4),
        "ötödik" => Some(5),
        "hatodik" => Some(6),
        "hetedik" => Some(7),
        "nyolcadik" => Some(8),
        "kilencedik" => Some(9),
        _ => None,
    }
}

fn cardinals_map(s: &str) -> Option<i64> {
    match s {
        "tizen" => Some(10),
        "huszon" => Some(20),
        "harminc" => Some(30),
        "negyven" => Some(40),
        "ötven" => Some(50),
        "hatvan" => Some(60),
        "hetven" => Some(70),
        "nyolcvan" => Some(80),
        "kilencven" => Some(90),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinals (first..twentieth,thirtieth,...)".to_string(),
            pattern: vec![regex("(első|második|harmadik|negyedik|ötödik|hatodik|hetedik|nyolcadik|kilencedik|tizedik|huszadik|harmincadik|negyvenedik|ötvenedik|hatvanadik|hetvenedik|nyolcvanadik|kilencvenedik)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(ordinals_map(text)?)))
            }),
        },
        Rule {
            name: "ordinals (composite, e.g., eighty-seven)".to_string(),
            pattern: vec![regex("(tizen|huszon|harminc|negyven|ötven|hatvan|hetven|nyolcvan|kilencven)\\-?(egyedik|kettedik|harmadik|negyedik|ötödik|hatodik|hetedik|nyolcadik|kilencedik)")],
            production: Box::new(|nodes| {
                let tens = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let units = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(
                    cardinals_map(tens)? + ordinals_map2(units)?,
                )))
            }),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+)\\.")],
            production: Box::new(|nodes| {
                let value: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(value)))
            }),
        },
    ]
}
