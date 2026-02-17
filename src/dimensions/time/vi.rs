use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};
use crate::dimensions::time_grain::Grain;
use super::{Direction, PartOfDay, TimeData, TimeForm};

fn time_data(td: &TokenData) -> Option<&TimeData> {
    match td {
        TokenData::Time(d) => Some(d),
        _ => None,
    }
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

fn vi_small_number(s: &str) -> Option<i32> {
    match s.trim().to_lowercase().as_str() {
        "một" | "mot" | "1" => Some(1),
        "hai" | "2" => Some(2),
        "ba" | "3" => Some(3),
        _ => None,
    }
}

fn vi_unit_to_grain(s: &str) -> Option<Grain> {
    match s.trim().to_lowercase().as_str() {
        "s" | "giây" => Some(Grain::Second),
        "phút" => Some(Grain::Minute),
        "tiếng" | "giờ" => Some(Grain::Hour),
        "ngày" => Some(Grain::Day),
        "tháng" => Some(Grain::Month),
        "năm" => Some(Grain::Year),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (vi)".to_string(),
            pattern: vec![regex(r"b[âa]y\s+gi[ờo]|ngay lúc này")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "bây giờ (vi)".to_string(),
            pattern: vec![regex("b[âa]y\\s+gi[ờo]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (vi)".to_string(),
            pattern: vec![regex("h[oô]m nay|bữa nay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (vi)".to_string(),
            pattern: vec![regex("ng[aà]y mai")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (vi)".to_string(),
            pattern: vec![regex("h[oô]m qua")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day before yesterday (vi)".to_string(),
            pattern: vec![regex("hôm kia")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))
            }),
        },
        Rule {
            name: "ngày hôm qua (vi)".to_string(),
            pattern: vec![regex("ngày\\s+h[oô]m\\s+qua")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "ngày hôm kia (vi)".to_string(),
            pattern: vec![regex("ngày\\s+h[oô]m\\s+kia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))),
        },
        Rule {
            name: "hôm nay (vi)".to_string(),
            pattern: vec![regex("h[oô]m\\s+nay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "ngày mai (vi)".to_string(),
            pattern: vec![regex("ngày\\s+mai")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "tết dương (vi)".to_string(),
            pattern: vec![regex("tết\\s+dương")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 1,
                day: 1,
                year: None,
            })))),
        },
        Rule {
            name: "quốc khánh (vi)".to_string(),
            pattern: vec![regex("quốc\\s+khánh")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 9,
                day: 2,
                year: None,
            })))),
        },
        Rule {
            name: "<time> tới (vi)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("tới|tiếp\\s+theo")],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[0].token_data)?.clone();
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<time> này (vi)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("này")],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[0].token_data)?.clone();
                t.direction = Some(Direction::Future);
                t.latent = false;
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<time> trước (vi)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("trước")],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[0].token_data)?.clone();
                t.direction = Some(Direction::Past);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "day of week (vi)".to_string(),
            pattern: vec![regex("th(ứ) (2|hai)|th(ứ) (3|ba)|th(ứ) 4|th(ứ) b(ố)n|th(ứ) t(ư)|th(ứ) (5|n(ă)m)|th(ứ) 6|th(ứ) s(á)u|th(ứ) (7|b((ả)|(ẩ))y)|ch((ủ)|(ú)a) nh(ậ)t")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("2") || s.contains("hai") {
                    0
                } else if s.contains("3") || s.contains("ba") {
                    1
                } else if s.contains("4") {
                    2
                } else if s.contains("5") {
                    3
                } else if s.contains("6") {
                    4
                } else if s.contains("7") {
                    5
                } else if s.contains("nhật") || s.contains("nhat") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "tháng 6 (vi)".to_string(),
            pattern: vec![regex("tháng\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month: u32 = m.parse().ok()?;
                if !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "tháng sáu (vi)".to_string(),
            pattern: vec![regex("tháng\\s+s[áa]u")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(6))))),
        },
        Rule {
            name: "ngày đầu tiên của tháng ba (vi)".to_string(),
            pattern: vec![regex("ngày đầu tiên của tháng ba")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "mồng 3 tháng ba (vi)".to_string(),
            pattern: vec![regex("mồng\\s*(\\d{1,2})\\s+tháng\\s+(ba|3)")],
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
            name: "ngày 7 tháng ba (vi)".to_string(),
            pattern: vec![regex("ngày\\s*(\\d{1,2})\\s+tháng\\s+(ba|3)")],
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
            name: "tháng mười năm 2017 (vi)".to_string(),
            pattern: vec![regex("tháng\\s+mười\\s+năm\\s+(\\d{4})")],
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
            name: "cuối năm (vi)".to_string(),
            pattern: vec![regex("cu(ố|o)i\\s+n(ă|a)m")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "thứ năm tuần tới (vi)".to_string(),
            pattern: vec![regex("thứ năm tuần tới")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(3))))),
        },
        Rule {
            name: "thứ sáu tới (vi)".to_string(),
            pattern: vec![regex("thứ\\s*(sáu|6)\\s+tới")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(4))))),
        },
        Rule {
            name: "thứ sáu tuần này (vi)".to_string(),
            pattern: vec![regex("thứ\\s*(sáu|6)\\s+(của\\s+)?tuần\\s+này")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(4))))),
        },
        Rule {
            name: "thứ <dow> tuần tới (vi)".to_string(),
            pattern: vec![regex("thứ\\s*(hai|ba|tư|4|năm|5|sáu|6|bảy|7)\\s+(của\\s+)?tuần\\s+tới")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let dow = match s.as_str() {
                    "hai" => 0,
                    "ba" => 1,
                    "tư" | "4" => 2,
                    "năm" | "5" => 3,
                    "sáu" | "6" => 4,
                    "bảy" | "7" => 5,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "<day-of-week> tới (vi)".to_string(),
            pattern: vec![regex("(thứ\\s*(hai|ba|tư|4|năm|5|sáu|6|bảy|7)|ch((ủ)|(ú)a)\\s+nh(ậ|â)t)\\s+tới")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("hai") {
                    0
                } else if s.contains("ba") {
                    1
                } else if s.contains("tư") || s.contains("4") {
                    2
                } else if s.contains("năm") || s.contains("5") {
                    3
                } else if s.contains("sáu") || s.contains("6") {
                    4
                } else if s.contains("bảy") || s.contains("7") {
                    5
                } else {
                    6
                };
                let mut t = TimeData::new(TimeForm::DayOfWeek(dow));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "vào lúc 2 giờ sáng (vi)".to_string(),
            pattern: vec![regex("(vào\\s+)?lúc\\s+(\\d{1,2})\\s+giờ\\s+sáng")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 11 || hour == 0 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "lúc 3 giờ tối (vi)".to_string(),
            pattern: vec![regex("(vào\\s+)?lúc\\s+(\\d{1,2})\\s+giờ\\s+t[ốo]i")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour == 0 || hour > 12 {
                    return None;
                }
                let out = if hour < 12 { hour + 12 } else { 12 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out, 0, false))))
            }),
        },
        Rule {
            name: "vào lúc 3 giờ chiều (vi)".to_string(),
            pattern: vec![regex("(vào\\s+)?lúc\\s+(\\d{1,2})\\s+giờ\\s+chiều")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour == 0 || hour > 12 {
                    return None;
                }
                let out = if hour < 12 { hour + 12 } else { 12 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out, 0, false))))
            }),
        },
        Rule {
            name: "vào khoảng 3 giờ chiều (vi)".to_string(),
            pattern: vec![regex("(vào\\s+)?khoảng\\s+(\\d{1,2})\\s+giờ\\s+chiều")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour == 0 || hour > 12 {
                    return None;
                }
                let out = if hour < 12 { hour + 12 } else { 12 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out, 0, false))))
            }),
        },
        Rule {
            name: "3 giờ rưỡi chiều (vi)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+giờ\\s+rưỡi\\s+chiều")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour == 0 || hour > 12 {
                    return None;
                }
                let out = if hour < 12 { hour + 12 } else { 12 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out, 30, false))))
            }),
        },
        Rule {
            name: "3:30 chiều (vi)".to_string(),
            pattern: vec![regex("(\\d{1,2}):(\\d{2})\\s+chiều")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                if hour == 0 || hour > 12 || minute > 59 {
                    return None;
                }
                let out = if hour < 12 { hour + 12 } else { 12 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out, minute, false))))
            }),
        },
        Rule {
            name: "ba giờ rưỡi chiều (vi)".to_string(),
            pattern: vec![regex("ba\\s+giờ\\s+rưỡi\\s+chiều")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 30, false))))),
        },
        Rule {
            name: "hai giờ rưỡi (vi)".to_string(),
            pattern: vec![regex("hai\\s+giờ\\s+rưỡi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(14, 30, false))))),
        },
        Rule {
            name: "11 giờ kém 15 (vi)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+giờ\\s+kém\\s+(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let minus: u32 = m.parse().ok()?;
                if hour == 0 || hour > 12 || minus > 59 {
                    return None;
                }
                let out_hour = if hour == 1 { 12 } else { hour - 1 };
                let out_min = 60 - minus;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out_hour, out_min, false))))
            }),
        },
        Rule {
            name: "10 giờ 45 phút (vi)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*giờ\\s*(\\d{1,2})(\\s*phút)?")],
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
            name: "hh:mm (vi)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3])):(\\d{2})")],
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
            name: "at hh:mm (vi)".to_string(),
            pattern: vec![regex("(l(ú|u)c|v(à|a)o)(\\s+l(ú|u)c)?"), regex("((?:[01]?\\d)|(?:2[0-3]))[:.hg]([0-5]\\d)")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[1].token_data {
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
            name: "<time-of-day> am|pm (vi)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))(?::(\\d{2}))?\\s*(am|pm)")],
            production: Box::new(|nodes| {
                let (h, m_opt, ap) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2), rm.group(3)?),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                let minute: u32 = m_opt.unwrap_or("0").parse().ok()?;
                if ap.eq_ignore_ascii_case("pm") && hour < 12 {
                    hour += 12;
                } else if ap.eq_ignore_ascii_case("am") && hour == 12 {
                    hour = 0;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "<time-of-day> giờ đúng (vi)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("gi(ờ|o)\\s+(đ|d)(ú|u)ng")],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[0].token_data)?.clone();
                t.latent = false;
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<time-of-day> approximately (vi)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("g(ầ|a)n")],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[0].token_data)?.clone();
                t.latent = false;
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "exactly <time-of-day> (vi)".to_string(),
            pattern: vec![regex("(v(à|a)o\\s+)?(đ|d)(ú|u)ng"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[1].token_data)?.clone();
                t.latent = false;
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<time-of-day> sharp (vi)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("(đ|d)(ú|u)ng")],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[0].token_data)?.clone();
                t.latent = false;
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "about <time-of-day> (vi)".to_string(),
            pattern: vec![regex("(v(à|a)o\\s+)?kho(ả|a)ng"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[1].token_data)?.clone();
                t.latent = false;
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "<time-of-day> sáng|chiều|tối (vi)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("(s(á|a)ng|chi(ề|e)u|t(ố|o)i)")],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[0].token_data)?.clone();
                let p = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                if p.contains("chi") || p.contains("t") {
                    if let TimeForm::HourMinute(h, m, is12h) = t.form {
                        let hh = if h < 12 { h + 12 } else { h };
                        t.form = TimeForm::HourMinute(hh, m, is12h);
                    }
                }
                t.latent = false;
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "time-of-day giờ (vi)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s*giờ(\\s+đúng)?")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "time-of-day (latent) (vi)".to_string(),
            pattern: vec![regex("\\b((?:[01]?\\d)|(?:2[0-3]))h\\b")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let mut td = TimeData::new(TimeForm::HourMinute(hour, 0, false));
                td.latent = true;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "10g45/10h45 (vi)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*[gh](\\d{1,2})")],
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
            name: "hh:mm:ss (vi)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3])):(\\d{2}):(\\d{2})")],
            production: Box::new(|nodes| {
                let (h, m, s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                let mm: u32 = m.parse().ok()?;
                let ss: u32 = s.parse().ok()?;
                if hh > 23 || mm > 59 || ss > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinuteSecond(hh, mm, ss))))
            }),
        },
        Rule {
            name: "ngày dd/mm/yyyy (vi)".to_string(),
            pattern: vec![regex("ngày\\s+(\\d{1,2})/(\\d{1,2})/(\\d{2,4})")],
            production: Box::new(|nodes| {
                let (d, m, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let mut year: i32 = y.parse().ok()?;
                if y.len() == 2 {
                    year += if year < 50 { 2000 } else { 1900 };
                }
                if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: Some(year),
                })))
            }),
        },
        Rule {
            name: "ngày dd/mm (vi)".to_string(),
            pattern: vec![regex("ng(à|a)y\\s+(\\d{1,2})/(\\d{1,2})")],
            production: Box::new(|nodes| {
                let (d, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(2)?, rm.group(3)?),
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
            name: "dd/mm (vi)".to_string(),
            pattern: vec![regex("(3[01]|[12]\\d|0?[1-9])/(0?[1-9]|1[0-2])")],
            production: Box::new(|nodes| {
                let (d, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "dd/mm/yyyy (vi)".to_string(),
            pattern: vec![regex("(3[01]|[12]\\d|0?[1-9])/(1[0-2]|0?[1-9])/(\\d{2,4})")],
            production: Box::new(|nodes| {
                let (d, m, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let mut year: i32 = y.parse().ok()?;
                if y.len() == 2 {
                    year += if year < 50 { 2000 } else { 1900 };
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "yyyy-mm-dd (vi)".to_string(),
            pattern: vec![regex("(\\d{2,4})-(0?[1-9]|1[0-2])-(3[01]|[12]\\d|0?[1-9])")],
            production: Box::new(|nodes| {
                let (y, m, d) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let month: u32 = m.parse().ok()?;
                let day: u32 = d.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "<time> timezone (vi)".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex("\\b(YEKT|YEKST|YAKT|YAKST|WITA|WIT|WIB|WGT|WGST|WFT|WET|WEST|WAT|WAST|VUT|VLAT|VLAST|VET|UZT|UYT|UYST|UTC|ULAT|TVT|TMT|TLT|TKT|TJT|TFT|TAHT|SST|SRT|SGT|SCT|SBT|SAST|SAMT|RET|PYT|PYST|PWT|PST|PONT|PMST|PMDT|PKT|PHT|PHOT|PGT|PETT|PETST|PET|PDT|OMST|OMSST|NZST|NZDT|NUT|NST|NPT|NOVT|NOVST|NFT|NDT|NCT|MYT|MVT|MUT|MST|MSK|MSD|MMT|MHT|MDT|MAWT|MART|MAGT|MAGST|LINT|LHST|LHDT|KUYT|KST|KRAT|KRAST|KGT|JST|IST|IRST|IRKT|IRKST|IRDT|IOT|IDT|ICT|HOVT|HKT|GYT|GST|GMT|GILT|GFT|GET|GAMT|GALT|FNT|FKT|FKST|FJT|FJST|EST|EGT|EGST|EET|EEST|EDT|ECT|EAT|EAST|EASST|DAVT|ChST|CXT|CVT|CST|COT|CLT|CLST|CKT|CHAST|CHADT|CET|CEST|CDT|CCT|CAT|CAST|BTT|BST|BRT|BRST|BOT|BNT|AZT|AZST|AZOT|AZOST|AWST|AWDT|AST|ART|AQTT|ANAT|ANAST|AMT|AMST|ALMT|AKST|AKDT|AFT|AEST|AEDT|ADT|ACST|ACDT)\\b"),
            ],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[0].token_data)?.clone();
                if !is_time_of_day(&nodes[0].token_data) {
                    return None;
                }
                let tz = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_uppercase(),
                    _ => return None,
                };
                t.timezone = Some(tz);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "8 giờ tối nay (vi)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+giờ\\s+tối\\s+nay")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour == 0 || hour > 12 {
                    return None;
                }
                let out = if hour < 12 { hour + 12 } else { 12 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out, 0, false))))
            }),
        },
        Rule {
            name: "mùa hè này (vi)".to_string(),
            pattern: vec![regex("mùa\\s+hè\\s+này|mùa\\s+hè\\s+năm\\s+nay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Season(1))))),
        },
        Rule {
            name: "mùa đông này (vi)".to_string(),
            pattern: vec![regex("mùa\\s+đông\\s+này|mùa\\s+đông\\s+năm\\s+nay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Season(3))))),
        },
        Rule {
            name: "tối nay (vi)".to_string(),
            pattern: vec![regex("tối\\s+(nay|h[oô]m\\s+nay)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Today)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "tối mai (vi)".to_string(),
            pattern: vec![regex("tối\\s+(mai|ngày\\s+mai)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Tomorrow)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "trưa mai (vi)".to_string(),
            pattern: vec![regex("trưa\\s+(mai|ngày\\s+mai)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Tomorrow)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Lunch))),
            ))))),
        },
        Rule {
            name: "tối qua (vi)".to_string(),
            pattern: vec![regex("tối\\s+(qua|h[oô]m\\s+qua)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Yesterday)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "sáng chủ nhật (vi)".to_string(),
            pattern: vec![regex("sáng\\s+ch((ủ)|(ú)a)\\s+nh(ậ|â)t")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::DayOfWeek(6))),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))),
            ))))),
        },
        Rule {
            name: "N unit vừa rồi/vừa qua (vi)".to_string(),
            pattern: vec![regex("(một|mot|hai|ba|\\d{1,2})\\s*(s|giây|phút|tiếng|giờ|ngày|tháng|năm)\\s+((vừa\\s+(rồi|qua))|qua)")],
            production: Box::new(|nodes| {
                let (ns, us) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n = vi_small_number(ns).or_else(|| ns.parse::<i32>().ok())?;
                let grain = vi_unit_to_grain(us)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::GrainOffset { grain, offset: -n })),
                    Box::new(TimeData::new(TimeForm::Now)),
                    false,
                ))))
            }),
        },
        Rule {
            name: "N unit tới/tiếp theo (vi)".to_string(),
            pattern: vec![regex("(một|mot|hai|ba|\\d{1,2})\\s*(s|giây|phút|tiếng|giờ|ngày|tháng|năm)\\s+(tới|tiếp\\s+theo)")],
            production: Box::new(|nodes| {
                let (ns, us) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let n = vi_small_number(ns).or_else(|| ns.parse::<i32>().ok())?;
                let grain = vi_unit_to_grain(us)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::Now)),
                    Box::new(TimeData::new(TimeForm::GrainOffset { grain, offset: n })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "sau bữa trưa (vi)".to_string(),
            pattern: vec![regex("sau\\s+b(ữ|u)a\\s+trưa")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Today)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))),
            ))))),
        },
        Rule {
            name: "after lunch (vi)".to_string(),
            pattern: vec![regex("sau\\s+b(ữ|u)a\\s+trưa")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))))),
        },
        Rule {
            name: "after work (vi)".to_string(),
            pattern: vec![regex("tan\\s+ca|sau\\s+giờ\\s+làm")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))))),
        },
        Rule {
            name: "morning (vi)".to_string(),
            pattern: vec![regex("sáng")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))))),
        },
        Rule {
            name: "afternoon (vi)".to_string(),
            pattern: vec![regex("chiều")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))))),
        },
        Rule {
            name: "lunch (vi)".to_string(),
            pattern: vec![regex("trưa")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Lunch))))),
        },
        Rule {
            name: "noon (vi)".to_string(),
            pattern: vec![regex("buổi\\s+trưa|giữa\\s+trưa")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(12, 0, false))))),
        },
        Rule {
            name: "tonight (vi)".to_string(),
            pattern: vec![regex("tối\\s+nay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Today)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "buổi sáng nay (vi)".to_string(),
            pattern: vec![regex("buổi\\s+sáng\\s+nay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Today)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))),
            ))))),
        },
        Rule {
            name: "thứ hai tới (vi)".to_string(),
            pattern: vec![regex("thứ\\s*(hai|2)\\s+tới")],
            production: Box::new(|_| {
                let mut t = TimeData::new(TimeForm::DayOfWeek(0));
                t.direction = Some(Direction::Future);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "tháng 4 / tháng tư (vi)".to_string(),
            pattern: vec![regex("tháng\\s*(4|tư)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(4))))),
        },
        Rule {
            name: "giáng sinh (vi)".to_string(),
            pattern: vec![regex("(ngày\\s+)?giáng\\s+sinh")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "christmas day".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "4pm CET (vi)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*pm\\s*CET")],
            production: Box::new(|nodes| {
                let hs = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let h: u32 = hs.parse().ok()?;
                if h == 0 || h > 12 {
                    return None;
                }
                let mut t = TimeData::new(TimeForm::HourMinute(if h < 12 { h + 12 } else { 12 }, 0, false));
                t.timezone = Some("CET".to_string());
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "hôm nay lúc 2 giờ chiều (vi)".to_string(),
            pattern: vec![regex("(h[oô]m\\s+nay\\s+)?lúc\\s+(\\d{1,2})\\s+giờ\\s+chiều")],
            production: Box::new(|nodes| {
                let hs = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let h: u32 = hs.parse().ok()?;
                if h == 0 || h > 12 {
                    return None;
                }
                let hour = if h < 12 { h + 12 } else { 12 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 0, false))))
            }),
        },
        Rule {
            name: "vào đúng 3 giờ chiều (vi)".to_string(),
            pattern: vec![regex("(vào\\s+)?đúng\\s+(\\d{1,2})\\s+giờ\\s+chiều")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour == 0 || hour > 12 {
                    return None;
                }
                let out = if hour < 12 { hour + 12 } else { 12 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(out, 0, false))))
            }),
        },
        Rule {
            name: "tuần này (vi)".to_string(),
            pattern: vec![regex("tuần này")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))),
        },
        Rule {
            name: "tuần trước (vi)".to_string(),
            pattern: vec![regex("tuần trước")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: -1 })))),
        },
        Rule {
            name: "tuần sau (vi)".to_string(),
            pattern: vec![regex("tuần sau")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Week, offset: 1 })))),
        },
        Rule {
            name: "tháng trước (vi)".to_string(),
            pattern: vec![regex("tháng trước")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -1 })))),
        },
        Rule {
            name: "cuối tháng (vi)".to_string(),
            pattern: vec![regex("cu(ố|o)i\\s+th(á|a)ng")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 1 })))),
        },
        Rule {
            name: "năm trước (vi)".to_string(),
            pattern: vec![regex("năm trước")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "năm ngoái (vi)".to_string(),
            pattern: vec![regex("năm ngoái")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -1 })))),
        },
        Rule {
            name: "năm nay (vi)".to_string(),
            pattern: vec![regex("năm nay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Year)))),
            ),
        },
        Rule {
            name: "năm sau (vi)".to_string(),
            pattern: vec![regex("năm sau")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "quý này (vi)".to_string(),
            pattern: vec![regex("quý này|quý nay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 0 })))),
        },
        Rule {
            name: "quý hiện tại (vi)".to_string(),
            pattern: vec![regex("quý hiện tại")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 0 })))),
        },
        Rule {
            name: "quý tới (vi)".to_string(),
            pattern: vec![regex("quý tới")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "quý tiếp (vi)".to_string(),
            pattern: vec![regex("quý tiếp")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "quý 4 của năm 2018 (vi)".to_string(),
            pattern: vec![regex("quý\\s*4\\s+của\\s+năm\\s*(\\d{4})")],
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
            name: "quý sau (vi)".to_string(),
            pattern: vec![regex("quý sau")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Quarter, offset: 1 })))),
        },
        Rule {
            name: "quý 3 (vi)".to_string(),
            pattern: vec![regex("quý\\s*3")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "quý ba (vi)".to_string(),
            pattern: vec![regex("quý\\s+ba")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "quý 4 năm 2018 (vi)".to_string(),
            pattern: vec![regex("quý\\s*4\\s+năm\\s*(\\d{4})")],
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
            name: "năm tiếp theo (vi)".to_string(),
            pattern: vec![regex("năm tiếp theo|năm kế tiếp|năm tới")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 1 })))),
        },
        Rule {
            name: "next <cycle> (vi)".to_string(),
            pattern: vec![regex("(tuần|tháng|năm|quý)\\s+(tới|sau|tiếp\\s+theo)")],
            production: Box::new(|nodes| {
                let g = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = if g == "tuần" { Grain::Week } else if g == "tháng" { Grain::Month } else if g == "năm" { Grain::Year } else { Grain::Quarter };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: 1 })))
            }),
        },
        Rule {
            name: "last <cycle> (vi)".to_string(),
            pattern: vec![regex("(tuần|tháng|năm|quý)\\s+trước")],
            production: Box::new(|nodes| {
                let g = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = if g == "tuần" { Grain::Week } else if g == "tháng" { Grain::Month } else if g == "năm" { Grain::Year } else { Grain::Quarter };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: -1 })))
            }),
        },
        Rule {
            name: "this <cycle> (vi)".to_string(),
            pattern: vec![regex("(tuần|tháng|năm|quý)\\s+này")],
            production: Box::new(|nodes| {
                let g = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = if g == "tuần" { Grain::Week } else if g == "tháng" { Grain::Month } else if g == "năm" { Grain::Year } else { Grain::Quarter };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset { grain, offset: 0 })))
            }),
        },
        Rule {
            name: "next n <cycle> (vi)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+(giây|phút|giờ|ngày|tháng|năm)\\s+(tới|tiếp\\s+theo)")],
            production: Box::new(|nodes| {
                let (n, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let nn: i64 = n.parse().ok()?;
                let grain = vi_unit_to_grain(u)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain { n: nn, grain, past: false, interval: true })))
            }),
        },
        Rule {
            name: "last n <cycle> (vi)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+(giây|phút|giờ|ngày|tháng|năm)\\s+(vừa\\s+qua|trước)")],
            production: Box::new(|nodes| {
                let (n, u) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let nn: i64 = n.parse().ok()?;
                let grain = vi_unit_to_grain(u)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain { n: nn, grain, past: true, interval: true })))
            }),
        },
        Rule {
            name: "quarter <number> (vi)".to_string(),
            pattern: vec![regex("quý\\s*(1|2|3|4)")],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data { TokenData::RegexMatch(m) => m.group(1)?, _ => return None };
                let qq: u32 = q.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Quarter(qq))))
            }),
        },
        Rule {
            name: "quarter <number> <year> (vi)".to_string(),
            pattern: vec![regex("quý\\s*(1|2|3|4)\\s+năm\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let (q, y) = match &nodes[0].token_data { TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?), _ => return None };
                let qq: u32 = q.parse().ok()?;
                let yy: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(qq, yy))))
            }),
        },
        Rule {
            name: "quarter <number> of <year> (vi)".to_string(),
            pattern: vec![regex("quý\\s*(1|2|3|4)\\s+của\\s+năm\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let (q, y) = match &nodes[0].token_data { TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?), _ => return None };
                let qq: u32 = q.parse().ok()?;
                let yy: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(qq, yy))))
            }),
        },
        Rule {
            name: "month (numeric with month symbol) (vi)".to_string(),
            pattern: vec![regex("tháng\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let ms = match &nodes[0].token_data { TokenData::RegexMatch(m) => m.group(1)?, _ => return None };
                let month: u32 = ms.parse().ok()?;
                if !(1..=12).contains(&month) { return None; }
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "year (numeric with year symbol) (vi)".to_string(),
            pattern: vec![regex("năm\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let ys = match &nodes[0].token_data { TokenData::RegexMatch(m) => m.group(1)?, _ => return None };
                let year: i32 = ys.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        Rule {
            name: "lễ tình nhân (vi)".to_string(),
            pattern: vec![regex("lễ\\s+tình\\s+nhân|valentine")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 14, year: None })))),
        },
        Rule {
            name: "quốc tế lao động (vi)".to_string(),
            pattern: vec![regex("quốc\\s+tế\\s+lao\\s+động")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day: 1, year: None })))),
        },
        Rule {
            name: "cách mạng tháng 8 (vi)".to_string(),
            pattern: vec![regex("cách\\s+mạng\\s+tháng\\s+8")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 8, day: 19, year: None })))),
        },
        Rule {
            name: "intersect (vi)".to_string(),
            pattern: vec![dim(DimensionKind::Time), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?.clone();
                let t2 = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(Box::new(t1), Box::new(t2)))))
            }),
        },
        Rule {
            name: "intersect by \\ (vi)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("c(ủ|u)a|trong"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?.clone();
                let t2 = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(Box::new(t1), Box::new(t2)))))
            }),
        },
        Rule {
            name: "week-end (vi)".to_string(),
            pattern: vec![regex("(cu(ố|o)i|h(ế|e)t)\\s+tu(ầ|a)n")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "<day-of-month> (numeric with day symbol) <named-month> (vi)".to_string(),
            pattern: vec![regex("ngày\\s*(\\d{1,2})\\s+tháng\\s*(\\d{1,2})")],
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
            name: "<day-of-month> <named-month> (vi)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+tháng\\s*(\\d{1,2}|một|hai|ba|tư|bốn|năm|sáu|bảy|tám|chín|mười|mười một|mười hai)")],
            production: Box::new(|nodes| {
                let (d, ms) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let day: u32 = d.parse().ok()?;
                let month: u32 = match ms {
                    "một" => 1,
                    "hai" => 2,
                    "ba" => 3,
                    "tư" | "bốn" => 4,
                    "năm" => 5,
                    "sáu" => 6,
                    "bảy" => 7,
                    "tám" => 8,
                    "chín" => 9,
                    "mười" => 10,
                    "mười một" => 11,
                    "mười hai" => 12,
                    _ => ms.parse().ok()?,
                };
                if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "<part-of-day> (hôm )?nay (vi)".to_string(),
            pattern: vec![regex("(sáng|chiều|tối|trưa)\\s+(h[oô]m\\s+)?nay")],
            production: Box::new(|nodes| {
                let p = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let pod = if p == "sáng" {
                    PartOfDay::Morning
                } else if p == "chiều" {
                    PartOfDay::Afternoon
                } else if p == "trưa" {
                    PartOfDay::Lunch
                } else {
                    PartOfDay::Evening
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(TimeData::new(TimeForm::PartOfDay(pod))),
                ))))
            }),
        },
        Rule {
            name: "<part-of-day> <time> (vi)".to_string(),
            pattern: vec![regex("(sáng|chiều|tối|trưa)"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let p = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let pod = if p == "sáng" {
                    PartOfDay::Morning
                } else if p == "chiều" {
                    PartOfDay::Afternoon
                } else if p == "trưa" {
                    PartOfDay::Lunch
                } else {
                    PartOfDay::Evening
                };
                let t = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t),
                    Box::new(TimeData::new(TimeForm::PartOfDay(pod))),
                ))))
            }),
        },
        Rule {
            name: "season (vi)".to_string(),
            pattern: vec![regex("mùa\\s+(xuân|hè|thu|đông)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let season = if s == "xuân" {
                    0
                } else if s == "hè" {
                    1
                } else if s == "thu" {
                    2
                } else {
                    3
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(season))))
            }),
        },
        Rule {
            name: "<hour-of-day> <integer> (vi)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+giờ\\s+(\\d{1,2})\\b")],
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
            name: "<hour-of-day> <integer> phút (vi)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+giờ\\s+(\\d{1,2})\\s+phút")],
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
            name: "(hour-of-day) quarter (vi)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+giờ\\s+một\\s+phần\\s+tư")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 15, false))))
            }),
        },
        Rule {
            name: "(hour-of-day) half (vi)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+giờ\\s+rưỡi")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 30, false))))
            }),
        },
        Rule {
            name: "<ordinal> <cycle> of <time> (vi)".to_string(),
            pattern: vec![regex("(thứ\\s*(nhất|hai|ba|tư|năm)|đầu\\s+tiên)"), regex("(tuần|tháng|quý)"), regex("của"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let idx = if n.contains("nhất") || n.contains("đầu") { 0 } else if n.contains("hai") { 1 } else if n.contains("ba") { 2 } else if n.contains("tư") { 3 } else { 4 };
                let g = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = if g == "tuần" { Grain::Week } else if g == "tháng" { Grain::Month } else { Grain::Quarter };
                let base = time_data(&nodes[3].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime { n: idx, grain, base: Box::new(base) })))
            }),
        },
        Rule {
            name: "<day-of-week> cuối cùng của <time> (vi)".to_string(),
            pattern: vec![regex("(thứ\\s*(hai|ba|tư|năm|sáu|bảy)|ch((ủ)|(ú)a)\\s+nh(ậ|â)t)"), regex("cuối\\s+cùng\\s+của"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("hai") { 0 } else if s.contains("ba") { 1 } else if s.contains("tư") { 2 } else if s.contains("năm") { 3 } else if s.contains("sáu") { 4 } else if s.contains("bảy") { 5 } else { 6 };
                let base = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::LastDOWOfTime { dow, base: Box::new(base) })))
            }),
        },
        Rule {
            name: "<cycle> cuối cùng của <time> (vi)".to_string(),
            pattern: vec![regex("(tuần|tháng|quý|năm)"), regex("cuối\\s+cùng\\s+của"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let g = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = if g == "tuần" { Grain::Week } else if g == "tháng" { Grain::Month } else if g == "quý" { Grain::Quarter } else { Grain::Year };
                let base = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::LastCycleOfTime { grain, base: Box::new(base) })))
            }),
        },
        Rule {
            name: "hhmm (military) am|pm (vi)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))([0-5]\\d)\\s*(am|pm)")],
            production: Box::new(|nodes| {
                let (h, m, ap) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                if ap.eq_ignore_ascii_case("pm") && hour < 12 {
                    hour += 12;
                } else if ap.eq_ignore_ascii_case("am") && hour == 12 {
                    hour = 0;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "hhmm (military) sáng|chiều|tối (vi)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))([0-5]\\d)\\s*(sáng|chiều|tối)")],
            production: Box::new(|nodes| {
                let (h, m, p) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?, rm.group(3)?),
                    _ => return None,
                };
                let mut hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                if p == "chiều" || p == "tối" {
                    if hour < 12 { hour += 12; }
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
    ]);
    rules
}
