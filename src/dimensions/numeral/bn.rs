use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn bengali_digit_to_ascii(c: char) -> char {
    match c {
        '০' => '0',
        '১' => '1',
        '২' => '2',
        '৩' => '3',
        '৪' => '4',
        '৫' => '5',
        '৬' => '6',
        '৭' => '7',
        '৮' => '8',
        '৯' => '9',
        _ => c,
    }
}

fn zero_to_ten(s: &str) -> Option<f64> {
    match s {
        "শূন্য" => Some(0.0),
        "এক" => Some(1.0),
        "দুই" => Some(2.0),
        "তিন" => Some(3.0),
        "চার" => Some(4.0),
        "পাঁচ" => Some(5.0),
        "ছয়" => Some(6.0),
        "সাত" => Some(7.0),
        "আট" => Some(8.0),
        "নয়" => Some(9.0),
        "দশ" => Some(10.0),
        _ => None,
    }
}

fn eleven_to_nineteen(s: &str) -> Option<f64> {
    match s {
        "এগারো" => Some(11.0),
        "বারো" => Some(12.0),
        "তেরো" => Some(13.0),
        "চৌদ্দ" => Some(14.0),
        "পনেরো" => Some(15.0),
        "ষোল" => Some(16.0),
        "সতেরো" => Some(17.0),
        "আঠারো" => Some(18.0),
        "উনিশ" => Some(19.0),
        _ => None,
    }
}

fn twenty_to_ninety(s: &str) -> Option<f64> {
    match s {
        "কুড়ি" => Some(20.0),
        "তিরিশ" => Some(30.0),
        "চল্লিশ" => Some(40.0),
        "পঞ্চাশ" => Some(50.0),
        "ষাট" => Some(60.0),
        "সত্তর" => Some(70.0),
        "আশি" => Some(80.0),
        "নব্বই" => Some(90.0),
        _ => None,
    }
}

fn twenty_ones(s: &str) -> Option<f64> {
    match s {
        "একুশ" => Some(21.0),
        "বাইশ" => Some(22.0),
        "তেইশ" => Some(23.0),
        "চব্বিশ" => Some(24.0),
        "পঁচিশ" => Some(25.0),
        "ছাব্বিশ" => Some(26.0),
        "সাতাশ" => Some(27.0),
        "আঠাশ" => Some(28.0),
        "ঊনত্রিশ" => Some(29.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "bengali forms".to_string(),
            pattern: vec![regex("([০১২৩৪৫৬৭৮৯]{1,18})")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let ascii: String = s.chars().map(bengali_digit_to_ascii).collect();
                Some(TokenData::Numeral(NumeralData::new(ascii.parse().ok()?)))
            }),
        },
        Rule {
            name: "number (0..10)".to_string(),
            pattern: vec![regex("(শূন্য|এক|দুই|তিন|চার|পাঁচ|ছয়|সাত|আট|নয়|দশ)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(zero_to_ten(s)?)))
            }),
        },
        Rule {
            name: "number (11..19)".to_string(),
            pattern: vec![regex("(এগারো|বারো|তেরো|চৌদ্দ|পনেরো|ষোল|সতেরো|আঠারো|উনিশ)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(eleven_to_nineteen(s)?)))
            }),
        },
        Rule {
            name: "number (21..29)".to_string(),
            pattern: vec![regex("(একুশ|বাইশ|তেইশ|চব্বিশ|পঁচিশ|ছাব্বিশ|সাতাশ|আঠাশ|ঊনত্রিশ)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(twenty_ones(s)?)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(কুড়ি|তিরিশ|চল্লিশ|পঞ্চাশ|ষাট|সত্তর|আশি|নব্বই)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(twenty_to_ninety(s)?)))
            }),
        },
    ]
}
