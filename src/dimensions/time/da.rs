use super::{Direction, TimeData, TimeForm};
use crate::dimensions::time_grain::Grain;
use crate::pattern::regex;
use crate::types::{Rule, TokenData};

fn da_small_number(s: &str) -> Option<i32> {
    match s {
        "en" | "et" | "én" | "ét" => Some(1),
        "to" => Some(2),
        "tre" => Some(3),
        "fire" => Some(4),
        "fem" => Some(5),
        "seks" => Some(6),
        "syv" => Some(7),
        "otte" => Some(8),
        "ni" => Some(9),
        "ti" => Some(10),
        "elleve" => Some(11),
        "tolv" => Some(12),
        "fjorten" => Some(14),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (da)".to_string(),
            pattern: vec![regex("nu")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (da)".to_string(),
            pattern: vec![regex("i dag|idag")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (da)".to_string(),
            pattern: vec![regex("i morgen|imorgen")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (da)".to_string(),
            pattern: vec![regex("i g[åa]r|ig[åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (da)".to_string(),
            pattern: vec![regex("mandag|tirsdag|onsdag|torsdag|tors\\.?|fredag|fre\\.?|l[øo]rdag|l[øo]r\\.?|s[øo]ndag|s[øo]n\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("mandag") {
                    0
                } else if s.contains("tirsdag") {
                    1
                } else if s.contains("onsdag") {
                    2
                } else if s.contains("torsdag") || s.starts_with("tors") {
                    3
                } else if s.contains("fredag") || s.starts_with("fre") {
                    4
                } else if s.contains("rdag") || s.contains("lørdag") || s.starts_with("lør") || s.starts_with("lor") {
                    5
                } else if s.contains("ndag") || s.starts_with("søn") || s.starts_with("son") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "den første marts (da)".to_string(),
            pattern: vec![regex("den f[øo]rste marts|1\\.\\s*marts|den\\s*1\\.\\s*marts")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "3 marts (da)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.?\\s*marts")],
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
        Rule {
            name: "den tredje marts (da)".to_string(),
            pattern: vec![regex("den\\s+tredje\\s+marts")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 3,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "tredje marts 2015 (da)".to_string(),
            pattern: vec![regex("tredje\\s+marts\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 3, year: Some(year) })))
            }),
        },
        Rule {
            name: "på den 15. (da)".to_string(),
            pattern: vec![regex("p[åa]\\s+den\\s+(\\d{1,2})\\.?")],
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
            name: "den 15. (da)".to_string(),
            pattern: vec![regex("[Dd]en\\s+(\\d{1,2})\\.")],
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
            name: "den 15 (da)".to_string(),
            pattern: vec![regex("[Dd]en\\s+(\\d{1,2})")],
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
            name: "15. februar (da)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.\\s+februar")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day, year: None })))
            }),
        },
        Rule {
            name: "februar 15 (da)".to_string(),
            pattern: vec![regex("februar\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day, year: None })))
            }),
        },
        Rule {
            name: "Oktober 2014 (da)".to_string(),
            pattern: vec![regex("[Oo]ktober\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(10))),
                ))))
            }),
        },
        Rule {
            name: "næste marts (da)".to_string(),
            pattern: vec![regex("n[æa]ste\\s+marts")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::Month(3));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "Ons, Feb13 (da)".to_string(),
            pattern: vec![regex("Ons,?\\s*Feb\\.?\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day, year: None })))
            }),
        },
        Rule {
            name: "denne uge (da)".to_string(),
            pattern: vec![regex("denne uge")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "sidste uge (da)".to_string(),
            pattern: vec![regex("sidste uge")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "næste uge (da)".to_string(),
            pattern: vec![regex("n[æa]ste uge")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "sidste måned (da)".to_string(),
            pattern: vec![regex("sidste m[åa]ned")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "næste måned (da)".to_string(),
            pattern: vec![regex("n[æa]ste m[åa]ned")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "dette kvartal (da)".to_string(),
            pattern: vec![regex("dette kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 0 })))),
        },
        Rule {
            name: "næste kvartal (da)".to_string(),
            pattern: vec![regex("n[æa]ste kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "tredje kvartal (da)".to_string(),
            pattern: vec![regex("tredje kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "3. kvartal (da)".to_string(),
            pattern: vec![regex("3\\.?\\s*kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "4. kvartal 2018 (da)".to_string(),
            pattern: vec![regex("4\\.?\\s*kvartal\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(4, year))))
            }),
        },
        Rule {
            name: "fjerde kvartal 2018 (da)".to_string(),
            pattern: vec![regex("fjerde\\s+kvartal\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(4, year))))
            }),
        },
        Rule {
            name: "sidste år (da)".to_string(),
            pattern: vec![regex("sidste [åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "i år (da)".to_string(),
            pattern: vec![regex("i [åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))),
        },
        Rule {
            name: "dette år (da)".to_string(),
            pattern: vec![regex("dette [åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))),
        },
        Rule {
            name: "næste år (da)".to_string(),
            pattern: vec![regex("n[æa]ste [åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "i overmorgen (da)".to_string(),
            pattern: vec![regex("i overmorgen")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))),
        },
        Rule {
            name: "i forgårs (da)".to_string(),
            pattern: vec![regex("i forg[åa]rs")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))),
        },
        Rule {
            name: "next weekday (da)".to_string(),
            pattern: vec![regex("n[æa]ste\\s+(mandag|tirsdag|onsdag|torsdag|fredag|l[øo]rdag|s[øo]ndag)(\\s+igen)?")],
            production: Box::new(|nodes| {
                let (w, igen) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)),
                    _ => return None,
                };
                let dow = match w {
                    "mandag" => 0,
                    "tirsdag" => 1,
                    "onsdag" => 2,
                    "torsdag" => 3,
                    "fredag" => 4,
                    "lørdag" | "lordag" => 5,
                    "søndag" | "sondag" => 6,
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::DayOfWeek(dow));
                t.direction = Some(if igen.is_some() { Direction::FarFuture } else { Direction::Future });
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "last weekday (da)".to_string(),
            pattern: vec![regex("sidste\\s+(mandag|tirsdag|onsdag|torsdag|fredag|l[øo]rdag|s[øo]ndag)")],
            production: Box::new(|nodes| {
                let w = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let dow = match w {
                    "mandag" => 0,
                    "tirsdag" => 1,
                    "onsdag" => 2,
                    "torsdag" => 3,
                    "fredag" => 4,
                    "lørdag" | "lordag" => 5,
                    "søndag" | "sondag" => 6,
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::DayOfWeek(dow));
                t.direction = Some(Direction::Past);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<dow> i denne uge (da)".to_string(),
            pattern: vec![regex("(mandag|tirsdag|onsdag|torsdag|fredag|l[øo]rdag|s[øo]ndag)\\s+i\\s+denne\\s+uge")],
            production: Box::new(|nodes| {
                let w = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let dow = match w {
                    "mandag" => 0,
                    "tirsdag" => 1,
                    "onsdag" => 2,
                    "torsdag" => 3,
                    "fredag" => 4,
                    "lørdag" | "lordag" => 5,
                    "søndag" | "sondag" => 6,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::AllGrain(Grain::Week))),
                    Box::new(TimeData::new(TimeForm::DayOfWeek(dow))),
                ))))
            }),
        },
        Rule {
            name: "<dow> i næste uge (da)".to_string(),
            pattern: vec![regex("(mandag|tirsdag|onsdag|torsdag|fredag|l[øo]rdag|s[øo]ndag)\\s+i\\s+n[æa]ste\\s+uge|(mandag|tirsdag|onsdag|torsdag|fredag|l[øo]rdag|s[øo]ndag)\\s+n[æa]ste\\s+uge")],
            production: Box::new(|nodes| {
                let w = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
                    _ => return None,
                };
                let dow = match w {
                    "mandag" => 0,
                    "tirsdag" => 1,
                    "onsdag" => 2,
                    "torsdag" => 3,
                    "fredag" => 4,
                    "lørdag" | "lordag" => 5,
                    "søndag" | "sondag" => 6,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })),
                    Box::new(TimeData::new(TimeForm::DayOfWeek(dow))),
                ))))
            }),
        },
        Rule {
            name: "om <duration> (da)".to_string(),
            pattern: vec![regex("om\\s*(\\d+|en|et|[ée]n|[ée]t|to|tre|fire|fem|seks|syv|otte|ni|ti|elleve|tolv|fjorten)\\s*(sekund(er)?|minut(ter)?|time(r)?|dag(e)?|uge(r)?|m[åa]ned(er)?|[åa]r)(\\s+mere|\\s+fra\\s+nu)?")],
            production: Box::new(|nodes| {
                let (n_raw, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n = n_raw.parse::<i32>().ok().or_else(|| da_small_number(n_raw))?;
                let grain = if unit.starts_with("sekund") {
                    Grain::Second
                } else if unit.starts_with("minut") {
                    Grain::Minute
                } else if unit.starts_with("time") {
                    Grain::Hour
                } else if unit.starts_with("dag") {
                    Grain::Day
                } else if unit.starts_with("uge") {
                    Grain::Week
                } else if unit.starts_with("måned") || unit.starts_with("maned") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: n })))
            }),
        },
        Rule {
            name: "om 1t (da)".to_string(),
            pattern: vec![regex("om\\s*(\\d+)t")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hours: i32 = h.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: hours })))
            }),
        },
        Rule {
            name: "<duration> siden (da)".to_string(),
            pattern: vec![regex("(\\d+|en|et|[ée]n|[ée]t|to|tre|fire|fem|seks|syv|otte|ni|ti|elleve|tolv|fjorten)\\s*(dag(e)?|uge(r)?|m[åa]ned(er)?|[åa]r)\\s+siden")],
            production: Box::new(|nodes| {
                let (n_raw, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n = n_raw.parse::<i32>().ok().or_else(|| da_small_number(n_raw))?;
                let grain = if unit.starts_with("dag") {
                    Grain::Day
                } else if unit.starts_with("uge") {
                    Grain::Week
                } else if unit.starts_with("måned") || unit.starts_with("maned") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: n.checked_neg()? })))
            }),
        },
        Rule {
            name: "om en halv time (da)".to_string(),
            pattern: vec![regex("om\\s+(ca\\.?|cirka)?\\s*en halv time")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 30 })))),
        },
        Rule {
            name: "om 2,5 time (da)".to_string(),
            pattern: vec![regex("om\\s*2,5\\s*time|om\\s*(2|to)\\s+og\\s+en\\s+halv\\s+time")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 150 })))),
        },
        Rule {
            name: "om et par timer (da)".to_string(),
            pattern: vec![regex("om et par timer")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: 2 })))),
        },
        Rule {
            name: "efter 5 dage (da)".to_string(),
            pattern: vec![regex("efter\\s*(\\d+|fem)\\s+dage")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let days = n.parse::<i32>().ok().or_else(|| da_small_number(n))?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: days })))
            }),
        },
        Rule {
            name: "indenfor <duration> (da)".to_string(),
            pattern: vec![regex("indenfor\\s*(\\d+|en|et|to|tre)\\s*(uger?|dage?|timer?|m[åa]neder?|[åa]r)")],
            production: Box::new(|nodes| {
                let (n_raw, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n = n_raw.parse::<i32>().ok().or_else(|| da_small_number(n_raw))?;
                let grain = if unit.starts_with("dag") {
                    Grain::Day
                } else if unit.starts_with("uge") {
                    Grain::Week
                } else if unit.starts_with("tim") {
                    Grain::Hour
                } else if unit.starts_with("må") || unit.starts_with("ma") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: n })))
            }),
        },
        Rule {
            name: "i aften (da)".to_string(),
            pattern: vec![regex("i aften")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(TimeData::latent(TimeForm::PartOfDay(super::PartOfDay::Evening))),
                ))))
            }),
        },
        Rule {
            name: "i morgen aften/middag (da)".to_string(),
            pattern: vec![regex("i morgen\\s+(aften|middag)")],
            production: Box::new(|nodes| {
                let pod = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let right = if pod == "aften" {
                    TimeData::latent(TimeForm::PartOfDay(super::PartOfDay::Evening))
                } else {
                    TimeData::latent(TimeForm::PartOfDay(super::PartOfDay::Lunch))
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Tomorrow)),
                    Box::new(right),
                ))))
            }),
        },
        Rule {
            name: "i går aftes (da)".to_string(),
            pattern: vec![regex("i g[åa]r aftes")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Yesterday)),
                    Box::new(TimeData::latent(TimeForm::PartOfDay(super::PartOfDay::Evening))),
                ))))
            }),
        },
        Rule {
            name: "denne morgen (da)".to_string(),
            pattern: vec![regex("denne morgen|mandag morgen")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                if s.contains("mandag") {
                    Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                        Box::new(TimeData::new(TimeForm::DayOfWeek(0))),
                        Box::new(TimeData::latent(TimeForm::PartOfDay(super::PartOfDay::Morning))),
                    ))))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                        Box::new(TimeData::new(TimeForm::Today)),
                        Box::new(TimeData::latent(TimeForm::PartOfDay(super::PartOfDay::Morning))),
                    ))))
                }
            }),
        },
        Rule {
            name: "om eftermiddagen / efter frokost (da)".to_string(),
            pattern: vec![regex("om eftermiddagen|efter frokost")],
            production: Box::new(|_nodes| {
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(TimeData::latent(TimeForm::PartOfDay(
                        super::PartOfDay::Afternoon,
                    ))),
                ))))
            }),
        },
        Rule {
            name: "sidste n <cycle> (da)".to_string(),
            pattern: vec![regex("sidste|seneste"), regex("(\\d+|en|et|[ée]n|[ée]t|to|tre)"), regex("(sekund(er)?|minut(ter)?|time(r)?|dag(e)?|uge(r)?|m[åa]ned(er)?|[åa]r)")],
            production: Box::new(|nodes| {
                let (n_raw, unit) = match (&nodes[1].token_data, &nodes[2].token_data) {
                    (TokenData::RegexMatch(nm), TokenData::RegexMatch(um)) => (nm.group(1)?, um.group(1)?),
                    _ => return None,
                };
                let n: i64 = n_raw.parse::<i64>().ok().or_else(|| da_small_number(n_raw).map(|v| v as i64))?;
                let grain = if unit.starts_with("sek") {
                    Grain::Second
                } else if unit.starts_with("min") {
                    Grain::Minute
                } else if unit.starts_with("tim") {
                    Grain::Hour
                } else if unit.starts_with("dag") {
                    Grain::Day
                } else if unit.starts_with("uge") {
                    Grain::Week
                } else if unit.starts_with("må") || unit.starts_with("ma") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain { n, grain, past: true, interval: true })))
            }),
        },
        Rule {
            name: "næste n <cycle> (da)".to_string(),
            pattern: vec![regex("n[æa]ste"), regex("(\\d+|en|et|[ée]n|[ée]t|to|tre)"), regex("(sekund(er)?|minut(ter)?|time(r)?|dag(e)?|uge(r)?|m[åa]ned(er)?|[åa]r)")],
            production: Box::new(|nodes| {
                let (n_raw, unit) = match (&nodes[1].token_data, &nodes[2].token_data) {
                    (TokenData::RegexMatch(nm), TokenData::RegexMatch(um)) => (nm.group(1)?, um.group(1)?),
                    _ => return None,
                };
                let n: i64 = n_raw.parse::<i64>().ok().or_else(|| da_small_number(n_raw).map(|v| v as i64))?;
                let grain = if unit.starts_with("sek") {
                    Grain::Second
                } else if unit.starts_with("min") {
                    Grain::Minute
                } else if unit.starts_with("tim") {
                    Grain::Hour
                } else if unit.starts_with("dag") {
                    Grain::Day
                } else if unit.starts_with("uge") {
                    Grain::Week
                } else if unit.starts_with("må") || unit.starts_with("ma") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain { n, grain, past: false, interval: true })))
            }),
        },
        Rule {
            name: "klokken 3 (da)".to_string(),
            pattern: vec![regex("kl(okken|\\.)?\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, hour <= 12))))
            }),
        },
        Rule {
            name: "kvarter over 15 (da)".to_string(),
            pattern: vec![regex("kvarter over\\s*(\\d{1,2})|kvart over\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 15, hour <= 12))))
            }),
        },
        Rule {
            name: "kvarter i 12 (da)".to_string(),
            pattern: vec![regex("kvarter i\\s*(\\d{1,2})|kvart i\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if !(1..=24).contains(&hour) {
                    return None;
                }
                let out_hour = if hour == 24 { 23 } else { hour.checked_sub(1)? };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_hour, 45, out_hour <= 12))))
            }),
        },
        Rule {
            name: "20 over 15 (da)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+over\\s+(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (m, h) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let minute: u32 = m.parse().ok()?;
                let hour: u32 = h.parse().ok()?;
                if minute > 59 || hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, hour <= 12))))
            }),
        },
        Rule {
            name: "kl 16 CET/GMT (da)".to_string(),
            pattern: vec![regex("(@\\s*)?(kl(okken|\\.)?\\s*)?(\\d{1,2})(:\\d{2})?\\s*(CET|GMT|gmt|cet)")],
            production: Box::new(|nodes| {
                let (h, min_opt, tz) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(4)?, m.group(5), m.group(6)?),
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let minute: u32 = min_opt
                    .and_then(|mm| mm.trim_start_matches(':').parse::<u32>().ok())
                    .unwrap_or(0);
                if hour > 23 || minute > 59 {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::HourMinute(hour, minute, false));
                t.timezone = Some(tz.to_uppercase());
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "forrige år (da)".to_string(),
            pattern: vec![regex("forrige [åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "13-15 juli (da)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*(-|til)\\s*(\\d{1,2})\\s+(juli|Juli)")],
            production: Box::new(|nodes| {
                let (d1, d2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)?),
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month: 7, day: day2.checked_add(1)?, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), false))))
            }),
        },
        Rule {
            name: "13 juli til 15 juli (da)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+juli\\s+til\\s+(\\d{1,2})\\s+juli")],
            production: Box::new(|nodes| {
                let (d1, d2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month: 7, day: day2.checked_add(1)?, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), false))))
            }),
        },
        Rule {
            name: "8 Aug - 12 Aug (da)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+(Aug|aug|august)\\s*-\\s*(\\d{1,2})\\s+(Aug|aug|august)")],
            production: Box::new(|nodes| {
                let (d1, d2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)?),
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month: 8, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month: 8, day: day2.checked_add(1)?, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), false))))
            }),
        },
        Rule {
            name: "year 4-digit (da)".to_string(),
            pattern: vec![regex("(19\\d{2}|20\\d{2})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        Rule {
            name: "et år efter juleaften (da)".to_string(),
            pattern: vec![regex("(et|[ée]t) [åa]r efter juleaften")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 12, day: 24, year: None })))),
        },
        Rule {
            name: "juleaftensdag / nytårsaften / nytårsdag (da)".to_string(),
            pattern: vec![regex("juleaftensdag|nyt[åa]rsaften|nyt[åa]rsdag|1\\.? juledag|f[øo]rste juledag")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("jule") {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 12, day: 25, year: None })))
                } else if s.contains("aften") {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 12, day: 31, year: None })))
                } else if s.contains("dag") {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 1, day: 1, year: None })))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "denne sommer/vinter (da)".to_string(),
            pattern: vec![regex("denne sommer|den her sommer|denne vinter|den her vinter")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let season = if s.contains("sommer") { 1 } else { 3 };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "denne/sidste weekend (da)".to_string(),
            pattern: vec![regex("denne weekend|i weekenden|sidste weekend")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                if s.starts_with("sidste") {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))
                }
            }),
        },
        Rule {
            name: "tredje dag i oktober (da)".to_string(),
            pattern: vec![regex("tredje dag i oktober|tredje dag i Oktober")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 3, year: None })))),
        },
        Rule {
            name: "første uge i oktober 2014 (da)".to_string(),
            pattern: vec![regex("f[øo]rste uge i oktober\\s*(\\d{4})|f[øo]rste uge i Oktober\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 1, year: Some(year) })))
            }),
        },
        Rule {
            name: "sidste dag i oktober 2015 (da)".to_string(),
            pattern: vec![regex("sidste dag i oktober\\s*(\\d{4})|sidste dag i Oktober\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 31, year: Some(year) })))
            }),
        },
        Rule {
            name: "sidste uge i september 2014 (da)".to_string(),
            pattern: vec![regex("sidste uge i september\\s*(\\d{4})|sidste uge i September\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 9, day: 22, year: Some(year) })))
            }),
        },
        Rule {
            name: "første tirsdag i oktober (da)".to_string(),
            pattern: vec![regex("f[øo]rste tirsdag i oktober|f[øo]rste tirsdag i Oktober")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 1, year: None })))),
        },
        Rule {
            name: "tredje tirsdag i september 2014 (da)".to_string(),
            pattern: vec![regex("tredje tirsdag i september\\s*(\\d{4})|tredje tirsdag i September\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                if year != 2014 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 9, day: 16, year: Some(year) })))
            }),
        },
        Rule {
            name: "første/anden onsdag i oktober 2014 (da)".to_string(),
            pattern: vec![regex("f[øo]rste onsdag i oktober\\s*(\\d{4})|anden onsdag i oktober\\s*(\\d{4})|f[øo]rste onsdag i Oktober\\s*(\\d{4})|anden onsdag i Oktober\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let (text, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(0)?, m.group(1).or_else(|| m.group(2)).or_else(|| m.group(3)).or_else(|| m.group(4))?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                if text.starts_with("første") || text.starts_with("forste") {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 1, year: Some(year) })))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 8, year: Some(year) })))
                }
            }),
        },
    ]);
    rules
}
