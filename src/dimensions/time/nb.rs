use super::Direction;
use super::{PartOfDay, TimeData, TimeForm};
use crate::dimensions::time_grain::Grain;
use crate::pattern::regex;
use crate::types::{Rule, TokenData};

fn parse_nb_month(s: &str) -> Option<u32> {
    let t = s.to_lowercase();
    if t.starts_with("jan") {
        Some(1)
    } else if t.starts_with("feb") {
        Some(2)
    } else if t.starts_with("mar") {
        Some(3)
    } else if t.starts_with("apr") {
        Some(4)
    } else if t.starts_with("mai") {
        Some(5)
    } else if t.starts_with("jun") {
        Some(6)
    } else if t.starts_with("jul") {
        Some(7)
    } else if t.starts_with("aug") {
        Some(8)
    } else if t.starts_with("sep") {
        Some(9)
    } else if t.starts_with("okt") {
        Some(10)
    } else if t.starts_with("nov") {
        Some(11)
    } else if t.starts_with("des") {
        Some(12)
    } else {
        None
    }
}

fn parse_nb_ordinal_day(s: &str) -> Option<u32> {
    match s {
        "første" | "forste" => Some(1),
        "andre" => Some(2),
        "tredje" => Some(3),
        "fjerde" => Some(4),
        _ => s.parse().ok(),
    }
}

fn parse_nb_qty(s: &str) -> Option<i64> {
    match s {
        "en" | "én" | "ett" | "ei" | "et" => Some(1),
        "to" => Some(2),
        "tre" => Some(3),
        "fire" => Some(4),
        "fem" => Some(5),
        "seks" => Some(6),
        "syv" => Some(7),
        _ => s.parse().ok(),
    }
}

fn parse_nb_grain(s: &str) -> Option<Grain> {
    let t = s.to_lowercase();
    if t.contains("sek") {
        Some(Grain::Second)
    } else if t.contains("min") {
        Some(Grain::Minute)
    } else if t.contains("time") || t.contains("tim") {
        Some(Grain::Hour)
    } else if t.contains("dag") {
        Some(Grain::Day)
    } else if t.contains("uke") {
        Some(Grain::Week)
    } else if t.contains("måned") || t.contains("maned") {
        Some(Grain::Month)
    } else if t.contains("år") || t.contains("ar") {
        Some(Grain::Year)
    } else {
        None
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (nb)".to_string(),
            pattern: vec![regex("n[åa]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (nb)".to_string(),
            pattern: vec![regex("i dag|idag")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (nb)".to_string(),
            pattern: vec![regex("i morgen|imorgen|i morra")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "part-of-day keywords (nb)".to_string(),
            pattern: vec![regex("i kveld|ikveld|kveld(en)?|i natt|inatt|natt(a|en)?|i morges|morges|morgen(en)?|morran|denne\\s+morran|denne\\s+morgen(en)?|ettermiddag(en)?|middag(en)?|om ettermiddagen")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let pod = if s.contains("kveld") {
                    PartOfDay::Evening
                } else if s.contains("ettermiddag") || s == "middag" || s == "middagen" {
                    PartOfDay::Afternoon
                } else if s.contains("natt") {
                    PartOfDay::Night
                } else {
                    PartOfDay::Morning
                };
                Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(pod))))
            }),
        },
        Rule {
            name: "etter frokost (nb)".to_string(),
            pattern: vec![regex("etter\\s+frokost")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))))),
        },
        Rule {
            name: "yesterday (nb)".to_string(),
            pattern: vec![regex("i g[åa]r|ig[åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day before yesterday / day after tomorrow (nb)".to_string(),
            pattern: vec![regex("i forig[åa]rs|forig[åa]rs|i overimorra|overimorra|i overmorgen|overmorgen")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("forig") {
                    Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))
                }
            }),
        },
        Rule {
            name: "day of week (nb)".to_string(),
            pattern: vec![regex("mandag|man\\.?|tirsdag|onsdag|torsdag|tors\\.?|fredag|fre\\.?|l[øo]rdag|l[øo]r\\.?|s[øo]ndag|s[øo]n\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("mandag") || s == "man." || s == "man" {
                    0
                } else if s.contains("tirsdag") {
                    1
                } else if s.contains("onsdag") {
                    2
                } else if s.contains("torsdag") || s == "tors." || s == "tors" {
                    3
                } else if s.contains("fredag") || s == "fre" || s == "fre." {
                    4
                } else if (s.contains("rdag") && (s.contains("lø") || s.contains("lo")))
                    || s == "lør"
                    || s == "lør."
                    || s == "lor"
                    || s == "lor."
                {
                    5
                } else if s.contains("ndag") || s == "søn" || s == "søn." || s == "son" || s == "son." {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "den første mars (nb)".to_string(),
            pattern: vec![regex("den\\s+f[øo]rste\\s+mars|1\\.\\s+mars")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "3 mars (nb)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+mars")],
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
            name: "den tredje mars (nb)".to_string(),
            pattern: vec![regex("den\\s+tredje\\s+mars")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 3, year: None })))),
        },
        Rule {
            name: "<ordinal> dag i <month> (nb)".to_string(),
            pattern: vec![regex("(f[øo]rste|andre|tredje|fjerde|\\d{1,2})\\s+dag\\s+i\\s+(januar|februar|mars|april|mai|juni|juli|august|september|oktober|november|desember)")],
            production: Box::new(|nodes| {
                let (d_s, m_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let day = parse_nb_ordinal_day(&d_s)?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                let month = parse_nb_month(&m_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "<day>-<day> <month> (nb)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*(?:-|til)\\s*(\\d{1,2})\\s+(januar|februar|mars|april|mai|juni|juli|august|september|oktober|november|desember)")],
            production: Box::new(|nodes| {
                let (d1, d2, m_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let month = parse_nb_month(&m_s)?;
                let from = TimeData::new(TimeForm::DateMDY { month, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "<day> <month> til <day> <month> (nb)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+(januar|februar|mars|april|mai|juni|juli|august|september|oktober|november|desember)\\s+til\\s+(\\d{1,2})\\s+(januar|februar|mars|april|mai|juni|juli|august|september|oktober|november|desember)")],
            production: Box::new(|nodes| {
                let (d1, m1s, d2, m2s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?.to_lowercase(), rm.group(3)?, rm.group(4)?.to_lowercase()),
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let month1 = parse_nb_month(&m1s)?;
                let month2 = parse_nb_month(&m2s)?;
                let from = TimeData::new(TimeForm::DateMDY { month: month1, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month: month2, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "den 3. mars (nb)".to_string(),
            pattern: vec![regex("den\\s+(\\d{1,2})\\.\\s+mars")],
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
            name: "30.10 (nb)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (d, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "tredje mars 2015 (nb)".to_string(),
            pattern: vec![regex("tredje\\s+mars\\s+(\\d{4})")],
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
            name: "3. mars 2015 (nb)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.\\s+mars\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (d, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let year: i32 = y.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "På den 15. (nb)".to_string(),
            pattern: vec![regex("[Pp]å\\s+den\\s+(\\d{1,2})\\.")],
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
            name: "På den 15 (nb)".to_string(),
            pattern: vec![regex("[Pp]å\\s+den\\s+(\\d{1,2})")],
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
            name: "Den 15. (nb)".to_string(),
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
            name: "Den femtende (nb)".to_string(),
            pattern: vec![regex("[Dd]en\\s+femtende")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(15))))),
        },
        Rule {
            name: "15. februar (nb)".to_string(),
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
            name: "februar 15 (nb)".to_string(),
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
            name: "Oktober 2014 (nb)".to_string(),
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
            name: "neste mars (nb)".to_string(),
            pattern: vec![regex("neste\\s+mars")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::Month(3));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "Ons, Feb13 (nb)".to_string(),
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
            name: "denne uken (nb)".to_string(),
            pattern: vec![regex("denne uken")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(crate::dimensions::time_grain::Grain::Week))))),
        },
        Rule {
            name: "forrige uke (nb)".to_string(),
            pattern: vec![regex("forrige uke")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "sist uke (nb)".to_string(),
            pattern: vec![regex("sist uke")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "neste uke (nb)".to_string(),
            pattern: vec![regex("neste uke")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "helg (nb)".to_string(),
            pattern: vec![regex("helg(en)?|weekend")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "forrige/neste/denne helg (nb)".to_string(),
            pattern: vec![regex("forrige\\s+helg|sist\\s+helg|neste\\s+helg|denne\\s+helg(en|a)?|i\\s+helg(a|en)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::Weekend);
                if s.starts_with("forrige") || s.starts_with("sist") {
                    t.direction = Some(Direction::Past);
                } else if s.starts_with("neste") {
                    t.direction = Some(Direction::Future);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "i romjulen (nb)".to_string(),
            pattern: vec![regex("i\\s+romjul(en|a)")],
            production: Box::new(|_| {
                let from = TimeData::new(TimeForm::DateMDY { month: 12, day: 24, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month: 12, day: 31, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "forrige måned (nb)".to_string(),
            pattern: vec![regex("forrige m[åa]ned")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "sist måned (nb)".to_string(),
            pattern: vec![regex("sist m[åa]ned")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "neste måned (nb)".to_string(),
            pattern: vec![regex("neste m[åa]ned")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "forrige/sist år (nb)".to_string(),
            pattern: vec![regex("forrige [åa]r|sist [åa]r|i fjor")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "i/dette år (nb)".to_string(),
            pattern: vec![regex("i [åa]r|dette [åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))),
        },
        Rule {
            name: "neste år (nb)".to_string(),
            pattern: vec![regex("neste [åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "klokken/kl <hour> (nb)".to_string(),
            pattern: vec![regex("klokken\\s*(\\d{1,2})|klokka\\s*(\\d{1,2})|kl\\.?\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2)).or_else(|| m.group(3))?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, true))))
            }),
        },
        Rule {
            name: "klokken/kl <hh:mm> (nb)".to_string(),
            pattern: vec![regex("(?:klokken|kl\\.?)\\s*(\\d{1,2})[:\\.](\\d{2})")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                if hour > 23 || minute > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "kvarter over <hour> (nb)".to_string(),
            pattern: vec![regex("(kvarter|kvart)\\s+over\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(2)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 15, false))))
            }),
        },
        Rule {
            name: "kvarter på <hour> (nb)".to_string(),
            pattern: vec![regex("(kvarter|kvart)\\s+p[åa]\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(2)?,
                    _ => return None,
                };
                let hour_in: u32 = h.parse().ok()?;
                if hour_in == 0 || hour_in > 24 {
                    return None;
                }
                let hour = if hour_in == 1 { 0 } else { hour_in.checked_sub(1)? };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 45, false))))
            }),
        },
        Rule {
            name: "om <duration> (nb)".to_string(),
            pattern: vec![regex("om\\s+(\\d+|en|én|ett|ei|et|to|tre|fire|fem|seks|syv)\\s+(sekund(?:er)?|minutt(?:er)?|time(?:r)?|dag(?:er)?|uk(?:e|er)|m[åa]ned(?:er)?|[åa]r)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_nb_qty(&n_s)?;
                let grain = parse_nb_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain })))
            }),
        },
        Rule {
            name: "innenfor <duration> (nb)".to_string(),
            pattern: vec![regex("innenfor\\s+(\\d+|en|én|ett|ei|et|to|tre|fire|fem|seks|syv)\\s+(sekund(?:er)?|minutt(?:er)?|time(?:r)?|dag(?:er)?|uk(?:e|er)|m[åa]ned(?:er)?|[åa]r)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_nb_qty(&n_s)?;
                let grain = parse_nb_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain })))
            }),
        },
        Rule {
            name: "etter <duration> (nb)".to_string(),
            pattern: vec![regex("etter\\s+(\\d+|en|én|ett|ei|et|to|tre|fire|fem|seks|syv)\\s+(sekund(?:er)?|minutt(?:er)?|time(?:r)?|dag(?:er)?|uk(?:e|er)|m[åa]ned(?:er)?|[åa]r)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_nb_qty(&n_s)?;
                let grain = parse_nb_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain })))
            }),
        },
        Rule {
            name: "om en halv time (nb)".to_string(),
            pattern: vec![regex("om\\s+((ca\\.?|cirka)\\s+)?en\\s+halv\\s+time")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: 30, grain: Grain::Minute })))),
        },
        Rule {
            name: "om <n>t compact (nb)".to_string(),
            pattern: vec![regex("om\\s*(\\d+)\\s*t")],
            production: Box::new(|nodes| {
                let n_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let n: i64 = n_s.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain: Grain::Hour })))
            }),
        },
        Rule {
            name: "om et par/noen timer (nb)".to_string(),
            pattern: vec![regex("om\\s+(et\\s+par|noen)\\s+timer")],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let n = if q == "et par" { 2 } else { 3 };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain: Grain::Hour })))
            }),
        },
        Rule {
            name: "om <decimal> time (nb)".to_string(),
            pattern: vec![regex("om\\s+(\\d+,\\d+)\\s+time")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let f: f64 = s.replace(',', ".").parse().ok()?;
                let mins = (f * 60.0).round() as i64;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: mins, grain: Grain::Minute })))
            }),
        },
        Rule {
            name: "om <n> og en halv time (nb)".to_string(),
            pattern: vec![regex("om\\s+(\\d+|to|tre|fire|fem)\\s+og\\s+en\\s+halv\\s+time")],
            production: Box::new(|nodes| {
                let n_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let n = parse_nb_qty(&n_s)?;
                let mins = n.checked_mul(60)?.checked_add(30)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: mins, grain: Grain::Minute })))
            }),
        },
        Rule {
            name: "<duration> siden (nb)".to_string(),
            pattern: vec![regex("(\\d+|en|én|ett|ei|et|to|tre|fire|fem|seks|syv)\\s+(sekund(?:er)?|minutt(?:er)?|time(?:r)?|dag(?:er)?|uk(?:e|er)|m[åa]ned(?:er)?|[åa]r)\\s+siden")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_nb_qty(&n_s)?;
                let grain = parse_nb_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: n.checked_neg()?, grain })))
            }),
        },
        Rule {
            name: "siste/neste n cycles (nb)".to_string(),
            pattern: vec![regex("(siste|seneste|neste)\\s+(\\d+|en|én|ett|ei|et|to|tre|fire|fem|seks|syv)\\s+(sekund(?:er)?|minutt(?:er)?|time(?:r)?|dag(?:er)?|uk(?:e|er)|m[åa]ned(?:er)?|[åa]r)")],
            production: Box::new(|nodes| {
                let (dir, n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase(), rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_nb_qty(&n_s)?;
                let grain = parse_nb_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: dir == "siste" || dir == "seneste",
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "dette kvartalet (nb)".to_string(),
            pattern: vec![regex("dette kvartalet")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Quarter, offset: 0 })))),
        },
        Rule {
            name: "neste kvartal (nb)".to_string(),
            pattern: vec![regex("neste kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: crate::dimensions::time_grain::Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "tredje kvartal (nb)".to_string(),
            pattern: vec![regex("tredje kvartal|3\\. kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "4. kvartal 2018 (nb)".to_string(),
            pattern: vec![regex("4\\.\\s*kvartal\\s*(\\d{4})")],
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
            name: "fjerde kvartal 2018 (nb)".to_string(),
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
            name: "year (nb)".to_string(),
            pattern: vec![regex("\\b(19\\d\\d|20\\d\\d)\\b")],
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
            name: "holidays (nb)".to_string(),
            pattern: vec![regex("julaften|1\\.?\\s*juledag|f[øo]rste\\s+juledag|nytt[åa]rsaften|nytt[åa]rsdag|morsdag(en)?|farsdag(en)?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let form = if s.contains("julaften") {
                    TimeForm::Holiday("christmas eve".to_string(), None)
                } else if s.contains("juledag") {
                    TimeForm::Holiday("christmas day".to_string(), None)
                } else if s.contains("morsdag") {
                    TimeForm::Holiday("mother's day".to_string(), None)
                } else if s.contains("farsdag") {
                    TimeForm::Holiday("father's day".to_string(), None)
                } else if s.contains("aften") {
                    TimeForm::Holiday("new year's eve".to_string(), None)
                } else {
                    TimeForm::Holiday("new year's day".to_string(), None)
                };
                Some(TokenData::Time(TimeData::new(form)))
            }),
        },
        Rule {
            name: "season (nb)".to_string(),
            pattern: vec![regex("v[åa]r(en)?|sommer(en)?|h[øo]st(en)?|vinter(en)?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let season = if s.starts_with("vår") || s.starts_with("var") {
                    0
                } else if s.starts_with("sommer") {
                    1
                } else if s.starts_with("høst") || s.starts_with("host") {
                    2
                } else {
                    3
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "this/next/last season (nb)".to_string(),
            pattern: vec![regex("denne\\s+(sommeren|vinteren|h[øo]sten|v[åa]ren)|neste\\s+(sommer|vinter|h[øo]st|v[åa]r)|forrige\\s+(sommer|vinter|h[øo]st|v[åa]r)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let season = if s.contains("vår") || s.contains("var") {
                    0
                } else if s.contains("sommer") {
                    1
                } else if s.contains("høst") || s.contains("host") {
                    2
                } else {
                    3
                };
                let mut t = TimeData::new(TimeForm::Season(season));
                if s.starts_with("neste") {
                    t.direction = Some(Direction::Future);
                } else if s.starts_with("forrige") {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<n> år etter julaften (nb)".to_string(),
            pattern: vec![regex("(et|ett|en|\\d+|to|tre)\\s+[åa]r\\s+etter\\s+julaften")],
            production: Box::new(|nodes| {
                let n_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let n = parse_nb_qty(&n_s)?;
                let base = TimeData::new(TimeForm::Holiday("christmas eve".to_string(), None));
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n,
                    grain: Grain::Year,
                    base: Box::new(base),
                })))
            }),
        },
    ]);
    rules
}
