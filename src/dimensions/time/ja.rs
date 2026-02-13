use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

fn parse_kanji_under_60(s: &str) -> Option<u32> {
    fn d(c: char) -> Option<u32> {
        match c {
            '一' => Some(1),
            '二' => Some(2),
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
    if s == "十" {
        return Some(10);
    }
    if let Some(rest) = s.strip_prefix('十') {
        return Some(10 + d(rest.chars().next()?)?);
    }
    if let Some(rest) = s.strip_suffix('十') {
        return Some(d(rest.chars().next()?)? * 10);
    }
    if let Some((a, b)) = s.split_once('十') {
        let aa = d(a.chars().next()?)?;
        let bb = d(b.chars().next()?)?;
        return Some(aa * 10 + bb);
    }
    d(s.chars().next()?)
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (ja)".to_string(),
            pattern: vec![regex("今|いま")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ja)".to_string(),
            pattern: vec![regex("今日|きょう")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ja)".to_string(),
            pattern: vec![regex("明日|あした")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (ja)".to_string(),
            pattern: vec![regex("昨日|きのう")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "hh時mm分 (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})時(\\d{1,2})分")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let h: u32 = h.parse().ok()?;
                let m: u32 = m.parse().ok()?;
                if h > 23 || m > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(h, m, false))))
            }),
        },
        Rule {
            name: "15時ちょうど (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})時(ちょうど|きっかり|ぴったり)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let h: u32 = h.parse().ok()?;
                if h > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(h, 0, false))))
            }),
        },
        Rule {
            name: "午後hh:mm (ja)".to_string(),
            pattern: vec![regex("午後\\s*(\\d{1,2})[:：](\\d{2})")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let mut h: u32 = h.parse().ok()?;
                let m: u32 = m.parse().ok()?;
                if h < 12 {
                    h += 12;
                }
                if h > 23 || m > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(h, m, false))))
            }),
        },
        Rule {
            name: "午前hh:mm (ja)".to_string(),
            pattern: vec![regex("午前\\s*(\\d{1,2})[:：](\\d{2})")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let mut h: u32 = h.parse().ok()?;
                let m: u32 = m.parse().ok()?;
                if h == 12 {
                    h = 0;
                }
                if h > 23 || m > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(h, m, false))))
            }),
        },
        Rule {
            name: "午前三時二十分 (ja)".to_string(),
            pattern: vec![regex("午前([一二三四五六七八九十]+)時([一二三四五六七八九十]+)分")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let mut h = parse_kanji_under_60(h)?;
                let m = parse_kanji_under_60(m)?;
                if h == 12 {
                    h = 0;
                }
                if h > 23 || m > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(h, m, false))))
            }),
        },
        Rule {
            name: "午後三時二十分 (ja)".to_string(),
            pattern: vec![regex("午後([一二三四五六七八九十]+)時([一二三四五六七八九十]+)分")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let mut h = parse_kanji_under_60(h)?;
                let m = parse_kanji_under_60(m)?;
                if h < 12 {
                    h += 12;
                }
                if h > 23 || m > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(h, m, false))))
            }),
        },
        Rule {
            name: "3月25日 (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})月(\\d{1,2})日")],
            production: Box::new(|nodes| {
                let (m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
    ]);
    rules
}
