use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn zero_to_ten(s: &str) -> Option<f64> {
    match s {
        "ສູນ" => Some(0.0),
        "ໜຶ່ງ" => Some(1.0),
        "ສອງ" => Some(2.0),
        "ສາມ" => Some(3.0),
        "ສີ່" => Some(4.0),
        "ຫ້າ" => Some(5.0),
        "ຫົກ" => Some(6.0),
        "ເຈັດ" => Some(7.0),
        "ແປດ" => Some(8.0),
        "ເກົ້າ" => Some(9.0),
        "ສິບ" => Some(10.0),
        _ => None,
    }
}

fn eleven_to_nineteen(s: &str) -> Option<f64> {
    match s {
        "ສິບເອັດ" => Some(11.0),
        "ສິບສອງ" => Some(12.0),
        "ສິບສາມ" => Some(13.0),
        "ສິບສີ່" => Some(14.0),
        "ສິບຫ້າ" => Some(15.0),
        "ສິບຫົກ" => Some(16.0),
        "ສິບເຈັດ" => Some(17.0),
        "ສິບແປດ" => Some(18.0),
        "ສິບເກົ້າ" => Some(19.0),
        _ => None,
    }
}

fn twenty_to_ninety(s: &str) -> Option<f64> {
    match s {
        "ຊາວ" => Some(20.0),
        "ສາມສິບ" => Some(30.0),
        "ສິບສີ່" => Some(40.0),
        "ຫ້າສິບ" => Some(50.0),
        "ຫົກສິບ" => Some(60.0),
        "ເຈັດສິບ" => Some(70.0),
        "ແປດສິບ" => Some(80.0),
        "ເກົ້າສິບ" => Some(90.0),
        _ => None,
    }
}

fn twenty_ones(s: &str) -> Option<f64> {
    match s {
        "ຊາວເອັດ" => Some(21.0),
        "ຊາວສອງ" => Some(22.0),
        "ຊາວສາມ" => Some(23.0),
        "ຊາວສີ່" => Some(24.0),
        "ຊາວຫ້າ" => Some(25.0),
        "ຊາວຫົກ" => Some(26.0),
        "ຊາວເຈັດ" => Some(27.0),
        "ຊາວແປດ" => Some(28.0),
        "ຊາວເກົ້າ" => Some(29.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "number (0..10)".to_string(),
            pattern: vec![regex("(ສູນ|ໜຶ່ງ|ສອງ|ສາມ|ສີ່|ຫ້າ|ຫົກ|ເຈັດ|ແປດ|ເກົ້າ|ສິບ)")],
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
            pattern: vec![regex("(ສິບເອັດ|ສິບສອງ|ສິບສາມ|ສິບສີ່|ສິບຫ້າ|ສິບຫົກ|ສິບເຈັດ|ສິບແປດ|ສິບເກົ້າ)")],
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
            pattern: vec![regex(
                "(ຊາວເອັດ|ຊາວສອງ|ຊາວສາມ|ຊາວສີ່|ຊາວຫ້າ|ຊາວຫົກ|ຊາວເຈັດ|ຊາວແປດ|ຊາວເກົ້າ)",
            )],
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
            pattern: vec![regex("(ຊາວ|ສາມສິບ|ສິບສີ່|ຫ້າສິບ|ຫົກສິບ|ເຈັດສິບ|ແປດສິບ|ເກົ້າສິບ)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(twenty_to_ninety(s)?)))
            }),
        },
        Rule {
            name: "integer ([3-9][1-9])".to_string(),
            pattern: vec![regex(
                "(ສາມສິບ|ສິບສີ່|ຫ້າສິບ|ຫົກສິບ|ເຈັດສິບ|ແປດສິບ|ເກົ້າສິບ)(ໜຶ່ງ|ສອງ|ສາມ|ສີ່|ຫ້າ|ຫົກ|ເຈັດ|ແປດ|ເກົ້າ)",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let t = m.group(1)?;
                let u = m.group(2)?;
                Some(TokenData::Numeral(NumeralData::new(
                    twenty_to_ninety(t)? + zero_to_ten(u)?,
                )))
            }),
        },
    ]
}
