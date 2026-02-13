use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn fi_num(s: &str) -> Option<f64> {
    match s {
        "nolla" => Some(0.0),
        "yksi" => Some(1.0),
        "kaksi" => Some(2.0),
        "kolme" => Some(3.0),
        "neljä" => Some(4.0),
        "viisi" => Some(5.0),
        "kuusi" => Some(6.0),
        "seitsemän" => Some(7.0),
        "kahdeksan" => Some(8.0),
        "yhdeksän" => Some(9.0),
        "kymmenen" => Some(10.0),
        "yksitoista" => Some(11.0),
        "kaksitoista" => Some(12.0),
        "kolmetoista" => Some(13.0),
        "neljätoista" => Some(14.0),
        "viisitoista" => Some(15.0),
        "kuusitoista" => Some(16.0),
        "seitsemäntoista" => Some(17.0),
        "kahdeksantoista" => Some(18.0),
        "yhdeksäntoista" => Some(19.0),
        "kaksikymmentä" => Some(20.0),
        "kolmekymmentä" => Some(30.0),
        "neljäkymmentä" => Some(40.0),
        "viisikymmentä" => Some(50.0),
        "kuusikymmentä" => Some(60.0),
        "seitsemänkymmentä" => Some(70.0),
        "kahdeksankymmentä" => Some(80.0),
        "yhdeksänkymmentä" => Some(90.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "integer (numeric)".to_string(),
            pattern: vec![regex(r"(\d{1,18})")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(t.parse().ok()?)))
            }),
        },
        Rule {
            name: "number (0..90)".to_string(),
            pattern: vec![regex(r"(nolla|yksi|kaksi|kolme|neljä|viisi|kuusi|seitsemän|kahdeksan|yhdeksän|kymmenen|yksitoista|kaksitoista|kolmetoista|neljätoista|viisitoista|kuusitoista|seitsemäntoista|kahdeksantoista|yhdeksäntoista|kaksikymmentä|kolmekymmentä|neljäkymmentä|viisikymmentä|kuusikymmentä|seitsemänkymmentä|kahdeksankymmentä|yhdeksänkymmentä)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(fi_num(&s)?)))
            }),
        },
        Rule {
            name: "composite tens".to_string(),
            pattern: vec![regex(r"(kaksikymmentä|kolmekymmentä|neljäkymmentä|viisikymmentä|kuusikymmentä|seitsemänkymmentä|kahdeksankymmentä|yhdeksänkymmentä)(yksi|kaksi|kolme|neljä|viisi|kuusi|seitsemän|kahdeksan|yhdeksän)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let t = m.group(1)?.to_lowercase();
                let u = m.group(2)?.to_lowercase();
                Some(TokenData::Numeral(NumeralData::new(fi_num(&t)? + fi_num(&u)?)))
            }),
        },
    ]
}
