use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::NumeralData;

fn persian_digit_to_ascii(c: char) -> char {
    match c {
        '۰' => '0',
        '۱' => '1',
        '۲' => '2',
        '۳' => '3',
        '۴' => '4',
        '۵' => '5',
        '۶' => '6',
        '۷' => '7',
        '۸' => '8',
        '۹' => '9',
        _ => c,
    }
}

fn zero_to_nineteen(s: &str) -> Option<f64> {
    match s {
        "صفر" => Some(0.0),
        "یک" => Some(1.0),
        "دو" => Some(2.0),
        "سه" => Some(3.0),
        "چهار" => Some(4.0),
        "پنج" => Some(5.0),
        "شش" | "شیش" => Some(6.0),
        "هفت" => Some(7.0),
        "هشت" => Some(8.0),
        "نه" => Some(9.0),
        "ده" => Some(10.0),
        "یازده" => Some(11.0),
        "دوازده" => Some(12.0),
        "سیزده" => Some(13.0),
        "چهارده" => Some(14.0),
        "پانزده" | "پونزده" => Some(15.0),
        "شانزده" | "شونزده" => Some(16.0),
        "هفده" | "هیفده" => Some(17.0),
        "هجده" | "هیجده" => Some(18.0),
        "نوزده" => Some(19.0),
        _ => None,
    }
}

fn tens_hundreds(s: &str) -> Option<f64> {
    match s {
        "بیست" => Some(20.0),
        "سی" => Some(30.0),
        "چهل" => Some(40.0),
        "پنجاه" => Some(50.0),
        "شصت" => Some(60.0),
        "هفتاد" => Some(70.0),
        "هشتاد" => Some(80.0),
        "نود" => Some(90.0),
        "صد" => Some(100.0),
        "دویست" => Some(200.0),
        "سیصد" | "سی صد" => Some(300.0),
        "چهارصد" | "چهار صد" => Some(400.0),
        "پانصد" | "پونصد" => Some(500.0),
        "شیشصد" | "شیش صد" | "ششصد" | "شش صد" => Some(600.0),
        "هفتصد" | "هفت صد" => Some(700.0),
        "هشتصد" | "هشت صد" => Some(800.0),
        "نهصد" | "نه صد" => Some(900.0),
        _ => None,
    }
}

fn power(s: &str) -> Option<NumeralData> {
    match s {
        "هزار" => Some(NumeralData::new(1e3).with_grain(3).with_multipliable(true)),
        "میلیون" | "ملیون" => Some(NumeralData::new(1e6).with_grain(6).with_multipliable(true)),
        "میلیارد" => Some(NumeralData::new(1e9).with_grain(9).with_multipliable(true)),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "Persian integer numeric".to_string(),
            pattern: vec![regex("([۰-۹]{1,18})")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let ascii: String = s.chars().map(persian_digit_to_ascii).collect();
                Some(TokenData::Numeral(NumeralData::new(ascii.parse().ok()?)))
            }),
        },
        Rule {
            name: "integer (0..19)".to_string(),
            pattern: vec![regex("(صفر|یک|دو|سه|چهار|پنج|شی?ش|هفت|هشت|نه|ده|یازده|دوازده|سیزده|چهارده|پ(ا|و)نزده|ش(ا|و)نزده|هی?فده|هی?جده|نوزده)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(zero_to_nineteen(&s)?)))
            }),
        },
        Rule {
            name: "integer tens/hundreds".to_string(),
            pattern: vec![regex("(دویست|(سی|چهار|پان|پون|شی?ش|هفت|هشت|نه)? ?صد|بیست|سی|چهل|پنجاه|شصت|هفتاد|هشتاد|نود)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(tens_hundreds(&s)?)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(هزار|میلیون|ملیون|میلیارد)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(power(&s)?))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if [20.0,30.0,40.0,50.0,60.0,70.0,80.0,90.0].contains(&d.value))),
                regex("و"),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..10.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "integer 101..999 with and".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if [100.0,200.0,300.0,400.0,500.0,600.0,700.0,800.0,900.0].contains(&d.value))),
                regex("و"),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..100.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "compose by multiplication".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), predicate(|td| matches!(td, TokenData::Numeral(d) if d.multipliable))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[1].token_data)?;
                if b.grain.is_none() || b.value > a.value {
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
    ]
}
