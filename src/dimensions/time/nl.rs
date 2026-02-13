use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};
use super::{IntervalDirection, TimeData, TimeForm};

fn is_time(td: &TokenData) -> bool { matches!(td, TokenData::Time(_)) }
fn time_data(td: &TokenData) -> Option<&TimeData> { match td { TokenData::Time(d) => Some(d), _ => None } }

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule { name: "now (nl)".to_string(), pattern: vec![regex("nu|direct|zojuist")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))) },
        Rule { name: "today (nl)".to_string(), pattern: vec![regex("vandaag|op deze dag")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))) },
        Rule { name: "tomorrow (nl)".to_string(), pattern: vec![regex("morgen")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))) },
        Rule { name: "yesterday (nl)".to_string(), pattern: vec![regex("gisteren")], production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))) },
        Rule {
            name: "day of week (nl)".to_string(),
            pattern: vec![regex("maandags?|ma\\.|dinsdags?|di\\.|woensdags?|woe\\.|donderdags?|do\\.|vrijdags?|vr(ij)?\\.|zaterdags?|zat?\\.|zondags?|zon?\\.")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("maan") || s == "ma." {
                    0
                } else if s.starts_with("dins") || s == "di." {
                    1
                } else if s.starts_with("woen") || s == "woe." {
                    2
                } else if s.starts_with("dond") || s == "do." {
                    3
                } else if s.starts_with("vrij") || s == "vr." || s == "vrij." {
                    4
                } else if s.starts_with("zater") || s == "zat." || s == "za." {
                    5
                } else if s.starts_with("zond") || s == "zon." || s == "zo." {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "voor <time> (nl)".to_string(),
            pattern: vec![regex("voor"), predicate(is_time)],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[1].token_data)?.clone();
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "sinterklaas (nl)".to_string(),
            pattern: vec![regex("sinterklaas")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 12, day: 6, year: None })))),
        },
        Rule {
            name: "1 maart (nl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+maart")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day, year: None })))
            }),
        },
    ]);
    rules
}
