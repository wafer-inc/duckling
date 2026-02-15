use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};
use super::{Direction, IntervalDirection, PartOfDay, TimeData, TimeForm};

fn parse_tr_number_word(s: &str) -> Option<u32> {
    let t = s.trim().to_lowercase();
    let t = t.replace("ü", "u").replace("ö", "o").replace("ı", "i").replace("ş", "s").replace("ç", "c").replace("ğ", "g");
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
            return Some(tn + unit(tokens[1]).unwrap_or(0));
        }
        if tokens[0] == "on" {
            return Some(10 + unit(tokens[1])?);
        }
    }
    None
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
            pattern: vec![regex("hafta\\s+boyunca")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "çocuk bayramı (tr)".to_string(),
            pattern: vec![regex("(ulusal\\s+egemenlik\\s+ve\\s+)?çocuk\\s+bayram[ıi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 4,
                day: 23,
                year: None,
            })))),
        },
        Rule {
            name: "gençlik/spor bayramı (tr)".to_string(),
            pattern: vec![regex("(gençlik\\s+ve\\s+spor|gençlik|spor)\\s+bayram[ıi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 5,
                day: 19,
                year: None,
            })))),
        },
        Rule {
            name: "zafer bayramı (tr)".to_string(),
            pattern: vec![regex("zafer\\s+bayram[ıi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 8,
                day: 30,
                year: None,
            })))),
        },
        Rule {
            name: "cumhuriyet bayramı (tr)".to_string(),
            pattern: vec![regex("cumhuriyet\\s+bayram[ıi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 10,
                day: 29,
                year: None,
            })))),
        },
        Rule {
            name: "emek ve dayanışma günü (tr)".to_string(),
            pattern: vec![regex("emek\\s+ve\\s+dayan[ıi][şs]ma\\s+g[üu]n[üu]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 5,
                day: 1,
                year: None,
            })))),
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
            name: "martın üçü (tr)".to_string(),
            pattern: vec![regex("mart[ıi]n\\s+[üu]([çc]|c)[üu]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 3, year: None })))),
        },
        Rule {
            name: "3 mart (tr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+mart")],
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
            name: "martın ortası (tr)".to_string(),
            pattern: vec![regex("mart[ıi]n\\s+ortas[ıi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 15, year: None })))),
        },
        Rule {
            name: "şubatın ortası (tr)".to_string(),
            pattern: vec![regex("[şs]ubat[ıi]n\\s+ortas[ıi]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 15, year: None })))),
        },
        Rule {
            name: "Ağu 8 (tr)".to_string(),
            pattern: vec![regex("ağu\\s*(\\d{1,2})|agu\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
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
            name: "Ağustos 8 (tr)".to_string(),
            pattern: vec![regex("ağustos\\s*(\\d{1,2})|agustos\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1).or_else(|| m.group(2))?,
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
            pattern: vec![regex("saat\\s+([[:alpha:]çğıöşü]+(?:\\s+[[:alpha:]çğıöşü]+)?)\\s+([[:alpha:]çğıöşü]+(?:\\s+[[:alpha:]çğıöşü]+)?)")],
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
            name: "saat dokuz elli dokuz (tr)".to_string(),
            pattern: vec![regex("saat\\s+dokuz\\s+elli\\s+dokuz")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(9, 59, false))))),
        },
        Rule {
            name: "önümüzdeki mart (tr)".to_string(),
            pattern: vec![regex("önümüzdeki\\s+mart|onumuzdeki\\s+mart")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::Month(3));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "sonraki mart (tr)".to_string(),
            pattern: vec![regex("sonraki\\s+mart")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::Month(3));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "sonraki ay (tr)".to_string(),
            pattern: vec![regex("sonraki\\s+ay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "önümüzdeki ay (tr)".to_string(),
            pattern: vec![regex("önümüzdeki\\s+ay|onumuzdeki\\s+ay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "bu hafta (tr)".to_string(),
            pattern: vec![regex("bu hafta")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "önümüzdeki hafta (tr)".to_string(),
            pattern: vec![regex("önümüzdeki\\s+hafta|onumuzdeki\\s+hafta")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "sonraki hafta (tr)".to_string(),
            pattern: vec![regex("sonraki\\s+hafta")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "gelecek hafta (tr)".to_string(),
            pattern: vec![regex("gelecek\\s+hafta")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "geçen ay (tr)".to_string(),
            pattern: vec![regex("geçen\\s+ay|gecen\\s+ay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "geçen hafta (tr)".to_string(),
            pattern: vec![regex("geçen\\s+hafta|gecen\\s+hafta")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "geçtiğimiz ay (tr)".to_string(),
            pattern: vec![regex("geçtiğimiz\\s+ay|gectigimiz\\s+ay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "geçtiğimiz hafta (tr)".to_string(),
            pattern: vec![regex("geçtiğimiz\\s+hafta|gectigimiz\\s+hafta")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "önceki hafta (tr)".to_string(),
            pattern: vec![regex("önceki\\s+hafta|onceki\\s+hafta")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "önceki ay (tr)".to_string(),
            pattern: vec![regex("önceki\\s+ay|onceki\\s+ay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "gelecek ay (tr)".to_string(),
            pattern: vec![regex("gelecek\\s+ay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "bu çeyrek yıl (tr)".to_string(),
            pattern: vec![regex("bu\\s+çeyrek\\s+y[ıi]l|bu\\s+ceyrek\\s+yil")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 0 })))),
        },
        Rule {
            name: "sonraki çeyrek yıl (tr)".to_string(),
            pattern: vec![regex("sonraki\\s+çeyrek\\s+y[ıi]l|sonraki\\s+ceyrek\\s+yil")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "üçüncü çeyrek yıl (tr)".to_string(),
            pattern: vec![regex("üçüncü\\s+çeyrek\\s+y[ıi]l|ucuncu\\s+ceyrek\\s+yil")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "geçen yıl (tr)".to_string(),
            pattern: vec![regex("geçen\\s+y[ıi]l|gecen\\s+yil")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "bu yıl (tr)".to_string(),
            pattern: vec![regex("bu\\s+y[ıi]l")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))),
        },
        Rule {
            name: "sonraki/gelecek yıl (tr)".to_string(),
            pattern: vec![regex("sonraki\\s+y[ıi]l|gelecek\\s+y[ıi]l")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
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
                    n: -d.value,
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
            name: "son 2 saniye (tr)".to_string(),
            pattern: vec![regex("son\\s*(\\d{1,2})\\s*saniye")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let secs: i64 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: secs,
                    grain: Grain::Second,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "son iki saniye (tr)".to_string(),
            pattern: vec![regex("son\\s+(bir|iki|[üu]ç|d[öo]rt|be[şs]|alt[ıi]|yedi|sekiz|dokuz|on)\\s+saniye")],
            production: Box::new(|nodes| {
                let w = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let n = parse_tr_number_word(w)? as i64;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain: Grain::Second,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "sonraki 3 saniye (tr)".to_string(),
            pattern: vec![regex("sonraki\\s*(\\d{1,2})\\s*saniye")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let secs: i64 = n.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: secs,
                    grain: Grain::Second,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "sonraki üç saniye (tr)".to_string(),
            pattern: vec![regex("sonraki\\s+(bir|iki|[üu]ç|d[öo]rt|be[şs]|alt[ıi]|yedi|sekiz|dokuz|on)\\s+saniye")],
            production: Box::new(|nodes| {
                let w = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let n = parse_tr_number_word(w)? as i64;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain: Grain::Second,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "son 2 dakika/saat/gün/hafta/ay/yıl (tr)".to_string(),
            pattern: vec![regex("son\\s*(\\d{1,2})\\s*(dakika|saat|g[üu]n|hafta|ay|y[ıi]l)")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let v: i64 = n.parse().ok()?;
                let grain = match unit.as_str() {
                    "dakika" => Grain::Minute,
                    "saat" => Grain::Hour,
                    "gün" | "gun" => Grain::Day,
                    "hafta" => Grain::Week,
                    "ay" => Grain::Month,
                    "yıl" | "yil" => Grain::Year,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: v,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "son iki dakika/saat/gün/hafta/ay/yıl (tr)".to_string(),
            pattern: vec![regex("son\\s+(bir|iki|[üu]ç|d[öo]rt|be[şs]|alt[ıi]|yedi|sekiz|dokuz|on)\\s*(dakika|saat|g[üu]n|hafta|ay|y[ıi]l)")],
            production: Box::new(|nodes| {
                let (w, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let v = parse_tr_number_word(w)? as i64;
                let grain = match unit.as_str() {
                    "dakika" => Grain::Minute,
                    "saat" => Grain::Hour,
                    "gün" | "gun" => Grain::Day,
                    "hafta" => Grain::Week,
                    "ay" => Grain::Month,
                    "yıl" | "yil" => Grain::Year,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: v,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "sonraki 3 dakika/saat/gün/hafta/ay/yıl (tr)".to_string(),
            pattern: vec![regex("sonraki\\s*(\\d{1,2})\\s*(dakika|saat|g[üu]n|hafta|ay|y[ıi]l)")],
            production: Box::new(|nodes| {
                let (n, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let v: i64 = n.parse().ok()?;
                let grain = match unit.as_str() {
                    "dakika" => Grain::Minute,
                    "saat" => Grain::Hour,
                    "gün" | "gun" => Grain::Day,
                    "hafta" => Grain::Week,
                    "ay" => Grain::Month,
                    "yıl" | "yil" => Grain::Year,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: v,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "sonraki üç dakika/saat/gün/hafta/ay/yıl (tr)".to_string(),
            pattern: vec![regex("sonraki\\s+(bir|iki|[üu]ç|d[öo]rt|be[şs]|alt[ıi]|yedi|sekiz|dokuz|on)\\s*(dakika|saat|g[üu]n|hafta|ay|y[ıi]l)")],
            production: Box::new(|nodes| {
                let (w, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let v = parse_tr_number_word(w)? as i64;
                let grain = match unit.as_str() {
                    "dakika" => Grain::Minute,
                    "saat" => Grain::Hour,
                    "gün" | "gun" => Grain::Day,
                    "hafta" => Grain::Week,
                    "ay" => Grain::Month,
                    "yıl" | "yil" => Grain::Year,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: v,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "geçtiğimiz iki hafta (tr)".to_string(),
            pattern: vec![regex("(ge[çc]ti[ğg]imiz|[öo]nceki)\\s+(bir|iki|[üu]ç|d[öo]rt|be[şs]|alt[ıi]|yedi|sekiz|dokuz|on)\\s+hafta")],
            production: Box::new(|nodes| {
                let w = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(2)?,
                    _ => return None,
                };
                let v = parse_tr_number_word(w)? as i64;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: v,
                    grain: Grain::Week,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "önümüzdeki üç hafta (tr)".to_string(),
            pattern: vec![regex("[öo]n[üu]m[üu]zdeki\\s+(\\d{1,2}|bir|iki|[üu]ç|d[öo]rt|be[şs]|alt[ıi]|yedi|sekiz|dokuz|on)\\s+hafta")],
            production: Box::new(|nodes| {
                let v: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => {
                        if let Some(n) = rm.group(1) {
                            if let Ok(k) = n.parse::<i64>() {
                                k
                            } else {
                                parse_tr_number_word(n)? as i64
                            }
                        } else {
                            return None;
                        }
                    }
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: v,
                    grain: Grain::Week,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "önümüzdeki üç yıl (tr)".to_string(),
            pattern: vec![regex("[öo]n[üu]m[üu]zdeki\\s+(\\d{1,2}|bir|iki|[üu]ç|d[öo]rt|be[şs]|alt[ıi]|yedi|sekiz|dokuz|on)\\s*(hafta|y[ıi]l)")],
            production: Box::new(|nodes| {
                let (n_raw, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let v: i64 = if let Ok(k) = n_raw.parse::<i64>() {
                    k
                } else {
                    parse_tr_number_word(n_raw)? as i64
                };
                let grain = if unit == "hafta" { Grain::Week } else { Grain::Year };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: v,
                    grain,
                    past: false,
                    interval: true,
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
