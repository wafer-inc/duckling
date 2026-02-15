use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use crate::dimensions::time_grain::Grain;
use super::{Direction, IntervalDirection, PartOfDay, TimeData, TimeForm};

fn parse_it_hour_token(s: &str) -> Option<u32> {
    match s {
        "un" | "uno" | "una" => Some(1),
        "due" => Some(2),
        "tre" => Some(3),
        "quattro" => Some(4),
        "cinque" => Some(5),
        "sei" => Some(6),
        "sette" => Some(7),
        "otto" => Some(8),
        "nove" => Some(9),
        "dieci" => Some(10),
        "undici" => Some(11),
        "dodici" => Some(12),
        _ => s.parse().ok(),
    }
}

fn parse_it_month_token(s: &str) -> Option<u32> {
    if s.starts_with("gen") {
        Some(1)
    } else if s.starts_with("feb") {
        Some(2)
    } else if s.starts_with("mar") {
        Some(3)
    } else if s.starts_with("apr") {
        Some(4)
    } else if s.starts_with("mag") {
        Some(5)
    } else if s.starts_with("giu") {
        Some(6)
    } else if s.starts_with("lug") {
        Some(7)
    } else if s.starts_with("ago") {
        Some(8)
    } else if s.starts_with("set") {
        Some(9)
    } else if s.starts_with("ott") {
        Some(10)
    } else if s.starts_with("nov") {
        Some(11)
    } else if s.starts_with("dic") {
        Some(12)
    } else {
        None
    }
}

fn parse_it_grain(s: &str) -> Option<Grain> {
    let t = s.to_lowercase();
    if t.contains("second") {
        Some(Grain::Second)
    } else if t.contains("minut") {
        Some(Grain::Minute)
    } else if t.contains("or") {
        Some(Grain::Hour)
    } else if t.contains("giorn") {
        Some(Grain::Day)
    } else if t.contains("settiman") {
        Some(Grain::Week)
    } else if t.contains("mes") {
        Some(Grain::Month)
    } else if t.contains("ann") {
        Some(Grain::Year)
    } else {
        None
    }
}

fn parse_it_qty(s: &str) -> Option<i64> {
    match s {
        "un" | "uno" | "una" => Some(1),
        "due" => Some(2),
        "tre" => Some(3),
        "quattro" => Some(4),
        "cinque" => Some(5),
        "quindici" => Some(15),
        "ventiquattro" => Some(24),
        _ => s.parse().ok(),
    }
}

fn parse_it_day_token(s: &str) -> Option<u32> {
    match s {
        "uno" | "una" | "un" | "primo" => Some(1),
        "due" => Some(2),
        "tre" => Some(3),
        "quattro" => Some(4),
        "cinque" => Some(5),
        "sei" => Some(6),
        "sette" => Some(7),
        "otto" => Some(8),
        "nove" => Some(9),
        "dieci" => Some(10),
        "undici" => Some(11),
        "dodici" => Some(12),
        "tredici" => Some(13),
        "quattordici" => Some(14),
        "quindici" => Some(15),
        "sedici" => Some(16),
        "diciassette" => Some(17),
        "diciotto" => Some(18),
        "diciannove" => Some(19),
        "venti" => Some(20),
        _ => s.parse().ok(),
    }
}

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
            name: "<day> <month> (it)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*(gennaio|genn?\\.?|febbraio|febb?\\.?|marzo|mar\\.?|aprile|apr\\.?|maggio|magg?\\.?|giugno|giu\\.?|luglio|lug\\.?|agosto|ago\\.?|settembre|sett?\\.?|ottobre|ott\\.?|novembre|nov\\.?|dicembre|dic\\.?)")],
            production: Box::new(|nodes| {
                let (d, mname) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                let month = parse_it_month_token(&mname)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "dal <day> al <day> <month> (it)".to_string(),
            pattern: vec![regex("dal\\s+([[:alpha:]0-9]+)\\s+al\\s+([[:alpha:]0-9]+)\\s+(gennaio|genn?\\.?|febbraio|febb?\\.?|marzo|mar\\.?|aprile|apr\\.?|maggio|magg?\\.?|giugno|giu\\.?|luglio|lug\\.?|agosto|ago\\.?|settembre|sett?\\.?|ottobre|ott\\.?|novembre|nov\\.?|dicembre|dic\\.?)")],
            production: Box::new(|nodes| {
                let (d1_s, d2_s, m_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase(), rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let d1 = parse_it_day_token(&d1_s)?;
                let d2 = parse_it_day_token(&d2_s)?;
                if !(1..=31).contains(&d1) || !(1..=31).contains(&d2) {
                    return None;
                }
                let month = parse_it_month_token(&m_s)?;
                let from = TimeData::new(TimeForm::DateMDY { month, day: d1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: d2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "dal <day> al <day> (it)".to_string(),
            pattern: vec![regex("dal\\s+([[:alpha:]0-9]+)\\s+al\\s+([[:alpha:]0-9]+)")],
            production: Box::new(|nodes| {
                let (d1_s, d2_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let d1 = parse_it_day_token(&d1_s)?;
                let d2 = parse_it_day_token(&d2_s)?;
                if !(1..=31).contains(&d1) || !(1..=31).contains(&d2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DayOfMonth(d1));
                let to = TimeData::new(TimeForm::DayOfMonth(d2));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "dal <day> (it)".to_string(),
            pattern: vec![regex("dal\\s+([[:alpha:]0-9]+)")],
            production: Box::new(|nodes| {
                let d_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let day = parse_it_day_token(&d_s)?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::DayOfMonth(day));
                t.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(t))
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
        Rule {
            name: "questa settimana (it)".to_string(),
            pattern: vec![regex("questa settimana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "la settimana scorsa (it)".to_string(),
            pattern: vec![regex("la settimana scorsa")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "la scorsa settimana (it)".to_string(),
            pattern: vec![regex("la scorsa settimana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "la settimana prossima (it)".to_string(),
            pattern: vec![regex("la settimana prossima|settimana prossima")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "in/per la settimana (it)".to_string(),
            pattern: vec![regex("in settimana|per la settimana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::RestOfGrain(Grain::Week))))),
        },
        Rule {
            name: "gli ultimi/prossimi n cycle (it)".to_string(),
            pattern: vec![regex("(gli\\s+ultimi|le\\s+ultime|i\\s+prossimi|le\\s+prossime)\\s+(\\d+|un|uno|una|due|tre|ventiquattro)\\s+(secondi|minuti|ore|giorni|settimane|mesi|anni)")],
            production: Box::new(|nodes| {
                let (dir, n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?, rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_it_qty(n_s)?;
                let grain = parse_it_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: dir.contains("ultim"),
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "tra/fra/in/entro <duration> (it)".to_string(),
            pattern: vec![regex("(tra|fra|in|entro)\\s+(\\d+|un|uno|una|due|tre|quattro|cinque|quindici|ventiquattro)\\s+(second[oi]|minut[oi]|or[ae]|giorn[oi]|settiman[ae]|mes[ei]|ann[oi])")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_it_qty(n_s)?;
                let grain = parse_it_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain })))
            }),
        },
        Rule {
            name: "<duration> fa (it)".to_string(),
            pattern: vec![regex("(\\d+|un|uno|una|due|tre|quattro|cinque|quindici|ventiquattro)\\s+(second[oi]|minut[oi]|or[ae]|giorn[oi]|settiman[ae]|mes[ei]|ann[oi])\\s+fa")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_it_qty(n_s)?;
                let grain = parse_it_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: -n, grain })))
            }),
        },
        Rule {
            name: "i/le n cycle passati/passate (it)".to_string(),
            pattern: vec![regex("(i|le)\\s+(\\d+|un|uno|una|due|tre|ventiquattro)\\s+(secondi|minuti|ore|giorni|settimane|mesi|anni)\\s+passat(i|e)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_it_qty(n_s)?;
                let grain = parse_it_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "le/i scorse/prossime n cycle (it)".to_string(),
            pattern: vec![regex("(le|i)\\s+(scors[ei]|prossim[ei])\\s+(\\d+|un|uno|una|due|tre|ventiquattro)\\s+(secondi|minuti|ore|giorni|settimane|mesi|anni)")],
            production: Box::new(|nodes| {
                let (dir_s, n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?.to_lowercase(), rm.group(3)?, rm.group(4)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_it_qty(n_s)?;
                let grain = parse_it_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: dir_s.starts_with("scors"),
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "le/i n cycle scorse/prossime (it)".to_string(),
            pattern: vec![regex("(le|i)\\s+(\\d+|un|uno|una|due|tre|ventiquattro)\\s+(secondi|minuti|ore|giorni|settimane|mesi|anni)\\s+(scors[ei]|prossim[ei])")],
            production: Box::new(|nodes| {
                let (n_s, g_s, dir_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(3)?.to_lowercase(), rm.group(4)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_it_qty(n_s)?;
                let grain = parse_it_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: dir_s.starts_with("scors"),
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "scorse/prossime n cycle (it)".to_string(),
            pattern: vec![regex("(scors[ei]|prossim[ei])\\s+(\\d+|un|uno|una|due|tre|ventiquattro)\\s+(secondi|minuti|ore|giorni|settimane|mesi|anni)")],
            production: Box::new(|nodes| {
                let (dir_s, n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?, rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_it_qty(n_s)?;
                let grain = parse_it_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: dir_s.starts_with("scors"),
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "le/i n ultime/prossime cycle (it)".to_string(),
            pattern: vec![regex("(le|i)\\s+(\\d+|un|uno|una|due|tre|ventiquattro)\\s+(ultim[ei]|prossim[ei])\\s+(secondi|minuti|ore|giorni|settimane|mesi|anni)")],
            production: Box::new(|nodes| {
                let (n_s, dir_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(3)?.to_lowercase(), rm.group(4)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_it_qty(n_s)?;
                let grain = parse_it_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: dir_s.starts_with("ultim"),
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "la prossima settimana (it)".to_string(),
            pattern: vec![regex("la prossima settimana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "prossima settimana (it)".to_string(),
            pattern: vec![regex("prossima settimana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "il mese scorso (it)".to_string(),
            pattern: vec![regex("il mese scorso")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "nel mese scorso (it)".to_string(),
            pattern: vec![regex("nel mese scorso")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "nel mese passato (it)".to_string(),
            pattern: vec![regex("nel mese passato")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "lo scorso mese (it)".to_string(),
            pattern: vec![regex("lo scorso mese")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "il mese prossimo (it)".to_string(),
            pattern: vec![regex("il mese prossimo")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "il prossimo mese (it)".to_string(),
            pattern: vec![regex("il prossimo mese")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "questo trimestre (it)".to_string(),
            pattern: vec![regex("questo trimestre")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 0 })))),
        },
        Rule {
            name: "il prossimo trimestre (it)".to_string(),
            pattern: vec![regex("il prossimo trimestre")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "nel prossimo trimestre (it)".to_string(),
            pattern: vec![regex("nel prossimo trimestre")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "terzo trimestre (it)".to_string(),
            pattern: vec![regex("terzo trimestre")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "quarto trimestre 2018 (it)".to_string(),
            pattern: vec![regex("quarto trimestre\\s*(\\d{4})")],
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
            name: "l'anno scorso (it)".to_string(),
            pattern: vec![regex("l'anno scorso|anno scorso")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "quest'anno (it)".to_string(),
            pattern: vec![regex("quest'anno|questo anno")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))),
        },
        Rule {
            name: "il prossimo anno (it)".to_string(),
            pattern: vec![regex("il prossimo anno")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "entro/fino alla fine del giorno/mese/anno (it)".to_string(),
            pattern: vec![regex("(?:entro\\s+la|fino\\s+alla)\\s+fine\\s+d(?:el\\s+|ell')(giorno|mese|anno)")],
            production: Box::new(|nodes| {
                let target = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let grain = if target == "giorno" {
                    Grain::Day
                } else if target == "mese" {
                    Grain::Month
                } else {
                    Grain::Year
                };
                let mut t = TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::AllGrain(grain)),
                });
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "la settimana del 6 ottobre (it)".to_string(),
            pattern: vec![regex("la settimana del\\s+(\\d{1,2})\\s+ott(obre)?")],
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
            name: "il we del 15 febbraio (it)".to_string(),
            pattern: vec![regex("il\\s+we\\s+del\\s+(\\d{1,2})\\s+febbraio")],
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
            name: "weekend (it)".to_string(),
            pattern: vec![regex("we|week[\\s-]?end|fine\\s*settimana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "this/next/last weekend (it)".to_string(),
            pattern: vec![regex("questo\\s+fine\\s*settimana|fine\\s*settimana\\s+prossimo|il\\s+prossimo\\s+fine\\s*settimana|lo\\s+scorso\\s+fine\\s*settimana|fine\\s*settimana\\s+scorso")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::Weekend);
                if s.contains("prossim") {
                    t.direction = Some(Direction::Future);
                } else if s.contains("scors") {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "season (it)".to_string(),
            pattern: vec![regex("primavera|estate|autunno|inverno")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let season = if s.contains("primavera") {
                    0
                } else if s.contains("estate") {
                    1
                } else if s.contains("autunno") {
                    2
                } else {
                    3
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "this/next/last season (it)".to_string(),
            pattern: vec![regex("quest['oa]?\\s*(primavera|estate|autunno|inverno)|prossim[oa]\\s+(primavera|estate|autunno|inverno)|scors[oa]\\s+(primavera|estate|autunno|inverno)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let season = if s.contains("primavera") {
                    0
                } else if s.contains("estate") {
                    1
                } else if s.contains("autunno") {
                    2
                } else {
                    3
                };
                let mut t = TimeData::new(TimeForm::Season(season));
                if s.contains("prossim") {
                    t.direction = Some(Direction::Future);
                } else if s.contains("scors") {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "il mese dopo natale 2015 (it)".to_string(),
            pattern: vec![regex("il mese dopo natale\\s+2015")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 1, day: 1, year: Some(2016) })))),
        },
        Rule {
            name: "holidays common (it)".to_string(),
            pattern: vec![regex("natale|vigilia\\s+di\\s+natale|alla\\s+vigilia|la\\s+vigilia|vigilia\\s+di\\s+capodanno|notte\\s+di\\s+san\\s+silvestro|san\\s+silvestro|capodanno|primo\\s+dell[' ]anno|san\\s+valentino|festa\\s+degli\\s+innamorati|festa\\s+del\\s+pap[àa]|festa\\s+di\\s+san\\s+giuseppe|san\\s+giuseppe|festa\\s+della\\s+mamma|ferragosto|assunzione|ognissanti|tutti\\s+i\\s+santi|festa\\s+dei\\s+santi|santo\\s+stefano|epifania|befana|festa\\s+della\\s+liberazione|anniversario\\s+della\\s+liberazione|la\\s+liberazione|liberazione|festa\\s+della\\s+repubblica|anniversario\\s+della\\s+repubblica|la\\s+repubblica|repubblica|festa\\s+del\\s+lavoro|halloween|commemorazione\\s+dei\\s+defunti|immacolata\\s+concezione|immacolata|ai\\s+morti|giorno\\s+dei\\s+morti")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let form = if s.contains("vigilia di natale") {
                    TimeForm::Holiday("christmas eve".to_string(), None)
                } else if s.contains("alla vigilia") || s.contains("la vigilia") || s.contains("vigilia di capodanno") || s.contains("san silvestro") {
                    TimeForm::Holiday("new year's eve".to_string(), None)
                } else if s == "natale" {
                    TimeForm::Holiday("christmas day".to_string(), None)
                } else if s.contains("capodanno") || s.contains("primo dell'anno") || s.contains("primo dell anno") {
                    TimeForm::Holiday("new year's day".to_string(), None)
                } else if s.contains("valentino") || s.contains("innamorati") {
                    TimeForm::Holiday("valentine's day".to_string(), None)
                } else if s.contains("pap") {
                    TimeForm::Holiday("father's day".to_string(), None)
                } else if s.contains("mamma") {
                    TimeForm::Holiday("mother's day".to_string(), None)
                } else if s.contains("ferragosto") || s.contains("assunzione") {
                    TimeForm::Holiday("ferragosto".to_string(), None)
                } else if s.contains("ognissanti") || s.contains("tutti i santi") || s.contains("festa dei santi") {
                    TimeForm::Holiday("all saints' day".to_string(), None)
                } else if s.contains("santo stefano") {
                    TimeForm::Holiday("st. stephen's day".to_string(), None)
                } else if s.contains("epifania") || s.contains("befana") {
                    TimeForm::Holiday("epiphany".to_string(), None)
                } else if s.contains("liberazione") {
                    TimeForm::Holiday("liberation day".to_string(), None)
                } else if s.contains("repubblica") {
                    TimeForm::Holiday("republic day".to_string(), None)
                } else if s.contains("lavoro") {
                    TimeForm::Holiday("labour day".to_string(), None)
                } else if s.contains("halloween") {
                    TimeForm::Holiday("halloween day".to_string(), None)
                } else if s.contains("commemorazione") || s.contains("morti") {
                    TimeForm::Holiday("all souls' day".to_string(), None)
                } else {
                    TimeForm::Holiday("immaculate conception".to_string(), None)
                };
                Some(TokenData::Time(TimeData::new(form)))
            }),
        },
        Rule {
            name: "ai morti alle <hour> (it)".to_string(),
            pattern: vec![regex("ai\\s+morti\\s+alle\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                let holiday = TimeData::new(TimeForm::Holiday("all souls' day".to_string(), None));
                let time = TimeData::new(TimeForm::HourMinute(hour, 0, true));
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(holiday),
                    Box::new(time),
                ))))
            }),
        },
        Rule {
            name: "per le 15 (it)".to_string(),
            pattern: vec![regex("per le\\s*(\\d{1,2})")],
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
            name: "verso le 15 (it)".to_string(),
            pattern: vec![regex("verso le\\s*(\\d{1,2})")],
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
            name: "alle <hour> <timezone> (it)".to_string(),
            pattern: vec![regex("alle\\s*(\\d{1,2})(?::(\\d{2}))?\\s*(CET|CEST|UTC|GMT)")],
            production: Box::new(|nodes| {
                let (h, m_opt, tz) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2), rm.group(3)?.to_string()),
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let minute: u32 = m_opt.unwrap_or("0").parse().ok()?;
                if hour > 23 || minute > 59 {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::HourMinute(hour, minute, false));
                t.timezone = Some(tz);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "dopo le <hour> (it)".to_string(),
            pattern: vec![regex("dopo\\s+le\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
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
            name: "prima delle <hour> (it)".to_string(),
            pattern: vec![regex("prima\\s+delle\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::HourMinute(hour, 0, false));
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "tre e 20 (it)".to_string(),
            pattern: vec![regex("tre\\s+e\\s+20|3\\s+e\\s+20")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(3, 20, false))))),
        },
        Rule {
            name: "alle 3 20 (it)".to_string(),
            pattern: vec![regex("alle\\s*(\\d{1,2})\\s*(\\d{1,2})")],
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
            name: "part-of-day (it)".to_string(),
            pattern: vec![regex("mattino|pomeriggio|sera|serata|notte")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let (from_h, to_h) = if s.contains("mattin") {
                    (4, 12)
                } else if s.contains("pomeriggio") {
                    (12, 19)
                } else if s.contains("sera") || s.contains("serata") {
                    (18, 24)
                } else {
                    (0, 4)
                };
                let from = TimeData::new(TimeForm::Hour(from_h, false));
                let to = TimeData::new(TimeForm::Hour(if to_h == 24 { 0 } else { to_h }, false));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    true,
                ))))
            }),
        },
        Rule {
            name: "stanotte / nottata (it)".to_string(),
            pattern: vec![regex("stanotte|nella\\s+notte|in\\s+nottata")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::PartOfDay(PartOfDay::Night));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "this/next/last part-of-day (it)".to_string(),
            pattern: vec![regex("quest['oa]\\s+(mattina|mattino|pomeriggio|sera|serata|notte)|domani\\s+(mattina|pomeriggio|sera|notte)|ieri\\s+(sera|notte)|stamattina|stasera")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let pod = if s.contains("mattin") || s.contains("stamattina") {
                    PartOfDay::Morning
                } else if s.contains("pomeriggio") {
                    PartOfDay::Afternoon
                } else if s.contains("sera") || s.contains("stasera") {
                    PartOfDay::Evening
                } else {
                    PartOfDay::Night
                };
                let mut t = TimeData::new(TimeForm::PartOfDay(pod));
                if s.starts_with("domani") {
                    t.direction = Some(Direction::Future);
                } else if s.starts_with("ieri") {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<time> al/di <part-of-day> (it)".to_string(),
            pattern: vec![regex("(.+)\\s+(al|alla|del|della|di)\\s+(mattino|pomeriggio|sera|serata|notte)")],
            production: Box::new(|nodes| {
                let (lhs, pod) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let time = if let Some(h) = lhs.trim().strip_prefix("alle ").or_else(|| lhs.trim().strip_prefix("le ")) {
                    let hour = parse_it_hour_token(h.trim())?;
                    TimeData::new(TimeForm::HourMinute(hour, 0, true))
                } else if lhs.trim().chars().all(|c| c.is_ascii_digit()) {
                    let hour: u32 = lhs.trim().parse().ok()?;
                    TimeData::new(TimeForm::HourMinute(hour, 0, true))
                } else {
                    return None;
                };
                let part = if pod.contains("mattin") {
                    TimeData::new(TimeForm::Interval(Box::new(TimeData::new(TimeForm::Hour(4, false))), Box::new(TimeData::new(TimeForm::Hour(12, false))), true))
                } else if pod.contains("pomeriggio") {
                    TimeData::new(TimeForm::Interval(Box::new(TimeData::new(TimeForm::Hour(12, false))), Box::new(TimeData::new(TimeForm::Hour(19, false))), true))
                } else if pod.contains("sera") {
                    TimeData::new(TimeForm::Interval(Box::new(TimeData::new(TimeForm::Hour(18, false))), Box::new(TimeData::new(TimeForm::Hour(0, false))), true))
                } else {
                    TimeData::new(TimeForm::Interval(Box::new(TimeData::new(TimeForm::Hour(0, false))), Box::new(TimeData::new(TimeForm::Hour(4, false))), true))
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(time),
                    Box::new(part),
                ))))
            }),
        },
        Rule {
            name: "<hour> del mattino/pomeriggio/sera/notte (it)".to_string(),
            pattern: vec![regex("(alle|le)?\\s*(\\d{1,2}|una|un|uno|due|tre|quattro|cinque|sei|sette|otto|nove|dieci|undici|dodici)\\s+d(i|el(la)?)\\s+(mattino|pomeriggio|(sta)?sera|notte)")],
            production: Box::new(|nodes| {
                let (h_raw, pod) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?.to_lowercase(), rm.group(5)?.to_lowercase()),
                    _ => return None,
                };
                let mut hour = parse_it_hour_token(&h_raw)?;
                if pod.contains("pomeriggio") || pod.contains("sera") {
                    if hour < 12 {
                        hour += 12;
                    }
                } else if pod.contains("mattino") {
                    if hour == 12 {
                        hour = 0;
                    }
                } else if pod.contains("notte") {
                    if (6..12).contains(&hour) {
                        hour += 12;
                    } else if hour == 12 {
                        hour = 0;
                    }
                }
                if hour > 23 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
    ]);
    rules
}
