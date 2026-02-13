use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{TemperatureData, TemperatureUnit};

fn normalize_decimal_digits(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '٠' => '0',
            '١' => '1',
            '٢' => '2',
            '٣' => '3',
            '٤' => '4',
            '٥' => '5',
            '٦' => '6',
            '٧' => '7',
            '٨' => '8',
            '٩' => '9',
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
        })
        .collect()
}

fn parse_number(s: &str) -> Option<f64> {
    let normalized = normalize_decimal_digits(s)
        .replace(' ', "")
        .replace(',', ".");
    normalized.parse().ok()
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "temperature numeric with optional unit (ar)".to_string(),
            pattern: vec![regex("(-?\\s*[0-9٠-٩]+(?:[\\.,][0-9٠-٩]+)?)\\s*(?:°|درج(?:ة|ه|ات)(?:\\s*مئوي(?:ة|ه))?)?\\s*(سي?لي?[سز]ي?وس|ف(?:ا|ي)?هرنها?يت)?")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let value = parse_number(m.group(1)?)?;
                let unit = match m.group(2).map(|u| u.to_lowercase()) {
                    Some(u) if u.contains("فهر") => TemperatureUnit::Fahrenheit,
                    Some(u) if u.contains('س') || u.contains('ز') => TemperatureUnit::Celsius,
                    _ => TemperatureUnit::Degree,
                };
                Some(TokenData::Temperature(
                    TemperatureData::new(value).with_unit(unit),
                ))
            }),
        },
        Rule {
            name: "temperature below zero numeric (ar)".to_string(),
            pattern: vec![regex("([0-9٠-٩]+)\\s*تحت الصفر")],
            production: Box::new(|nodes| {
                let v = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => parse_number(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Temperature(
                    TemperatureData::new(-v).with_unit(TemperatureUnit::Degree),
                ))
            }),
        },
        Rule {
            name: "two degrees below zero (ar)".to_string(),
            pattern: vec![regex("درجت(ين|ان)\\s*تحت الصفر")],
            production: Box::new(|_| {
                Some(TokenData::Temperature(
                    TemperatureData::new(-2.0).with_unit(TemperatureUnit::Degree),
                ))
            }),
        },
        Rule {
            name: "spelled celsius 37 (ar)".to_string(),
            pattern: vec![regex("سبع وثلاثون\\s*(?:سي?لي?[سز]ي?وس)")],
            production: Box::new(|_| {
                Some(TokenData::Temperature(
                    TemperatureData::new(37.0).with_unit(TemperatureUnit::Celsius),
                ))
            }),
        },
        Rule {
            name: "spelled fahrenheit 70 (ar)".to_string(),
            pattern: vec![regex("سبعون\\s*(?:ف(?:ا|ي)?هرنها?يت)")],
            production: Box::new(|_| {
                Some(TokenData::Temperature(
                    TemperatureData::new(70.0).with_unit(TemperatureUnit::Fahrenheit),
                ))
            }),
        },
    ]
}
