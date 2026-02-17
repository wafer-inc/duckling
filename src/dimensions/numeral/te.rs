use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn telugu_digit_to_ascii(c: char) -> char {
    match c {
        '౦' => '0',
        '౧' => '1',
        '౨' => '2',
        '౩' => '3',
        '౪' => '4',
        '౫' => '5',
        '౬' => '6',
        '౭' => '7',
        '౮' => '8',
        '౯' => '9',
        _ => c,
    }
}

fn zero_to_nine(s: &str) -> Option<f64> {
    match s {
        "సున్న" => Some(0.0),
        "ఒకటి" => Some(1.0),
        "రెండు" => Some(2.0),
        "మూడు" => Some(3.0),
        "నాలుగు" => Some(4.0),
        "ఐదు" => Some(5.0),
        "ఆరు" => Some(6.0),
        "ఏడు" => Some(7.0),
        "ఎనిమిది" => Some(8.0),
        "తొమ్మిది" => Some(9.0),
        _ => None,
    }
}

fn ten_to_nineteen(s: &str) -> Option<f64> {
    match s {
        "పది" => Some(10.0),
        "పదకొండు" => Some(11.0),
        "పన్నెండు" => Some(12.0),
        "పదమూడు" => Some(13.0),
        "పద్నాల్గు" => Some(14.0),
        "పదిహేను" => Some(15.0),
        "పదహారు" => Some(16.0),
        "పదిహేడు" => Some(17.0),
        "పద్దెనిమిది" => Some(18.0),
        "పంతొమ్మిది" => Some(19.0),
        _ => None,
    }
}

fn tens(s: &str) -> Option<f64> {
    match s {
        "ఇరవై" => Some(20.0),
        "ముప్పై" => Some(30.0),
        "నలబై" => Some(40.0),
        "యాబై" => Some(50.0),
        "అరవై" => Some(60.0),
        "డెబ్బై" => Some(70.0),
        "ఎనబై" => Some(80.0),
        "తొంబై" => Some(90.0),
        _ => None,
    }
}

fn high_units(s: &str) -> Option<f64> {
    match s {
        "వంద" => Some(100.0),
        "వెయ్యి" => Some(1000.0),
        "లక్ష" => Some(100000.0),
        "కోటి" => Some(10000000.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "telugu forms".to_string(),
            pattern: vec![regex("([౦౧౨౩౪౫౬౭౮౯]{1,18})")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let ascii: String = s.chars().map(telugu_digit_to_ascii).collect();
                let v: f64 = ascii.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer (0..9)".to_string(),
            pattern: vec![regex("(సున్న|ఒకటి|రెండు|మూడు|నాలుగు|ఐదు|ఆరు|ఏడు|ఎనిమిది|తొమ్మిది)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(zero_to_nine(s)?)))
            }),
        },
        Rule {
            name: "integer (10..19)".to_string(),
            pattern: vec![regex(
                "(పదకొండు|పన్నెండు|పదమూడు|పద్నాల్గు|పదిహేను|పదహారు|పదిహేడు|పద్దెనిమిది|పంతొమ్మిది|పది)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(ten_to_nineteen(s)?)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(ఇరవై|ముప్పై|నలబై|యాబై|అరవై|డెబ్బై|ఎనబై|తొంబై)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(tens(s)?)))
            }),
        },
        Rule {
            name: "integer (100,1000,100000,10000000)".to_string(),
            pattern: vec![regex("(వంద|వెయ్యి|లక్ష|కోటి)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(high_units(s)?)))
            }),
        },
    ]
}
