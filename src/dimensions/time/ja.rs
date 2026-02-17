use super::{Direction, IntervalDirection, TimeData, TimeForm};
use crate::pattern::regex;
use crate::types::{Rule, TokenData};

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
        return d(rest.chars().next()?)?.checked_add(10);
    }
    if let Some(rest) = s.strip_suffix('十') {
        return d(rest.chars().next()?)?.checked_mul(10);
    }
    if let Some((a, b)) = s.split_once('十') {
        let aa = d(a.chars().next()?)?;
        let bb = d(b.chars().next()?)?;
        return aa.checked_mul(10)?.checked_add(bb);
    }
    d(s.chars().next()?)
}

fn normalize_zenkaku_digits(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '０' => '0',
            '１' => '1',
            '２' => '2',
            '３' => '3',
            '４' => '4',
            '５' => '5',
            '６' => '6',
            '７' => '7',
            '８' => '8',
            '９' => '9',
            _ => c,
        })
        .collect()
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (ja)".to_string(),
            pattern: vec![regex("今|いま|現在|ただいま|只今|即|ただちに|直ちに|すぐ(に)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ja)".to_string(),
            pattern: vec![regex("今日|きょう|本日|ほんじつ|当日")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ja)".to_string(),
            pattern: vec![regex("明日|あした|あす|みょうにち")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "weekend (ja)".to_string(),
            pattern: vec![regex("週末|しゅうまつ")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "この1時間 (ja)".to_string(),
            pattern: vec![regex("(この|当|現)1時間")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(crate::dimensions::time_grain::Grain::Hour))))),
        },
        Rule {
            name: "この1日 (ja)".to_string(),
            pattern: vec![regex("(この|当|現)1日|今日1日")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "このNヶ月 (ja)".to_string(),
            pattern: vec![regex("(この|当|現)(\\d+)ヶ?月")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(2)?,
                    _ => return None,
                };
                let months: i64 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: months,
                    grain: crate::dimensions::time_grain::Grain::Month,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "このN年/週/日/時間/分 (ja)".to_string(),
            pattern: vec![regex("(この|当|現)(\\d+)(年|週間|週|日|時間|分)")],
            production: Box::new(|nodes| {
                let (n, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let count: i64 = n.parse().ok()?;
                let grain = if u == "年" {
                    crate::dimensions::time_grain::Grain::Year
                } else if u == "週間" || u == "週" {
                    crate::dimensions::time_grain::Grain::Week
                } else if u == "日" {
                    crate::dimensions::time_grain::Grain::Day
                } else if u == "時間" {
                    crate::dimensions::time_grain::Grain::Hour
                } else {
                    crate::dimensions::time_grain::Grain::Minute
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: count,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "この二年/二週間... (ja)".to_string(),
            pattern: vec![regex("(この|当|現)([一二三四五六七八九十]+)(年|週間|週|日|時間|分)")],
            production: Box::new(|nodes| {
                let (n, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let count: i64 = parse_kanji_under_60(n)? as i64;
                let grain = if u == "年" {
                    crate::dimensions::time_grain::Grain::Year
                } else if u == "週間" || u == "週" {
                    crate::dimensions::time_grain::Grain::Week
                } else if u == "日" {
                    crate::dimensions::time_grain::Grain::Day
                } else if u == "時間" {
                    crate::dimensions::time_grain::Grain::Hour
                } else {
                    crate::dimensions::time_grain::Grain::Minute
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: count,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "過去N時間/日/週/月/年 (ja)".to_string(),
            pattern: vec![regex("((?:過去)|(?:最後の?)|(?:直近(?:の)?)|(?:直在)|(?:前の?))\\s*(\\d+)\\s*(秒|分|時間|日|週間|週|ヶ?月|四半期|年)(間)?")],
            production: Box::new(|nodes| {
                let (n, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let count: i64 = n.parse().ok()?;
                let grain = if u == "秒" {
                    crate::dimensions::time_grain::Grain::Second
                } else if u == "分" {
                    crate::dimensions::time_grain::Grain::Minute
                } else if u == "時間" {
                    crate::dimensions::time_grain::Grain::Hour
                } else if u == "日" {
                    crate::dimensions::time_grain::Grain::Day
                } else if u == "週間" || u == "週" {
                    crate::dimensions::time_grain::Grain::Week
                } else if u.contains("月") {
                    crate::dimensions::time_grain::Grain::Month
                } else if u == "四半期" {
                    crate::dimensions::time_grain::Grain::Quarter
                } else if u == "年" {
                    crate::dimensions::time_grain::Grain::Year
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: count,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "前秒/前分/前日... (ja)".to_string(),
            pattern: vec![regex("前秒|前分|前時間|前日|前週|前月|前年|前四半期")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?,
                    _ => return None,
                };
                let grain = if s == "前秒" {
                    crate::dimensions::time_grain::Grain::Second
                } else if s == "前分" {
                    crate::dimensions::time_grain::Grain::Minute
                } else if s == "前時間" {
                    crate::dimensions::time_grain::Grain::Hour
                } else if s == "前日" {
                    crate::dimensions::time_grain::Grain::Day
                } else if s == "前週" {
                    crate::dimensions::time_grain::Grain::Week
                } else if s == "前月" {
                    crate::dimensions::time_grain::Grain::Month
                } else if s == "前四半期" {
                    crate::dimensions::time_grain::Grain::Quarter
                } else {
                    crate::dimensions::time_grain::Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: -1 })))
            }),
        },
        Rule {
            name: "昨秒/昨分/昨時間... (ja)".to_string(),
            pattern: vec![regex("昨秒|昨分|昨時間|昨週|昨月|昨年|昨四半期")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?,
                    _ => return None,
                };
                let grain = if s == "昨秒" {
                    crate::dimensions::time_grain::Grain::Second
                } else if s == "昨分" {
                    crate::dimensions::time_grain::Grain::Minute
                } else if s == "昨時間" {
                    crate::dimensions::time_grain::Grain::Hour
                } else if s == "昨週" {
                    crate::dimensions::time_grain::Grain::Week
                } else if s == "昨月" {
                    crate::dimensions::time_grain::Grain::Month
                } else if s == "昨四半期" {
                    crate::dimensions::time_grain::Grain::Quarter
                } else {
                    crate::dimensions::time_grain::Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: -1 })))
            }),
        },
        Rule {
            name: "this/next/last weekend (ja)".to_string(),
            pattern: vec![regex("今週末|来週末|先週末")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?,
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::Weekend);
                if s.starts_with("来週") {
                    t.direction = Some(Direction::Future);
                } else if s.starts_with("先週") {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "今/当/現/来/次/前/先 <cycle> (ja)".to_string(),
            pattern: vec![regex("(今|当|現|本|来|次|翌|前|先)(週|月|年|四半期)")],
            production: Box::new(|nodes| {
                let (q, c) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let grain = match c {
                    "週" => crate::dimensions::time_grain::Grain::Week,
                    "月" => crate::dimensions::time_grain::Grain::Month,
                    "年" => crate::dimensions::time_grain::Grain::Year,
                    "四半期" => crate::dimensions::time_grain::Grain::Quarter,
                    _ => return None,
                };
                let offset = if q == "今" || q == "当" || q == "現" || q == "本" {
                    0
                } else if q == "来" || q == "次" || q == "翌" {
                    1
                } else {
                    -1
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset })))
            }),
        },
        Rule {
            name: "現在の週/この1週間/このひと月 (ja)".to_string(),
            pattern: vec![regex("現在の週|この1週間|このひと月")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?,
                    _ => return None,
                };
                let grain = if s.contains("週") {
                    crate::dimensions::time_grain::Grain::Week
                } else {
                    crate::dimensions::time_grain::Grain::Month
                };
                Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(grain))))
            }),
        },
        Rule {
            name: "この/次の/前の <cycle> (ja)".to_string(),
            pattern: vec![regex("(この|次の|前の|先の|当|現)\\s*(週|月|年|四半期)")],
            production: Box::new(|nodes| {
                let (q, c) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let grain = match c {
                    "週" => crate::dimensions::time_grain::Grain::Week,
                    "月" => crate::dimensions::time_grain::Grain::Month,
                    "年" => crate::dimensions::time_grain::Grain::Year,
                    "四半期" => crate::dimensions::time_grain::Grain::Quarter,
                    _ => return None,
                };
                let offset = if q == "この" || q == "当" || q == "現" {
                    0
                } else if q == "次の" {
                    1
                } else {
                    -1
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset })))
            }),
        },
        Rule {
            name: "こんしゅう/らいしゅう/せんしゅう (ja)".to_string(),
            pattern: vec![regex("こんしゅう|らいしゅう|せんしゅう|こんげつ|らいげつ|せんげつ|ことし|らいねん|きょねん")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?,
                    _ => return None,
                };
                let (grain, offset) = match s {
                    "こんしゅう" => (crate::dimensions::time_grain::Grain::Week, 0),
                    "らいしゅう" => (crate::dimensions::time_grain::Grain::Week, 1),
                    "せんしゅう" => (crate::dimensions::time_grain::Grain::Week, -1),
                    "こんげつ" => (crate::dimensions::time_grain::Grain::Month, 0),
                    "らいげつ" => (crate::dimensions::time_grain::Grain::Month, 1),
                    "せんげつ" => (crate::dimensions::time_grain::Grain::Month, -1),
                    "ことし" => (crate::dimensions::time_grain::Grain::Year, 0),
                    "らいねん" => (crate::dimensions::time_grain::Grain::Year, 1),
                    "きょねん" => (crate::dimensions::time_grain::Grain::Year, -1),
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset })))
            }),
        },
        Rule {
            name: "今週/来週/先週/今年/去年... (ja)".to_string(),
            pattern: vec![regex("今週|来週|先週|今月|来月|先月|今年|来年|去年")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?,
                    _ => return None,
                };
                let (grain, offset) = match s {
                    "今週" => (crate::dimensions::time_grain::Grain::Week, 0),
                    "来週" => (crate::dimensions::time_grain::Grain::Week, 1),
                    "先週" => (crate::dimensions::time_grain::Grain::Week, -1),
                    "今月" => (crate::dimensions::time_grain::Grain::Month, 0),
                    "来月" => (crate::dimensions::time_grain::Grain::Month, 1),
                    "先月" => (crate::dimensions::time_grain::Grain::Month, -1),
                    "今年" => (crate::dimensions::time_grain::Grain::Year, 0),
                    "来年" => (crate::dimensions::time_grain::Grain::Year, 1),
                    "去年" => (crate::dimensions::time_grain::Grain::Year, -1),
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset })))
            }),
        },
        Rule {
            name: "当QTR/次QTR (ja)".to_string(),
            pattern: vec![regex("(今|当|現|来|次|前|先)QTR")],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let offset = if q == "今" || q == "当" || q == "現" {
                    0
                } else if q == "来" || q == "次" {
                    1
                } else {
                    -1
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: crate::dimensions::time_grain::Grain::Quarter,
                    offset,
                })))
            }),
        },
        Rule {
            name: "現行四半期 (ja)".to_string(),
            pattern: vec![regex("現行四半期|現在四半期|現在の四半期")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: crate::dimensions::time_grain::Grain::Quarter,
                offset: 0,
            })))),
        },
        Rule {
            name: "Q3 (ja)".to_string(),
            pattern: vec![regex("Q([1-4])")],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let quarter: u32 = q.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Quarter(quarter))))
            }),
        },
        Rule {
            name: "yesterday (ja)".to_string(),
            pattern: vec![regex("昨日|きのう|さくじつ|前日|ぜんじつ")],
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
                    h = h.checked_add(12)?;
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
                    h = h.checked_add(12)?;
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
        Rule {
            name: "03/25に (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})[/／](\\d{1,2})に")],
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
        Rule {
            name: "2021年三月25日に (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年\\s*三月\\s*(\\d{1,2})日に")],
            production: Box::new(|nodes| {
                let (y, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "2021年三月に (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年\\s*三月に")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(3))),
                ))))
            }),
        },
        Rule {
            name: "2021/03に (ja)".to_string(),
            pattern: vec![regex("(\\d{4})[/.](\\d{1,2})に")],
            production: Box::new(|nodes| {
                let (y, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(month))),
                ))))
            }),
        },
        Rule {
            name: "2020年6月に (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年\\s*(\\d{1,2})月(の間|中)?(に|は|で)?")],
            production: Box::new(|nodes| {
                let (y, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(month))),
                ))))
            }),
        },
        Rule {
            name: "2020年6月より前 (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年\\s*(\\d{1,2})月(までに?|よりも?前|以前)")],
            production: Box::new(|nodes| {
                let (y, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(month))),
                ));
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "2020年6月以降 (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年\\s*(\\d{1,2})月(初め)?(以(降|来)に?|～|~|よりも?後|から)")],
            production: Box::new(|nodes| {
                let (y, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(month))),
                ));
                t.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "2020年6月1日までに (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年\\s*(\\d{1,2})月\\s*(\\d{1,2})日(までに?|よりも?前|以前)")],
            production: Box::new(|nodes| {
                let (y, m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) });
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "6月1日までに (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})月\\s*(\\d{1,2})日(までに?|よりも?前|以前)")],
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
                let mut t = TimeData::new(TimeForm::DateMDY { month, day, year: None });
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "2ヶ月で (ja)".to_string(),
            pattern: vec![regex("(\\d+)ヶ?月(後|で|間で|経ったら)")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let m: i64 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: m,
                    grain: crate::dimensions::time_grain::Grain::Month,
                })))
            }),
        },
        Rule {
            name: "2分後/2時間で/... (ja)".to_string(),
            pattern: vec![regex("(\\d+)(秒|分|時間|日|週間|週|ヶ?月|四半期|年)(後|(間)?で|経ったら|経ってから|経過後に|経過してから|経過したら|過ぎに|過ぎたら)")],
            production: Box::new(|nodes| {
                let (n, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let value: i64 = n.parse().ok()?;
                let grain = if u == "秒" {
                    crate::dimensions::time_grain::Grain::Second
                } else if u == "分" {
                    crate::dimensions::time_grain::Grain::Minute
                } else if u == "時間" {
                    crate::dimensions::time_grain::Grain::Hour
                } else if u == "日" {
                    crate::dimensions::time_grain::Grain::Day
                } else if u == "週間" || u == "週" {
                    crate::dimensions::time_grain::Grain::Week
                } else if u.contains("月") {
                    crate::dimensions::time_grain::Grain::Month
                } else if u == "四半期" {
                    crate::dimensions::time_grain::Grain::Quarter
                } else {
                    crate::dimensions::time_grain::Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: value, grain })))
            }),
        },
        Rule {
            name: "一週間後/二ヶ月後 (ja)".to_string(),
            pattern: vec![regex("([一二三四五六七八九十]+)(秒|分|時間|日|週間|週|ヶ?月|四半期|年)(後|(間)?で|経ったら|経ってから|経過後に|経過してから|経過したら|過ぎに|過ぎたら)")],
            production: Box::new(|nodes| {
                let (n, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let value: i64 = parse_kanji_under_60(n)? as i64;
                let grain = if u == "秒" {
                    crate::dimensions::time_grain::Grain::Second
                } else if u == "分" {
                    crate::dimensions::time_grain::Grain::Minute
                } else if u == "時間" {
                    crate::dimensions::time_grain::Grain::Hour
                } else if u == "日" {
                    crate::dimensions::time_grain::Grain::Day
                } else if u == "週間" || u == "週" {
                    crate::dimensions::time_grain::Grain::Week
                } else if u.contains("月") {
                    crate::dimensions::time_grain::Grain::Month
                } else if u == "四半期" {
                    crate::dimensions::time_grain::Grain::Quarter
                } else {
                    crate::dimensions::time_grain::Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: value, grain })))
            }),
        },
        Rule {
            name: "2021.03.25に (ja)".to_string(),
            pattern: vec![regex("(\\d{4})[./／](\\d{1,2})[./／](\\d{1,2})に")],
            production: Box::new(|nodes| {
                let (y, m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "2021.03.25から/以降 (ja)".to_string(),
            pattern: vec![regex("(\\d{4})[./／](\\d{1,2})[./／](\\d{1,2})(初め)?(以(降|来)に?|～|~|よりも?後|から)")],
            production: Box::new(|nodes| {
                let (y, m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) });
                t.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "2021.03.25まで/以前 (ja)".to_string(),
            pattern: vec![regex("(\\d{4})[./／](\\d{1,2})[./／](\\d{1,2})(までに?|よりも?前|以前)")],
            production: Box::new(|nodes| {
                let (y, m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) });
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "2021/03/25から2021/03/30まで (ja)".to_string(),
            pattern: vec![regex("(\\d{4})[./／](\\d{1,2})[./／](\\d{1,2})(から|以降|より|～|~)(\\d{4})[./／](\\d{1,2})[./／](\\d{1,2})(まで|以前)?")],
            production: Box::new(|nodes| {
                let (y1, m1, d1, y2, m2, d2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?, rm.group(5)?, rm.group(6)?, rm.group(7)?),
                    _ => return None,
                };
                let year1: i32 = y1.parse().ok()?;
                let month1: u32 = m1.parse().ok()?;
                let day1: u32 = d1.parse().ok()?;
                let year2: i32 = y2.parse().ok()?;
                let month2: u32 = m2.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                if !(1..=12).contains(&month1) || !(1..=31).contains(&day1) || !(1..=12).contains(&month2) || !(1..=31).contains(&day2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month: month1, day: day1, year: Some(year1) });
                let to = TimeData::new(TimeForm::DateMDY { month: month2, day: day2, year: Some(year2) });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "2021/03/25～03/30 (ja)".to_string(),
            pattern: vec![regex("(\\d{4})[./／](\\d{1,2})[./／](\\d{1,2})(～|~|から)(\\d{1,2})[./／](\\d{1,2})")],
            production: Box::new(|nodes| {
                let (y, m1, d1, m2, d2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?, rm.group(5)?, rm.group(6)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month1: u32 = m1.parse().ok()?;
                let day1: u32 = d1.parse().ok()?;
                let month2: u32 = m2.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                if !(1..=12).contains(&month1) || !(1..=31).contains(&day1) || !(1..=12).contains(&month2) || !(1..=31).contains(&day2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month: month1, day: day1, year: Some(year) });
                let to = TimeData::new(TimeForm::DateMDY { month: month2, day: day2, year: Some(year) });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "火曜日に (ja)".to_string(),
            pattern: vec![regex("月曜日に|火曜日に|水曜日に|木曜日に|金曜日に|土曜日に|日曜日に")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.starts_with('月') {
                    0
                } else if s.starts_with('火') {
                    1
                } else if s.starts_with('水') {
                    2
                } else if s.starts_with('木') {
                    3
                } else if s.starts_with('金') {
                    4
                } else if s.starts_with('土') {
                    5
                } else if s.starts_with('日') {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "火曜日 (ja)".to_string(),
            pattern: vec![regex("月曜日|火曜日|水曜日|木曜日|金曜日|土曜日|日曜日|月曜|火曜|水曜|木曜|金曜|土曜|日曜")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.starts_with('月') {
                    0
                } else if s.starts_with('火') {
                    1
                } else if s.starts_with('水') {
                    2
                } else if s.starts_with('木') {
                    3
                } else if s.starts_with('金') {
                    4
                } else if s.starts_with('土') {
                    5
                } else if s.starts_with('日') {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "げつようび (ja)".to_string(),
            pattern: vec![regex("げつようび|かようび|すいようび|もくようび|きんようび|どようび|にちようび")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = match s {
                    "げつようび" => 0,
                    "かようび" => 1,
                    "すいようび" => 2,
                    "もくようび" => 3,
                    "きんようび" => 4,
                    "どようび" => 5,
                    "にちようび" => 6,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "火曜に (ja)".to_string(),
            pattern: vec![regex("月曜に|火曜に|水曜に|木曜に|金曜に|土曜に|日曜に")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.starts_with('月') {
                    0
                } else if s.starts_with('火') {
                    1
                } else if s.starts_with('水') {
                    2
                } else if s.starts_with('木') {
                    3
                } else if s.starts_with('金') {
                    4
                } else if s.starts_with('土') {
                    5
                } else if s.starts_with('日') {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "月に (ja)".to_string(),
            pattern: vec![regex("月に|火に|水に|木に|金に|土に|日に")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.starts_with('月') {
                    0
                } else if s.starts_with('火') {
                    1
                } else if s.starts_with('水') {
                    2
                } else if s.starts_with('木') {
                    3
                } else if s.starts_with('金') {
                    4
                } else if s.starts_with('土') {
                    5
                } else if s.starts_with('日') {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "火は/で (ja)".to_string(),
            pattern: vec![regex("月は|火は|水は|木は|金は|土は|日は|月で|火で|水で|木で|金で|土で|日で")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let c = s.chars().next()?;
                let dow = match c {
                    '月' => 0,
                    '火' => 1,
                    '水' => 2,
                    '木' => 3,
                    '金' => 4,
                    '土' => 5,
                    '日' => 6,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "(火)に (ja)".to_string(),
            pattern: vec![regex("\\(月\\)に|\\(火\\)に|\\(水\\)に|\\(木\\)に|\\(金\\)に|\\(土\\)に|\\(日\\)に")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.contains('月') {
                    0
                } else if s.contains('火') {
                    1
                } else if s.contains('水') {
                    2
                } else if s.contains('木') {
                    3
                } else if s.contains('金') {
                    4
                } else if s.contains('土') {
                    5
                } else if s.contains('日') {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "（火）に (ja)".to_string(),
            pattern: vec![regex("（月）に|（火）に|（水）に|（木）に|（金）に|（土）に|（日）に")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.contains('月') {
                    0
                } else if s.contains('火') {
                    1
                } else if s.contains('水') {
                    2
                } else if s.contains('木') {
                    3
                } else if s.contains('金') {
                    4
                } else if s.contains('土') {
                    5
                } else if s.contains('日') {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "かようびに (ja)".to_string(),
            pattern: vec![regex("げつようびに|かようびに|すいようびに|もくようびに|きんようびに|どようびに|にちようびに")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.starts_with("げつ") {
                    0
                } else if s.starts_with("か") {
                    1
                } else if s.starts_with("すい") {
                    2
                } else if s.starts_with("もく") {
                    3
                } else if s.starts_with("きん") {
                    4
                } else if s.starts_with("ど") {
                    5
                } else if s.starts_with("にち") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "かように (ja)".to_string(),
            pattern: vec![regex("げつように|かように|すいように|もくように|きんように|どように|にちように")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.starts_with("げつ") {
                    0
                } else if s.starts_with("か") {
                    1
                } else if s.starts_with("すい") {
                    2
                } else if s.starts_with("もく") {
                    3
                } else if s.starts_with("きん") {
                    4
                } else if s.starts_with("ど") {
                    5
                } else if s.starts_with("にち") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "火曜日は (ja)".to_string(),
            pattern: vec![regex("月曜日は|火曜日は|水曜日は|木曜日は|金曜日は|土曜日は|日曜日は")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.starts_with('月') {
                    0
                } else if s.starts_with('火') {
                    1
                } else if s.starts_with('水') {
                    2
                } else if s.starts_with('木') {
                    3
                } else if s.starts_with('金') {
                    4
                } else if s.starts_with('土') {
                    5
                } else if s.starts_with('日') {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "火曜日で (ja)".to_string(),
            pattern: vec![regex("月曜日で|火曜日で|水曜日で|木曜日で|金曜日で|土曜日で|日曜日で")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let dow = if s.starts_with('月') {
                    0
                } else if s.starts_with('火') {
                    1
                } else if s.starts_with('水') {
                    2
                } else if s.starts_with('木') {
                    3
                } else if s.starts_with('金') {
                    4
                } else if s.starts_with('土') {
                    5
                } else if s.starts_with('日') {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "25日に (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})日に")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "25日は (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})日は")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "25日で (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})日で")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "6月に (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})月に")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "1月 (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})月")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "１月 (ja)".to_string(),
            pattern: vec![regex("([０-９]{1,2})月")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = normalize_zenkaku_digits(m).parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "6がつに (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})がつに")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "6月中に (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})月中に")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "6月の間に (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})月の間に")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "6月は (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})月は")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "6月で (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})月で")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "6月より前 (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})月(までに?|よりも?前|以前)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::Month(month));
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "6月から/以降 (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})月(初め)?(以(降|来)に?|～|~|よりも?後|から)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::Month(month));
                t.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "六月に (ja)".to_string(),
            pattern: vec![regex("六月に")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(6))))),
        },
        Rule {
            name: "一月..十二月 (ja)".to_string(),
            pattern: vec![regex("一月|二月|三月|四月|五月|六月|七月|八月|九月|十月|十一月|十二月")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?,
                    _ => return None,
                };
                let month = match s {
                    "一月" => 1,
                    "二月" => 2,
                    "三月" => 3,
                    "四月" => 4,
                    "五月" => 5,
                    "六月" => 6,
                    "七月" => 7,
                    "八月" => 8,
                    "九月" => 9,
                    "十月" => 10,
                    "十一月" => 11,
                    "十二月" => 12,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "いちがつ..じゅうにがつ (ja)".to_string(),
            pattern: vec![regex("いちがつ|にがつ|さんがつ|しがつ|よんがつ|ごがつ|ろくがつ|しちがつ|なながつ|はちがつ|くがつ|じゅうがつ|じゅういちがつ|じゅうにがつ")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?,
                    _ => return None,
                };
                let month = match s {
                    "いちがつ" => 1,
                    "にがつ" => 2,
                    "さんがつ" => 3,
                    "しがつ" | "よんがつ" => 4,
                    "ごがつ" => 5,
                    "ろくがつ" => 6,
                    "しちがつ" | "なながつ" => 7,
                    "はちがつ" => 8,
                    "くがつ" => 9,
                    "じゅうがつ" => 10,
                    "じゅういちがつ" => 11,
                    "じゅうにがつ" => 12,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "ろくがつに (ja)".to_string(),
            pattern: vec![regex("ろくがつに")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(6))))),
        },
        Rule {
            name: "2020年に (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年に")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        Rule {
            name: "2020年は (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年は")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        Rule {
            name: "2020年で (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年で")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        Rule {
            name: "2020年より前 (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年(までに?|よりも?前|以前)")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let mut t = TimeData::new(TimeForm::Year(year));
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "2020年から/以降 (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年(初め)?(以(降|来)に?|～|~|よりも?後|から)")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let mut t = TimeData::new(TimeForm::Year(year));
                t.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "2020年から2021年まで (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年(-|~|～|・|、|から)(\\d{4})年?(まで|以前|の間|にかけて)?")],
            production: Box::new(|nodes| {
                let (y1, y2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?),
                    _ => return None,
                };
                let year1: i32 = y1.parse().ok()?;
                let year2: i32 = y2.parse().ok()?;
                let from = TimeData::new(TimeForm::Year(year1));
                let to = TimeData::new(TimeForm::Year(year2));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "2020から2021年 (ja)".to_string(),
            pattern: vec![regex("(\\d{4})(-|~|～|・|、|から)(\\d{4})年")],
            production: Box::new(|nodes| {
                let (y1, y2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?),
                    _ => return None,
                };
                let year1: i32 = y1.parse().ok()?;
                let year2: i32 = y2.parse().ok()?;
                let from = TimeData::new(TimeForm::Year(year1));
                let to = TimeData::new(TimeForm::Year(year2));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "6月頭から7月いっぱい (ja)".to_string(),
            pattern: vec![regex("(\\d{1,2})月(頭|初め)?から(\\d{1,2})月(末|いっぱい)?(まで|にかけて)?")],
            production: Box::new(|nodes| {
                let (m1, m2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?),
                    _ => return None,
                };
                let month1: u32 = m1.parse().ok()?;
                let month2: u32 = m2.parse().ok()?;
                if !(1..=12).contains(&month1) || !(1..=12).contains(&month2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::Month(month1));
                let to = TimeData::new(TimeForm::Month(month2));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "2020年6月頭から7月末まで (ja)".to_string(),
            pattern: vec![regex("(\\d{4})年(\\d{1,2})月(頭|初め)?から(\\d{1,2})月(末|いっぱい)?(まで|にかけて)?")],
            production: Box::new(|nodes| {
                let (y, m1, m2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(4)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month1: u32 = m1.parse().ok()?;
                let month2: u32 = m2.parse().ok()?;
                if !(1..=12).contains(&month1) || !(1..=12).contains(&month2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(month1))),
                ));
                let to = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(month2))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "令和2年に (ja)".to_string(),
            pattern: vec![regex("令和\\s*(\\d+)年(に|は|で)")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let reiwa: i32 = n.parse().ok()?;
                let year = 2018_i32.checked_add(reiwa)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        Rule {
            name: "和暦年に (ja)".to_string(),
            pattern: vec![regex("(明治|大正|昭和|平成|令和)\\s*(\\d+)年(に|は|で)")],
            production: Box::new(|nodes| {
                let (era, n) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let e = era.trim();
                let v: i32 = n.parse().ok()?;
                let base: i32 = match e {
                    "明治" => 1867,
                    "大正" => 1911,
                    "昭和" => 1925,
                    "平成" => 1988,
                    "令和" => 2018,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Year(base.checked_add(v)?))))
            }),
        },
        Rule {
            name: "和暦元年に (ja)".to_string(),
            pattern: vec![regex("(明治|大正|昭和|平成|令和)元年(に|は|で)")],
            production: Box::new(|nodes| {
                let era = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let e = era.trim();
                let year = match e {
                    "明治" => 1868,
                    "大正" => 1912,
                    "昭和" => 1926,
                    "平成" => 1989,
                    "令和" => 2019,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
    ]);
    rules
}
