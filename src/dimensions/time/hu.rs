use super::Direction;
use super::{TimeData, TimeForm};
use crate::dimensions::time_grain::Grain;
use crate::pattern::regex;
use crate::types::{Rule, TokenData};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (hu)".to_string(),
            pattern: vec![regex("most|azonnal|hónap vége|hó vége|hó végi|hó végén")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (hu)".to_string(),
            pattern: vec![regex("ma")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (hu)".to_string(),
            pattern: vec![regex("holnap")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (hu)".to_string(),
            pattern: vec![regex("tegnap")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "part of day (hu)".to_string(),
            pattern: vec![regex("(reggel(i(t)?)?|d[ée]lel[őo]tt(i(t)?)?|d[ée]lben|d[ée]li(t)?|d[ée]lut[áa]n(i(t)?)?|est(e|i(t)?)?|[ée]jszaka(i(t)?)?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let (start_h, end_h) = if s.starts_with("reggel") {
                    (6, 10)
                } else if s.starts_with("délelőtt") || s.starts_with("delelőtt") {
                    (8, 12)
                } else if s.starts_with("délben") || s.starts_with("delben") || s.starts_with("déli") || s.starts_with("deli") {
                    (12, 13)
                } else if s.starts_with("délután") || s.starts_with("delutan") {
                    (12, 18)
                } else if s.starts_with("est") {
                    (16, 20)
                } else {
                    (20, 23)
                };
                let from = TimeData::new(TimeForm::Hour(start_h, false));
                let to = TimeData::new(TimeForm::Hour(end_h, false));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    true,
                ))))
            }),
        },
        Rule {
            name: "day of week (hu)".to_string(),
            pattern: vec![regex("hétf(ő(n|t|i(t)?)?|\\.)|kedd(en|et|i(t)?)?|szerda(i(t)?)?|szerdá(n|t)|szer\\.?|csütörtök(ö(n|t)|i(t)?)?|csüt\\.?|péntek(e(n|t)|i(t)?)?|pén\\.?|szombat(o(n|t)|i(t)?)?|szom\\.?|vasárnap(ot|i(t)?)?|vas\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("hét") || s.starts_with("het") {
                    0
                } else if s.starts_with("ked") {
                    1
                } else if s.starts_with("szer") {
                    2
                } else if s.starts_with("csü") || s.starts_with("csu") {
                    3
                } else if s.starts_with("pé") || s.starts_with("pen") {
                    4
                } else if s.starts_with("szo") {
                    5
                } else if s.starts_with("vas") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "year end (hu)".to_string(),
            pattern: vec![regex("év vége|év végi|év végén")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::AllGrain(Grain::Year)),
                })))
            }),
        },
        Rule {
            name: "hét. monday (hu)".to_string(),
            pattern: vec![regex("h[eé]t\\.")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(0))))),
        },
        Rule {
            name: "hét monday (hu)".to_string(),
            pattern: vec![regex("h[eé]t")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(0))))),
        },
        Rule {
            name: "január (hu)".to_string(),
            pattern: vec![regex("január|jan\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(1))))),
        },
        Rule {
            name: "február (hu)".to_string(),
            pattern: vec![regex("február|februárban|februári|februárit|feb\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(2))))),
        },
        Rule {
            name: "március (hu)".to_string(),
            pattern: vec![regex("március|márc\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(3))))),
        },
        Rule {
            name: "már (hu)".to_string(),
            pattern: vec![regex("már\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(3))))),
        },
        Rule {
            name: "április (hu)".to_string(),
            pattern: vec![regex("április|ápr\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(4))))),
        },
        Rule {
            name: "május (hu)".to_string(),
            pattern: vec![regex("május|májusban|májusi|májusit|máj\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(5))))),
        },
        Rule {
            name: "június (hu)".to_string(),
            pattern: vec![regex("június|jún\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(6))))),
        },
        Rule {
            name: "július (hu)".to_string(),
            pattern: vec![regex("július|júl\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(7))))),
        },
        Rule {
            name: "augusztus (hu)".to_string(),
            pattern: vec![regex("augusztus|aug\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(8))))),
        },
        Rule {
            name: "szeptember (hu)".to_string(),
            pattern: vec![regex("szeptember|szept\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(9))))),
        },
        Rule {
            name: "szep (hu)".to_string(),
            pattern: vec![regex("szep\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(9))))),
        },
        Rule {
            name: "október (hu)".to_string(),
            pattern: vec![regex("október|okt\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(10))))),
        },
        Rule {
            name: "novemberben (hu)".to_string(),
            pattern: vec![regex("novemberben|november")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(11))))),
        },
        Rule {
            name: "decemberben (hu)".to_string(),
            pattern: vec![regex("decemberben|december|dec\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(12))))),
        },
        Rule {
            name: "season (hu)".to_string(),
            pattern: vec![regex("ny[áa]r(on)?|t[ée]l(en)?|tavasszal|tavasz|[őo]sz(el)?|[őo]sszel")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let season = if s.contains("tavasz") {
                    0
                } else if s.contains("ny") {
                    1
                } else if s.contains("sz") || s.contains("ő") || s.contains("osz") {
                    2
                } else {
                    3
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "következő hónap (hu)".to_string(),
            pattern: vec![regex("következő hónap")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::AllGrain(Grain::Month));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "jövő hónap (hu)".to_string(),
            pattern: vec![regex("jövő hónap|jovo honap")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "előző év (hu)".to_string(),
            pattern: vec![regex("előző év|elozo ev")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "múlt év (hu)".to_string(),
            pattern: vec![regex("múlt év|mult ev")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "de 11 (hu)".to_string(),
            pattern: vec![regex("de\\.?\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 12 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(if hour == 12 { 0 } else { hour }, 0, false))))
            }),
        },
        Rule {
            name: "du 3 (hu)".to_string(),
            pattern: vec![regex("du\\.?\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                if hour > 12 {
                    return None;
                }
                if hour < 12 {
                    hour = hour.checked_add(12)?;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "délelőtt 11 (hu)".to_string(),
            pattern: vec![regex("d[ée]lel[őo]tt\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 12 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(if hour == 12 { 0 } else { hour }, 0, false))))
            }),
        },
        Rule {
            name: "délután 11 (hu)".to_string(),
            pattern: vec![regex("d[ée]lut[áa]n\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                if hour > 12 {
                    return None;
                }
                if hour < 12 {
                    hour = hour.checked_add(12)?;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "2013 . 08 . 20 (hu)".to_string(),
            pattern: vec![regex("(\\d{4})\\s*\\.\\s*(\\d{1,2})\\s*\\.\\s*(\\d{1,2})")],
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
            name: "08 . 20 (hu)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*\\.\\s*(\\d{1,2})")],
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
