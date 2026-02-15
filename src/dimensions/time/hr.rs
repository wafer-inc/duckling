use crate::dimensions::time_grain::Grain;
use crate::pattern::regex;
use super::{Direction, IntervalDirection};
use crate::types::{Rule, TokenData};
use super::{PartOfDay, TimeData, TimeForm};

fn parse_dow_hr(s: &str) -> Option<u32> {
    if s.starts_with("pon") {
        Some(0)
    } else if s.starts_with("uto") {
        Some(1)
    } else if s.starts_with("sri") || s.starts_with("srijed") {
        Some(2)
    } else if s.starts_with("čet") || s.starts_with("cet") {
        Some(3)
    } else if s.starts_with("pet") {
        Some(4)
    } else if s.starts_with("sub") {
        Some(5)
    } else if s.starts_with("ned") {
        Some(6)
    } else {
        None
    }
}

fn parse_month_hr(s: &str) -> Option<u32> {
    if s.starts_with("sij") {
        Some(1)
    } else if s.starts_with("velj") || s.starts_with("feb") {
        Some(2)
    } else if s.starts_with("ožu") || s.starts_with("ozu") || s.starts_with("mar") {
        Some(3)
    } else if s.starts_with("tra") || s.starts_with("apr") {
        Some(4)
    } else if s.starts_with("svi") || s.starts_with("maj") {
        Some(5)
    } else if s.starts_with("lip") || s.starts_with("jun") {
        Some(6)
    } else if s.starts_with("srp") || s.starts_with("jul") {
        Some(7)
    } else if s.starts_with("kol") || s.starts_with("aug") {
        Some(8)
    } else if s.starts_with("ruj") || s.starts_with("sep") {
        Some(9)
    } else if s.starts_with("lis") || s.starts_with("okt") {
        Some(10)
    } else if s.starts_with("stu") || s.starts_with("nov") {
        Some(11)
    } else if s.starts_with("pro") || s.starts_with("dec") {
        Some(12)
    } else {
        None
    }
}

fn parse_ordinal_hr(s: &str) -> Option<i32> {
    match s {
        "prvi" | "prva" | "prvo" => Some(1),
        "drugi" | "druga" | "drugo" => Some(2),
        "treci" | "treći" | "treca" | "treća" => Some(3),
        "cetvrti" | "četvrti" | "cetvrta" | "četvrta" => Some(4),
        "peti" | "peta" => Some(5),
        _ => s.parse().ok(),
    }
}

fn month_base(month: u32, year: Option<i32>) -> TimeData {
    match year {
        Some(y) => TimeData::new(TimeForm::Composed(
            Box::new(TimeData::new(TimeForm::Year(y))),
            Box::new(TimeData::new(TimeForm::Month(month))),
        )),
        None => TimeData::new(TimeForm::Month(month)),
    }
}

fn parse_hr_hour_word(s: &str) -> Option<u32> {
    match s {
        "jedan" | "jednu" => Some(1),
        "dva" => Some(2),
        "tri" => Some(3),
        "cetiri" | "četiri" => Some(4),
        "pet" => Some(5),
        "sest" | "šest" => Some(6),
        "sedam" => Some(7),
        "osam" => Some(8),
        "devet" => Some(9),
        "deset" => Some(10),
        "jedanaest" => Some(11),
        "dvanaest" => Some(12),
        _ => None,
    }
}

fn parse_hr_small_num(s: &str) -> Option<u32> {
    match s {
        "petnaest" => Some(15),
        "dvadeset" => Some(20),
        "trideset" => Some(30),
        _ => parse_hr_hour_word(s),
    }
}

fn parse_hr_quantity(s: &str) -> Option<i64> {
    let t = s.trim().to_lowercase();
    match t.as_str() {
        "jedan" | "jednu" | "jednog" | "1" => Some(1),
        "dva" | "dvije" | "2" => Some(2),
        "tri" | "3" => Some(3),
        "sedam" | "7" => Some(7),
        "četrnaest" | "cetrnaest" | "14" => Some(14),
        "dvadeset i cetiri" | "dvadeset i četiri" | "24" => Some(24),
        _ => t.parse::<i64>().ok(),
    }
}

fn parse_hr_grain(s: &str) -> Option<Grain> {
    let t = s.to_lowercase();
    if t.contains("sek") {
        Some(Grain::Second)
    } else if t.contains("min") {
        Some(Grain::Minute)
    } else if t.contains("sat") {
        Some(Grain::Hour)
    } else if t.contains("dan") {
        Some(Grain::Day)
    } else if t.contains("tjed") {
        Some(Grain::Week)
    } else if t.contains("mjes") {
        Some(Grain::Month)
    } else if t.contains("god") {
        Some(Grain::Year)
    } else {
        None
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (hr)".to_string(),
            pattern: vec![regex("(upravo\\s+)?sad(a)?|ovaj\\s+tren")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (hr)".to_string(),
            pattern: vec![regex("danas")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (hr)".to_string(),
            pattern: vec![regex("sutra")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "tonight (hr)".to_string(),
            pattern: vec![regex("ve(c|č)eras|ove\\s+ve(c|č)er(i)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))))),
        },
        Rule {
            name: "previous/next evening (hr)".to_string(),
            pattern: vec![regex("prethodne\\s+ve(c|č)eri|pro(s|š)le\\s+ve(c|č)eri|sljede(c|ć)e\\s+ve(c|č)eri")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening));
                if s.starts_with("sljede") {
                    t.direction = Some(Direction::Future);
                } else {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "this/next/last afternoon (hr)".to_string(),
            pattern: vec![regex("ov(a|o|i)\\s+(poslijepodne|popodne)|sljede(c|ć)e\\s+(poslijepodne|popodne)|pro(s|š)lo\\s+(poslijepodne|popodne)|prethodno\\s+(poslijepodne|popodne)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon));
                if s.starts_with("sljede") {
                    t.direction = Some(Direction::Future);
                } else if s.starts_with("pro") || s.starts_with("pret") {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "after lunch (hr)".to_string(),
            pattern: vec![regex("poslije\\s+ru(c|č)ka")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))))),
        },
        Rule {
            name: "this/next/last morning (hr)".to_string(),
            pattern: vec![regex("ov(e|o)\\s+jutro|sljede(c|ć)e\\s+jutro|pro(s|š)lo\\s+jutro|prethodno\\s+jutro")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning));
                if s.starts_with("sljede") {
                    t.direction = Some(Direction::Future);
                } else if s.starts_with("pro") || s.starts_with("pret") {
                    t.direction = Some(Direction::Past);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "yesterday (hr)".to_string(),
            pattern: vec![regex("ju[čc]er")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (hr)".to_string(),
            pattern: vec![regex("ponedjelja?ka?|pon\\.?|utora?ka?|uto?\\.?|srijed(a|e|u)|sri\\.?|(č|c)etvrta?ka?|(č|c)et\\.?|peta?ka?|pet\\.?|subot(a|e|u)|sub?\\.?|nedjelj(a|e|u)|ned\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = parse_dow_hr(&s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "named month (hr)".to_string(),
            pattern: vec![regex("sije(c|č)a?nj(a|u)?|januar(a|u)?|jan\\.?|sij\\.?|velja(c|č)(a|e|i)|februar(a|u)?|feb\\.?|velj\\.?|o(z|ž)uja?k(a|u)?|mart(a|u)?|mar\\.?|o(z|ž)u\\.?|trava?nj(a|u)?|april(a|u)?|apr\\.?|tra\\.?|sviba?nj(a|u)?|maj|svi\\.?|lipa?nj(a|u)?|jun(i|u|a)?|jun\\.?|lip\\.?|srpa?nj(a|u)?|jul(i|u|a)?|jul\\.?|srp\\.?|kolovoz(a|u)?|august(a|u)?|aug\\.?|kol\\.?|ruja?n(a|u)?|septemba?r(a|u)?|sept?\\.?|ruj\\.?|listopad(a|u)?|oktobar(a|u)?|okt\\.?|lis\\.?|studen(i|oga?|om)|novemba?r(a|u)?|nov\\.?|stu\\.?|prosina?c(a|u)?|decemba?r(a|u)?|dec\\.?|pros\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let month = parse_month_hr(&s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "this/next day-of-week (hr)".to_string(),
            pattern: vec![regex("(ov(aj|a|e|og|u)|sljede(c|ć)(i|u|a|eg|eg?))\\s+(ponedjelja?ka?|pon\\.?|utora?ka?|uto?\\.?|srijed(a|e|u)|sri\\.?|(č|c)etvrta?ka?|(č|c)et\\.?|peta?ka?|pet\\.?|subot(a|e|u)|sub?\\.?|nedjelj(a|e|u)|ned\\.?)")],
            production: Box::new(|nodes| {
                let (q, dow_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.to_lowercase(), m.group(7)?.to_lowercase()),
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::DayOfWeek(parse_dow_hr(&dow_s)?));
                if q.starts_with("sljede") {
                    t.direction = Some(Direction::Future);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "last day-of-week (hr)".to_string(),
            pattern: vec![regex("((pro(s|š)l(i|u|e|og))|prethodn(a|i|u|e|og))\\s+(ponedjelja?ka?|pon\\.?|utora?ka?|uto?\\.?|srijed(a|e|u)|sri\\.?|(č|c)etvrta?ka?|(č|c)et\\.?|peta?ka?|pet\\.?|subot(a|e|u)|sub?\\.?|nedjelj(a|e|u)|ned\\.?)")],
            production: Box::new(|nodes| {
                let dow_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(8)?.to_lowercase(),
                    _ => return None,
                };
                let mut t = TimeData::new(TimeForm::DayOfWeek(parse_dow_hr(&dow_s)?));
                t.direction = Some(Direction::Past);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "day before yesterday / day after tomorrow (hr)".to_string(),
            pattern: vec![regex("prekju(c|č)er|prekosutra")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.starts_with("prekj") {
                    Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))
                }
            }),
        },
        Rule {
            name: "hour in the morning/night (hr)".to_string(),
            pattern: vec![regex("(?:u\\s+)?(\\d{1,2})\\s+(?:sati\\s+)?(ujutro|u\\s+noci|u\\s+no(ć|c)i)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                if hour > 24 {
                    return None;
                }
                if hour == 12 {
                    hour = 0;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "at hour (hr)".to_string(),
            pattern: vec![regex("u\\s+(\\d{1,2})(\\s+sati)?")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
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
            name: "noon/midnight (hr)".to_string(),
            pattern: vec![regex("u\\s+podne|podne(va)?|pono(c|ć)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.contains("pono") {
                    Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(0, 0, false))))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(12, 0, false))))
                }
            }),
        },
        Rule {
            name: "hour word in the night (hr)".to_string(),
            pattern: vec![regex("u\\s+(jedan|jednu|dva|tri|cetiri|četiri|pet|sest|šest|sedam|osam|devet|deset|jedanaest|dvanaest)\\s+sata\\s+u\\s+no(ć|c)i")],
            production: Box::new(|nodes| {
                let w = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let mut hour = parse_hr_hour_word(&w)?;
                if hour == 12 {
                    hour = 0;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "hour in afternoon/evening (hr)".to_string(),
            pattern: vec![regex("(?:u\\s+|oko\\s+|otprilike\\s+u\\s+|cca\\s+)?(\\d{1,2})\\s*(?:sati\\s+)?(poslijepodne|popodne|nave(c|č)er)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                if !(0..=23).contains(&hour) {
                    return None;
                }
                if hour < 12 {
                    hour += 12;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "hour-word in evening (hr)".to_string(),
            pattern: vec![regex("(jedan|jednu|dva|tri|cetiri|četiri|pet|sest|šest|sedam|osam|devet|deset|jedanaest|dvanaest)\\s+sati\\s+nave(c|č)er")],
            production: Box::new(|nodes| {
                let w = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let mut hour = parse_hr_hour_word(&w)?;
                if hour < 12 {
                    hour += 12;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "hour-minute in day-part (hr)".to_string(),
            pattern: vec![regex("(\\d{1,2}):(\\d{2})\\s*(rano|poslijepodne|popodne)")],
            production: Box::new(|nodes| {
                let (h, m, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                if hour > 23 || minute > 59 {
                    return None;
                }
                if (part.contains("poslijepodne") || part.contains("popodne")) && hour < 12 {
                    hour += 12;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "@ hour / cca hour (hr)".to_string(),
            pattern: vec![regex("(?:@|cca)\\s*(\\d{1,2})")],
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
            name: "hour i minute [part-of-day] (hr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+i\\s+(\\d{1,2})(?:\\s+(popodne|poslijepodne))?")],
            production: Box::new(|nodes| {
                let (h, m, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                if hour > 23 || minute > 59 {
                    return None;
                }
                if part.is_some() && hour < 12 {
                    hour += 12;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "<hour-word> i po [popodne] (hr)".to_string(),
            pattern: vec![regex("(jedan|jednu|dva|tri|cetiri|četiri|pet|sest|šest|sedam|osam|devet|deset|jedanaest|dvanaest)\\s+i\\s+po(?:\\s+(popodne|poslijepodne))?")],
            production: Box::new(|nodes| {
                let (h, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)),
                    _ => return None,
                };
                let mut hour = parse_hr_hour_word(&h)?;
                if part.is_some() && hour < 12 {
                    hour += 12;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 30, false))))
            }),
        },
        Rule {
            name: "pola <hour> [popodne] (hr)".to_string(),
            pattern: vec![regex("pola\\s+(\\d{1,2}|jedan|jednu|dva|tri|cetiri|četiri|pet|sest|šest|sedam|osam|devet|deset|jedanaest|dvanaest)(?:\\s+(popodne|poslijepodne))?")],
            production: Box::new(|nodes| {
                let (h_raw, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)),
                    _ => return None,
                };
                let in_hour = h_raw.parse::<u32>().ok().or_else(|| parse_hr_hour_word(&h_raw))?;
                if !(1..=24).contains(&in_hour) {
                    return None;
                }
                let mut out_hour = if in_hour == 1 { 0 } else { in_hour - 1 };
                if part.is_some() && out_hour < 12 {
                    out_hour += 12;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_hour, 30, false))))
            }),
        },
        Rule {
            name: "quarter/twenty/petnaest after hour (hr)".to_string(),
            pattern: vec![regex("(cetvrt|četvrt|petnaest|dvadeset)\\s+nakon\\s+(\\d{1,2})\\s*(popodne|poslijepodne)?")],
            production: Box::new(|nodes| {
                let (m_raw, h, part) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?, rm.group(3)),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                let minute = if m_raw.contains("čet") || m_raw.contains("cetv") { 15 } else { parse_hr_small_num(&m_raw)? };
                if part.is_some() && hour < 12 {
                    hour += 12;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "quarter/petnaest to noon (hr)".to_string(),
            pattern: vec![regex("(petnaest|cetvrt|četvrt)\\s+do\\s+podne(va)?")],
            production: Box::new(|nodes| {
                let m_raw = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let minute = if m_raw.contains("čet") || m_raw.contains("cetv") { 45 } else { 45 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(11, minute, false))))
            }),
        },
        Rule {
            name: "in <duration> (hr)".to_string(),
            pattern: vec![regex("za\\s+(?:jo[sš]\\s+|oko\\s+)?(jedan|jednu|jednog|dva|dvije|tri|sedam|\\d+(?:\\.\\d+)?)\\s+(sekund[auie]?|minut[auie]?|sat[aie]?|dan[a]?|tjed(?:an|na)|mjesec[a]?|godin[aeu])")],
            production: Box::new(|nodes| {
                let (q, g) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                if q.contains('.') {
                    let hours: f64 = q.parse().ok()?;
                    let minutes = (hours * 60.0).round() as i64;
                    return Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: minutes, grain: Grain::Minute })));
                }
                let n = parse_hr_quantity(&q)?;
                let grain = parse_hr_grain(&g)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain })))
            }),
        },
        Rule {
            name: "<duration> from now (hr)".to_string(),
            pattern: vec![regex("(jedan|jednu|jednog|dva|dvije|tri|\\d+)\\s+(sekund[auie]?|minut[auie]?|sat[aie]?|dan[a]?|tjed(?:an|na)|mjesec[a]?|godin[aeu])\\s+od\\s+sad")],
            production: Box::new(|nodes| {
                let (q, g) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_hr_quantity(&q)?;
                let grain = parse_hr_grain(&g)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain })))
            }),
        },
        Rule {
            name: "ago <duration> (hr)".to_string(),
            pattern: vec![regex("prije\\s+(jedan|jednu|jednog|dva|dvije|tri|sedam|\\d+)\\s+(sat[aie]?|dan[a]?|tjed(?:an|na)|mjesec[a]?|godin[aeu])")],
            production: Box::new(|nodes| {
                let (q, g) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_hr_quantity(&q)?;
                let grain = parse_hr_grain(&g)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: -n, grain })))
            }),
        },
        Rule {
            name: "after <duration> (hr)".to_string(),
            pattern: vec![regex("nakon\\s+(jedan|jednu|jednog|dva|dvije|tri|sedam|\\d+)\\s+(sekund[auie]?|minut[auie]?|sat[aie]?|dan[a]?|tjed(?:an|na)|mjesec[a]?|godin[aeu])")],
            production: Box::new(|nodes| {
                let (q, g) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_hr_quantity(&q)?;
                let grain = parse_hr_grain(&g)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain })))
            }),
        },
        Rule {
            name: "in half/quarter/three-quarter hour (hr)".to_string(),
            pattern: vec![regex("(?:za|oko\\s+)?\\s*(pola|pol|1/2|cetvrt|četvrt|1/4|tri-cetvrt|3/4)\\s*(?:h|sata?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let minutes = if s.contains("tri-cetvrt") || s.contains("3/4") {
                    45
                } else if s.contains("cetvrt") || s.contains("četvrt") || s.contains("1/4") {
                    15
                } else {
                    30
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n: minutes, grain: Grain::Minute })))
            }),
        },
        Rule {
            name: "in a few hours (hr)".to_string(),
            pattern: vec![regex("za\\s+(par|nekoliko)\\s+sati")],
            production: Box::new(|nodes| {
                let w = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let n = if w == "par" { 2 } else { 3 };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain: Grain::Hour })))
            }),
        },
        Rule {
            name: "next few days (hr)".to_string(),
            pattern: vec![regex("sljede(c|ć)ih\\s+(par|nekoliko)\\s+dana")],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(2)?,
                    _ => return None,
                };
                let n = if q == "par" { 2 } else { 3 };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain: Grain::Day,
                    past: false,
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "in <n>h/<n>' compact (hr)".to_string(),
            pattern: vec![regex("za\\s*(\\d+)\\s*([h'])")],
            production: Box::new(|nodes| {
                let (n_s, unit) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let n: i64 = n_s.parse().ok()?;
                let grain = if unit == "h" { Grain::Hour } else { Grain::Minute };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain })))
            }),
        },
        Rule {
            name: "last/next <n>h compact (hr)".to_string(),
            pattern: vec![regex("(?:prethodn(?:a|e)|pro(?:s|š)le|sljede(?:c|ć)(?:a|e))\\s*(\\d+)\\s*h")],
            production: Box::new(|nodes| {
                let (full, n_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(0)?.to_lowercase(), rm.group(1)?),
                    _ => return None,
                };
                let n: i64 = n_s.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain: Grain::Hour,
                    past: full.starts_with("preth") || full.starts_with("pro"),
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "last/next n cycles (hr)".to_string(),
            pattern: vec![regex("(pro(s|š)l(e|a)|prethodn(a|e)|sljede(c|ć)(a|e))\\s+(dvije|tri|dvadeset\\s+i\\s+cetiri|dvadeset\\s+i\\s+četiri|\\d+)\\s+(sekund[ae]?|sat[ai]?|minut[ae]?|dan[ae]?|tjedn[a]?|mjesec[a]?|godin[ae])")],
            production: Box::new(|nodes| {
                let (q_s, g_s, s0) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (
                        rm.group(8)?.to_lowercase(),
                        rm.group(9)?.to_lowercase(),
                        rm.group(1)?.to_lowercase(),
                    ),
                    _ => return None,
                };
                let n = parse_hr_quantity(&q_s)?;
                let grain = parse_hr_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: s0.starts_with("pro") || s0.starts_with("pret"),
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "last/next n cycles simple (hr)".to_string(),
            pattern: vec![regex("(pro(?:s|š)l(?:a|e)|prethodn(?:a|e)|sljede(?:c|ć)(?:a|e))\\s+(\\d+)\\s+(sekunde?|sata|minute?|dana|tjedna|mjeseca|godine)")],
            production: Box::new(|nodes| {
                let (dir_s, n_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?, rm.group(3)?.to_lowercase()),
                    _ => return None,
                };
                let n: i64 = n_s.parse().ok()?;
                let grain = parse_hr_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: dir_s.starts_with("pro") || dir_s.starts_with("pret"),
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "last/next n cycles word qty (hr)".to_string(),
            pattern: vec![regex("(?:pro(?:s|š)l(?:a|e)|prethodn(?:a|e)|sljede(?:c|ć)(?:a|e))\\s+(jedan|jednu|dva|dvije|tri|dvadeset\\s+i\\s+cetiri|dvadeset\\s+i\\s+četiri)\\s+(sekunde?|sata|minute?|dana|tjedna|mjeseca|godine)")],
            production: Box::new(|nodes| {
                let (full, q_s, g_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(0)?.to_lowercase(), rm.group(1)?.to_lowercase(), rm.group(2)?.to_lowercase()),
                    _ => return None,
                };
                let n = parse_hr_quantity(&q_s)?;
                let grain = parse_hr_grain(&g_s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n,
                    grain,
                    past: full.starts_with("pro") || full.starts_with("preth"),
                    interval: true,
                })))
            }),
        },
        Rule {
            name: "1. ozujak (hr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.?\\s*o(z|ž)uja?k")],
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
            name: "prvi ozujka (hr)".to_string(),
            pattern: vec![regex("prvi\\s+o(z|ž)ujka")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "treci ozujka (hr)".to_string(),
            pattern: vec![regex("tre(c|ć)i\\s+o(z|ž)ujka")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 3, year: None })))),
        },
        Rule {
            name: "15ti drugi (hr)".to_string(),
            pattern: vec![regex("(\\d{1,2})ti\\s+drugi")],
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
            name: "15. veljace (hr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.\\s*velja(c|č)e")],
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
            name: "8. kolovoza (hr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.\\s*kolovoza")],
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
            name: "8. kolovoz (hr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.\\s*kolovoz")],
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
            name: "listopad 2014 (hr)".to_string(),
            pattern: vec![regex("listopad\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Year(year))),
                    Box::new(TimeData::new(TimeForm::Month(11))),
                ))))
            }),
        },
        Rule {
            name: "74-10-31 (hr)".to_string(),
            pattern: vec![regex("(\\d{2})-(\\d{1,2})-(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (yy, m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let year2: i32 = yy.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                if !(1..=12).contains(&month) || !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(1900 + year2) })))
            }),
        },
        Rule {
            name: "14travanj 2015 (hr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*travanj\\s*(\\d{4})")],
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
            name: "14. travnja, 2015 (hr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.\\s*travnja,?\\s*(\\d{4})")],
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
            name: "14. travanj 15 (hr)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.\\s*travanj\\s*(\\d{2})")],
            production: Box::new(|nodes| {
                let (d, yy) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let year2: i32 = yy.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day, year: Some(2000 + year2) })))
            }),
        },
        Rule {
            name: "sljedeci ozujak (hr)".to_string(),
            pattern: vec![regex("sljede(c|ć)i\\s+o(z|ž)ujak")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::Month(3));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "ozujak nakon sljedeceg (hr)".to_string(),
            pattern: vec![regex("o(z|ž)ujak\\s+nakon\\s+sljede(c|ć)eg")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::Month(3));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "ovaj tjedan (hr)".to_string(),
            pattern: vec![regex("ovaj tjedan")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "prosli tjedan (hr)".to_string(),
            pattern: vec![regex("pro(s|š)li tjedan")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "prethodni tjedan (hr)".to_string(),
            pattern: vec![regex("prethodni tjedan")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "sljedeci tjedan (hr)".to_string(),
            pattern: vec![regex("sljede(c|ć)i tjedan")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "weekend (hr)".to_string(),
            pattern: vec![regex("vikend")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "last/next/this weekend (hr)".to_string(),
            pattern: vec![regex("pro(s|š)li\\s+vikend|sljede(c|ć)i\\s+vikend|ovaj\\s+vikend")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.starts_with("pro") {
                    let mut t = TimeData::new(TimeForm::Weekend);
                    t.direction = Some(Direction::Past);
                    Some(TokenData::Time(t))
                } else if s.starts_with("sljede") {
                    let mut t = TimeData::new(TimeForm::Weekend);
                    t.direction = Some(Direction::Future);
                    Some(TokenData::Time(t))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))
                }
            }),
        },
        Rule {
            name: "ovo tromjesecje (hr)".to_string(),
            pattern: vec![regex("ovo\\s+tromjese(c|č)je")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 0 })))),
        },
        Rule {
            name: "sljedeci kvartal (hr)".to_string(),
            pattern: vec![regex("sljede(c|ć)i\\s+kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "treci kvartal (hr)".to_string(),
            pattern: vec![regex("tre(c|ć)i\\s+kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "3. kvartal (hr)".to_string(),
            pattern: vec![regex("3\\.?\\s+kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "trece tromjesecje (hr)".to_string(),
            pattern: vec![regex("tre(c|ć)e\\s+tromjese(c|č)je")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "3. tromjesečje (hr)".to_string(),
            pattern: vec![regex("3\\.?\\s+tromjese(c|č)je")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "4. kvartal 2018 (hr)".to_string(),
            pattern: vec![regex("4\\.?\\s+kvartal\\s*(\\d{4})")],
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
            name: "prethodni mjesec (hr)".to_string(),
            pattern: vec![regex("prethodni mjesec")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "sljedeci mjesec (hr)".to_string(),
            pattern: vec![regex("sljede(c|ć)i mjesec")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "ovaj kvartal (hr)".to_string(),
            pattern: vec![regex("ovaj kvartal")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 0 })))),
        },
        Rule {
            name: "last/this/next year (hr)".to_string(),
            pattern: vec![regex("pro(s|š)l(a|e)?\\s+godin(a|e|u)|prethodn(a|e)?\\s+godin(a|e|u)|ov(a|e)?\\s+godin(a|e|u)|sljede(c|ć)(a|e)?\\s+godin(a|e|u)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.starts_with("pro") || s.starts_with("pret") {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))
                } else if s.starts_with("ova") || s.starts_with("ove") {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 0 })))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))
                }
            }),
        },
        Rule {
            name: "until end of day/month/year (hr)".to_string(),
            pattern: vec![regex("do\\s+kraja\\s+((ovog\\s+)?dana|(ovog\\s+)?mjeseca|(ovog\\s+)?godine)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let target = if s.contains("dan") {
                    TimeForm::AllGrain(Grain::Day)
                } else if s.contains("mjesec") {
                    TimeForm::AllGrain(Grain::Month)
                } else {
                    TimeForm::AllGrain(Grain::Year)
                };
                let mut t = TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(target),
                });
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "until end of next month (hr)".to_string(),
            pattern: vec![regex("do\\s+kraja\\s+sljede(c|ć)eg\\s+mjeseca")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::GrainOffset {
                        grain: Grain::Month,
                        offset: 1,
                    }),
                });
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<month> <day>-<day> (hr)".to_string(),
            pattern: vec![regex("([[:alpha:]čćžšđ]+)\\s+([012]?\\d|30|31)\\s*(?:-|do)\\s*([012]?\\d|30|31)")],
            production: Box::new(|nodes| {
                let (m_s, d1_s, d2_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let month = parse_month_hr(&m_s)?;
                let d1: u32 = d1_s.parse().ok()?;
                let d2: u32 = d2_s.parse().ok()?;
                if !(1..=31).contains(&d1) || !(1..=31).contains(&d2) {
                    return None;
                }
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
            name: "<month> <day> - <month> <day> (hr)".to_string(),
            pattern: vec![regex("([[:alpha:]čćžšđ]+)\\s+([012]?\\d|30|31)\\s*[-]\\s*([[:alpha:]čćžšđ]+)\\s+([012]?\\d|30|31)")],
            production: Box::new(|nodes| {
                let (m1_s, d1_s, m2_s, d2_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (
                        rm.group(1)?.to_lowercase(),
                        rm.group(2)?,
                        rm.group(3)?.to_lowercase(),
                        rm.group(4)?,
                    ),
                    _ => return None,
                };
                let m1 = parse_month_hr(&m1_s)?;
                let m2 = parse_month_hr(&m2_s)?;
                let d1: u32 = d1_s.parse().ok()?;
                let d2: u32 = d2_s.parse().ok()?;
                if !(1..=31).contains(&d1) || !(1..=31).contains(&d2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month: m1, day: d1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month: m2, day: d2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "year (hr)".to_string(),
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
            name: "christmas (hr)".to_string(),
            pattern: vec![regex("bo(z|ž)i(c|ć)(a)?|zicbo")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "christmas day".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "new year's eve/day (hr)".to_string(),
            pattern: vec![regex("star(a|u|e)\\s+godin(a|e|u)|nov(a|u|e)\\s+godin(a|e|u)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                if s.starts_with("star") {
                    Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                        "new year's eve".to_string(),
                        None,
                    ))))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                        "new year's day".to_string(),
                        None,
                    ))))
                }
            }),
        },
        Rule {
            name: "holidays aliases (hr)".to_string(),
            pattern: vec![regex("badnjak(a)?|no(c|ć)\\s+vje(s|š)tica|valentinov(og|a|o)?|maj(c|č)in\\s+dan|dan\\s+(o(c|č)eva|tata)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let form = if s.starts_with("badnjak") {
                    TimeForm::Holiday("christmas eve".to_string(), None)
                } else if s.starts_with("no") {
                    TimeForm::Holiday("halloween day".to_string(), None)
                } else if s.starts_with("valent") {
                    TimeForm::Holiday("valentine's day".to_string(), None)
                } else if s.starts_with("maj") {
                    TimeForm::Holiday("mother's day".to_string(), None)
                } else {
                    TimeForm::Holiday("father's day".to_string(), None)
                };
                Some(TokenData::Time(TimeData::new(form)))
            }),
        },
        Rule {
            name: "<n> year after christmas (hr)".to_string(),
            pattern: vec![regex("(jedan|jednu|dva|dvije|tri|\\d+)\\s+godin[aeu]\\s+poslije\\s+bo(z|ž)i(c|ć)(a)?")],
            production: Box::new(|nodes| {
                let n_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let n = parse_hr_quantity(&n_s)?;
                let base = TimeData::new(TimeForm::Holiday("christmas day".to_string(), None));
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n,
                    grain: Grain::Year,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "season (hr)".to_string(),
            pattern: vec![regex("prolje(c|ć)e|ljeto|ljetos|jesen|zima|zimus")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let season = if s.starts_with("prol") {
                    0
                } else if s.starts_with("ljet") {
                    1
                } else if s.starts_with("jese") {
                    2
                } else {
                    3
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "this/next season (hr)".to_string(),
            pattern: vec![regex("(ov(og|o|e)|sljede(c|ć)(eg|e))\\s+(prolje(c|ć)e|ljeta|zime|jeseni)")],
            production: Box::new(|nodes| {
                let (q, s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?.to_lowercase(), rm.group(5)?.to_lowercase()),
                    _ => return None,
                };
                let season = if s.starts_with("prol") {
                    0
                } else if s.starts_with("ljet") {
                    1
                } else if s.starts_with("jese") {
                    2
                } else {
                    3
                };
                let mut t = TimeData::new(TimeForm::Season(season));
                if q.starts_with("sljede") {
                    t.direction = Some(Direction::Future);
                }
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<ordinal> day in <month> [year] (hr)".to_string(),
            pattern: vec![regex("(prvi|drugi|tre(c|ć)i|cetvrti|četvrti|peti|\\d{1,2})\\s+dan\\s+u\\s+([[:alpha:]čćžšđ]+)(?:\\s+(\\d{4}))?")],
            production: Box::new(|nodes| {
                let (ord_s, month_s, year_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.to_lowercase(), m.group(3)?.to_lowercase(), m.group(4)),
                    _ => return None,
                };
                let n = parse_ordinal_hr(&ord_s)? as u32;
                if !(1..=31).contains(&n) {
                    return None;
                }
                let month = parse_month_hr(&month_s)?;
                let year = year_s.and_then(|y| y.parse::<i32>().ok());
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day: n, year })))
            }),
        },
        Rule {
            name: "<ordinal> week in <month> [year] (hr)".to_string(),
            pattern: vec![regex("(prvi|drugi|tre(c|ć)i|cetvrti|četvrti|peti|\\d{1,2})\\s+tjedan\\s+u\\s+([[:alpha:]čćžšđ]+)(?:\\s+(\\d{4}))?")],
            production: Box::new(|nodes| {
                let (ord_s, month_s, year_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.to_lowercase(), m.group(3)?.to_lowercase(), m.group(4)),
                    _ => return None,
                };
                let n = parse_ordinal_hr(&ord_s)?;
                let month = parse_month_hr(&month_s)?;
                let year = year_s.and_then(|y| y.parse::<i32>().ok());
                let base = month_base(month, year);
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime {
                    n,
                    grain: Grain::Week,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "last day in <month> [year] (hr)".to_string(),
            pattern: vec![regex("zadnji\\s+dan\\s+u\\s+([[:alpha:]čćžšđ]+)(?:\\s+(\\d{4}))?")],
            production: Box::new(|nodes| {
                let (month_s, year_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.to_lowercase(), m.group(2)),
                    _ => return None,
                };
                let month = parse_month_hr(&month_s)?;
                let year = year_s.and_then(|y| y.parse::<i32>().ok());
                let base = month_base(month, year);
                Some(TokenData::Time(TimeData::new(TimeForm::NthLastDayOfTime {
                    n: 1,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "last week in <month> [year] (hr)".to_string(),
            pattern: vec![regex("zadnji\\s+tjedan\\s+u\\s+([[:alpha:]čćžšđ]+)(?:\\s+(\\d{4}))?")],
            production: Box::new(|nodes| {
                let (month_s, year_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.to_lowercase(), m.group(2)),
                    _ => return None,
                };
                let month = parse_month_hr(&month_s)?;
                let year = year_s.and_then(|y| y.parse::<i32>().ok());
                let base = month_base(month, year);
                Some(TokenData::Time(TimeData::new(TimeForm::LastCycleOfTime {
                    grain: Grain::Week,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "<ordinal> <dow> in <month> [year] (hr)".to_string(),
            pattern: vec![regex("(prvi|drugi|tre(c|ć)i|cetvrti|četvrti|peti|\\d{1,2})\\s+(ponedjelja?ka?|pon\\.?|utora?ka?|uto?\\.?|srijed(a|e|u)|sri\\.?|(č|c)etvrta?ka?|(č|c)et\\.?|peta?ka?|pet\\.?|subot(a|e|u)|sub?\\.?|nedjelj(a|e|u)|ned\\.?)\\s+u\\s+([[:alpha:]čćžšđ]+)(?:\\s+(\\d{4}))?")],
            production: Box::new(|nodes| {
                let (ord_s, dow_s, month_s, year_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (
                        m.group(1)?.to_lowercase(),
                        m.group(3)?.to_lowercase(),
                        m.group(10)?.to_lowercase(),
                        m.group(11),
                    ),
                    _ => return None,
                };
                let n = parse_ordinal_hr(&ord_s)?;
                let dow = parse_dow_hr(&dow_s)?;
                let month = parse_month_hr(&month_s)?;
                let year = year_s.and_then(|y| y.parse::<i32>().ok());
                let base = month_base(month, year);
                Some(TokenData::Time(TimeData::new(TimeForm::NthDOWOfTime {
                    n,
                    dow,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "last <dow> in <month> [year] (hr)".to_string(),
            pattern: vec![regex("zadnji\\s+(ponedjelja?ka?|pon\\.?|utora?ka?|uto?\\.?|srijed(a|e|u)|sri\\.?|(č|c)etvrta?ka?|(č|c)et\\.?|peta?ka?|pet\\.?|subot(a|e|u)|sub?\\.?|nedjelj(a|e|u)|ned\\.?)\\s+u\\s+([[:alpha:]čćžšđ]+)(?:\\s+(\\d{4}))?")],
            production: Box::new(|nodes| {
                let (dow_s, month_s, year_s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?.to_lowercase(), m.group(8)?.to_lowercase(), m.group(9)),
                    _ => return None,
                };
                let dow = parse_dow_hr(&dow_s)?;
                let month = parse_month_hr(&month_s)?;
                let year = year_s.and_then(|y| y.parse::<i32>().ok());
                let base = month_base(month, year);
                Some(TokenData::Time(TimeData::new(TimeForm::LastDOWOfTime {
                    dow,
                    base: Box::new(base),
                })))
            }),
        },
    ]);
    rules
}
