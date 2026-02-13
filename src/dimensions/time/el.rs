use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (el)".to_string(),
            pattern: vec![regex("τ[ώω]ρα")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (el)".to_string(),
            pattern: vec![regex("σ[ήη]μερα")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (el)".to_string(),
            pattern: vec![regex("α[ύυ]ριο")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (el)".to_string(),
            pattern: vec![regex("χθες")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (el)".to_string(),
            pattern: vec![regex("δευτ(έρας?|\\.?)|τρ[ιί](της?|\\.?)|τετ(άρτης?|\\.?)|π[εέ]μ(πτης?|\\.?)|παρ(ασκευής?|\\.?)|σ[αά]β(β[αά]το[νυ]?|\\.?)|κυρ(ιακής?|\\.?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("δευ") {
                    0
                } else if s.contains("τρί") || s.contains("τρι") {
                    1
                } else if s.contains("τετ") {
                    2
                } else if s.contains("πέμ") || s.contains("πεμ") {
                    3
                } else if s.contains("παρασκευ") || s.starts_with("παρ") {
                    4
                } else if s.contains("σάβ") || s.contains("σαβ") {
                    5
                } else if s.contains("κυριακ") || s.starts_with("κυρ") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "date with flevary (el)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+φ(λεβ[άα]ρη|εβρουαρ[ίι]ου)(,\\s*δευτ)?")],
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
    ]);
    rules
}
