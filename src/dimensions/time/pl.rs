use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use crate::dimensions::time_grain::Grain;
use super::{Direction, PartOfDay, TimeData, TimeForm};

fn parse_pl_number_word(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "jeden" | "jedna" | "jedno" => Some(1),
        "dwa" | "dwie" | "dwóch" | "dwoch" => Some(2),
        "trzy" => Some(3),
        "cztery" => Some(4),
        "pięć" | "piec" => Some(5),
        "sześć" | "szesc" => Some(6),
        "siedem" => Some(7),
        "osiem" => Some(8),
        "dziewięć" | "dziewiec" => Some(9),
        "dziesięć" | "dziesiec" => Some(10),
        "jedenaście" | "jedenascie" => Some(11),
        "dwanaście" | "dwanascie" => Some(12),
        "trzynaście" | "trzynascie" => Some(13),
        "czternaście" | "czternascie" => Some(14),
        "piętnaście" | "pietnascie" => Some(15),
        _ => None,
    }
}
fn parse_pl_grain(s: &str) -> Option<Grain> {
    let u = s.to_lowercase();
    if u.starts_with("sekund") {
        Some(Grain::Second)
    } else if u.starts_with("minut") {
        Some(Grain::Minute)
    } else if u.starts_with("godzin") || u == "godzina" || u == "godziny" {
        Some(Grain::Hour)
    } else if u.starts_with("dzie") || u.starts_with("dni") {
        Some(Grain::Day)
    } else if u.starts_with("tydzie") || u.starts_with("tygodn") {
        Some(Grain::Week)
    } else if u.starts_with("miesią") || u.starts_with("miesia") || u.starts_with("miesię") || u.starts_with("miesie") {
        Some(Grain::Month)
    } else if u.starts_with("rok") || u.starts_with("lat") {
        Some(Grain::Year)
    } else {
        None
    }
}
fn parse_pl_hour_word(s: &str) -> Option<u32> {
    match s.to_lowercase().as_str() {
        "pierwsza" | "pierwszej" => Some(1),
        "druga" | "drugiej" => Some(2),
        "trzecia" | "trzeciej" => Some(3),
        "czwarta" | "czwartej" => Some(4),
        "piąta" | "piata" | "piątej" | "piatej" => Some(5),
        "szósta" | "szosta" | "szóstej" | "szostej" => Some(6),
        "siódma" | "siodma" | "siódmej" | "siodmej" => Some(7),
        "ósma" | "osma" | "ósmej" | "osmej" => Some(8),
        "dziewiąta" | "dziewiata" | "dziewiątej" | "dziewiatej" => Some(9),
        "dziesiąta" | "dziesiata" | "dziesiątej" | "dziesiatej" => Some(10),
        "jedenasta" | "jedenastej" => Some(11),
        "dwunasta" | "dwunastej" => Some(12),
        "trzynasta" | "trzynastej" => Some(13),
        "czternasta" | "czternastej" => Some(14),
        "piętnasta" | "pietnasta" | "piętnastej" | "pietnastej" => Some(15),
        "szesnasta" | "szesnastej" => Some(16),
        "siedemnasta" | "siedemnastej" => Some(17),
        "osiemnasta" | "osiemnastej" => Some(18),
        "dziewiętnasta" | "dziewietnasta" | "dziewiętnastej" | "dziewietnastej" => Some(19),
        "dwudziesta" | "dwudziestej" => Some(20),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (pl)".to_string(),
            pattern: vec![regex("teraz|w tej chwili|w tym momencie")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (pl)".to_string(),
            pattern: vec![regex("dzisiaj|dzi[śs]|obecnego dnia|tego dnia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "ta niedziela (pl)".to_string(),
            pattern: vec![regex("ta\\s+niedziela")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(6))))),
        },
        Rule {
            name: "tomorrow (pl)".to_string(),
            pattern: vec![regex("jutro")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "day after tomorrow (pl)".to_string(),
            pattern: vec![regex("pojutrze|po jutrze")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))),
        },
        Rule {
            name: "yesterday (pl)".to_string(),
            pattern: vec![regex("wczoraj")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (pl)".to_string(),
            pattern: vec![regex("poniedzia(l|ł)(ek|ku|kowi|kiem|kowy)|pon\\.?|wtorek|wtorku|wtorkowi|wtorkiem|wtr?\\.?|(Ś|ś|s)rod(a|ą|y|e|ę|zie|owy|o)|(s|ś|Ś)ro?\\.?|czwartek|czwartku|czwartkowi|czwartkiem|czwr?\\.?|piątek|piatek|piątku|piatku|piątkowi|piatkowi|piątkiem|piatkiem|pi(ą|a)tkowy|pia\\.?|sobota|soboty|sobocie|sobotę|sobote|sobotą|soboto|sob\\.?|niedziel(a|i|ę|e|ą|o)|n(ie)?dz?\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("pon") {
                    0
                } else if s.starts_with("wt") {
                    1
                } else if s.starts_with("śr") || s.starts_with("sr") || s.starts_with("śro") || s.starts_with("sro") {
                    2
                } else if s.starts_with("czw") {
                    3
                } else if s.starts_with("pi") {
                    4
                } else if s.starts_with("sob") {
                    5
                } else if s.starts_with("n") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "pierwszy marca (pl)".to_string(),
            pattern: vec![regex("pierwszy\\s+marca")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "pierwszego marca (pl)".to_string(),
            pattern: vec![regex("pierwszego\\s+marca")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "marzec pierwszy (pl)".to_string(),
            pattern: vec![regex("marzec\\s+pierwszy")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "1szy marca (pl)".to_string(),
            pattern: vec![regex("1szy\\s+marca|1szy\\s+marzec")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "marzec 3 (pl)".to_string(),
            pattern: vec![regex("marzec\\s+(\\d{1,2})")],
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
            name: "3go marca (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})go\\s+marca")],
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
            name: "3ci marca 2015 (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})ci\\s+marca\\s+(\\d{4})")],
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
            name: "trzeci marca 2015 (pl)".to_string(),
            pattern: vec![regex("trzeci\\s+marca\\s+(\\d{4})")],
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
            name: "15 Luty (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Ll]uty")],
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
            name: "15 Lutego (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Ll]utego")],
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
            name: "Luty 15 (pl)".to_string(),
            pattern: vec![regex("[Ll]uty\\s*(\\d{1,2})")],
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
            name: "15-tego Lutego (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})-tego\\s+[Ll]utego")],
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
            name: "Pietnastego Lutego (pl)".to_string(),
            pattern: vec![regex("Pietnastego\\s+[Ll]utego|Piętnastego\\s+[Ll]utego")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 15, year: None })))),
        },
        Rule {
            name: "Piętnasty Luty (pl)".to_string(),
            pattern: vec![regex("Piętnasty\\s+[Ll]uty|Pietnasty\\s+[Ll]uty")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 15, year: None })))),
        },
        Rule {
            name: "Luty Piętnastego (pl)".to_string(),
            pattern: vec![regex("[Ll]uty\\s+Piętnastego|[Ll]uty\\s+Pietnastego")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 15, year: None })))),
        },
        Rule {
            name: "Sierpień 8 (pl)".to_string(),
            pattern: vec![regex("Sierpie[ńn]\\s*(\\d{1,2})")],
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
            name: "Sie 8 (pl)".to_string(),
            pattern: vec![regex("Sie\\.?\\s*(\\d{1,2})")],
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
            name: "Sier 8 (pl)".to_string(),
            pattern: vec![regex("Sier\\.?\\s*(\\d{1,2})")],
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
            name: "Sierp. 8 (pl)".to_string(),
            pattern: vec![regex("Sierp\\.?\\s*(\\d{1,2})")],
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
            name: "8 Sie. (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Ss]ie\\.?")],
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
            name: "8 Sier. (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Ss]ier\\.?")],
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
            name: "8 Sierp. (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Ss]ierp\\.?")],
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
            name: "Ósmy Sie. (pl)".to_string(),
            pattern: vec![regex("Ósmy\\s+[Ss]ie\\.?|Osmy\\s+[Ss]ie\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 8, day: 8, year: None })))),
        },
        Rule {
            name: "Osmego Sie. (pl)".to_string(),
            pattern: vec![regex("Osmego\\s+[Ss]ie\\.?|Ósmego\\s+[Ss]ie\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 8, day: 8, year: None })))),
        },
        Rule {
            name: "20 listopada (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+listopada")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 11, day, year: None })))
            }),
        },
        Rule {
            name: "20 maja (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+maja")],
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
            name: "20 maj (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+maj")],
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
            name: "Październik 2014 (pl)".to_string(),
            pattern: vec![regex("[Pp]a[źz]dziernik(a)?\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
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
            name: "14kwiecien 2015 (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*kwiecie[ńn]\\s*(\\d{4})")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "Kwiecień 14, 2015 (pl)".to_string(),
            pattern: vec![regex("[Kk]wiecie[ńn]\\s+(\\d{1,2}),\\s*(\\d{4})")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "14tego Kwietnia 15 (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})(-?tego)?\\s+[Kk]wietnia\\s+(\\d{2,4})")],
            production: Box::new(|nodes| {
                let (d, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                let y_raw: i32 = y.parse().ok()?;
                let year = if y_raw < 100 { 2000 + y_raw } else { y_raw };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "14-ty Kwietnia 15 (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})-ty\\s+[Kk]wietnia\\s+(\\d{2,4})")],
            production: Box::new(|nodes| {
                let (d, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                let y_raw: i32 = y.parse().ok()?;
                let year = if y_raw < 100 { 2000 + y_raw } else { y_raw };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "Czternasty/Czternastego Kwietnia 15 (pl)".to_string(),
            pattern: vec![regex("Czternast(y|ego)\\s+[Kk]wietnia\\s+(\\d{2,4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let y_raw: i32 = y.parse().ok()?;
                let year = if y_raw < 100 { 2000 + y_raw } else { y_raw };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day: 14, year: Some(year) })))
            }),
        },
        Rule {
            name: "nastepny marzec (pl)".to_string(),
            pattern: vec![regex("nast[ęe]pny\\s+[Mm]arzec")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::Month(3));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "Marzec po nastepnym (pl)".to_string(),
            pattern: vec![regex("[Mm]arzec\\s+po\\s+nast[ęe]pnym")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::Month(3));
                t.direction = Some(Direction::FarFuture);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "ten tydzien (pl)".to_string(),
            pattern: vec![regex("ten\\s+tydzie[nń]|ten\\s+tyg|tym\\s+tygodniu")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(crate::dimensions::time_grain::Grain::Week))))),
        },
        Rule {
            name: "ostatni tydzien (pl)".to_string(),
            pattern: vec![regex("ostatni\\s+tydzie[nń]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: crate::dimensions::time_grain::Grain::Week,
                offset: -1,
            })))),
        },
        Rule {
            name: "nastepny tydzien (pl)".to_string(),
            pattern: vec![regex("nast[ęe]pny\\s+tydzie[nń]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: crate::dimensions::time_grain::Grain::Week,
                offset: 1,
            })))),
        },
        Rule {
            name: "nastepnego tygodnia (pl)".to_string(),
            pattern: vec![regex("nast[ęe]pnego\\s+tygodnia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: crate::dimensions::time_grain::Grain::Week,
                offset: 1,
            })))),
        },
        Rule {
            name: "poprzedniego tygodnia (pl)".to_string(),
            pattern: vec![regex("poprzedniego\\s+tygodnia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: crate::dimensions::time_grain::Grain::Week,
                offset: -1,
            })))),
        },
        Rule {
            name: "ostatni miesiac (pl)".to_string(),
            pattern: vec![regex("ostatni\\s+miesi[ąa]c|ostatniego\\s+miesi[ąa]ca|poprzedni\\s+miesi[ąa]c|poprzedniego\\s+miesi[ąa]ca|po\\s*przedniego\\s+miesi[ąa]ca")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: crate::dimensions::time_grain::Grain::Month,
                offset: -1,
            })))),
        },
        Rule {
            name: "nastepnego miesiaca (pl)".to_string(),
            pattern: vec![regex("nast[ęe]pnego\\s+miesi[ąa]ca")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: crate::dimensions::time_grain::Grain::Month,
                offset: 1,
            })))),
        },
        Rule {
            name: "ten kwartał (pl)".to_string(),
            pattern: vec![regex("ten\\s+kwarta[łl]|tego\\s+kwarta[łl]u|tym\\s+kwartale")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: crate::dimensions::time_grain::Grain::Quarter,
                offset: 0,
            })))),
        },
        Rule {
            name: "nastepny kwartał (pl)".to_string(),
            pattern: vec![regex("nast[ęe]pny\\s+kwarta[łl]|nast[ęe]pny\\s+kwartal|kolejnym\\s+kwartale")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: crate::dimensions::time_grain::Grain::Quarter,
                offset: 1,
            })))),
        },
        Rule {
            name: "trzeci kwartał (pl)".to_string(),
            pattern: vec![regex("trzeci\\s+kwarta[łl]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "4ty kwartał 2018 (pl)".to_string(),
            pattern: vec![regex("4ty\\s+kwarta[łl]\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(4, year))))
            }),
        },
        Rule {
            name: "20 listopadowi (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+listopadowi")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 11, day, year: None })))
            }),
        },
        Rule {
            name: "20 listopadem (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+listopadem")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 11, day, year: None })))
            }),
        },
        Rule {
            name: "20 listopad (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+listopad")],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 11, day, year: None })))
            }),
        },
        Rule {
            name: "poprzedni/ostatni rok (pl)".to_string(),
            pattern: vec![regex("poprzedni\\s+rok|ostatni\\s+rok")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Year,
                offset: -1,
            })))),
        },
        Rule {
            name: "ten/obecny rok (pl)".to_string(),
            pattern: vec![regex("ten\\s+rok|tym\\s+roku|obecny\\s+rok|w\\s+obecny\\s+rok|w\\s+obecnym\\s+roku")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year))))),
        },
        Rule {
            name: "kolejny rok (pl)".to_string(),
            pattern: vec![regex("w\\s+kolejnym\\s+roku|kolejny\\s+rok")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Year,
                offset: 1,
            })))),
        },
        Rule {
            name: "<n> <grain> temu (pl)".to_string(),
            pattern: vec![regex("(\\d+|jeden|jedna|dwa|dwie|trzy|cztery|pięć|piec|sześć|szesc|siedem|osiem|dziewięć|dziewiec|dziesięć|dziesiec)\\s+(sekund[ay]?|minut[ay]?|godzin[ay]?|dni|dzie[ńn]|tygodni[ae]?|tydzie[nń]|miesi[aą]c[ae]?|lata|lat|rok[ui]?)\\s+temu")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n = n_s.parse::<i64>().ok().or_else(|| parse_pl_number_word(n_s))?;
                let grain = parse_pl_grain(g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: -(n as i32),
                })))
            }),
        },
        Rule {
            name: "za <n> <grain> (pl)".to_string(),
            pattern: vec![regex("za\\s+(jeszcze\\s+)?(\\d+|jeden|jedna|dwa|dwie|trzy|cztery|pięć|piec|sześć|szesc|siedem|osiem|dziewięć|dziewiec|dziesięć|dziesiec)\\s+(sekund[ay]?|minut[ay]?|godzin[ay]?|dni|dzie[ńn]|tygodni[ae]?|tydzie[nń]|miesi[aą]c[ae]?|lat|rok[ui]?)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let n = n_s.parse::<i64>().ok().or_else(|| parse_pl_number_word(n_s))?;
                let grain = parse_pl_grain(g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: n as i32,
                })))
            }),
        },
        Rule {
            name: "przez minutę / w sekundę (pl)".to_string(),
            pattern: vec![regex("przez\\s+minut[ęe]|za\\s+jedn[ąa]\\s+minut[ęe]|w\\s+sekund[ęe]|za\\s+sekund[ęe]|sekunde\\s+od\\s+teraz")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let (grain, offset) = if s.contains("sekund") {
                    (Grain::Second, 1)
                } else {
                    (Grain::Minute, 1)
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset })))
            }),
        },
        Rule {
            name: "w <n> <grain> (pl)".to_string(),
            pattern: vec![regex("w\\s+(\\d+|jeden|jedn[ąa]|dwa|dwie|trzy|kilka|pi[eę][tć]na[sś]cie|pi[eę][ćc])\\s+(minuty|minut|godzin[ęe]?|godziny|godzin|dni|dzie[ńn]|tydzie[ńn]|tygodnie|tygodni|lata|lat|miesi[aą]ce|miesi[ąa]c)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n = if n_s == "kilka" {
                    3
                } else {
                    n_s.parse::<i64>().ok().or_else(|| parse_pl_number_word(n_s))?
                };
                let grain = parse_pl_grain(g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: n as i32,
                })))
            }),
        },
        Rule {
            name: "w ciągu 2 tygodni (pl)".to_string(),
            pattern: vec![regex("w\\s+ci[ąa]gu\\s+(\\d+|jeden|dwa|dwie|dw[óo]ch|trzy)\\s+(dni|tygodni|tygodnie|miesi[ęe]cy|miesi[ąa]ce|lat|lata)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n = n_s.parse::<i64>().ok().or_else(|| parse_pl_number_word(n_s))?;
                let grain = parse_pl_grain(g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "w/za pół godziny (pl)".to_string(),
            pattern: vec![regex("w\\s+p[óo][łl]\\s+godziny|za\\s+oko[łl]o\\s+p[óo][łl]\\s+godziny|za\\s+jakie[śs]\\s+p[óo][łl]\\s+godziny")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Minute,
                offset: 30,
            })))),
        },
        Rule {
            name: "w 2.5 godziny / 2 i pół godziny (pl)".to_string(),
            pattern: vec![regex("w\\s+2\\.5\\s+godziny|w\\s+2\\s+i\\s+p[óo][łl]\\s+godziny")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Minute,
                offset: 150,
            })))),
        },
        Rule {
            name: "w godzinę / w 1h / w przeciągu godziny (pl)".to_string(),
            pattern: vec![regex("w\\s+godzin[ęe]|w\\s*1h|w\\s+przeci[ąa]gu\\s+godziny")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                grain: Grain::Hour,
                offset: 1,
            })))),
        },
        Rule {
            name: "w kilka godzin / w 24 godziny / w jeden dzień / dzień od dziś (pl)".to_string(),
            pattern: vec![regex("w\\s+kilka\\s+godzin|w\\s+24\\s+godziny|w\\s+jeden\\s+dzie[ńn]|dzie[ńn]\\s+od\\s+dzi[śs]|3\\s+lata\\s+od\\s+dzi[śs]|w\\s+jeden\\s+tydzie[ńn]|w\\s+tydzie[ńn]")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let (grain, offset) = if s.contains("kilka godzin") {
                    (Grain::Hour, 3)
                } else if s.contains("24 godziny") {
                    (Grain::Hour, 24)
                } else if s.contains("3 lata") {
                    (Grain::Year, 3)
                } else if s.contains("tydzie") {
                    (Grain::Week, 1)
                } else {
                    (Grain::Day, 1)
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset })))
            }),
        },
        Rule {
            name: "tydzień/miesiąc/rok temu (pl)".to_string(),
            pattern: vec![regex("tydzie[ńn]\\s+temu|miesi[ąa]c\\s+temu|rok\\s+temu|jeden\\s+tydzie[ńn]\\s+temu|1\\s+tydzie[ńn]\\s+temu")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let grain = if s.contains("tydzie") {
                    Grain::Week
                } else if s.contains("miesi") {
                    Grain::Month
                } else {
                    Grain::Year
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: -1,
                })))
            }),
        },
        Rule {
            name: "<n> <grain> później/potem (pl)".to_string(),
            pattern: vec![regex("(\\d+|jeden|dwa|trzy)\\s+(dni|tygodnie|tygodni|miesi[ąa]ce|lata|tydzie[ńn])\\s+(p[óo][źz]niej|potem)|tydzie[ńn]\\s+p[óo][źz]niej|jeden\\s+tydzie[ńn]\\s+p[óo][źz]niej|1\\s+tydzie[ńn]\\s+p[óo][źz]niej|trzy\\s+tygodnie\\s+p[óo][źz]niej|trzy\\s+miesi[ąa]ce\\s+p[óo][źz]niej|dwa\\s+lata\\s+p[óo][źz]niej")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let (grain, offset) = if s.contains("dwa lata") {
                    (Grain::Year, 2)
                } else if s.contains("miesi") && s.contains("trzy") {
                    (Grain::Month, 3)
                } else if s.contains("dni") {
                    if s.contains("14") { (Grain::Day, 14) } else { (Grain::Day, 7) }
                } else if s.contains("trzy tyg") {
                    (Grain::Week, 3)
                } else {
                    (Grain::Week, 1)
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset })))
            }),
        },
        Rule {
            name: "ostatnie <n> <grain> (pl)".to_string(),
            pattern: vec![regex("(ostatni(e|a|y)?|poprzedni(e|a|y)?)\\s+(\\d+|jeden|jedna|dwa|dwie|trzy)\\s+(sekunda|sekundy|minuta|minuty|godzina|godziny|dzie[ńn]|dni|tydzie[ńn]|tygodnie|miesi[ąa]c|miesi[ąa]ce|rok|lata)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(4)?, m.group(5)?),
                    _ => return None,
                };
                let n = n_s.parse::<i64>().ok().or_else(|| parse_pl_number_word(n_s))?;
                let grain = parse_pl_grain(g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: true,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "następne <n> <grain> (pl)".to_string(),
            pattern: vec![regex("(nast[ęe]pne|kolejne)\\s+(\\d+|dwa|dwie|trzy|kilka)\\s+(sekundy|minuty|godziny|dni|tygodnie|miesi[ąa]ce|lata)")],
            production: Box::new(|nodes| {
                let (n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(2)?, m.group(3)?),
                    _ => return None,
                };
                let n = if n_s == "kilka" {
                    3
                } else {
                    n_s.parse::<i64>().ok().or_else(|| parse_pl_number_word(n_s))?
                };
                let grain = parse_pl_grain(g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "weekend (pl)".to_string(),
            pattern: vec![regex("weekend|ten\\s+weekend|w\\s+ten\\s+weekend|w\\s+weekend")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "part-of-day relative (pl)".to_string(),
            pattern: vec![regex("dzisiaj\\s+wieczorem|jutro\\s+wieczorem|wczoraj\\s+wieczorem|rano|po\\s+południu|popołudniu|wieczorem|w\\s+nocy|nocą")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let date = if s.starts_with("jutro") {
                    TimeData::new(TimeForm::Tomorrow)
                } else if s.starts_with("wczoraj") {
                    TimeData::new(TimeForm::Yesterday)
                } else {
                    TimeData::new(TimeForm::Today)
                };
                let pod = if s.contains("połud") || s.contains("popoł") {
                    PartOfDay::Afternoon
                } else if s.contains("wiecz") {
                    PartOfDay::Evening
                } else if s.contains("noc") {
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
        Rule {
            name: "ten wieczór (pl)".to_string(),
            pattern: vec![regex("ten\\s+wiecz[óo]r|tego\\s+wieczora")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Today)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "jutrzejszy/wczorajszy wieczór (pl)".to_string(),
            pattern: vec![regex("jutrzejszy\\s+wiecz[óo]r|wczorajszy\\s+wiecz[óo]r")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let date = if s.contains("jutrz") {
                    TimeData::new(TimeForm::Tomorrow)
                } else {
                    TimeData::new(TimeForm::Yesterday)
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(date),
                    Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
                ))))
            }),
        },
        Rule {
            name: "jutrzejsza/wczorajsza noc (pl)".to_string(),
            pattern: vec![regex("jutrzejsza\\s+noc|wczorajsza\\s+noc")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let date = if s.contains("jutrz") {
                    TimeData::new(TimeForm::Tomorrow)
                } else {
                    TimeData::new(TimeForm::Yesterday)
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(date),
                    Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Night))),
                ))))
            }),
        },
        Rule {
            name: "przed drugą (po południu) (pl)".to_string(),
            pattern: vec![regex("przed\\s+drug[ąa](\\s+po\\s+po[łl]udniu)?")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::HourMinute(14, 0, false));
                t.open_interval_direction = Some(super::IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "po drugiej po południu (pl)".to_string(),
            pattern: vec![regex("po\\s+drugiej\\s+po\\s+po[łl]udniu|po\\s+drugiej\\s+po\\s+poludniu")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::HourMinute(14, 0, false));
                t.open_interval_direction = Some(super::IntervalDirection::After);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "po pięciu dniach (pl)".to_string(),
            pattern: vec![regex("po\\s+(\\d+|jednym|dwoch|dw[óo]ch|trzech|czterech|pi[eę]ciu)\\s+dniach")],
            production: Box::new(|nodes| {
                let n_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let n = match n_s {
                    "jednym" => 1,
                    "dwoch" | "dwóch" => 2,
                    "trzech" => 3,
                    "czterech" => 4,
                    "pieciu" | "pięciu" => 5,
                    _ => n_s.parse::<i32>().ok()?,
                };
                let mut t = TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: n });
                t.open_interval_direction = Some(super::IntervalDirection::After);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "to w południe (pl)".to_string(),
            pattern: vec![regex("to\\s+w\\s+po[łl]udnie")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Today)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))),
            ))))),
        },
        Rule {
            name: "3 z rana / o 3 rano (pl)".to_string(),
            pattern: vec![regex("(o\\s+)?(\\d{1,2})\\s*(z\\s+rana|rano)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour == 0 || hour > 12 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "o trzeciej z rana (pl)".to_string(),
            pattern: vec![regex("o\\s+trzeciej\\s+(z\\s+rana|rano)|trzecia\\s+(popoludniu|popołudniu)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("popolud") || s.contains("popołud") {
                    Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 0, false))))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(3, 0, false))))
                }
            }),
        },
        Rule {
            name: "o pierwszy / o drugiej (pl)".to_string(),
            pattern: vec![regex("o\\s+pierwszy|o\\s+drugiej")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("pierwszy") {
                    Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(13, 0, false))))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(14, 0, false))))
                }
            }),
        },
        Rule {
            name: "o trzeciej (pl)".to_string(),
            pattern: vec![regex("o\\s+trzeciej")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 0, false))))),
        },
        Rule {
            name: "piętnasta godzina / 15sta godzina (pl)".to_string(),
            pattern: vec![regex("pi[ęe]tnasta\\s+godzina|15sta\\s+godzina|o\\s+pi[ęe]tnastej|o\\s+15stej")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 0, false))))),
        },
        Rule {
            name: "o 18stej (pl)".to_string(),
            pattern: vec![regex("o\\s*(\\d{1,2})stej")],
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
            name: "osiemnasta godzina (pl)".to_string(),
            pattern: vec![regex("osiemnasta\\s+godzina|dziewietnasta\\s+godzina|dwudziesta\\s+godzina")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let hour = if s.contains("osiemnasta") {
                    18
                } else if s.contains("dziewietnasta") {
                    19
                } else {
                    20
                };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "o osiemnastej / o dziewiętnastej (pl)".to_string(),
            pattern: vec![regex("o\\s+osiemnastej|o\\s+dziewi[ęe]tnastej")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let hour = if s.contains("dziew") { 19 } else { 18 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "dwudziesta pierwsza/druga/trzecia godzina (pl)".to_string(),
            pattern: vec![regex("dwudziesta\\s+pierwsza\\s+godzina|dwudziestapierwsza\\s+godzina|o\\s+dwudziestej\\s+drugiej|o\\s+dwudziestejtrzeciej")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let hour = if s.contains("pierwsza") || s.contains("pierwsza") {
                    21
                } else if s.contains("drugiej") {
                    22
                } else {
                    23
                };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "mniej wiecej o 3 (pl)".to_string(),
            pattern: vec![regex("mniej\\s+wiecej\\s+o\\s+3|oko[łl]o\\s+3\\s+po\\s+po[łl]udniu|oko[łl]o\\s+trzeciej|ko[łl]o\\s+trzeciej|o\\s+ko[łl]o\\s+trzeciej|tak\\s+o\\s+15stej")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 0, false))))),
        },
        Rule {
            name: "kwadrans po 3 (pl)".to_string(),
            pattern: vec![regex("kwadrans\\s+po\\s+3|pi[ęe]tna[śs]cie\\s+po\\s+trzeciej|15\\s+po\\s+trzeciej|o\\s+trzecia\\s+pi[ęe]tna[śs]cie")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 15, false))))),
        },
        Rule {
            name: "20 po 3 (pl)".to_string(),
            pattern: vec![regex("20\\s+po\\s+3|3:20|o\\s+trzecia\\s+dwadzie[śs]cia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 20, false))))),
        },
        Rule {
            name: "w pół do szesnastej (pl)".to_string(),
            pattern: vec![regex("w\\s+p[óo][łl]\\s+do\\s+([a-zA-Ząćęłńóśźż]+|\\d{1,2}stej)|p[óo][łl]\\s+po\\s+trzeciej")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("po trzeciej") {
                    return Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 30, false))));
                }
                let h_raw = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let target: u32 = if let Ok(v) = h_raw.parse::<u32>() {
                    v
                } else {
                    parse_pl_hour_word(h_raw)?
                };
                if target == 0 || target > 23 {
                    return None;
                }
                let out = if target == 0 { 23 } else { target - 1 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out, 30, false))))
            }),
        },
        Rule {
            name: "kwadrans do/przed południem (pl)".to_string(),
            pattern: vec![regex("kwadrans\\s+do\\s+po[łl]udnia|kwadrans\\s+przed\\s+po[łl]udniem|kwadrans\\s+do\\s+12stej")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(11, 45, false))))),
        },
        Rule {
            name: "kwadrans do południa spelling variants (pl)".to_string(),
            pattern: vec![regex("kwadrans\\s+do\\s+po[łl]udnia|kwadrans\\s+przed\\s+po[łl]udniem|kwadrans\\s+do\\s+12stej|11:45")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(11, 45, false))))),
        },
        Rule {
            name: "8 tego wieczora (pl)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+tego\\s+wieczora")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour == 0 || hour > 23 {
                    return None;
                }
                let out = if hour < 12 { hour + 12 } else { hour };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out, 0, false))))
            }),
        },
        Rule {
            name: "word hour part-of-day (pl)".to_string(),
            pattern: vec![regex("(pierwsza|pierwszej|druga|drugiej|trzecia|trzeciej|czwarta|czwartej|piąta|piata|piątej|piatej|szósta|szosta|szóstej|szostej|siódma|siodma|siódmej|siodmej|ósma|osma|ósmej|osmej|dziewiąta|dziewiata|dziesiąta|dziesiata)\\s+(popoludniu|popołudniu|wieczorem|w\\s+nocy|nocą)")],
            production: Box::new(|nodes| {
                let w = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let base = parse_pl_hour_word(w)?;
                let mut hour = base;
                if hour < 12 {
                    hour += 12;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "seasons (pl)".to_string(),
            pattern: vec![regex("ta\\s+wiosna|to\\s+lato|ta\\s+jesień|ta\\s+zima|tej\\s+wiosny|tego\\s+lata|tej\\s+jesieni|tej\\s+zimy")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let season = if s.contains("wiosn") {
                    0
                } else if s.contains("lato") || s.contains("lata") {
                    1
                } else if s.contains("jesie") {
                    2
                } else {
                    3
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "holidays (pl)".to_string(),
            pattern: vec![regex("[śs]wi[ęe]ta\\s+bo[żz]ego\\s+narodzenia|boże\\s+narodzenie|boze\\s+narodzenie|wigilia|sylwester|nowy\\s+rok|walentynki|dzien\\s+matki|dzień\\s+matki|dzie[ńn]\\s+mamy|dzien\\s+ojca|dzień\\s+ojca|dzie[ńn]\\s+taty|dzie[ńn]\\s+dziękczynienia|dzie[ńn]\\s+dziekczynienia|dziękczynienie|dziekczynienie|halloween|wszystkich\\s+świętych|wszystkich\\s+swietych")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let h = if s.contains("boże narodzenie") || s.contains("boze narodzenie") {
                    "christmas day"
                } else if s.contains("wigilia") || s.contains("sylwester") {
                    "new year's eve"
                } else if s.contains("nowy rok") {
                    "new year's day"
                } else if s.contains("walent") {
                    "valentine's day"
                } else if s.contains("matki") || s.contains("mamy") {
                    "mother's day"
                } else if s.contains("ojca") || s.contains("taty") {
                    "father's day"
                } else if s.contains("dziękczynienia") || s.contains("dziekczynienia") {
                    "thanksgiving"
                } else if s.contains("halloween") {
                    "halloween"
                } else if s.contains("wszystkich") {
                    "all saints' day"
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(h.to_string(), None))))
            }),
        },
        Rule {
            name: "Święta Bożego Narodzenia (pl)".to_string(),
            pattern: vec![regex("Święta\\s+Bożego\\s+Narodzenia|Swieta\\s+Bozego\\s+Narodzenia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "christmas day".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "Dziękczynienie (pl)".to_string(),
            pattern: vec![regex("Dzi.kczynienie|dziekczynienie")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "thanksgiving".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "drugi dzień świąt Bożego Narodzenia (pl)".to_string(),
            pattern: vec![regex("drugi\\s+dzie[ńn]\\s+[śs]wi[ąa]t\\s+bo[żz]ego\\s+narodzenia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 12,
                day: 26,
                year: None,
            })))),
        },
        Rule {
            name: "Lipiec 13-15 / 13-15 lipca (pl)".to_string(),
            pattern: vec![regex("[Ll]ipiec\\s*(\\d{1,2})\\s*[-–]\\s*(\\d{1,2})|(\\d{1,2})\\s*[-–]\\s*(\\d{1,2})\\s+lipca")],
            production: Box::new(|nodes| {
                let (d1, d2) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => {
                        if m.group(1).is_some() {
                            (m.group(1)?, m.group(2)?)
                        } else {
                            (m.group(3)?, m.group(4)?)
                        }
                    }
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
            name: "Lipca 13 do 15 (pl)".to_string(),
            pattern: vec![regex("[Ll]ipca\\s*(\\d{1,2})\\s+do\\s+(\\d{1,2})")],
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
            name: "Lipiec 13 - Lipiec 15 (pl)".to_string(),
            pattern: vec![regex("[Ll]ipiec\\s*(\\d{1,2})\\s*[-–]\\s*[Ll]ipiec\\s*(\\d{1,2})")],
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
            name: "fixed holidays map (pl)".to_string(),
            pattern: vec![regex("nowy\\s+rok|((ś|s)wi(e|ę)to)?\\s*trzech\\s+kr(o|ó)li|dzie[ńn]\\s+kobiet|(ś|s)wi(e|ę)to\\s+pracy|(ś|s)wi(e|ę)to\\s+konstytucj(i|a)|((ś|s)wi(e|ę)to\\s+wojska\\s+polskiego|wniebowzi(e|ę)cie\\s+naj(s|ś)wi(e|ę)tszej\\s+maryi\\s+panny)|((ś|s)wi(e|ę)to)?\\s*wszystkich\\s+(s|ś)wi(e|ę)tych|(s|ś)wi(e|ę)t(a|o)\\s+niepodleg(l|ł)o(s|ś)ci|mikołajki|sylwester")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let (month, day) = if s.contains("nowy rok") {
                    (1, 1)
                } else if s.contains("trzech") || s.contains("kroli") || s.contains("króli") {
                    (1, 6)
                } else if s.contains("kobiet") {
                    (3, 8)
                } else if s.contains("pracy") {
                    (5, 1)
                } else if s.contains("konstytucj") {
                    (5, 3)
                } else if s.contains("wojska") || s.contains("wniebow") {
                    (8, 15)
                } else if s.contains("wszystkich") {
                    (11, 1)
                } else if s.contains("niepodleg") {
                    (11, 11)
                } else if s.contains("mikoł") {
                    (12, 6)
                } else {
                    (12, 31)
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "computed holidays map (pl)".to_string(),
            pattern: vec![regex("popielec|niedziela\\s+palmowa|wielki\\s+pi(a|ą)tek|wielka\\s+sobota|wielkanoc|niedziela\\s+wielkanocna|poniedzia(l|ł)ek\\s+wielkanocny|zielone\\s+(s|ś)wi(a|ą)tki|zes(l|ł)anie\\s+ducha\\s+(ś|s)wi(e|ę)tego|bo(z|ż)e\\s+cia(l|ł)o")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let name = if s.contains("popielec") {
                    "ash wednesday"
                } else if s.contains("palmowa") {
                    "palm sunday"
                } else if s.contains("wielki") && s.contains("pi") {
                    "good friday"
                } else if s.contains("wielka sobota") {
                    "holy saturday"
                } else if s.contains("poniedzia") && s.contains("wielkanoc") {
                    "easter monday"
                } else if s.contains("wielkanoc") {
                    "easter sunday"
                } else if s.contains("zielone") || s.contains("zes") {
                    "pentecost"
                } else {
                    "corpus christi"
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(name.to_string(), None))))
            }),
        },
        Rule {
            name: "ordinal day of month name (pl)".to_string(),
            pattern: vec![regex("(pierwszy|drugi|trzeci|czwarty|piąty|piaty|szósty|szosty|siódmy|siodmy|ósmy|osmy|dziewiąty|dziewiaty|dziesiąty|dziesiaty)\\s+dzie[ńn]\\s+(w\\s+)?pa[źz]dzierniku")],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => match m.group(1)? {
                        "pierwszy" => 1,
                        "drugi" => 2,
                        "trzeci" => 3,
                        "czwarty" => 4,
                        "piąty" | "piaty" => 5,
                        "szósty" | "szosty" => 6,
                        "siódmy" | "siodmy" => 7,
                        "ósmy" | "osmy" => 8,
                        "dziewiąty" | "dziewiaty" => 9,
                        "dziesiąty" | "dziesiaty" => 10,
                        _ => return None,
                    },
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: n, year: None })))
            }),
        },
        Rule {
            name: "trzeci dzień października (pl)".to_string(),
            pattern: vec![regex("trzeci\\s+dzie[ńn]\\s+pa[źz]dziernika")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 10,
                day: 3,
                year: None,
            })))),
        },
        Rule {
            name: "pierwszy tydzień października 2014 (pl)".to_string(),
            pattern: vec![regex("(pierwszy|drugi|trzeci|ostatni)\\s+tydzie[ńn]\\s+(w\\s+)?pa[źz]dzierniku\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (ord, y_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)?),
                    _ => return None,
                };
                let year: i32 = y_s.parse().ok()?;
                let base = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(10))),
                ));
                if ord == "ostatni" {
                    Some(TokenData::Time(TimeData::new(TimeForm::NthLastCycleOfTime {
                        n: 1,
                        grain: Grain::Week,
                        base: Box::new(base),
                    })))
                } else {
                    let n = match ord {
                        "pierwszy" => 1,
                        "drugi" => 2,
                        "trzeci" => 3,
                        _ => return None,
                    };
                    Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime {
                        n,
                        grain: Grain::Week,
                        base: Box::new(base),
                    })))
                }
            }),
        },
        Rule {
            name: "ostatni dzień października 2015 (pl)".to_string(),
            pattern: vec![regex("ostatni\\s+dzie[ńn]\\s+(w\\s+)?pa[źz]dzierniku\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let year: i32 = y_s.parse().ok()?;
                let base = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(10))),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::NthLastDayOfTime {
                    n: 1,
                    base: Box::new(base),
                })))
            }),
        },
    ]);
    rules
}
