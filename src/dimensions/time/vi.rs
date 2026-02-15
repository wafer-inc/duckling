use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use crate::dimensions::time_grain::Grain;
use super::{Direction, PartOfDay, TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (vi)".to_string(),
            pattern: vec![regex(r"b[âa]y\s+gi[ờo]|ngay lúc này")],
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
            name: "2 giây vừa rồi (vi)".to_string(),
            pattern: vec![regex("2\\s+giây\\s+vừa\\s+rồi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Second, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "3 giây tới (vi)".to_string(),
            pattern: vec![regex("3\\s+(giây|s)\\s+(tới|tiếp\\s+theo)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Second, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "2 phút vừa rồi (vi)".to_string(),
            pattern: vec![regex("2\\s+phút\\s+vừa\\s+rồi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "3 phút tới (vi)".to_string(),
            pattern: vec![regex("3\\s+phút\\s+(tới|tiếp\\s+theo)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Minute, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "một tiếng vừa rồi (vi)".to_string(),
            pattern: vec![regex("một\\s+tiếng\\s+vừa\\s+rồi|1\\s+giờ\\s+vừa\\s+qua")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: -1 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "3 tiếng tiếp theo (vi)".to_string(),
            pattern: vec![regex("3\\s+(tiếng|giờ)\\s+(tiếp\\s+theo|tới)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Hour, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "2 ngày vừa rồi (vi)".to_string(),
            pattern: vec![regex("2\\s+ngày\\s+vừa\\s+(rồi|qua)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "3 ngày tới (vi)".to_string(),
            pattern: vec![regex("3\\s+ngày\\s+(tới|tiếp\\s+theo)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Day, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "2 tháng vừa rồi (vi)".to_string(),
            pattern: vec![regex("2\\s+tháng\\s+(vừa\\s+rồi|qua)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "3 tháng tới (vi)".to_string(),
            pattern: vec![regex("3\\s+tháng\\s+tới|ba\\s+tháng\\s+tiếp\\s+theo")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 3 })),
                false,
            ))))),
        },
        Rule {
            name: "2 năm vừa rồi (vi)".to_string(),
            pattern: vec![regex("2\\s+năm\\s+vừa\\s+rồi")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: -2 })),
                Box::new(TimeData::new(TimeForm::Now)),
                false,
            ))))),
        },
        Rule {
            name: "3 năm tới (vi)".to_string(),
            pattern: vec![regex("3\\s+năm\\s+(tới|tiếp\\s+theo)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                Box::new(TimeData::new(TimeForm::Now)),
                Box::new(TimeData::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 3 })),
                false,
            ))))),
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
    ]);
    rules
}
