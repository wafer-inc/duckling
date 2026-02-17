use super::{Direction, IntervalDirection, PartOfDay, TimeData, TimeForm};
use crate::dimensions::numeral::helpers::{is_natural, numeral_data};
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

fn time_data(td: &TokenData) -> Option<&TimeData> {
    match td {
        TokenData::Time(d) => Some(d),
        _ => None,
    }
}

fn is_not_latent_time(td: &TokenData) -> bool {
    matches!(td, TokenData::Time(t) if !t.latent)
}

fn is_time_of_day(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Time(TimeData {
            form: TimeForm::Hour(..) | TimeForm::HourMinute(..) | TimeForm::HourMinuteSecond(..),
            ..
        })
    )
}

fn is_part_of_day(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Time(TimeData {
            form: TimeForm::PartOfDay(..),
            ..
        })
    )
}

fn is_day_of_week(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Time(TimeData {
            form: TimeForm::DayOfWeek(..),
            ..
        })
    )
}

fn parse_tr_number_word(s: &str) -> Option<u32> {
    let t = s.trim().to_lowercase();
    let t = t
        .replace("ü", "u")
        .replace("ö", "o")
        .replace("ı", "i")
        .replace("ş", "s")
        .replace("ç", "c")
        .replace("ğ", "g");
    let tokens: Vec<&str> = t.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }
    let unit = |w: &str| -> Option<u32> {
        match w {
            "sifir" => Some(0),
            "bir" => Some(1),
            "iki" => Some(2),
            "uc" => Some(3),
            "dort" => Some(4),
            "bes" => Some(5),
            "alti" => Some(6),
            "yedi" => Some(7),
            "sekiz" => Some(8),
            "dokuz" => Some(9),
            _ => None,
        }
    };
    let tens = |w: &str| -> Option<u32> {
        match w {
            "on" => Some(10),
            "yirmi" => Some(20),
            "otuz" => Some(30),
            "kirk" => Some(40),
            "elli" => Some(50),
            _ => None,
        }
    };
    if tokens.len() == 1 {
        return unit(tokens[0]).or_else(|| tens(tokens[0]));
    }
    if tokens.len() == 2 {
        if let Some(tn) = tens(tokens[0]) {
            return tn.checked_add(unit(tokens[1]).unwrap_or(0));
        }
        if tokens[0] == "on" {
            return unit(tokens[1])?.checked_add(10);
        }
    }
    None
}

fn tr_month_num(s: &str) -> Option<u32> {
    let t = s
        .trim()
        .to_lowercase()
        .replace("ı", "i")
        .replace("ş", "s")
        .replace("ç", "c")
        .replace("ğ", "g")
        .replace("ö", "o")
        .replace("ü", "u")
        .replace(".", "");
    if t.starts_with("ocak") {
        Some(1)
    } else if t.starts_with("sub") {
        Some(2)
    } else if t.starts_with("mart") {
        Some(3)
    } else if t.starts_with("nisan") {
        Some(4)
    } else if t.starts_with("may") {
        Some(5)
    } else if t.starts_with("haz") {
        Some(6)
    } else if t.starts_with("tem") {
        Some(7)
    } else if t.starts_with("agu") {
        Some(8)
    } else if t.starts_with("eyl") {
        Some(9)
    } else if t.starts_with("ekim") {
        Some(10)
    } else if t.starts_with("kas") {
        Some(11)
    } else if t.starts_with("ara") {
        Some(12)
    } else {
        None
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (tr)".to_string(),
            pattern: vec![regex("[şs]imdi|şu an|su an")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (tr)".to_string(),
            pattern: vec![regex("bug[üu]n")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (tr)".to_string(),
            pattern: vec![regex("yar[ıi]n|sonraki\\s+g[üu]n|gelecek\\s+g[üu]n")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (tr)".to_string(),
            pattern: vec![regex("d[üu]n")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day before yesterday (tr)".to_string(),
            pattern: vec![regex("d[üu]n\\s+de[ğg]il\\s+evvelsi\\s+g[üu]n|d[üu]nden\\s+[öo]nceki\\s+g[üu]n|[öo]b[üu]rki\\s+g[üu]n|[öo]b[üu]rs[üu]\\s+g[üu]n")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))),
        },
        Rule {
            name: "after tomorrow (tr)".to_string(),
            pattern: vec![regex("(yar[ıi]ndan\\s+sonraki)\\s*(g[üu]n)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))),
        },
        Rule {
            name: "day of week monday (tr)".to_string(),
            pattern: vec![regex("pazartesi'?(si|den|ye)?|pzts?|salı?'?(sı|dan|ya)?|çar(şamba)?'?(sı|dan|ya)?|per(şembe)?'?(si|den|ye)?|cum|cuma?'?(sı|dan|ya)?|cumartesi'?(si|den|ye)?|cmt|paz(ar)?'?(ı|dan|a)?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("paz") || s.starts_with("pzt") {
                    0
                } else if s.starts_with("sal") {
                    1
                } else if s.contains("çarşamba") || s.contains("carsamba") {
                    2
                } else if s.contains("perşembe") || s.contains("persembe") || s.starts_with("per") {
                    3
                } else if s.starts_with("cumartesi") || s == "cmt" {
                    5
                } else if s.starts_with("cuma") || s == "cum" {
                    4
                } else if s.starts_with("pazar") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "this|next <day-of-week> (tr)".to_string(),
            pattern: vec![regex("bu|sonraki"), predicate(is_day_of_week)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Future);
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "absorption of , after named day (tr)".to_string(),
            pattern: vec![predicate(is_day_of_week), regex(",")],
            production: Box::new(|nodes| Some(nodes[0].token_data.clone())),
        },
        Rule {
            name: "week-end (tr)".to_string(),
            pattern: vec![regex("hafta\\s+sonu")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "part of days (tr)".to_string(),
            pattern: vec![regex("sabah([ıia]ndan?)?|ö[ğg]len?|ak[şs]am(a|dan)?|gece(ye|den)?|ö[ğg]le\\s+yeme[ğg]i")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let pod = if s.starts_with("sabah") {
                    PartOfDay::Morning
                } else if s.starts_with("öğlen")
                    || s.starts_with("oglen")
                    || s.starts_with("öğle")
                    || s.starts_with("ogle")
                {
                    PartOfDay::Lunch
                } else if s.starts_with("akşam") || s.starts_with("aksam") {
                    PartOfDay::Evening
                } else if s.starts_with("gece") {
                    PartOfDay::Night
                } else {
                    PartOfDay::Afternoon
                };
                Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(pod))))
            }),
        },
        Rule {
            name: "noon (tr)".to_string(),
            pattern: vec![regex("ö[ğg]le(n|den|ye)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(12, 0, false))))),
        },
        Rule {
            name: "midnight|EOD|end of day (tr)".to_string(),
            pattern: vec![regex("gece\\s+yar[ıi]s[ıi]|g[üu]n\\s+sonu|g[üu]n\\s+bitimi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(0, 0, false))))),
        },
        Rule {
            name: "lunch (tr)".to_string(),
            pattern: vec![regex("ö[ğg]len?\\s+(yeme[ğg]i|aras[ıi])")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Lunch))))),
        },
        Rule {
            name: "afternoon (tr)".to_string(),
            pattern: vec![regex("ö[ğg]leden\\s+sonra")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))))),
        },
        Rule {
            name: "evening (tr)".to_string(),
            pattern: vec![regex("ak[şs]am(a|dan)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))))),
        },
        Rule {
            name: "night (tr)".to_string(),
            pattern: vec![regex("gece(ye|den)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Night))))),
        },
        Rule {
            name: "this <part-of-day> (tr)".to_string(),
            pattern: vec![regex("bu"), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let td = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(td),
                ))))
            }),
        },
        Rule {
            name: "<time> <part-of-day> (tr)".to_string(),
            pattern: vec![dim(DimensionKind::Time), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?.clone();
                let t2 = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t1),
                    Box::new(t2),
                ))))
            }),
        },
        Rule {
            name: "<part-of-day> <time> (tr)".to_string(),
            pattern: vec![predicate(is_part_of_day), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?.clone();
                let t2 = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t2),
                    Box::new(t1),
                ))))
            }),
        },
        Rule {
            name: "about|exactly <time-of-day> (tr)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("gibi|civar[ıi](nda)?")],
            production: Box::new(|nodes| match &nodes[0].token_data {
                TokenData::Time(td) => {
                    let mut t = td.clone();
                    t.latent = false;
                    Some(TokenData::Time(t))
                }
                _ => None,
            }),
        },
        Rule {
            name: "about|exactly <time-of-day> #2 (tr)".to_string(),
            pattern: vec![regex("yakla[şs][ıi]k|tam(\\s+olarak)?"), predicate(is_time_of_day)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Time(td) => {
                    let mut t = td.clone();
                    t.latent = false;
                    Some(TokenData::Time(t))
                }
                _ => None,
            }),
        },
        Rule {
            name: "<named-month> (tr)".to_string(),
            pattern: vec![regex("oca(?:k)?'?(ğın|tan|ğ?a)?|[şs]uba(?:t)?'?(a|ın|tan)?|mart?'?(ın|a|tan)?|nisa(?:n)?'?(ın|a|dan)?|may[ıi]s'?(ın|a|tan)?|hazi(?:ran)?'?(ın|a|dan)?|tem(?:muz)?'?(un|a|dan)?|a[ğg]u(?:stos)?'?(un|a|tan)?|eyl(?:[üu]l)?'?(ün|e|den)?|eki(?:m)?'?(in|den|e)?|kası(?:m)?'?(ın|dan|a)?|aralı(?:k)?'?(ğın|ğa|tan)?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let month = tr_month_num(s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "year (tr)".to_string(),
            pattern: vec![regex("(\\d{4})")],
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
            name: "ay sonu (tr)".to_string(),
            pattern: vec![regex("ay sonu|y[ıi]l sonu")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::AllGrain(Grain::Month)),
                })))
            }),
        },
        Rule {
            name: "gün sonuna kadar (tr)".to_string(),
            pattern: vec![regex("g[üu]n\\s+sonuna\\s+kadar")],
            production: Box::new(|_| {
                let mut td = TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::GrainOffset {
                        grain: Grain::Day,
                        offset: 0,
                    }),
                });
                td.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "ay sonuna kadar (tr)".to_string(),
            pattern: vec![regex("ay\\s+sonuna\\s+kadar")],
            production: Box::new(|_| {
                let mut td = TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::GrainOffset {
                        grain: Grain::Month,
                        offset: 0,
                    }),
                });
                td.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "ay sonunda (tr)".to_string(),
            pattern: vec![regex("ay\\s+sonunda")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::GrainOffset {
                        grain: Grain::Month,
                        offset: 0,
                    }),
                })))
            }),
        },
        Rule {
            name: "hafta boyunca (tr)".to_string(),
            pattern: vec![regex("((bu\\s+)?hafta\\s+boyunca)|(bu\\s+hafta)|(haftan[ıi]n\\s+geri\\s+kalan[ıi])")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("geri kalan") {
                    Some(TokenData::Time(TimeData::new(TimeForm::RestOfGrain(Grain::Week))))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))
                }
            }),
        },
        Rule {
            name: "çocuk bayramı (tr)".to_string(),
            pattern: vec![regex("(ulusal\\s+egemenlik\\s+ve\\s+)?çocuk\\s+bayram[ıi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "ulusal egemenlik ve çocuk bayramı".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "gençlik/spor bayramı (tr)".to_string(),
            pattern: vec![regex("(gençlik\\s+ve\\s+spor|gençlik|spor)\\s+bayram[ıi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "atatürk’ü anma, gençlik ve spor bayramı".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "zafer bayramı (tr)".to_string(),
            pattern: vec![regex("zafer\\s+bayram[ıi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "zafer bayramı".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "cumhuriyet bayramı (tr)".to_string(),
            pattern: vec![regex("cumhuriyet\\s+bayram[ıi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "cumhuriyet bayramı".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "emek ve dayanışma günü (tr)".to_string(),
            pattern: vec![regex("emek\\s+ve\\s+dayan[ıi][şs]ma\\s+g[üu]n[üu]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "emek ve dayanışma günü".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "ramazan bayramı (tr)".to_string(),
            pattern: vec![regex("ramazan\\s+bayram[ıi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "eid al-fitr".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "YYYY ramazan bayramı (tr)".to_string(),
            pattern: vec![regex("(\\d{4})\\s+ramazan\\s+bayram[ıi]|ramazan\\s+bayram[ıi]\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "eid al-fitr".to_string(),
                    Some(year),
                ))))
            }),
        },
        Rule {
            name: "kurban bayramı (tr)".to_string(),
            pattern: vec![regex("kurban\\s+bayram[ıi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "eid al-adha".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "YYYY kurban bayramı (tr)".to_string(),
            pattern: vec![regex("(\\d{4})\\s+kurban\\s+bayram[ıi]|kurban\\s+bayram[ıi]\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "eid al-adha".to_string(),
                    Some(year),
                ))))
            }),
        },
        Rule {
            name: "<month>'ın üçü (tr)".to_string(),
            pattern: vec![regex("([a-zA-ZçğıöşüÇĞİÖŞÜ]+)(?:'?[ıiuü]n)?\\s+[üu]([çc]|c)[üu]")],
            production: Box::new(|nodes| {
                let ms = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let month = tr_month_num(ms)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day: 3, year: None })))
            }),
        },
        Rule {
            name: "<month> ortası (tr)".to_string(),
            pattern: vec![regex("([a-zA-ZçğıöşüÇĞİÖŞÜ]+)(?:'?[ıiuü]n)?\\s+ortas[ıi]")],
            production: Box::new(|nodes| {
                let ms = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let month = tr_month_num(ms)?;
                let day = if matches!(month, 3 | 5 | 7 | 10) { 15 } else { 13 };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "<day> <month> (tr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+(oca(?:k)?|[şs]uba(?:t)?|mar(?:t)?|nisa(?:n)?|may[ıi]s|hazi(?:ran)?|tem(?:muz)?|a[ğg]u(?:stos)?|eyl(?:[üu]l)?|eki(?:m)?|kası(?:m)?|aralı(?:k)?)")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let ms = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let month = tr_month_num(ms)?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "<month> <day> (tr)".to_string(),
            pattern: vec![regex("(oca(?:k)?|[şs]uba(?:t)?|mar(?:t)?|nisa(?:n)?|may[ıi]s|hazi(?:ran)?|tem(?:muz)?|a[ğg]u(?:stos)?|eyl(?:[üu]l)?|eki(?:m)?|kası(?:m)?|aralı(?:k)?)\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (ms, ds) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let day: u32 = ds.parse().ok()?;
                let month = tr_month_num(ms)?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "mm.dd (tr)".to_string(),
            pattern: vec![regex("([012]?\\d|30|31)\\.(10|11|12|0?[1-9])\\.?")],
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
            name: "mm.dd.yyyy (tr)".to_string(),
            pattern: vec![regex("([012]?\\d|30|31)[./-](10|11|12|0?[1-9])[./-](\\d{2,4})")],
            production: Box::new(|nodes| {
                let (d, m, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let mut year: i32 = y.parse().ok()?;
                if y.len() == 2 {
                    year = year.checked_add(if year < 50 { 2000 } else { 1900 })?;
                }
                if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "yyyy-mm-dd (tr)".to_string(),
            pattern: vec![regex("(\\d{2,4})-(0?[1-9]|10|11|12)-([012]?[1-9]|10|20|30|31)")],
            production: Box::new(|nodes| {
                let (y, m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "dd.(mm.)? - dd.mm.(yy[yy]?)? (tr)".to_string(),
            pattern: vec![regex("(10|20|30|31|[012]?[1-9])(?:\\.(10|11|12|0?[1-9]))?\\.?\\s*(?:\\-|/)\\s*(10|20|30|31|[012]?[1-9])\\.(10|11|12|0?[1-9])\\.?(?:\\.(\\d{2,4}))?")],
            production: Box::new(|nodes| {
                let (d1, m1_opt, d2, m2, y_opt) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2), rm.group(3)?, rm.group(4)?, rm.group(5)),
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                let month2: u32 = m2.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) || !(1..=12).contains(&month2) {
                    return None;
                }
                let month1: u32 = if let Some(m1s) = m1_opt {
                    m1s.parse().ok()?
                } else {
                    month2
                };
                let start = if let Some(ys) = y_opt {
                    let mut year: i32 = ys.parse().ok()?;
                    if ys.len() == 2 {
                        year = year.checked_add(if year < 50 { 2000 } else { 1900 })?;
                    }
                    TimeData::new(TimeForm::DateMDY { month: month1, day: day1, year: Some(year) })
                } else {
                    TimeData::new(TimeForm::DateMDY { month: month1, day: day1, year: None })
                };
                let end = if let Some(ys) = y_opt {
                    let mut year: i32 = ys.parse().ok()?;
                    if ys.len() == 2 {
                        year = year.checked_add(if year < 50 { 2000 } else { 1900 })?;
                    }
                    TimeData::new(TimeForm::DateMDY { month: month2, day: day2, year: Some(year) })
                } else {
                    TimeData::new(TimeForm::DateMDY { month: month2, day: day2, year: None })
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(start), Box::new(end), true))))
            }),
        },
        Rule {
            name: "18 aralık(tan) (tr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+aral[ıi]k(tan)?")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 12, day, year: None })))
            }),
        },
        Rule {
            name: "saat <hour> (tr)".to_string(),
            pattern: vec![regex("saat\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
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
            name: "saat <hh:mm> (tr)".to_string(),
            pattern: vec![regex("saat\\s*((?:[01]?\\d)|(?:2[0-3]))[:.]([0-5]\\d)")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "saat <hour-word> <minute-word> (tr)".to_string(),
            pattern: vec![regex("saat\\s+([[:alpha:]çğıöşü]+)\\s+([[:alpha:]çğıöşü]+(?:\\s+[[:alpha:]çğıöşü]+)?)")],
            production: Box::new(|nodes| {
                let (h_s, m_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hour = parse_tr_number_word(h_s)?;
                let minute = parse_tr_number_word(m_s)?;
                if hour > 23 || minute > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "at <time-of-day> (tr)".to_string(),
            pattern: vec![regex("saat"), predicate(is_time_of_day)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Time(td) => {
                    let mut t = td.clone();
                    t.latent = false;
                    Some(TokenData::Time(t))
                }
                _ => None,
            }),
        },
        Rule {
            name: "at <time-of-day> #2 (tr)".to_string(),
            pattern: vec![regex("saat"), predicate(is_time_of_day), regex("'?(den|dan)?")],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Time(td) => {
                    let mut t = td.clone();
                    t.latent = false;
                    Some(TokenData::Time(t))
                }
                _ => None,
            }),
        },
        Rule {
            name: "between <time-of-day> and <time-of-day> (tr)".to_string(),
            pattern: vec![
                predicate(is_time_of_day),
                regex("ile"),
                predicate(is_time_of_day),
                regex("aras[ıi]"),
            ],
            production: Box::new(|nodes| {
                let from = time_data(&nodes[0].token_data)?.clone();
                let to = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    true,
                ))))
            }),
        },
        Rule {
            name: "<time-of-day> - <time-of-day> (interval) (tr)".to_string(),
            pattern: vec![predicate(is_not_latent_time), regex("\\-|/"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let from = time_data(&nodes[0].token_data)?.clone();
                let to = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    true,
                ))))
            }),
        },
        Rule {
            name: "<datetime> - <datetime> (interval) (tr)".to_string(),
            pattern: vec![predicate(is_not_latent_time), regex("\\-"), predicate(is_not_latent_time)],
            production: Box::new(|nodes| {
                let from = time_data(&nodes[0].token_data)?.clone();
                let to = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    true,
                ))))
            }),
        },
        Rule {
            name: "intersect by ',' (tr)".to_string(),
            pattern: vec![predicate(is_not_latent_time), regex(","), predicate(is_not_latent_time)],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?.clone();
                let t2 = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t1),
                    Box::new(t2),
                ))))
            }),
        },
        Rule {
            name: "<time> kadar (tr)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("kadar")],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[0].token_data)?.clone();
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<time> timezone (tr)".to_string(),
            pattern: vec![predicate(is_not_latent_time), regex("\\b(YEKT|YEKST|YAKT|YAKST|WITA|WIT|WIB|WGT|WGST|WFT|WET|WEST|WAT|WAST|VUT|VLAT|VLAST|VET|UZT|UYT|UYST|UTC|ULAT|TVT|TMT|TLT|TKT|TJT|TFT|TAHT|SST|SRT|SGT|SCT|SBT|SAST|SAMT|RET|PYT|PYST|PWT|PST|PONT|PMST|PMDT|PKT|PHT|PHOT|PGT|PETT|PETST|PET|PDT|OMST|OMSST|NZST|NZDT|NUT|NST|NPT|NOVT|NOVST|NFT|NDT|NCT|MYT|MVT|MUT|MST|MSK|MSD|MMT|MHT|MDT|MAWT|MART|MAGT|MAGST|LINT|LHST|LHDT|KUYT|KST|KRAT|KRAST|KGT|JST|IST|IRST|IRKT|IRKST|IRDT|IOT|IDT|ICT|HOVT|HKT|GYT|GST|GMT|GILT|GFT|GET|GAMT|GALT|FNT|FKT|FKST|FJT|FJST|EST|EGT|EGST|EET|EEST|EDT|ECT|EAT|EAST|EASST|DAVT|ChST|CXT|CVT|CST|COT|CLT|CLST|CKT|CHAST|CHADT|CET|CEST|CDT|CCT|CAT|CAST|BTT|BST|BRT|BRST|BOT|BNT|AZT|AZST|AZOT|AZOST|AWST|AWDT|AST|ART|AQTT|ANAT|ANAST|AMT|AMST|ALMT|AKST|AKDT|AFT|AEST|AEDT|ADT|ACST|ACDT)\\b")],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[0].token_data)?.clone();
                let tz = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_uppercase(),
                    _ => return None,
                };
                t.timezone = Some(tz);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "next <cycle> (tr)".to_string(),
            pattern: vec![
                regex("sonraki|[öo]n[üu]m[üu]zdeki|gelecek"),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: 1,
                })))
            }),
        },
        Rule {
            name: "last <cycle> (tr)".to_string(),
            pattern: vec![
                regex("son|ge[çc]en|ge[çc]ti[ğg]imiz|[öo]nceki"),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: -1,
                })))
            }),
        },
        Rule {
            name: "this <cycle> (tr)".to_string(),
            pattern: vec![regex("bu"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: 0,
                })))
            }),
        },
        Rule {
            name: "next n <cycle> (tr)".to_string(),
            pattern: vec![
                regex("[öo]n[üu]m[üu]zdeki|sonraki"),
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?.value as i64;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "last n <cycle> (tr)".to_string(),
            pattern: vec![
                regex("son|ge[çc]en|ge[çc]ti[ğg]imiz|[öo]nceki"),
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?.value as i64;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "last <cycle> of <time> (tr)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("son"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let base = time_data(&nodes[0].token_data)?.clone();
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::LastCycleOfTime {
                    grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "last <day-of-week> of <time> (tr)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("son"), predicate(is_day_of_week)],
            production: Box::new(|nodes| {
                let base = time_data(&nodes[0].token_data)?.clone();
                let dow = match &nodes[2].token_data {
                    TokenData::Time(TimeData {
                        form: TimeForm::DayOfWeek(d),
                        ..
                    }) => *d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::LastDOWOfTime {
                    dow,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "next <time> (tr)".to_string(),
            pattern: vec![
                regex("[öo]n[üu]m[üu]zdeki|gelecek|sonraki"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let mut td = match &nodes[1].token_data {
                    TokenData::Time(t) => t.clone(),
                    _ => return None,
                };
                td.direction = Some(Direction::Future);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "last <time> (tr)".to_string(),
            pattern: vec![
                regex("ge[çc]en|[öo]nceki|ge[çc]ti[ğg]imiz|son"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let mut td = match &nodes[1].token_data {
                    TokenData::Time(t) => t.clone(),
                    _ => return None,
                };
                td.direction = Some(Direction::Past);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "this <time> (tr)".to_string(),
            pattern: vec![regex("bu"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Time(t) => Some(TokenData::Time(t.clone())),
                _ => None,
            }),
        },
        Rule {
            name: "üçüncü çeyrek yıl (tr)".to_string(),
            pattern: vec![regex("üçüncü\\s+çeyrek\\s+y[ıi]l|ucuncu\\s+ceyrek\\s+yil")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "yılbaşı (tr)".to_string(),
            pattern: vec![regex("y[ıi]lba[şs][ıi](ndan|na)?|y[ıi]lba[şs][ıi]\\s+tatili(nden|ne)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 1, day: 1, year: None })))),
        },
        Rule {
            name: "this/next/last season (tr)".to_string(),
            pattern: vec![regex("(bu|sonraki|önümüzdeki|gelecek|geçen|önceki)\\s+(ilkbahar|yaz|sonbahar|k[ıi]ş)")],
            production: Box::new(|nodes| {
                let (q, s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.to_lowercase(), m.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let season = if s.contains("ilkbahar") {
                    0
                } else if s.contains("yaz") {
                    1
                } else if s.contains("sonbahar") {
                    2
                } else {
                    3
                };
                let mut t = TimeData::new(TimeForm::Season(season));
                if q == "sonraki" || q == "önümüzdeki" || q == "gelecek" {
                    t.direction = Some(Direction::Future);
                } else if q == "geçen" || q == "önceki" {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "bu/yarın/dün akşam (tr)".to_string(),
            pattern: vec![regex("(bu|yar[ıi]n|d[üu]n)\\s+ak[şs]am")],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let day = if q == "yarın" || q == "yarin" {
                    TimeData::new(TimeForm::Tomorrow)
                } else if q == "dün" || q == "dun" {
                    TimeData::new(TimeForm::Yesterday)
                } else {
                    TimeData::new(TimeForm::Today)
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(day),
                    Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
                ))))
            }),
        },
        Rule {
            name: "15 şubat sabahı (tr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[şs]ubat\\s+sabah[ıi]")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 2, day, year: None })),
                    Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))),
                ))))
            }),
        },
        Rule {
            name: "<duration> içinde/içerisinde (tr)".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex("i[çc]inde|i[çc]erisinde")],
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
            name: "<time> sonra <duration> (tr)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("sonra(ki)?"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let (base, d) = match (&nodes[0].token_data, &nodes[2].token_data) {
                    (TokenData::Time(t), TokenData::Duration(d)) => (t.clone(), d),
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n: d.value,
                    grain: d.grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "<time> <duration> sonra (tr)".to_string(),
            pattern: vec![dim(DimensionKind::Time), dim(DimensionKind::Duration), regex("sonra")],
            production: Box::new(|nodes| {
                let (base, d) = match (&nodes[0].token_data, &nodes[1].token_data) {
                    (TokenData::Time(t), TokenData::Duration(d)) => (t.clone(), d),
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n: d.value,
                    grain: d.grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "<time> itibaren <duration> boyunca (tr)".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex("itibaren"),
                dim(DimensionKind::Duration),
                regex("boyunca|s[üu]resince"),
            ],
            production: Box::new(|nodes| {
                let (start, d) = match (&nodes[0].token_data, &nodes[2].token_data) {
                    (TokenData::Time(t), TokenData::Duration(d)) => (t.clone(), d),
                    _ => return None,
                };
                let end = TimeData::new(TimeForm::DurationAfter {
                    n: d.value,
                    grain: d.grain,
                    base: Box::new(start.clone()),
                });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(start),
                    Box::new(end),
                    false,
                ))))
            }),
        },
        Rule {
            name: "18 aralıktan itibaren 10 gün boyunca (tr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+aral[ıi]ktan\\s+itibaren\\s+(\\d{1,2})\\s+g[üu]n\\s+(boyunca|s[üu]resince)")],
            production: Box::new(|nodes| {
                let (d, n) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let days: i64 = n.parse().ok()?;
                if !(1..=31).contains(&day) || days < 1 {
                    return None;
                }
                let start = TimeData::new(TimeForm::DateMDY { month: 12, day, year: None });
                let end = TimeData::new(TimeForm::DurationAfter {
                    n: days,
                    grain: Grain::Day,
                    base: Box::new(start.clone()),
                });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(start),
                    Box::new(end),
                    false,
                ))))
            }),
        },
        Rule {
            name: "<duration> önce (tr)".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex("[öo]nce")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: d.value.checked_neg()?,
                    grain: d.grain,
                })))
            }),
        },
        Rule {
            name: "<duration> sonra (tr)".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex("sonra")],
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
            name: "sonra(ki)? <duration> (tr)".to_string(),
            pattern: vec![regex("(bug[üu]nden\\s+)?sonra(ki)?"), dim(DimensionKind::Duration)],
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
            name: "8 Ağu - 12 Ağu (tr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Aa][ğg][uü]\\s*-\\s*(\\d{1,2})\\s+[Aa][ğg][uü]")],
            production: Box::new(|nodes| {
                let (d1, d2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let start = TimeData::new(TimeForm::DateMDY {
                    month: 8,
                    day: day1,
                    year: None,
                });
                let end = TimeData::new(TimeForm::DateMDY {
                    month: 8,
                    day: day2,
                    year: None,
                });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(start),
                    Box::new(end),
                    true,
                ))))
            }),
        },
    ]);
    rules
}
