use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (ro)".to_string(),
            pattern: vec![regex("acum")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ro)".to_string(),
            pattern: vec![regex("azi|astăzi|astazi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ro)".to_string(),
            pattern: vec![regex("m[âa]ine")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (ro)".to_string(),
            pattern: vec![regex("ieri")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (ro)".to_string(),
            pattern: vec![regex("lu(n(ea|i)?)?|ma(r((t|ț)(ea|i))?)?|mi(e(rcur(ea|i))?)?|jo(ia?)?|vi(n(er(ea|i))?)?|s(a|â)mb(a|ă)t(a|ă)|s(a|â)m|du(m(inic(a|ă))?)?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("lu") {
                    0
                } else if s.starts_with("ma") {
                    1
                } else if s.starts_with("mi") {
                    2
                } else if s.starts_with("jo") {
                    3
                } else if s.starts_with("vi") {
                    4
                } else if s.starts_with("sâ") || s.starts_with("sa") {
                    5
                } else if s.starts_with("du") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "lunea asta (ro)".to_string(),
            pattern: vec![regex("lunea asta|lunea aceasta")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(0))))),
        },
        Rule {
            name: "iunie 19-20 (ro)".to_string(),
            pattern: vec![regex("iunie\\s+(\\d{1,2})-(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 6, day, year: None })))
            }),
        },
        Rule {
            name: "christmas day (ro)".to_string(),
            pattern: vec![regex("ziua de craciun|ziua de crăciun")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 12, day: 25, year: None })))),
        },
        Rule {
            name: "craciun (ro)".to_string(),
            pattern: vec![regex("craciun|crăciun")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 12, day: 25, year: None })))),
        },
    ]);
    rules
}
