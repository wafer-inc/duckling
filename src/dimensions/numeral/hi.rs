use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn devanagari_to_ascii(c: char) -> char {
    match c {
        '०' => '0',
        '१' => '1',
        '२' => '2',
        '३' => '3',
        '४' => '4',
        '५' => '5',
        '६' => '6',
        '७' => '7',
        '८' => '8',
        '९' => '9',
        _ => c,
    }
}

fn zero_to_ninety_nine(s: &str) -> Option<f64> {
    match s {
        "शून्य" => Some(0.0),
        "एक" => Some(1.0),
        "दो" => Some(2.0),
        "तीन" => Some(3.0),
        "चार" => Some(4.0),
        "पाँच" => Some(5.0),
        "छः" | "छह" | "छे" => Some(6.0),
        "सात" => Some(7.0),
        "आठ" => Some(8.0),
        "नौ" => Some(9.0),
        "दस" => Some(10.0),
        "ग्यारह" => Some(11.0),
        "बारह" => Some(12.0),
        "तेरह" => Some(13.0),
        "चौदह" => Some(14.0),
        "पन्द्रह" => Some(15.0),
        "सोलह" => Some(16.0),
        "सत्रह" => Some(17.0),
        "अठारह" => Some(18.0),
        "उन्नीस" => Some(19.0),
        "बीस" => Some(20.0),
        "बाईस" => Some(22.0),
        "चौबीस" => Some(24.0),
        "छब्बीस" => Some(26.0),
        "अट्ठाईस" => Some(28.0),
        "पचास" => Some(50.0),
        "इक्यासी" => Some(81.0),
        _ => None,
    }
}

fn power(s: &str) -> Option<NumeralData> {
    match s {
        "सौ" => Some(NumeralData::new(1e2).with_grain(2).with_multipliable(true)),
        "हज़ार" | "हज़ार" => Some(NumeralData::new(1e3).with_grain(3).with_multipliable(true)),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "devanagari forms".to_string(),
            pattern: vec![regex("([०१२३४५६७८९]{1,18})")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let ascii: String = s.chars().map(devanagari_to_ascii).collect();
                Some(TokenData::Numeral(NumeralData::new(ascii.parse().ok()?)))
            }),
        },
        Rule {
            name: "number (0..99)".to_string(),
            pattern: vec![regex("(शून्य|एक|दो|तीन|चार|पाँच|छे|छह|छः|सात|आठ|नौ|दस|ग्यारह|बारह|तेरह|चौदह|पन्द्रह|सोलह|सत्रह|अठारह|उन्नीस|बीस|बाईस|चौबीस|छब्बीस|अट्ठाईस|पचास|इक्यासी)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(zero_to_ninety_nine(s)?)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(सौ|हज़ार|हज़ार)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(power(s)?))
            }),
        },
        Rule {
            name: "compose by multiplication".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value > 0.0)),
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.multipliable)),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[1].token_data)?;
                if b.value > a.value {
                    let mut out = NumeralData::new(a.value * b.value);
                    if let Some(g) = b.grain {
                        out = out.with_grain(g);
                    }
                    Some(TokenData::Numeral(out))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "integer 100s..".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if (100.0..=5000.0).contains(&d.value) && d.value.fract() == 0.0)),
                regex("[\\s\\-]+"),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..100.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let h = numeral_data(&nodes[0].token_data)?.value;
                let r = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(h + r)))
            }),
        },
        Rule {
            name: "integer 1000s..".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1000.0..=50000.0).contains(&d.value) && d.value.fract() == 0.0)),
                regex("[\\s\\-]+"),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..1000.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let t = numeral_data(&nodes[0].token_data)?.value;
                let r = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(t + r)))
            }),
        },
    ]
}
