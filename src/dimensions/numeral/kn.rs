use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn kannada_digit_to_ascii(c: char) -> char {
    match c {
        '೦' => '0',
        '೧' => '1',
        '೨' => '2',
        '೩' => '3',
        '೪' => '4',
        '೫' => '5',
        '೬' => '6',
        '೭' => '7',
        '೮' => '8',
        '೯' => '9',
        _ => c,
    }
}

fn word_digit(s: &str) -> Option<f64> {
    match s {
        "ಸೊನ್ನೆ" => Some(0.0),
        "ಒಂದು" => Some(1.0),
        "ಎರಡು" => Some(2.0),
        "ಮೂರು" => Some(3.0),
        "ನಾಲ್ಕು" => Some(4.0),
        "ಐದು" => Some(5.0),
        "ಆರು" => Some(6.0),
        "ಏಳು" => Some(7.0),
        "ಎಂಟು" => Some(8.0),
        "ಒಂಬತ್ತು" => Some(9.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "anki forms".to_string(),
            pattern: vec![regex("([೦೧೨೩೪೫೬೭೮೯]{1,10})")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let ascii: String = s.chars().map(kannada_digit_to_ascii).collect();
                let v: f64 = ascii.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number words".to_string(),
            pattern: vec![regex("(ಸೊನ್ನೆ|ಒಂದು|ಎರಡು|ಮೂರು|ನಾಲ್ಕು|ಐದು|ಆರು|ಏಳು|ಎಂಟು|ಒಂಬತ್ತು)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(word_digit(s)?)))
            }),
        },
    ]
}
