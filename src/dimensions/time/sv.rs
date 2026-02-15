use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, regex};
use super::{Direction, IntervalDirection};
use crate::types::{DimensionKind, Rule, TokenData};
use super::{PartOfDay, TimeData, TimeForm};

fn parse_sv_month(s: &str) -> Option<u32> {
    let t = s.to_lowercase();
    if t.starts_with("jan") {
        Some(1)
    } else if t.starts_with("feb") {
        Some(2)
    } else if t.starts_with("mar") {
        Some(3)
    } else if t.starts_with("apr") {
        Some(4)
    } else if t.starts_with("maj") {
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
    } else if t.starts_with("dec") {
        Some(12)
    } else {
        None
    }
}

fn parse_sv_ordinal_day(s: &str) -> Option<u32> {
    match s {
        "första" | "forsta" => Some(1),
        "andra" => Some(2),
        "tredje" => Some(3),
        "fjärde" | "fjarde" => Some(4),
        _ => s.parse().ok(),
    }
}

fn parse_sv_small_minutes(s: &str) -> Option<u32> {
    match s {
        "fem" => Some(5),
        "tio" => Some(10),
        "femton" => Some(15),
        "tjugo" => Some(20),
        "tjugofem" => Some(25),
        _ => s.parse().ok(),
    }
}

fn parse_sv_hour_token(s: &str) -> Option<u32> {
    match s {
        "ett" | "en" => Some(1),
        "två" | "tva" => Some(2),
        "tre" => Some(3),
        "fyra" => Some(4),
        "fem" => Some(5),
        "sex" => Some(6),
        "sju" => Some(7),
        "åtta" | "atta" => Some(8),
        "nio" => Some(9),
        "tio" => Some(10),
        "elva" => Some(11),
        "tolv" => Some(12),
        _ => s.parse().ok(),
    }
}

fn parse_sv_qty(s: &str) -> Option<i64> {
    match s {
        "en" | "ett" => Some(1),
        "två" | "tva" => Some(2),
        "tre" => Some(3),
        "fyra" => Some(4),
        "fem" => Some(5),
        "sex" => Some(6),
        "sju" => Some(7),
        "åtta" | "atta" => Some(8),
        "nio" => Some(9),
        "tio" => Some(10),
        "fjorton" => Some(14),
        _ => s.parse().ok(),
    }
}

fn parse_sv_grain(s: &str) -> Option<Grain> {
    let t = s.to_lowercase();
    if t.contains("sek") {
        Some(Grain::Second)
    } else if t.contains("minut") {
        Some(Grain::Minute)
    } else if t.contains("timm") {
        Some(Grain::Hour)
    } else if t.contains("dag") {
        Some(Grain::Day)
    } else if t.contains("veck") {
        Some(Grain::Week)
    } else if t.contains("mån") || t.contains("man") {
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
            name: "now (sv)".to_string(),
            pattern: vec![regex("nu")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (sv)".to_string(),
            pattern: vec![regex("idag|i dag")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (sv)".to_string(),
            pattern: vec![regex("imorgon|i morgon")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "part-of-day keywords (sv)".to_string(),
            pattern: vec![regex("morgon(en)?|lunch(en)?|eftermiddag(en)?|kv[äa]ll(en)?|natt(en)?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let pod = if s.contains("morgon") {
                    PartOfDay::Morning
                } else if s.contains("lunch") {
                    PartOfDay::Lunch
                } else if s.contains("eftermiddag") {
                    PartOfDay::Afternoon
                } else if s.contains("kväll") || s.contains("kvall") {
                    PartOfDay::Evening
                } else {
                    PartOfDay::Night
                };
                Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(pod))))
            }),
        },
        Rule {
            name: "ikväll (sv)".to_string(),
            pattern: vec![regex("ikv[äa]ll")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(TimeData::latent(TimeForm::PartOfDay(PartOfDay::Evening))),
                ))))
            }),
        },
        Rule {
            name: "under eftermiddagen / efter lunch (sv)".to_string(),
            pattern: vec![regex("under\\s+eftermiddag(en)?|efter\\s+lunch(en)?")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(TimeData::latent(TimeForm::PartOfDay(PartOfDay::Afternoon))),
                ))))
            }),
        },
        Rule {
            name: "yesterday (sv)".to_string(),
            pattern: vec![regex("ig[åa]r|i g[åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day before yesterday / day after tomorrow (sv)".to_string(),
            pattern: vec![regex("i f[öo]rrg[åa]r|f[öo]rrg[åa]r|i [öo]vermorgon|[öo]vermorgon")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("förr") || s.contains("forr") {
                    Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))
                }
            }),
        },
        Rule {
            name: "day of week (sv)".to_string(),
            pattern: vec![regex("måndag(en)?s?|mån\\.?|tisdag(en)?s?|tis?\\.?|onsdag(en)?s?|ons\\.?|torsdag(en)?s?|tors?\\.?|fredag(en)?s?|fre\\.?|lördag(en)?s?|lör\\.?|söndag(en)?s?|sön\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("måndag")
                    || s.contains("mandag")
                    || s == "mån"
                    || s == "man"
                    || s == "mån."
                    || s == "man."
                {
                    0
                } else if s.contains("tisdag") {
                    1
                } else if s.contains("onsdag") {
                    2
                } else if s.contains("torsdag") || s == "tors" || s == "tors." {
                    3
                } else if s.contains("fredag") || s == "fre" || s == "fre." {
                    4
                } else if s.contains("lördag")
                    || s.contains("lordag")
                    || s == "lör"
                    || s == "lör."
                    || s == "lor"
                    || s == "lor."
                {
                    5
                } else if s.contains("söndag")
                    || s.contains("sondag")
                    || s == "sön"
                    || s == "sön."
                    || s == "son"
                    || s == "son."
                {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "den förste mars (sv)".to_string(),
            pattern: vec![regex("den\\s+f[öo]rste\\s+mars|den\\s+f[öo]rsta\\s+mars")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "1:a mars (sv)".to_string(),
            pattern: vec![regex("(\\d{1,2}):a\\s+mars")],
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
            name: "3 mars (sv)".to_string(),
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
            name: "den tredje mars (sv)".to_string(),
            pattern: vec![regex("den\\s+tredje\\s+mars")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 3, year: None })))),
        },
        Rule {
            name: "<ordinal> dagen i <month> (sv)".to_string(),
            pattern: vec![regex("(f[öo]rsta|andra|tredje|fj[äa]rde|\\d{1,2})\\s+dagen\\s+i\\s+(januari|februari|mars|april|maj|juni|juli|augusti|september|oktober|november|december)")],
            production: Box::new(|nodes| {
                let (d_s, m_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let day = parse_sv_ordinal_day(&d_s)?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                let month = parse_sv_month(&m_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "<day>-<day> <month> (sv)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*(?:-|till)\\s*(\\d{1,2})\\s+(januari|februari|mars|april|maj|juni|juli|augusti|aug|september|oktober|november|december)")],
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
                let month = parse_sv_month(&m_s)?;
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
            name: "<day> <month> till <day> <month> (sv)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+(januari|februari|mars|april|maj|juni|juli|augusti|aug|september|oktober|november|december)\\s*(?:till|-)\\s*(\\d{1,2})\\s+(januari|februari|mars|april|maj|juni|juli|augusti|aug|september|oktober|november|december)")],
            production: Box::new(|nodes| {
                let (d1, m1s, d2, m2s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (
                        rm.group(1)?,
                        rm.group(2)?.to_lowercase(),
                        rm.group(3)?,
                        rm.group(4)?.to_lowercase(),
                    ),
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let month1 = parse_sv_month(&m1s)?;
                let month2 = parse_sv_month(&m2s)?;
                let from =
                    TimeData::new(TimeForm::DateMDY { month: month1, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month: month2, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "den 3:e mars (sv)".to_string(),
            pattern: vec![regex("den\\s+(\\d{1,2}):e\\s+mars")],
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
            name: "tredje mars 2015 (sv)".to_string(),
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
            name: "3:e mars 2015 (sv)".to_string(),
            pattern: vec![regex("(\\d{1,2}):e\\s+mars\\s+(\\d{4})")],
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
            name: "På den 15:e (sv)".to_string(),
            pattern: vec![regex("[Pp]å\\s+den\\s+(\\d{1,2}):e")],
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
            name: "På den 15 (sv)".to_string(),
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
            name: "Den 15:e (sv)".to_string(),
            pattern: vec![regex("[Dd]en\\s+(\\d{1,2}):e")],
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
            name: "15:e februari (sv)".to_string(),
            pattern: vec![regex("(\\d{1,2}):e\\s+februari")],
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
            name: "februari 15 (sv)".to_string(),
            pattern: vec![regex("februari\\s*(\\d{1,2})")],
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
            name: "Oktober 2014 (sv)".to_string(),
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
            name: "nästa mars (sv)".to_string(),
            pattern: vec![regex("n[äa]sta\\s+mars")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::Month(3));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "Ons, Feb13 (sv)".to_string(),
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
            name: "denna vecka (sv)".to_string(),
            pattern: vec![regex("denna vecka")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "förra vecka (sv)".to_string(),
            pattern: vec![regex("f[öo]rra vecka")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "nästa vecka (sv)".to_string(),
            pattern: vec![regex("n[äa]sta vecka")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "helg (sv)".to_string(),
            pattern: vec![regex("helg(en)?|weekend")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "förra/nästa/denna helg (sv)".to_string(),
            pattern: vec![regex("f[öo]rra\\s+helg|n[äa]sta\\s+helg|denna\\s+helg(en)?|i\\s+helgen")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::Weekend);
                if s.starts_with("förra") || s.starts_with("forra") {
                    t.direction = Some(Direction::Past);
                } else if s.starts_with("nästa") || s.starts_with("nasta") {
                    t.direction = Some(Direction::Future);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "detta kvartal (sv)".to_string(),
            pattern: vec![regex("detta kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 0 })))),
        },
        Rule {
            name: "nästa kvartal (sv)".to_string(),
            pattern: vec![regex("n[äa]sta kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "tredje kvartalet (sv)".to_string(),
            pattern: vec![regex("tredje kvartalet|3:e kvartalet|3\\. kvartalet")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "3:e kvartal (sv)".to_string(),
            pattern: vec![regex("3:e kvartal|3\\. kvartal|tredje kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "4:e kvartal 2018 (sv)".to_string(),
            pattern: vec![regex("4:e kvartal\\s*(\\d{4})|4\\. kvartal\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(4, year))))
            }),
        },
        Rule {
            name: "fjärde kvartalet 2018 (sv)".to_string(),
            pattern: vec![regex("fj[äa]rde kvartalet\\s*(\\d{4})")],
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
            name: "year (sv)".to_string(),
            pattern: vec![regex("(19\\d\\d|20[0-2]\\d|2030)")],
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
            name: "season (sv)".to_string(),
            pattern: vec![regex("v[åa]r(en)?|sommar(en)?|h[öo]st(en)?|vinter(n)?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let season = if s.starts_with("vår") || s.starts_with("var") {
                    0
                } else if s.starts_with("sommar") {
                    1
                } else if s.starts_with("höst") || s.starts_with("host") {
                    2
                } else {
                    3
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "this/next/last season (sv)".to_string(),
            pattern: vec![regex("(denna|den\\s+h[äa]r)\\s+(sommaren|vintern|h[öo]sten|v[åa]ren)|n[äa]sta\\s+(sommar|vinter|h[öo]st|v[åa]r)|f[öo]rra\\s+(sommar|vinter|h[öo]st|v[åa]r)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let season = if s.contains("vår") || s.contains("var") {
                    0
                } else if s.contains("sommar") {
                    1
                } else if s.contains("höst") || s.contains("host") {
                    2
                } else {
                    3
                };
                let mut t = TimeData::new(TimeForm::Season(season));
                if s.starts_with("nästa") || s.starts_with("nasta") {
                    t.direction = Some(Direction::Future);
                } else if s.starts_with("förra") || s.starts_with("forra") {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "holiday basic (sv)".to_string(),
            pattern: vec![regex("juldagen|ny[åa]rsafton|ny[åa]rsdagen|ny[åa]rsdag")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let form = if s.starts_with("juldag") {
                    TimeForm::Holiday("christmas day".to_string(), None)
                } else if s.contains("afton") {
                    TimeForm::Holiday("new year's eve".to_string(), None)
                } else {
                    TimeForm::Holiday("new year's day".to_string(), None)
                };
                Some(TokenData::Time(TimeData::new(form)))
            }),
        },
        Rule {
            name: "förra år (sv)".to_string(),
            pattern: vec![regex("f[öo]rra [åa]r(et)?|f[öo]rra året")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "föregående år (sv)".to_string(),
            pattern: vec![regex("f[öo]reg[åa]ende [åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "i fjol (sv)".to_string(),
            pattern: vec![regex("i fjol")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "i/detta år (sv)".to_string(),
            pattern: vec![regex("i [åa]r|detta [åa]r(et)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))),
        },
        Rule {
            name: "nästa år (sv)".to_string(),
            pattern: vec![regex("n[äa]sta [åa]r(et)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "klockan/kl <hour> (sv)".to_string(),
            pattern: vec![regex("klockan\\s*(\\d{1,2})|kl\\.?\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
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
            name: "klockan/kl <hh:mm> (sv)".to_string(),
            pattern: vec![regex("(?:klockan|kl\\.?)\\s*(\\d{1,2})[:\\.](\\d{2})")],
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
            name: "@/kl/klockan <hour>[:mm] <timezone> (sv)".to_string(),
            pattern: vec![regex("(@\\s*)?(kl(ockan|\\.)?\\s*)?(\\d{1,2})(:(\\d{2}))?\\s*(CET|CEST|GMT|UTC|cet|cest|gmt|utc)")],
            production: Box::new(|nodes| {
                let (h, m_opt, tz) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (
                        rm.group(4)?,
                        rm.group(6),
                        rm.group(7)?.to_uppercase(),
                    ),
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let minute: u32 = m_opt.unwrap_or("0").parse().ok()?;
                if hour > 23 || minute > 59 {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::HourMinute(hour, minute, m_opt.is_none()));
                t.timezone = Some(tz);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "kvart över <hour> (sv)".to_string(),
            pattern: vec![regex("kvart\\s+[öo]ver\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
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
            name: "kvart i <hour> (sv)".to_string(),
            pattern: vec![regex("kvart\\s+i\\s*(\\d{1,2}|ett|en|tv[åa]|tre|fyra|fem|sex|sju|[åa]tta|nio|tio|elva|tolv)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let in_hour = parse_sv_hour_token(&h)?;
                if in_hour == 0 || in_hour > 24 {
                    return None;
                }
                let hour = if in_hour == 1 { 0 } else { in_hour - 1 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 45, false))))
            }),
        },
        Rule {
            name: "<minutes> över <hour> (sv)".to_string(),
            pattern: vec![regex("(fem|tio|femton|tjugo|tjugofem|\\d{1,2})\\s+[öo]ver\\s*(\\d{1,2}|ett|en|tv[åa]|tre|fyra|fem|sex|sju|[åa]tta|nio|tio|elva|tolv)")],
            production: Box::new(|nodes| {
                let (m_s, h_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let mins = parse_sv_small_minutes(&m_s)?;
                let hour = parse_sv_hour_token(&h_s)?;
                if hour > 23 || mins > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, mins, false))))
            }),
        },
        Rule {
            name: "om/i <duration> (sv)".to_string(),
            pattern: vec![regex("om|i"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let d = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: d.value,
                    grain: d.grain,
                })))
            }),
        },
        Rule {
            name: "<duration> från nu/idag (sv)".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex("från\\s+(idag|nu)")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: d.value,
                    grain: d.grain,
                })))
            }),
        },
        Rule {
            name: "inom <duration> (sv)".to_string(),
            pattern: vec![regex("innanf[öo]r|inom"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let d = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: d.value,
                    grain: d.grain,
                })))
            }),
        },
        Rule {
            name: "efter <duration> (sv)".to_string(),
            pattern: vec![regex("efter"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let d = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::RelativeGrain {
                    n: d.value,
                    grain: d.grain,
                });
                t.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<duration> sedan (sv)".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex("sedan")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: -d.value,
                    grain: d.grain,
                })))
            }),
        },
        Rule {
            name: "senaste/nästa n cycles (sv)".to_string(),
            pattern: vec![regex("(senaste|sista|f[öo]rra|f[öo]reg[åa]ende|n[äa]sta)\\s+(\\d+|en|ett|tv[åa]|tre|fyra|fem|sex|sju|[åa]tta|nio|tio|fjorton)\\s+(sekund(?:er|erna)?|minut(?:er|erna)?|timm(?:e|ar|arna)?|dag(?:ar|arna)?|veck(?:a|or|orna)?|m[åa]nad(?:er|erna)?|[åa]r)")],
            production: Box::new(|nodes| {
                let (dir, n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (
                        rm.group(1)?.to_lowercase(),
                        rm.group(2)?.to_lowercase(),
                        rm.group(3)?.to_lowercase(),
                    ),
                    _ => return None,
                };
                let n = parse_sv_qty(&n_s)?;
                let grain = parse_sv_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: dir.starts_with("senaste")
                        || dir.starts_with("sista")
                        || dir.starts_with("förra")
                        || dir.starts_with("forra")
                        || dir.starts_with("föreg")
                        || dir.starts_with("foreg"),
                    interval: true,
                })))
            }),
        },
    ]);
    rules
}
