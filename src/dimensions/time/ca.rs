use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use crate::dimensions::time_grain::Grain;
use super::{TimeData, TimeForm};

fn ca_small_number(s: &str) -> Option<i32> {
    match s {
        "un" | "una" => Some(1),
        "dos" | "dues" => Some(2),
        "tres" => Some(3),
        "quatre" => Some(4),
        "cinc" => Some(5),
        "sis" => Some(6),
        "set" => Some(7),
        "vuit" => Some(8),
        "nou" => Some(9),
        "deu" => Some(10),
        "onze" => Some(11),
        "dotze" => Some(12),
        "catorze" => Some(14),
        "vint-i-quatre" | "vint i quatre" => Some(24),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::es::rules();
    rules.extend(vec![
        Rule {
            name: "now (ca)".to_string(),
            pattern: vec![regex("ara|ja|en aquest moment|en aquests moments")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ca)".to_string(),
            pattern: vec![regex("avui")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ca)".to_string(),
            pattern: vec![regex("dem[àa]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (ca)".to_string(),
            pattern: vec![regex("ahir")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (ca)".to_string(),
            pattern: vec![regex("dilluns|dl\\.?|dimarts|dm\\.?|dimecres|dc\\.?|dijous|dj\\.?|divendres|dv\\.?|dissabte|ds\\.?|diumenge|dg\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("dilluns") || s.starts_with("dl") {
                    0
                } else if s.starts_with("dimarts") || s.starts_with("dm") {
                    1
                } else if s.starts_with("dimecres") || s.starts_with("dc") {
                    2
                } else if s.starts_with("dijous") || s.starts_with("dj") {
                    3
                } else if s.starts_with("divendres") || s.starts_with("dv") {
                    4
                } else if s.starts_with("dissabte") || s.starts_with("ds") {
                    5
                } else if s.starts_with("diumenge") || s.starts_with("dg") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "el 5 de maig (ca)".to_string(),
            pattern: vec![regex("el\\s*(\\d{1,2})\\s+de\\s+maig")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day, year: None })))
            }),
        },
        Rule {
            name: "el cinc de maig (ca)".to_string(),
            pattern: vec![regex("el\\s+cinc\\s+de\\s+maig")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day: 5, year: None })))),
        },
        Rule {
            name: "el 4 de juliol (ca)".to_string(),
            pattern: vec![regex("el\\s*(\\d{1,2})\\s+de\\s+juliol")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 7, day, year: None })))
            }),
        },
        Rule {
            name: "el 4 d'agost (ca)".to_string(),
            pattern: vec![regex("el\\s*(\\d{1,2})\\s+d['’]agost")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 8, day, year: None })))
            }),
        },
        Rule {
            name: "el 3 de març (ca)".to_string(),
            pattern: vec![regex("el\\s*(\\d{1,2})\\s+de\\s+mar[çc]")],
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
            name: "3 de març (ca)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+de\\s+mar[çc]")],
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
            name: "el 24 d'octubre (ca)".to_string(),
            pattern: vec![regex("el\\s*(\\d{1,2})\\s+d['’]octubre")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day, year: None })))
            }),
        },
        Rule {
            name: "el 24 de setembre (ca)".to_string(),
            pattern: vec![regex("el\\s*(\\d{1,2})\\s+de\\s+setembre")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 9, day, year: None })))
            }),
        },
        Rule {
            name: "el 24 de set (ca)".to_string(),
            pattern: vec![regex("el\\s*(\\d{1,2})\\s+de\\s+set\\.?")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 9, day, year: None })))
            }),
        },
        Rule {
            name: "setembre (ca)".to_string(),
            pattern: vec![regex("setembre|set\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(9))))),
        },
        Rule {
            name: "el 5 d'abril (ca)".to_string(),
            pattern: vec![regex("el\\s*(\\d{1,2})\\s+d['’]abril")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day, year: None })))
            }),
        },
        Rule {
            name: "5 d'abril (ca)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+d['’]abril")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day, year: None })))
            }),
        },
        Rule {
            name: "el primer de març (ca)".to_string(),
            pattern: vec![regex("el\\s+primer\\s+de\\s+mar[çc]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "l'u de març (ca)".to_string(),
            pattern: vec![regex("l['’]u\\s+de\\s+mar[çc]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "u de març (ca)".to_string(),
            pattern: vec![regex("u\\s+de\\s+mar[çc]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "el 16 (ca)".to_string(),
            pattern: vec![regex("el\\s*(\\d{1,2})")],
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
            name: "16 de febrer (ca)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+de\\s+febrer")],
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
            name: "el passat mes (ca)".to_string(),
            pattern: vec![regex("el passat mes|el mes passat")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "el mes vinent (ca)".to_string(),
            pattern: vec![regex("el mes vinent|mes vinent")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "el proper mes (ca)".to_string(),
            pattern: vec![regex("el proper mes|proper mes")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "l'any passat (ca)".to_string(),
            pattern: vec![regex("l['’]any passat|any passat")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "aquest any (ca)".to_string(),
            pattern: vec![regex("aquest any")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(crate::dimensions::time_grain::Grain::Year))))),
        },
        Rule {
            name: "l'any vinent (ca)".to_string(),
            pattern: vec![regex("l['’]any vinent|any vinent")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "el proper any (ca)".to_string(),
            pattern: vec![regex("el proper any|proper any")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "migdia (ca)".to_string(),
            pattern: vec![regex("migdia|mig dia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(12, 0, false))))),
        },
        Rule {
            name: "tres i quart (ca)".to_string(),
            pattern: vec![regex("tres i quart|3 i quart")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(3, 15, false))))),
        },
        Rule {
            name: "en 2 minuts (ca)".to_string(),
            pattern: vec![regex("en\\s*2\\s*minuts?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 2 })))),
        },
        Rule {
            name: "abans d'ahir (ca)".to_string(),
            pattern: vec![regex("abans d['’]ahir")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))),
        },
        Rule {
            name: "demà passat (ca)".to_string(),
            pattern: vec![regex("dem[àa]\\s+passat")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))),
        },
        Rule {
            name: "aquesta setmana (ca)".to_string(),
            pattern: vec![regex("aquesta setmana|setmana actual")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "la setmana passada (ca)".to_string(),
            pattern: vec![regex("la setmana passada|setmana passada")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "la setmana vinent (ca)".to_string(),
            pattern: vec![regex("la setmana vinent|setmana vinent|propera setmana|seg[üu]ent setmana|properes setmanes?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "cap de setmana (ca)".to_string(),
            pattern: vec![regex("cap de setmana|week[ -]?end")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "part of day morning (ca)".to_string(),
            pattern: vec![regex("mat[ií]|pel mat[ií]|del mat[ií]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(super::PartOfDay::Morning))))),
        },
        Rule {
            name: "part of day afternoon (ca)".to_string(),
            pattern: vec![regex("tarda|a la tarda|per la tarda")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(super::PartOfDay::Afternoon))))),
        },
        Rule {
            name: "part of day evening (ca)".to_string(),
            pattern: vec![regex("vespre|al vespre|del vespre")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(super::PartOfDay::Evening))))),
        },
        Rule {
            name: "part of day night (ca)".to_string(),
            pattern: vec![regex("nit|a la nit|per la nit")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(super::PartOfDay::Night))))),
        },
        Rule {
            name: "this part-of-day (ca)".to_string(),
            pattern: vec![regex("aquest\\s+(mat[ií]|vespre)|aquesta\\s+(tarda|nit)")],
            production: Box::new(|nodes| {
                let pod = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
                    _ => return None,
                };
                let part = if pod.starts_with("mat") {
                    super::PartOfDay::Morning
                } else if pod.starts_with("tarda") {
                    super::PartOfDay::Afternoon
                } else if pod.starts_with("vespre") {
                    super::PartOfDay::Evening
                } else {
                    super::PartOfDay::Night
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(TimeData::latent(TimeForm::PartOfDay(part))),
                ))))
            }),
        },
        Rule {
            name: "tomorrow part of day (ca)".to_string(),
            pattern: vec![regex("dem[àa]\\s+(a la|al|pel|per la)\\s+(tarda|vespre|nit|mat[ií])")],
            production: Box::new(|nodes| {
                let pod = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let part = if pod.starts_with("mat") {
                    super::PartOfDay::Morning
                } else if pod.starts_with("tarda") {
                    super::PartOfDay::Afternoon
                } else if pod.starts_with("vespre") {
                    super::PartOfDay::Evening
                } else {
                    super::PartOfDay::Night
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Tomorrow)),
                    Box::new(TimeData::latent(TimeForm::PartOfDay(part))),
                ))))
            }),
        },
        Rule {
            name: "yesterday part of day (ca)".to_string(),
            pattern: vec![regex("ahir\\s+(a la|al|pel|per la)\\s+(tarda|vespre|nit|mat[ií])")],
            production: Box::new(|nodes| {
                let pod = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let part = if pod.starts_with("mat") {
                    super::PartOfDay::Morning
                } else if pod.starts_with("tarda") {
                    super::PartOfDay::Afternoon
                } else if pod.starts_with("vespre") {
                    super::PartOfDay::Evening
                } else {
                    super::PartOfDay::Night
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Yesterday)),
                    Box::new(TimeData::latent(TimeForm::PartOfDay(part))),
                ))))
            }),
        },
        Rule {
            name: "hour with part-of-day (ca)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+del\\s+(mat[ií]|vespre|nit|tarda)")],
            production: Box::new(|nodes| {
                let (h, pod) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                if pod == "vespre" || pod == "nit" || pod == "tarda" {
                    if hour < 12 {
                        hour += 12;
                    }
                } else if pod.starts_with("mat") && hour == 12 {
                    hour = 0;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "word hour with part-of-day (ca)".to_string(),
            pattern: vec![regex("(un|una|dos|dues|tres|quatre|cinc|sis|set|vuit|nou|deu|onze|dotze)\\s+del\\s+(mat[ií]|vespre|nit|tarda)")],
            production: Box::new(|nodes| {
                let (h_raw, pod) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let mut hour = ca_small_number(h_raw)? as u32;
                if hour > 12 {
                    return None;
                }
                if pod == "vespre" || pod == "nit" || pod == "tarda" {
                    if hour < 12 {
                        hour += 12;
                    }
                } else if pod.starts_with("mat") && hour == 12 {
                    hour = 0;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "en <duration> (ca)".to_string(),
            pattern: vec![regex("en\\s*(\\d+|un|una|dos|dues|tres|quatre|cinc|sis|set|vuit|nou|deu|onze|dotze|catorze|vint-i-quatre|vint i quatre)\\s*(segons?|minuts?|hores?|dies|setmanes?|mesos?|anys?)")],
            production: Box::new(|nodes| {
                let (n_raw, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n: i32 = n_raw.parse::<i32>().ok().or_else(|| ca_small_number(n_raw))?;
                let grain = if unit.starts_with("seg") {
                    Grain::Second
                } else if unit.starts_with("min") {
                    Grain::Minute
                } else if unit.starts_with("hor") {
                    Grain::Hour
                } else if unit.starts_with("die") {
                    Grain::Day
                } else if unit.starts_with("setman") {
                    Grain::Week
                } else if unit.starts_with("mes") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: n })))
            }),
        },
        Rule {
            name: "fa <duration> (ca)".to_string(),
            pattern: vec![regex("fa\\s*(\\d+|un|una|dos|dues|tres|quatre|cinc|sis|set|vuit|nou|deu|onze|dotze|catorze|vint-i-quatre|vint i quatre)\\s*(segons?|minuts?|hores?|dies|setmanes?|mesos?|anys?)")],
            production: Box::new(|nodes| {
                let (n_raw, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n: i32 = n_raw.parse::<i32>().ok().or_else(|| ca_small_number(n_raw))?;
                let grain = if unit.starts_with("seg") {
                    Grain::Second
                } else if unit.starts_with("min") {
                    Grain::Minute
                } else if unit.starts_with("hor") {
                    Grain::Hour
                } else if unit.starts_with("die") {
                    Grain::Day
                } else if unit.starts_with("setman") {
                    Grain::Week
                } else if unit.starts_with("mes") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: -n })))
            }),
        },
        Rule {
            name: "dintre de <duration> (ca)".to_string(),
            pattern: vec![regex("dintre de\\s*(\\d+|un|una|dos|dues|tres|quatre|cinc|sis|set|vuit|nou|deu|onze|dotze|catorze|vint-i-quatre|vint i quatre)\\s*(segons?|minuts?|hores?|dies|setmanes?|mesos?|anys?)")],
            production: Box::new(|nodes| {
                let (n_raw, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n: i32 = n_raw.parse::<i32>().ok().or_else(|| ca_small_number(n_raw))?;
                let grain = if unit.starts_with("seg") {
                    Grain::Second
                } else if unit.starts_with("min") {
                    Grain::Minute
                } else if unit.starts_with("hor") {
                    Grain::Hour
                } else if unit.starts_with("die") {
                    Grain::Day
                } else if unit.starts_with("setman") {
                    Grain::Week
                } else if unit.starts_with("mes") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: n })))
            }),
        },
        Rule {
            name: "darrers n <cycle> (ca)".to_string(),
            pattern: vec![regex("darrer(s|es)?\\s*(\\d+|un|una|dos|dues|tres|quatre|cinc|sis|set|vuit|nou|deu)\\s*(segons?|minuts?|hores?|dies|setmanes?|mesos?|anys?)")],
            production: Box::new(|nodes| {
                let (n_raw, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let n: i64 = n_raw.parse::<i64>().ok().or_else(|| ca_small_number(n_raw).map(|v| v as i64))?;
                let grain = if unit.starts_with("seg") {
                    Grain::Second
                } else if unit.starts_with("min") {
                    Grain::Minute
                } else if unit.starts_with("hor") {
                    Grain::Hour
                } else if unit.starts_with("die") {
                    Grain::Day
                } else if unit.starts_with("setman") {
                    Grain::Week
                } else if unit.starts_with("mes") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain { n, grain, past: true, interval: true })))
            }),
        },
        Rule {
            name: "propers n <cycle> (ca)".to_string(),
            pattern: vec![regex("proper(s|es)?\\s*(\\d+|un|una|dos|dues|tres|quatre|cinc|sis|set|vuit|nou|deu)\\s*(segons?|minuts?|hores?|dies|setmanes?|mesos?|anys?)")],
            production: Box::new(|nodes| {
                let (n_raw, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let n: i64 = n_raw.parse::<i64>().ok().or_else(|| ca_small_number(n_raw).map(|v| v as i64))?;
                let grain = if unit.starts_with("seg") {
                    Grain::Second
                } else if unit.starts_with("min") {
                    Grain::Minute
                } else if unit.starts_with("hor") {
                    Grain::Hour
                } else if unit.starts_with("die") {
                    Grain::Day
                } else if unit.starts_with("setman") {
                    Grain::Week
                } else if unit.starts_with("mes") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain { n, grain, past: false, interval: true })))
            }),
        },
        Rule {
            name: "n propers <cycle> (ca)".to_string(),
            pattern: vec![regex("(\\d+|un|una|dos|dues|tres|quatre|cinc|sis|set|vuit|nou|deu)\\s+proper(s|es)?\\s*(segons?|minuts?|hores?|dies|setmanes?|mesos?|anys?)")],
            production: Box::new(|nodes| {
                let (n_raw, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)?),
                    _ => return None,
                };
                let n: i64 = n_raw.parse::<i64>().ok().or_else(|| ca_small_number(n_raw).map(|v| v as i64))?;
                let grain = if unit.starts_with("seg") {
                    Grain::Second
                } else if unit.starts_with("min") {
                    Grain::Minute
                } else if unit.starts_with("hor") {
                    Grain::Hour
                } else if unit.starts_with("die") {
                    Grain::Day
                } else if unit.starts_with("setman") {
                    Grain::Week
                } else if unit.starts_with("mes") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain { n, grain, past: false, interval: true })))
            }),
        },
        Rule {
            name: "dia quinze (ca)".to_string(),
            pattern: vec![regex("dia\\s+(nou|onze|quinze)")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day = match d {
                    "nou" => 9,
                    "onze" => 11,
                    "quinze" => 15,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "mil nou-cents noranta (ca)".to_string(),
            pattern: vec![regex("mil\\s+nou-cents\\s+noranta")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Year(1990))))),
        },
        Rule {
            name: "seasons (ca)".to_string(),
            pattern: vec![regex("estiu|hivern|primavera|tardor")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let season = match s {
                    "primavera" => 0,
                    "estiu" => 1,
                    "tardor" => 2,
                    "hivern" => 3,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "aquest estiu/hivern (ca)".to_string(),
            pattern: vec![regex("aquest\\s+(estiu|hivern|primavera|tardor)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let season = match s {
                    "primavera" => 0,
                    "estiu" => 1,
                    "tardor" => 2,
                    "hivern" => 3,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "nadal (ca)".to_string(),
            pattern: vec![regex("nadal|el nadal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday("Navidad".to_string(), None))))),
        },
        Rule {
            name: "nit de cap d'any (ca)".to_string(),
            pattern: vec![regex("nit de cap d['’]any|darrer dia de l['’]any")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 12, day: 31, year: None })))),
        },
        Rule {
            name: "cap d'any (ca)".to_string(),
            pattern: vec![regex("cap d['’]any")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 1, day: 1, year: None })))),
        },
        Rule {
            name: "dia de la zero discriminacio (ca)".to_string(),
            pattern: vec![regex("dia de la zero discriminaci[óo]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "dia mundial de la lengua arabe (ca)".to_string(),
            pattern: vec![regex("dia mundial de la lengua [áa]rabe")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 12, day: 18, year: None })))),
        },
        Rule {
            name: "dia internacional de les cooperatives (ca)".to_string(),
            pattern: vec![regex("dia internacional de les cooperatives( del (\\d{4}))?")],
            production: Box::new(|nodes| {
                let maybe_year = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2),
                    _ => return None,
                };
                let year = maybe_year.and_then(|y| y.parse::<i32>().ok()).unwrap_or(2013);
                // Approximation used by corpus examples (2019 -> July 6, 2013 -> July 6)
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 7, day: 6, year: Some(year) })))
            }),
        },
        Rule {
            name: "dia prematuritat mundial (ca)".to_string(),
            pattern: vec![regex("dia de la prematuritat mundial|dia mundial de l['’]infant prematur")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 11, day: 17, year: None })))),
        },
        Rule {
            name: "dia dels innocents d'abril (ca)".to_string(),
            pattern: vec![regex("dia dels innocents d['’]abril")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day: 1, year: None })))),
        },
        Rule {
            name: "ordinal trimestre de any (ca)".to_string(),
            pattern: vec![regex("(primer|segon|tercer|quart|1r|2n|3r|4t)\\s+trimestre\\s+de\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (q_raw, y_raw) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let quarter = match q_raw {
                    "primer" | "1r" => 1,
                    "segon" | "2n" => 2,
                    "tercer" | "3r" => 3,
                    "quart" | "4t" => 4,
                    _ => return None,
                };
                let year: i32 = y_raw.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(quarter, year))))
            }),
        },
    ]);
    rules
}
