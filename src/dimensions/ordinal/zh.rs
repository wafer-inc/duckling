use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn zh_digit(ch: char) -> Option<i64> {
    match ch {
        '零' => Some(0),
        '一' => Some(1),
        '二' | '两' => Some(2),
        '三' => Some(3),
        '四' => Some(4),
        '五' => Some(5),
        '六' => Some(6),
        '七' => Some(7),
        '八' => Some(8),
        '九' => Some(9),
        _ => None,
    }
}

fn parse_zh_numeral(s: &str) -> Option<i64> {
    if s.is_empty() {
        return None;
    }
    if let Ok(v) = s.parse::<i64>() {
        return Some(v);
    }
    if s == "十" {
        return Some(10);
    }
    if let Some(pos) = s.find('十') {
        let (left, right_with_ten) = s.split_at(pos);
        let right = right_with_ten.trim_start_matches('十');
        let tens = if left.is_empty() {
            1
        } else {
            zh_digit(left.chars().next()?)?
        };
        let ones = if right.is_empty() {
            0
        } else {
            zh_digit(right.chars().next()?)?
        };
        return Some(tens * 10 + ones);
    }
    if s.chars().count() == 1 {
        return zh_digit(s.chars().next()?);
    }
    None
}

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "ordinal (zh 第...)".to_string(),
        pattern: vec![regex("第([零一二两三四五六七八九十0-9]+)")],
        production: Box::new(|nodes| {
            let s = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            Some(TokenData::Ordinal(OrdinalData::new(parse_zh_numeral(s)?)))
        }),
    }]
}
