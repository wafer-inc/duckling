use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{QuantityData, QuantityUnit};

fn zh_digit(ch: char) -> Option<f64> {
    match ch {
        '零' => Some(0.0),
        '一' => Some(1.0),
        '二' | '两' | '兩' => Some(2.0),
        '三' => Some(3.0),
        '四' => Some(4.0),
        '五' => Some(5.0),
        '六' => Some(6.0),
        '七' => Some(7.0),
        '八' => Some(8.0),
        '九' => Some(9.0),
        _ => None,
    }
}

fn parse_zh_integer(s: &str) -> Option<f64> {
    if let Ok(v) = s.parse::<f64>() {
        return Some(v);
    }
    let mut total = 0.0f64;
    let mut current = 0.0f64;
    for ch in s.chars() {
        match ch {
            '十' => {
                let lhs = if current == 0.0 { 1.0 } else { current };
                total += lhs * 10.0;
                current = 0.0;
            }
            '百' => {
                let lhs = if current == 0.0 { 1.0 } else { current };
                total += lhs * 100.0;
                current = 0.0;
            }
            '千' => {
                let lhs = if current == 0.0 { 1.0 } else { current };
                total += lhs * 1000.0;
                current = 0.0;
            }
            _ => current = zh_digit(ch)?,
        }
    }
    Some(total + current)
}

fn parse_zh_number(s: &str) -> Option<f64> {
    if let Ok(v) = s.parse::<f64>() {
        return Some(v);
    }
    if let Some((i, f)) = s.split_once('點').or_else(|| s.split_once('点')) {
        let int_v = parse_zh_integer(i)?;
        let mut frac = 0.0;
        let mut place = 0.1;
        for ch in f.chars() {
            let d = zh_digit(ch)?;
            frac += d * place;
            place /= 10.0;
        }
        return Some(int_v + frac);
    }
    parse_zh_integer(s)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "zh quantity number + unit".to_string(),
            pattern: vec![regex("([0-9]+(?:\\.[0-9]+)?)\\s*(千克|公斤|斤|两|兩|克|毫克|kg|g|mg)")],
            production: Box::new(|nodes| {
                let (v, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.parse::<f64>().ok()?, m.group(2)?),
                    _ => return None,
                };
                let value = match u.to_lowercase().as_str() {
                    "千克" | "公斤" | "kg" => v * 1000.0,
                    "斤" => v * 500.0,
                    "两" | "兩" => v * 50.0,
                    "毫克" | "mg" => v / 1000.0,
                    _ => v,
                };
                Some(TokenData::Quantity(QuantityData::new(value, QuantityUnit::Gram)))
            }),
        },
        Rule {
            name: "zh quantity zh-number + unit".to_string(),
            pattern: vec![regex("([零一二两兩三四五六七八九十百千]+)\\s*(斤|两|兩|克|毫克)")],
            production: Box::new(|nodes| {
                let (v, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (parse_zh_number(m.group(1)?)?, m.group(2)?),
                    _ => return None,
                };
                let value = match u {
                    "斤" => v * 500.0,
                    "两" | "兩" => v * 50.0,
                    "毫克" => v / 1000.0,
                    _ => v,
                };
                Some(TokenData::Quantity(QuantityData::new(value, QuantityUnit::Gram)))
            }),
        },
        Rule {
            name: "zh quantity catty half".to_string(),
            pattern: vec![regex("([0-9]+|[零一二两兩三四五六七八九十百千]+)斤半")],
            production: Box::new(|nodes| {
                let x = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => parse_zh_number(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    x * 500.0 + 250.0,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "zh quantity tael half".to_string(),
            pattern: vec![regex("([0-9]+|[零一二两兩三四五六七八九十百千]+)(两|兩)半")],
            production: Box::new(|nodes| {
                let x = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => parse_zh_number(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    x * 50.0 + 25.0,
                    QuantityUnit::Gram,
                )))
            }),
        },
        Rule {
            name: "zh quantity catty tael".to_string(),
            pattern: vec![regex("([0-9]+|[零一二两兩三四五六七八九十百千]+)斤([0-9]+|[零一二两兩三四五六七八九十百千]+)(两|兩)")],
            production: Box::new(|nodes| {
                let (x, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (parse_zh_number(m.group(1)?)?, parse_zh_number(m.group(2)?)?),
                    _ => return None,
                };
                Some(TokenData::Quantity(QuantityData::new(
                    x * 500.0 + y * 50.0,
                    QuantityUnit::Gram,
                )))
            }),
        },
    ]
}
