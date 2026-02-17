use super::{Direction, IntervalDirection, PartOfDay, TimeData, TimeForm};
use crate::dimensions::time_grain::Grain;
use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

fn is_time(td: &TokenData) -> bool {
    matches!(td, TokenData::Time(_))
}
fn time_data(td: &TokenData) -> Option<&TimeData> {
    match td {
        TokenData::Time(d) => Some(d),
        _ => None,
    }
}
fn parse_nl_number_word(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "een" | "één" => Some(1),
        "twee" => Some(2),
        "drie" => Some(3),
        "vier" => Some(4),
        "vijf" => Some(5),
        "zes" => Some(6),
        "zeven" => Some(7),
        "acht" => Some(8),
        "negen" => Some(9),
        "tien" => Some(10),
        "paar" => Some(2),
        _ => None,
    }
}
fn parse_nl_grain(s: &str) -> Option<Grain> {
    let u = s.to_lowercase();
    if u.starts_with("seconde") {
        Some(Grain::Second)
    } else if u.starts_with("minuut") {
        Some(Grain::Minute)
    } else if u.starts_with("uur") {
        Some(Grain::Hour)
    } else if u.starts_with("dag") {
        Some(Grain::Day)
    } else if u.starts_with("week") {
        Some(Grain::Week)
    } else if u.starts_with("maand") {
        Some(Grain::Month)
    } else if u.starts_with("jaar") {
        Some(Grain::Year)
    } else {
        None
    }
}

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
        Rule {
            name: "15 februari (nl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+februari")],
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
            name: "februari 15 (nl)".to_string(),
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
            name: "8 augustus (nl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+augustus")],
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
            name: "Oktober 2014 (nl)".to_string(),
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
            name: "31.10.74 (nl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.(\\d{1,2})\\.(\\d{2})")],
            production: Box::new(|nodes| {
                let (d, m, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let yy: i32 = y.parse().ok()?;
                if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(1900_i32.checked_add(yy)?) })))
            }),
        },
        Rule {
            name: "18de juli 2014 (nl)".to_string(),
            pattern: vec![regex("(\\d{1,2})(de|ste)\\s+juli\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (d, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let year: i32 = y.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 7, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "maart (nl)".to_string(),
            pattern: vec![regex("maart")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(3))))),
        },
        Rule {
            name: "Deze week (nl)".to_string(),
            pattern: vec![regex("[Dd]eze week")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "vorige week (nl)".to_string(),
            pattern: vec![regex("vorige week")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "volgende week (nl)".to_string(),
            pattern: vec![regex("volgende week")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "vorige maand (nl)".to_string(),
            pattern: vec![regex("vorige maand")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "volgende maand (nl)".to_string(),
            pattern: vec![regex("volgende maand")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "dit kwartaal (nl)".to_string(),
            pattern: vec![regex("dit kwartaal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 0 })))),
        },
        Rule {
            name: "volgende kwartaal (nl)".to_string(),
            pattern: vec![regex("volgende kwartaal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "derde kwartaal (nl)".to_string(),
            pattern: vec![regex("derde kwartaal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "op de eerste (nl)".to_string(),
            pattern: vec![regex("op de eerste")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(1))))),
        },
        Rule {
            name: "op de 15de (nl)".to_string(),
            pattern: vec![regex("op de\\s*(\\d{1,2})de")],
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
            name: "4de kwartaal 2018 (nl)".to_string(),
            pattern: vec![regex("4de kwartaal\\s*(\\d{4})")],
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
            name: "vorig jaar (nl)".to_string(),
            pattern: vec![regex("vorig jaar")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "dit jaar (nl)".to_string(),
            pattern: vec![regex("dit jaar")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))),
        },
        Rule {
            name: "volgend jaar (nl)".to_string(),
            pattern: vec![regex("volgend jaar")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "derde dag in oktober (nl)".to_string(),
            pattern: vec![regex("derde dag in oktober")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 3, year: None })))),
        },
        Rule {
            name: "4 uur in de nacht (nl)".to_string(),
            pattern: vec![regex("4 uur in de nacht")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(4, 0, false))))),
        },
        Rule {
            name: "4 uur in de ochtend (nl)".to_string(),
            pattern: vec![regex("4 uur in de ochtend")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(4, 0, false))))),
        },
        Rule {
            name: "4 uur 's ochtends (nl)".to_string(),
            pattern: vec![regex("4 uur\\s+'s\\s+ochtends")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(4, 0, false))))),
        },
        Rule {
            name: "3 uur (nl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+uur")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "om drie uur (nl)".to_string(),
            pattern: vec![regex("om\\s+drie\\s+uur")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(3, 0, false))))),
        },
        Rule {
            name: "drie uur 's middags (nl)".to_string(),
            pattern: vec![regex("drie\\s+uur\\s+'s\\s+middags")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 0, false))))),
        },
        Rule {
            name: "rond drie uur (nl)".to_string(),
            pattern: vec![regex("rond\\s+drie\\s+uur")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(3, 0, false))))),
        },
        Rule {
            name: "om ongeveer drie uur (nl)".to_string(),
            pattern: vec![regex("om\\s+ongeveer\\s+drie\\s+uur")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(3, 0, false))))),
        },
        Rule {
            name: "om circa 15u (nl)".to_string(),
            pattern: vec![regex("(om\\s+)?(circa|ca\\.)\\s*(\\d{1,2})[uh]")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(3)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "rond kwart over 2 (nl)".to_string(),
            pattern: vec![regex("rond\\s+kwart\\s+over\\s+2|kwart\\s+over\\s+twee\\s+uur")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(2, 15, false))))),
        },
        Rule {
            name: "15 over 14 (nl)".to_string(),
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
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "twintig over 3 (nl)".to_string(),
            pattern: vec![regex("twintig\\s+over\\s+3")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(3, 20, false))))),
        },
        Rule {
            name: "kwart voor 12 (nl)".to_string(),
            pattern: vec![regex("kwart\\s+voor\\s+(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                let out_hour = if hour == 0 { 23 } else { hour.checked_sub(1)? };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_hour, 45, false))))
            }),
        },
        Rule {
            name: "over/in een seconde (nl)".to_string(),
            pattern: vec![regex("(over|in)\\s+een\\s+seconde")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Second,
                offset: 1,
            })))),
        },
        Rule {
            name: "over/in een minuut (nl)".to_string(),
            pattern: vec![regex("(over|in)\\s+een\\s+minuut")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Minute,
                offset: 1,
            })))),
        },
        Rule {
            name: "over/in een uur (nl)".to_string(),
            pattern: vec![regex("(over|in)\\s+een\\s+uur")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Hour,
                offset: 1,
            })))),
        },
        Rule {
            name: "over twee uur (nl)".to_string(),
            pattern: vec![regex("over\\s+twee\\s+uur|in\\s+een\\s+paar\\s+uur")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Hour,
                offset: 2,
            })))),
        },
        Rule {
            name: "over 24 uur (nl)".to_string(),
            pattern: vec![regex("over\\s*24\\s+uur")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Hour,
                offset: 24,
            })))),
        },
        Rule {
            name: "over een week (nl)".to_string(),
            pattern: vec![regex("over\\s+een\\s+week")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Week,
                offset: 1,
            })))),
        },
        Rule {
            name: "7 dagen geleden (nl)".to_string(),
            pattern: vec![regex("7\\s+dagen\\s+geleden")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Day,
                offset: -7,
            })))),
        },
        Rule {
            name: "14 dagen geleden (nl)".to_string(),
            pattern: vec![regex("14\\s+dagen\\s+geleden")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Day,
                offset: -14,
            })))),
        },
        Rule {
            name: "<n> <grain> geleden (nl)".to_string(),
            pattern: vec![regex("(\\d+|een|één|twee|drie|vier|vijf|zes|zeven|acht|negen|tien)\\s+(seconde(n)?|minu(u)?t(en)?|uur|dagen?|weken?|maanden?|jaar)\\s+geleden")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n = n_s.parse::<i64>().ok().or_else(|| parse_nl_number_word(n_s))?;
                let grain = parse_nl_grain(g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: (n as i32).checked_neg()?,
                })))
            }),
        },
        Rule {
            name: "worded geleden (nl)".to_string(),
            pattern: vec![regex("(een|twee|drie)\\s+(week|weken|maand|maanden|jaar)\\s+geleden")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n = parse_nl_number_word(n_s)?;
                let grain = parse_nl_grain(g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: (n as i32).checked_neg()?,
                })))
            }),
        },
        Rule {
            name: "twee weken geleden (nl)".to_string(),
            pattern: vec![regex("twee\\s+weken\\s+geleden")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Week,
                offset: -2,
            })))),
        },
        Rule {
            name: "vorige 2 weken (nl)".to_string(),
            pattern: vec![regex("vorige\\s+2\\s+weken|afgelopen\\s+twee\\s+weken")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                n: 2,
                grain: Grain::Week,
                past: true,
                interval: true,
            })))),
        },
        Rule {
            name: "komende 3 weken (nl)".to_string(),
            pattern: vec![regex("komende\\s+3\\s+weken|komende\\s+drie\\s+weken")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                n: 3,
                grain: Grain::Week,
                past: false,
                interval: true,
            })))),
        },
        Rule {
            name: "vorige 2 jaren (nl)".to_string(),
            pattern: vec![regex("vorige\\s+2\\s+jaren|afgelopen\\s+twee\\s+jaren")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                n: 2,
                grain: Grain::Year,
                past: true,
                interval: true,
            })))),
        },
        Rule {
            name: "komende 3 jaren (nl)".to_string(),
            pattern: vec![regex("komende\\s+3\\s+jaren|komende\\s+drie\\s+jaren")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                n: 3,
                grain: Grain::Year,
                past: false,
                interval: true,
            })))),
        },
        Rule {
            name: "named durations geleden (nl)".to_string(),
            pattern: vec![regex("een\\s+week\\s+geleden|drie\\s+weken\\s+geleden|drie\\s+maanden\\s+geleden|twee\\s+jaar\\s+geleden")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let (grain, offset) = if s.contains("week") && s.contains("een") {
                    (Grain::Week, -1)
                } else if s.contains("weken") && s.contains("drie") {
                    (Grain::Week, -3)
                } else if s.contains("maanden") && s.contains("drie") {
                    (Grain::Month, -3)
                } else if s.contains("jaar") && s.contains("twee") {
                    (Grain::Year, -2)
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset })))
            }),
        },
        Rule {
            name: "afgelopen/vorige n cycle (nl)".to_string(),
            pattern: vec![regex("(afgelopen|vorige)\\s+(\\d+|een|twee|drie)\\s+(seconde(n)?|secondes|minu(u)?t(en)?|uur|dagen?|weken?|maanden?|jaar)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let n = n_s.parse::<i64>().ok().or_else(|| parse_nl_number_word(n_s))?;
                let grain = parse_nl_grain(g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "komende/volgende n cycle (nl)".to_string(),
            pattern: vec![regex("(komende|volgende)\\s+(\\d+|een|twee|drie|paar)\\s+(seconde(n)?|secondes|minu(u)?t(en)?|uur|dagen?|weken?|maanden?|jaar)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let n = n_s.parse::<i64>().ok().or_else(|| parse_nl_number_word(n_s))?;
                let grain = parse_nl_grain(g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "binnen 2 weken (nl)".to_string(),
            pattern: vec![regex("binnen\\s+(\\d+|een|twee|drie)\\s+(dagen?|weken?|maanden?|jaar)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n = n_s.parse::<i64>().ok().or_else(|| parse_nl_number_word(n_s))?;
                let grain = parse_nl_grain(g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "binnen 2 weken literal (nl)".to_string(),
            pattern: vec![regex("binnen\\s+2\\s+weken")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                n: 2,
                grain: Grain::Week,
                past: false,
                interval: true,
            })))),
        },
        Rule {
            name: "over een kwartier (nl)".to_string(),
            pattern: vec![regex("over\\s+een\\s+kwartier")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Minute,
                offset: 15,
            })))),
        },
        Rule {
            name: "in/over <n> <grain> (nl)".to_string(),
            pattern: vec![regex("(in|over)\\s+(\\d+|een|één|twee|drie|vier|vijf|zes|zeven|acht|negen|tien|paar)\\s+(seconde(n)?|minu(u)?t(en)?|uur|dagen?|weken?|maanden?|jaar)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let n = n_s.parse::<i64>().ok().or_else(|| parse_nl_number_word(n_s))?;
                let grain = parse_nl_grain(g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: n as i32,
                })))
            }),
        },
        Rule {
            name: "een jaar na kerst (nl)".to_string(),
            pattern: vec![regex("(een|1)\\s+jaar\\s+na\\s+kerst")],
            production: Box::new(|_| {
                let base = TimeData::new(TimeForm::Holiday("christmas day".to_string(), None));
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n: 1,
                    grain: Grain::Year,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "weekend (nl)".to_string(),
            pattern: vec![regex("weekend|dit\\s+weekend|komend\\s+weekend")],
            production: Box::new(|nodes| {
                let mut t = TimeData::new(TimeForm::Weekend);
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("komend") {
                    t.direction = Some(Direction::Future);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "season this (nl)".to_string(),
            pattern: vec![regex("deze\\s+(lente|zomer|herfst|winter)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let season = match s {
                    "lente" => 0,
                    "zomer" => 1,
                    "herfst" => 2,
                    "winter" => 3,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "13 - 15 juli (nl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*[-–]\\s*(\\d{1,2})\\s+juli")],
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
                let start = TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None });
                let end = TimeData::new(TimeForm::DateMDY { month: 7, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(start),
                    Box::new(end),
                    true,
                ))))
            }),
        },
        Rule {
            name: "13 t/m 15 juli (nl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+t/m\\s*(\\d{1,2})\\s+juli")],
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
                let start = TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None });
                let end = TimeData::new(TimeForm::DateMDY { month: 7, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(start),
                    Box::new(end),
                    true,
                ))))
            }),
        },
        Rule {
            name: "juli 13 - juli 15 (nl)".to_string(),
            pattern: vec![regex("juli\\s*(\\d{1,2})\\s*[-–]\\s*juli\\s*(\\d{1,2})")],
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
                let start = TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None });
                let end = TimeData::new(TimeForm::DateMDY { month: 7, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(start),
                    Box::new(end),
                    true,
                ))))
            }),
        },
        Rule {
            name: "13 tot en met 15 juli (nl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+tot\\s+en\\s+met\\s+(\\d{1,2})\\s+juli")],
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
                let start = TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None });
                let end = TimeData::new(TimeForm::DateMDY { month: 7, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(start),
                    Box::new(end),
                    true,
                ))))
            }),
        },
        Rule {
            name: "tot het einde van de maand (nl)".to_string(),
            pattern: vec![regex("tot\\s+het\\s+einde\\s+van\\s+de\\s+maand")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::AllGrain(Grain::Month)),
                })))
            }),
        },
        Rule {
            name: "na de lunch (nl)".to_string(),
            pattern: vec![regex("na\\s+de\\s+lunch")],
            production: Box::new(|_| {
                let start = TimeData::new(TimeForm::HourMinute(13, 0, false));
                let end = TimeData::new(TimeForm::HourMinute(17, 0, false));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(start),
                    Box::new(end),
                    true,
                ))))
            }),
        },
        Rule {
            name: "18:30u - 19:00u (nl)".to_string(),
            pattern: vec![regex("(\\d{1,2}):(\\d{2})[uh]\\s*[-–]\\s*(\\d{1,2}):(\\d{2})[uh]|(\\d{1,2}):(\\d{2})[uh]\\s+tot\\s+(\\d{1,2}):(\\d{2})[uh]")],
            production: Box::new(|nodes| {
                let (h1, m1, h2, m2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => {
                        if m.group(1).is_some() {
                            (m.group(1)?, m.group(2)?, m.group(3)?, m.group(4)?)
                        } else {
                            (m.group(5)?, m.group(6)?, m.group(7)?, m.group(8)?)
                        }
                    }
                    _ => return None,
                };
                let (hh1, mm1, hh2, mm2): (u32, u32, u32, u32) =
                    (h1.parse().ok()?, m1.parse().ok()?, h2.parse().ok()?, m2.parse().ok()?);
                if hh1 > 23 || hh2 > 23 || mm1 > 59 || mm2 > 59 {
                    return None;
                }
                let start = TimeData::new(TimeForm::HourMinute(hh1, mm1, false));
                let end = TimeData::new(TimeForm::HourMinute(hh2, mm2, false));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(start),
                    Box::new(end),
                    true,
                ))))
            }),
        },
        Rule {
            name: "17u10 / 17h10 (nl)".to_string(),
            pattern: vec![regex("(\\d{1,2})[uh](\\d{2})")],
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
            name: "vanaf 14u / op zijn vroegst om 14 uur (nl)".to_string(),
            pattern: vec![regex("(vanaf|op\\s+zijn\\s+vroegst\\s+om)\\s*(\\d{1,2})\\s*(u|uur|h)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::HourMinute(hour, 0, false));
                t.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "twee uur op zijn vroegst (nl)".to_string(),
            pattern: vec![regex("(\\d{1,2}|een|twee|drie|vier|vijf|zes|zeven|acht|negen|tien|elf|twaalf)\\s+uur\\s+op\\s+zijn\\s+vroegst")],
            production: Box::new(|nodes| {
                let h_raw = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let mut hour: u32 = if let Ok(v) = h_raw.parse() {
                    v
                } else {
                    parse_nl_number_word(h_raw)? as u32
                };
                if hour > 23 || hour == 0 {
                    return None;
                }
                if hour < 12 {
                    hour = hour.checked_add(12)?;
                }
                let mut t = TimeData::new(TimeForm::HourMinute(hour, 0, false));
                t.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "holidays (nl)".to_string(),
            pattern: vec![regex("kerstmis|kerst|oudjaar|oudejaarsavond|nieuwjaarsdag|nieuwjaar|valentijnsdag|moederdag|vaderdag|halloween|allerheiligen")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let h = if s.contains("kerst") {
                    "christmas day"
                } else if s.contains("oudjaar") || s.contains("oudejaarsavond") {
                    "new year's eve"
                } else if s.contains("nieuwjaar") {
                    "new year's day"
                } else if s.contains("valentijn") {
                    "valentine's day"
                } else if s.contains("moederdag") {
                    "mother's day"
                } else if s.contains("vaderdag") {
                    "father's day"
                } else if s.contains("halloween") {
                    "halloween"
                } else if s.contains("allerheiligen") {
                    "all saints' day"
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(h.to_string(), None))))
            }),
        },
        Rule {
            name: "part of day relative (nl)".to_string(),
            pattern: vec![regex("vanavond|deze\\s+avond|vandaag\\s+avond|morgenavond|gisteravond|gisterenavond|vanmorgen|morgenochtend|gisterenochtend|morgennacht|gisterennacht|vanmiddag|deze\\s+namiddag|vandaag\\s+namiddag|morgenmiddag|morgen\\s+'s\\s+middags|morgen\\s+namiddag|gisterenmiddag|gistermiddag|gisterennamiddag")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let date = if s.starts_with("morgen") {
                    TimeData::new(TimeForm::Tomorrow)
                } else if s.starts_with("gister") {
                    TimeData::new(TimeForm::Yesterday)
                } else {
                    TimeData::new(TimeForm::Today)
                };
                let pod = if s.contains("middag") || s.contains("namiddag") {
                    PartOfDay::Afternoon
                } else if s.contains("avond") {
                    PartOfDay::Evening
                } else if s.contains("nacht") {
                    PartOfDay::Night
                } else {
                    PartOfDay::Morning
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(date),
                    Box::new(TimeData::new(TimeForm::PartOfDay(pod))),
                ))))
            }),
        },
    ]);
    rules
}
