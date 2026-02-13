use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn ordinals_map(s: &str) -> Option<i64> {
    match s {
        "शून्य" => Some(0),
        "प्रथम" | "पहला" | "पहली" | "पहले" => Some(1),
        "द्वितीय" | "दूसरा" | "दूसरी" | "दूसरे" => Some(2),
        "तृतीय" | "तीसरा" | "तीसरी" | "तीसरे" => Some(3),
        "चौथा" | "चौथी" | "चौथे" => Some(4),
        "छठा" | "छठी" | "छठे" => Some(6),
        _ => None,
    }
}

fn cardinals_map(s: &str) -> Option<i64> {
    match s {
        "पाँच" => Some(5),
        "सात" => Some(7),
        "आठ" => Some(8),
        "नौ" => Some(9),
        "दस" => Some(10),
        "ग्यारह" => Some(11),
        "बारह" => Some(12),
        "तेरह" => Some(13),
        "चौदह" => Some(14),
        "पन्द्रह" => Some(15),
        "सोलह" => Some(16),
        "सत्रह" => Some(17),
        "अठारह" => Some(18),
        "उन्नीस" => Some(19),
        "बीस" => Some(20),
        "इक्कीस" => Some(21),
        "बाईस" => Some(22),
        "तेईस" => Some(23),
        "चौबीस" => Some(24),
        "पच्चीस" => Some(25),
        "छब्बीस" => Some(26),
        "सत्ताईस" => Some(27),
        "अट्ठाईस" => Some(28),
        "उनतीस" => Some(29),
        "तीस" => Some(30),
        "चालीस" => Some(40),
        "पचास" => Some(50),
        "साठ" => Some(60),
        "सत्तर" => Some(70),
        "अस्सी" => Some(80),
        "नब्बे" => Some(90),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinals (first..fourth, sixth)".to_string(),
            pattern: vec![regex("(शून्य|प्रथम|पहला|पहली|पहले|द्वितीय|दूसरा|दूसरी|दूसरे|तृतीय|तीसरा|तीसरी|तीसरे|चौथा|चौथी|चौथे|छठा|छठी|छठे)")],
            production: Box::new(|nodes| {
                let value = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => ordinals_map(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(value)))
            }),
        },
        Rule {
            name: "ordinals (fifth, seventh ...)".to_string(),
            pattern: vec![regex("(पाँच|सात|आठ|नौ|दस|ग्यारह|बारह|तेरह|चौदह|पन्द्रह|सोलह|सत्रह|अठारह|उन्नीस|बीस|इक्कीस|बाईस|तेईस|चौबीस|पच्चीस|छब्बीस|सत्ताईस|अट्ठाईस|उनतीस|तीस|चालीस|पचास|साठ|सत्तर|अस्सी|नब्बे)(वा|वी|वे)")],
            production: Box::new(|nodes| {
                let value = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => cardinals_map(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(value)))
            }),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+) ?(वा|वी|वे)")],
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
