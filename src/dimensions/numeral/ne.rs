use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn zero_to_nineteen(s: &str) -> Option<f64> {
    match s {
        "शुन्य" | "सुन्ना" => Some(0.0),
        "एक" => Some(1.0),
        "दुई" => Some(2.0),
        "तीन" => Some(3.0),
        "चार" => Some(4.0),
        "पाँच" => Some(5.0),
        "छ" => Some(6.0),
        "सात" => Some(7.0),
        "आठ" => Some(8.0),
        "नौ" => Some(9.0),
        "दश" => Some(10.0),
        "एघार" => Some(11.0),
        "बाह्र" => Some(12.0),
        "तेह्र" => Some(13.0),
        "चौध" => Some(14.0),
        "पन्ध्र" => Some(15.0),
        "सोह्र" => Some(16.0),
        "सत्र" => Some(17.0),
        "अठार" => Some(18.0),
        "उन्नाइस" => Some(19.0),
        _ => None,
    }
}

fn twenty_nine(s: &str) -> Option<f64> {
    match s {
        "एक्काइस" => Some(21.0),
        "बाइस" => Some(22.0),
        "तेइस" => Some(23.0),
        "चौबिस" => Some(24.0),
        "पच्चिस" => Some(25.0),
        "छब्बिस" => Some(26.0),
        "सत्ताइस" => Some(27.0),
        "अट्ठाइस" => Some(28.0),
        "उनन्तिस" => Some(29.0),
        _ => None,
    }
}

fn tens(s: &str) -> Option<f64> {
    match s {
        "बिस" => Some(20.0),
        "तिस" => Some(30.0),
        "चालिस" => Some(40.0),
        "पचास" => Some(50.0),
        "साठी" => Some(60.0),
        "सत्तरी" => Some(70.0),
        "असी" => Some(80.0),
        "नब्बे" => Some(90.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "integer (0..19)".to_string(),
            pattern: vec![regex("(शुन्य|सुन्ना|एक|दुई|तीन|चार|पाँच|छ|सात|आठ|नौ|दश|एघार|बाह्र|तेह्र|चौध|पन्ध्र|सोह्र|सत्र|अठार|उन्नाइस)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(zero_to_nineteen(&s)?)))
            }),
        },
        Rule {
            name: "integer (21..29)".to_string(),
            pattern: vec![regex("(एक्काइस|बाइस|तेइस|चौबिस|पच्चिस|छब्बिस|सत्ताइस|अट्ठाइस|उनन्तिस)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(twenty_nine(&s)?)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(बिस|तिस|चालिस|पचास|साठी|सत्तरी|असी|नब्बे)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(tens(&s)?)))
            }),
        },
    ]
}
