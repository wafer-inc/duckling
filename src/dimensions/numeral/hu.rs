use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn hu_num(s: &str) -> Option<f64> {
    match s {
        "nulla" | "zéró" => Some(0.0),
        "egy" => Some(1.0),
        "kettő" => Some(2.0),
        "három" => Some(3.0),
        "négy" => Some(4.0),
        "öt" => Some(5.0),
        "hat" => Some(6.0),
        "hét" => Some(7.0),
        "nyolc" => Some(8.0),
        "kilenc" => Some(9.0),
        "tíz" => Some(10.0),
        "tizenegy" => Some(11.0),
        "tizenkettő" => Some(12.0),
        "tizenhárom" => Some(13.0),
        "tizennégy" => Some(14.0),
        "tizenöt" => Some(15.0),
        "tizenhat" => Some(16.0),
        "tizenhét" => Some(17.0),
        "tizennyolc" => Some(18.0),
        "tizenkilenc" => Some(19.0),
        "húsz" => Some(20.0),
        "harminc" => Some(30.0),
        "negyven" => Some(40.0),
        "ötven" => Some(50.0),
        "hatvan" => Some(60.0),
        "hetven" => Some(70.0),
        "nyolcvan" => Some(80.0),
        "kilencven" => Some(90.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "number (0..10)".to_string(),
            pattern: vec![regex(
                r"(nulla|zéró|egy|kettő|három|négy|öt|hat|hét|nyolc|kilenc|tíz)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(hu_num(&s)?)))
            }),
        },
        Rule {
            name: "number (11..19)".to_string(),
            pattern: vec![regex(
                r"(tizenegy|tizenkettő|tizenhárom|tizennégy|tizenöt|tizenhat|tizenhét|tizennyolc|tizenkilenc)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(hu_num(&s)?)))
            }),
        },
        Rule {
            name: "number (21..29)".to_string(),
            pattern: vec![regex(
                r"(huszonegy|huszonkettő|huszonhárom|huszonnégy|huszonöt|huszonhat|huszonhét|huszonnyolc|huszonkilenc)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match s.as_str() {
                    "huszonegy" => 21.0,
                    "huszonkettő" => 22.0,
                    "huszonhárom" => 23.0,
                    "huszonnégy" => 24.0,
                    "huszonöt" => 25.0,
                    "huszonhat" => 26.0,
                    "huszonhét" => 27.0,
                    "huszonnyolc" => 28.0,
                    "huszonkilenc" => 29.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "tens".to_string(),
            pattern: vec![regex(
                r"(húsz|harminc|negyven|ötven|hatvan|hetven|nyolcvan|kilencven)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(hu_num(&s)?)))
            }),
        },
        Rule {
            name: "composite tens".to_string(),
            pattern: vec![regex(
                r"(harminc|negyven|ötven|hatvan|hetven|nyolcvan|kilencven)(egy|kettő|három|négy|öt|hat|hét|nyolc|kilenc)",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let t = m.group(1)?.to_lowercase();
                let u = m.group(2)?.to_lowercase();
                Some(TokenData::Numeral(NumeralData::new(
                    hu_num(&t)? + hu_num(&u)?,
                )))
            }),
        },
    ]
}
