use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn parse_ja_numeral(s: &str) -> Option<i64> {
    if let Ok(v) = s.parse::<i64>() {
        return Some(v);
    }
    let mut total = 0i64;
    let mut current = 0i64;
    for ch in s.chars() {
        match ch {
            '一' => current += 1,
            '二' => current += 2,
            '三' => current += 3,
            '四' => current += 4,
            '五' => current += 5,
            '六' => current += 6,
            '七' => current += 7,
            '八' => current += 8,
            '九' => current += 9,
            '十' => {
                let lhs = if current == 0 { 1 } else { current };
                total += lhs * 10;
                current = 0;
            }
            '百' => {
                let lhs = if current == 0 { 1 } else { current };
                total += lhs * 100;
                current = 0;
            }
            '千' => {
                let lhs = if current == 0 { 1 } else { current };
                total += lhs * 1000;
                current = 0;
            }
            _ => return None,
        }
    }
    Some(total + current)
}

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "ordinal (ja 第...)".to_string(),
        pattern: vec![regex("第([0-9一二三四五六七八九十百千]+)")],
        production: Box::new(|nodes| {
            let s = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            Some(TokenData::Ordinal(OrdinalData::new(parse_ja_numeral(s)?)))
        }),
    }]
}
