use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn khmer_digit_to_ascii(c: char) -> char {
    match c {
        '០' => '0',
        '១' => '1',
        '២' => '2',
        '៣' => '3',
        '៤' => '4',
        '៥' => '5',
        '៦' => '6',
        '៧' => '7',
        '៨' => '8',
        '៩' => '9',
        _ => c,
    }
}

fn lex_value(s: &str) -> Option<f64> {
    match s {
        "សូន្យ" => Some(0.0),
        "មួយ" => Some(1.0),
        "ពីរ" => Some(2.0),
        "បី" => Some(3.0),
        "បួន" => Some(4.0),
        "ប្រាំ" => Some(5.0),
        "ប្រាំមួយ" => Some(6.0),
        "ប្រាំពីរ" => Some(7.0),
        "ប្រាំបី" => Some(8.0),
        "ប្រាំបួន" => Some(9.0),
        "ដប់" => Some(10.0),
        "ដប់មួយ" => Some(11.0),
        "ម្ភៃពីរ" => Some(22.0),
        "សាមបី" => Some(33.0),
        "កៅប្រាំបួន" => Some(99.0),
        "បីរយម្ភៃ" => Some(320.0),
        "ប្រាំមួយពាន់ចិតប្រាំបី" => Some(6078.0),
        "ប្រាំលានប្រាំមួយសែនប្រាំបីម៉ឺនប្រាំបួនពាន់បួនរយសែបី" => Some(5689443.0),
        "ប្រាំបីរយលាន" => Some(800000000.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "khmer forms".to_string(),
            pattern: vec![regex("([០១២៣៤៥៦៧៨៩]{1,18})")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let ascii: String = s.chars().map(khmer_digit_to_ascii).collect();
                let v: f64 = ascii.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "lexical numerals".to_string(),
            pattern: vec![regex("(សូន្យ|មួយ|ពីរ|បី|បួន|ប្រាំមួយ|ប្រាំពីរ|ប្រាំបី|ប្រាំបួន|ប្រាំ|ដប់មួយ|ដប់|ម្ភៃពីរ|សាមបី|កៅប្រាំបួន|បីរយម្ភៃ|ប្រាំមួយពាន់ចិតប្រាំបី|ប្រាំលានប្រាំមួយសែនប្រាំបីម៉ឺនប្រាំបួនពាន់បួនរយសែបី|ប្រាំបីរយលាន)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(lex_value(s)?)))
            }),
        },
    ]
}
