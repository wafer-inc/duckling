use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn myanmar_digit_to_ascii(c: char) -> char {
    match c {
        '၀' => '0',
        '၁' => '1',
        '၂' => '2',
        '၃' => '3',
        '၄' => '4',
        '၅' => '5',
        '၆' => '6',
        '၇' => '7',
        '၈' => '8',
        '၉' => '9',
        _ => c,
    }
}

fn unit(s: &str) -> Option<f64> {
    match s {
        "သုံည" | "မရှိ" => Some(0.0),
        "တစ်" | "ပထမ" => Some(1.0),
        "နှစ်" | "ဒုတိယ" => Some(2.0),
        "သုံး" | "တတိယ" => Some(3.0),
        "လေး" => Some(4.0),
        "ငါး" => Some(5.0),
        "ခြောက်" => Some(6.0),
        "ခုနှစ်" => Some(7.0),
        "ရှစ်" => Some(8.0),
        "ကိုး" => Some(9.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "myanmar forms".to_string(),
            pattern: vec![regex("([၀၁၂၃၄၅၆၇၈၉]{1,18})")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let ascii: String = s.chars().map(myanmar_digit_to_ascii).collect();
                let v: f64 = ascii.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "basic numerals".to_string(),
            pattern: vec![regex(
                "(သုံည|မရှိ|တစ်|ပထမ|နှစ်|ဒုတိယ|သုံး|တတိယ|လေး|ငါး|ခြောက်|ခုနှစ်|ရှစ်|ကိုး)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(unit(s)?)))
            }),
        },
        Rule {
            name: "tens".to_string(),
            pattern: vec![regex("(သုံး)ဆယ်")],
            production: Box::new(|nodes| {
                let u = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(unit(u)? * 10.0)))
            }),
        },
        Rule {
            name: "teens".to_string(),
            pattern: vec![regex("ဆယ့်(လေး|ခုနှစ်)")],
            production: Box::new(|nodes| {
                let u = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(10.0 + unit(u)?)))
            }),
        },
        Rule {
            name: "composite tens".to_string(),
            pattern: vec![regex("(သုံး)ဆယ့်(သုံး)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let t = m.group(1)?;
                let u = m.group(2)?;
                Some(TokenData::Numeral(NumeralData::new(
                    unit(t)? * 10.0 + unit(u)?,
                )))
            }),
        },
        Rule {
            name: "hundreds".to_string(),
            pattern: vec![regex("(နှစ်|ကိုး)ရာ")],
            production: Box::new(|nodes| {
                let u = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(unit(u)? * 100.0)))
            }),
        },
        Rule {
            name: "thousands".to_string(),
            pattern: vec![regex("(ငါး)ထောင်")],
            production: Box::new(|nodes| {
                let u = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(unit(u)? * 1000.0)))
            }),
        },
        Rule {
            name: "ten-thousands".to_string(),
            pattern: vec![regex("(ရှစ်)သောင်း")],
            production: Box::new(|nodes| {
                let u = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(unit(u)? * 10000.0)))
            }),
        },
    ]
}
