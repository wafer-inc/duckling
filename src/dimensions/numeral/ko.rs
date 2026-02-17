use crate::dimensions::numeral::helpers::{decimals_to_double, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::NumeralData;

fn sino_digit(c: char) -> Option<f64> {
    match c {
        '영' | '빵' | '공' => Some(0.0),
        '일' => Some(1.0),
        '이' => Some(2.0),
        '삼' => Some(3.0),
        '사' => Some(4.0),
        '오' => Some(5.0),
        '육' => Some(6.0),
        '칠' => Some(7.0),
        '팔' => Some(8.0),
        '구' => Some(9.0),
        _ => None,
    }
}

fn native_unit(s: &str) -> Option<f64> {
    match s {
        "하나" | "한" => Some(1.0),
        "둘" | "두" => Some(2.0),
        "셋" | "세" => Some(3.0),
        "넷" | "네" => Some(4.0),
        "다섯" => Some(5.0),
        "여섯" => Some(6.0),
        "일곱" => Some(7.0),
        "여덟" => Some(8.0),
        "아홉" => Some(9.0),
        _ => None,
    }
}

fn native_tens(s: &str) -> Option<f64> {
    match s {
        "열" => Some(10.0),
        "스물" => Some(20.0),
        "서른" => Some(30.0),
        "마흔" => Some(40.0),
        "쉰" => Some(50.0),
        "예순" => Some(60.0),
        "일흔" => Some(70.0),
        "여든" => Some(80.0),
        "아흔" => Some(90.0),
        _ => None,
    }
}

fn parse_sino_integer(s: &str) -> Option<f64> {
    if s.chars().all(|c| sino_digit(c).is_some()) {
        let mut n = String::new();
        for c in s.chars() {
            n.push(char::from(b'0'.checked_add(sino_digit(c)? as u8)?));
        }
        return n.parse::<f64>().ok();
    }

    let mut total = 0.0;
    let mut section = 0.0;
    let mut number = 0.0;

    for c in s.chars() {
        if let Some(d) = sino_digit(c) {
            number = d;
            continue;
        }
        let unit = match c {
            '십' => Some(10.0),
            '백' => Some(100.0),
            '천' => Some(1000.0),
            _ => None,
        };
        if let Some(u) = unit {
            if number == 0.0 {
                number = 1.0;
            }
            section += number * u;
            number = 0.0;
            continue;
        }
        let big = match c {
            '만' => Some(1e4),
            '억' => Some(1e8),
            _ => None,
        };
        if let Some(u) = big {
            section += number;
            if section == 0.0 {
                section = 1.0;
            }
            total += section * u;
            section = 0.0;
            number = 0.0;
            continue;
        }
        return None;
    }

    Some(total + section + number)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "korean sino integer".to_string(),
            pattern: vec![regex("([영빵공일이삼사오육칠팔구십백천만억]+)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(parse_sino_integer(s)?)))
            }),
        },
        Rule {
            name: "korean native tens".to_string(),
            pattern: vec![regex("(열|스물|서른|마흔|쉰|예순|일흔|여든|아흔)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(native_tens(s)?)))
            }),
        },
        Rule {
            name: "korean native units".to_string(),
            pattern: vec![regex("(하나|한|둘|두|셋|세|넷|네|다섯|여섯|일곱|여덟|아홉)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(native_unit(s)?)))
            }),
        },
        Rule {
            name: "native 21..99".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if [20.0,30.0,40.0,50.0,60.0,70.0,80.0,90.0,10.0].contains(&d.value))),
                predicate(|td| matches!(td, TokenData::Numeral(d) if (1.0..10.0).contains(&d.value))),
            ],
            production: Box::new(|nodes| {
                let t = numeral_data(&nodes[0].token_data)?.value;
                let u = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(t + u)))
            }),
        },
        Rule {
            name: "dot decimal".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("점"), predicate(|td| matches!(td, TokenData::Numeral(d) if d.grain.is_none()))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + decimals_to_double(b))))
            }),
        },
        Rule {
            name: "fraction bun-eui".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("분의"), dim(DimensionKind::Numeral)],
            production: Box::new(|nodes| {
                let d = numeral_data(&nodes[0].token_data)?.value;
                let n = numeral_data(&nodes[2].token_data)?.value;
                if d == 0.0 {
                    None
                } else {
                    Some(TokenData::Numeral(NumeralData::new(n / d)))
                }
            }),
        },
        Rule {
            name: "negative".to_string(),
            pattern: vec![regex("(마이너스|마이나스|-)") , predicate(|td| matches!(td, TokenData::Numeral(d) if d.value >= 0.0))],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
    ]
}
