use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (it)".to_string(),
            pattern: vec![regex("subito|adesso|ora|immediatamente|in questo momento|in giornata")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (it)".to_string(),
            pattern: vec![regex("oggi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (it)".to_string(),
            pattern: vec![regex("domani")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (it)".to_string(),
            pattern: vec![regex("ieri")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (it)".to_string(),
            pattern: vec![regex("luned[ìi]|lun\\.?|marted[ìi]|mar\\.?|mercoled[ìi]|mer\\.?|gioved[ìi]|gio\\.?|venerd[ìi]|ven\\.?|sabato|sab\\.?|domenica|dom\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("luned") || s == "lun" || s == "lun." {
                    0
                } else if s.starts_with("marted") || s == "mar" || s == "mar." {
                    1
                } else if s.starts_with("mercoled") || s == "mer" || s == "mer." {
                    2
                } else if s.starts_with("gioved") || s == "gio" || s == "gio." {
                    3
                } else if s.starts_with("venerd") || s == "ven" || s == "ven." {
                    4
                } else if s == "sabato" || s == "sab" || s == "sab." {
                    5
                } else if s == "domenica" || s == "dom" || s == "dom." {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "lunedì 18 febbraio (it)".to_string(),
            pattern: vec![regex("luned[ìi]\\s*(\\d{1,2})\\s+febbraio")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 2,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "il 19 (it)".to_string(),
            pattern: vec![regex("il\\s*(\\d{1,2})")],
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
            name: "gennaio 2013 (it)".to_string(),
            pattern: vec![regex("(gennaio|genn?\\.?|febbraio|febb?\\.?|marzo|mar\\.?|aprile|apr\\.?|maggio|magg?\\.?|giugno|giu\\.?|luglio|lug\\.?|agosto|ago\\.?|settembre|sett?\\.?|ottobre|ott\\.?|novembre|nov\\.?|dicembre|dic\\.?)[\\s]+(\\d{4})")],
            production: Box::new(|nodes| {
                let (mname, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month = if mname.starts_with("gen") {
                    1
                } else if mname.starts_with("feb") {
                    2
                } else if mname.starts_with("mar") {
                    3
                } else if mname.starts_with("apr") {
                    4
                } else if mname.starts_with("mag") {
                    5
                } else if mname.starts_with("giu") {
                    6
                } else if mname.starts_with("lug") {
                    7
                } else if mname.starts_with("ago") {
                    8
                } else if mname.starts_with("set") {
                    9
                } else if mname.starts_with("ott") {
                    10
                } else if mname.starts_with("nov") {
                    11
                } else if mname.starts_with("dic") {
                    12
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(month))),
                ))))
            }),
        },
    ]);
    rules
}
