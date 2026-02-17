use crate::dimensions::time_grain::Grain;
use crate::dimensions::numeral::helpers::{is_natural, numeral_data};
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};
use super::{Direction, IntervalDirection, PartOfDay, TimeData, TimeForm};

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

fn is_day_of_week(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Time(TimeData {
            form: TimeForm::DayOfWeek(..),
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

fn is_month(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Time(TimeData {
            form: TimeForm::Month(..),
            ..
        })
    )
}

fn ru_month_num(s: &str) -> Option<u32> {
    let t = s
        .trim()
        .to_lowercase()
        .replace('ё', "е")
        .replace('.', "");
    if t.starts_with("январ") || t.starts_with("янв") {
        Some(1)
    } else if t.starts_with("феврал") || t.starts_with("фев") {
        Some(2)
    } else if t.starts_with("март") || t.starts_with("мар") {
        Some(3)
    } else if t.starts_with("апрел") || t.starts_with("апр") {
        Some(4)
    } else if t == "май" || t == "мая" {
        Some(5)
    } else if t.starts_with("июн") {
        Some(6)
    } else if t.starts_with("июл") {
        Some(7)
    } else if t.starts_with("август") || t.starts_with("авг") {
        Some(8)
    } else if t.starts_with("сентябр") || t.starts_with("сен") {
        Some(9)
    } else if t.starts_with("октябр") || t.starts_with("окт") {
        Some(10)
    } else if t.starts_with("ноябр") || t.starts_with("ноя") {
        Some(11)
    } else if t.starts_with("декабр") || t.starts_with("дек") {
        Some(12)
    } else {
        None
    }
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (ru)".to_string(),
            pattern: vec![regex("сейчас")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (ru)".to_string(),
            pattern: vec![regex("сегодня")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (ru)".to_string(),
            pattern: vec![regex("завтра")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (ru)".to_string(),
            pattern: vec![regex("вчера")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (ru)".to_string(),
            pattern: vec![regex("понедельник(а)?|пн|вторник(а)?|вт|сред(а|у)|ср|четверг(а)?|чт|пятниц(а|у)|пт|суббот(а|у)|сб|воскресенье|вс|в\\s+пятницу")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.trim(),
                    _ => return None,
                };
                let dow = match s {
                    "понедельник" | "понедельника" | "пн" => 0,
                    "вторник" | "вторника" | "вт" => 1,
                    "среда" | "среду" | "ср" => 2,
                    "четверг" | "четверга" | "чт" => 3,
                    "пятница" | "пятницу" | "пт" | "в пятницу" => 4,
                    "суббота" | "субботу" | "сб" => 5,
                    "воскресенье" | "вс" => 6,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "this|next <day-of-week> (ru)".to_string(),
            pattern: vec![regex("(эт(от|а|и|у)|следующ(ий|ая|ие|ую))"), predicate(is_day_of_week)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Future);
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "absorption of , after named day (ru)".to_string(),
            pattern: vec![predicate(is_day_of_week), regex(",")],
            production: Box::new(|nodes| Some(nodes[0].token_data.clone())),
        },
        Rule {
            name: "last <time> (ru)".to_string(),
            pattern: vec![regex("(на |в )?прошл(ый|ого|ому|ом|ое|ые|ых|ыми|ым|ая|ой|ую)"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Past);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "this <time> (ru)".to_string(),
            pattern: vec![regex("(на |в )?эт(от|а|и|у|ом)"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Future);
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "on <date> (ru)".to_string(),
            pattern: vec![regex("(на|в)"), predicate(is_not_latent_time)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Time(td) => Some(TokenData::Time(td.clone())),
                _ => None,
            }),
        },
        Rule {
            name: "lunch (ru)".to_string(),
            pattern: vec![regex("обед(а|ом)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Lunch))))),
        },
        Rule {
            name: "afternoon (ru)".to_string(),
            pattern: vec![regex("после\\s+обеда")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Afternoon))))),
        },
        Rule {
            name: "evening (ru)".to_string(),
            pattern: vec![regex("вечер(а|ом)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))))),
        },
        Rule {
            name: "morning (ru)".to_string(),
            pattern: vec![regex("утр(о|а|ом)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Morning))))),
        },
        Rule {
            name: "night (ru)".to_string(),
            pattern: vec![regex("ноч(ь|и|ью)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(PartOfDay::Night))))),
        },
        Rule {
            name: "this <part-of-day> (ru)".to_string(),
            pattern: vec![regex("эт(им|ой)"), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let p = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(p),
                ))))
            }),
        },
        Rule {
            name: "<time> <part-of-day> (ru)".to_string(),
            pattern: vec![dim(DimensionKind::Time), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?.clone();
                let p = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(Box::new(t), Box::new(p)))))
            }),
        },
        Rule {
            name: "between <time-of-day> and <time-of-day> (interval) (ru)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("(и|до)"), predicate(is_time_of_day)],
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
            name: "<time-of-day> - <time-of-day> (interval) (ru)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("\\-|/"), predicate(is_time_of_day)],
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
            name: "<datetime> - <datetime> (interval) (ru)".to_string(),
            pattern: vec![predicate(is_not_latent_time), regex("\\-|до|по"), predicate(is_not_latent_time)],
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
            name: "intersect by ',' (ru)".to_string(),
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
            name: "intersect (ru)".to_string(),
            pattern: vec![predicate(is_not_latent_time), predicate(is_not_latent_time)],
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
            name: "at <time-of-day> (ru)".to_string(),
            pattern: vec![regex("в"), predicate(is_time_of_day)],
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
            name: "until <time-of-day> (ru)".to_string(),
            pattern: vec![regex("(до|к)"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[1].token_data)?.clone();
                t.open_interval_direction = Some(IntervalDirection::Before);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "after <time-of-day> (ru)".to_string(),
            pattern: vec![regex("после"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let mut t = time_data(&nodes[1].token_data)?.clone();
                t.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(t))
            }),
        },
        Rule {
            name: "week (ru)".to_string(),
            pattern: vec![regex("(вся\\s+неделя|эта\\s+неделя|остаток\\s+недели)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                if s.contains("остаток") {
                    Some(TokenData::Time(TimeData::new(TimeForm::RestOfGrain(Grain::Week))))
                } else {
                    Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(Grain::Week))))
                }
            }),
        },
        Rule {
            name: "week-end (ru)".to_string(),
            pattern: vec![regex("выходн(ые|ой)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "<named-month> (ru)".to_string(),
            pattern: vec![regex("январ(ь|я)|янв\\.?|феврал(ь|я)|фев\\.?|март(а)?|мар\\.?|апрел(ь|я)|апр\\.?|ма(й|я)|июн(ь|я)|июн\\.?|июл(ь|я)|июл\\.?|август(а)?|авг\\.?|сентябр(ь|я)|сен\\.?|октябр(ь|я)|окт\\.?|ноябр(ь|я)?|ноя\\.?|декабр(ь|я)|дек\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let month = ru_month_num(s)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "<day-of-month> (non ordinal) <named-month> (ru)".to_string(),
            pattern: vec![
                regex("(\\d{1,2})(?:-?(?:е|го|ого|ое))?"),
                predicate(is_month),
            ],
            production: Box::new(|nodes| {
                let day_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = day_s.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                let month = match &nodes[1].token_data {
                    TokenData::Time(TimeData {
                        form: TimeForm::Month(m),
                        ..
                    }) => *m,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "<named-month> <day-of-month> (non ordinal) (ru)".to_string(),
            pattern: vec![
                predicate(is_month),
                regex("(\\d{1,2})(?:-?(?:е|го|ого|ое))?"),
            ],
            production: Box::new(|nodes| {
                let month = match &nodes[0].token_data {
                    TokenData::Time(TimeData {
                        form: TimeForm::Month(m),
                        ..
                    }) => *m,
                    _ => return None,
                };
                let day_s = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = day_s.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "mm/dd (ru)".to_string(),
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
            name: "yyyy-mm-dd (ru)".to_string(),
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
            name: "dd.(mm.)? - dd.mm.(yy[yy]?)? (interval) (ru)".to_string(),
            pattern: vec![regex("(?:с\\s+)?(10|20|30|31|[012]?[1-9])(?:\\.(10|11|12|0?[1-9]))?\\.?\\s*(?:\\-|/|по)\\s*(10|20|30|31|[012]?[1-9])\\.(10|11|12|0?[1-9])\\.?(?:\\.(\\d{2,4}))?")],
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
                let month1: u32 = if let Some(m1) = m1_opt {
                    m1.parse().ok()?
                } else {
                    month2
                };
                let start = if let Some(ys) = y_opt {
                    let mut year: i32 = ys.parse().ok()?;
                    if ys.len() == 2 {
                        year += if year < 50 { 2000 } else { 1900 };
                    }
                    TimeData::new(TimeForm::DateMDY { month: month1, day: day1, year: Some(year) })
                } else {
                    TimeData::new(TimeForm::DateMDY { month: month1, day: day1, year: None })
                };
                let end = if let Some(ys) = y_opt {
                    let mut year: i32 = ys.parse().ok()?;
                    if ys.len() == 2 {
                        year += if year < 50 { 2000 } else { 1900 };
                    }
                    TimeData::new(TimeForm::DateMDY { month: month2, day: day2, year: Some(year) })
                } else {
                    TimeData::new(TimeForm::DateMDY { month: month2, day: day2, year: None })
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(start), Box::new(end), true))))
            }),
        },
        Rule {
            name: "<month> dd-dd (interval) (ru)".to_string(),
            pattern: vec![
                regex("(?:с\\s+)?([012]?\\d|30|31)(?:го|\\.)?"),
                regex("(\\-|по|до)"),
                regex("([012]?\\d|30|31)(?:ое|\\.)?"),
                predicate(is_month),
            ],
            production: Box::new(|nodes| {
                let d1 = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let d2 = match &nodes[2].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let month = match &nodes[3].token_data {
                    TokenData::Time(TimeData {
                        form: TimeForm::Month(m),
                        ..
                    }) => *m,
                    _ => return None,
                };
                let day1: u32 = d1.parse().ok()?;
                let day2: u32 = d2.parse().ok()?;
                if !(1..=31).contains(&day1) || !(1..=31).contains(&day2) {
                    return None;
                }
                let from = TimeData::new(TimeForm::DateMDY { month, day: day1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: day2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(Box::new(from), Box::new(to), true))))
            }),
        },
        Rule {
            name: "hh:mm (ru)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))[:.ч]([0-5]\\d)(?:час(ов|а|у)?|ч)?")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                let mm: u32 = m.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, mm, false))))
            }),
        },
        Rule {
            name: "hhmm (military) (ru)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))([0-5]\\d)")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                let mm: u32 = m.parse().ok()?;
                let mut td = TimeData::new(TimeForm::HourMinute(hh, mm, false));
                td.latent = true;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "year (ru)".to_string(),
            pattern: vec![regex("\\b(1\\d{3}|20\\d{2}|2100)\\b")],
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
            name: "year (latent) (ru)".to_string(),
            pattern: vec![regex("\\b(21\\d{2}|[3-9]\\d{3})\\b")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                let mut td = TimeData::new(TimeForm::Year(year));
                td.latent = true;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "time-of-day (latent) (ru)".to_string(),
            pattern: vec![regex("\\b((?:[01]?\\d)|(?:2[0-3]))ч\\b")],
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
            name: "this <cycle> (ru)".to_string(),
            pattern: vec![
                regex("(в )?(эту|этот|этого|этому|эти|эта|это)"),
                dim(DimensionKind::TimeGrain),
            ],
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
            name: "next <cycle> (ru)".to_string(),
            pattern: vec![
                regex("(на |в |к )?след(ующ(ий|его|ему|ими|ем|ие|их|им|ей|ая|ую))?"),
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
            name: "last <cycle> (ru)".to_string(),
            pattern: vec![
                regex("(на |в )?прошл(ый|ого|ому|ом|ое|ые|ых|ыми|ым|ая|ой|ую)"),
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
            name: "one <cycle> before last (ru)".to_string(),
            pattern: vec![
                regex("(на |в )?позапрошл(ый|ого|ому|ыми|ом|ое|ые|ых|ым|ая|ой|ую)"),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: -2,
                })))
            }),
        },
        Rule {
            name: "третий квартал (ru)".to_string(),
            pattern: vec![regex("трет(ий|ьего|ьем)\\s+квартал(е)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Quarter(3))))),
        },
        Rule {
            name: "четвертый квартал 2018 (ru)".to_string(),
            pattern: vec![regex("четв(е|ё)рт(ый|ого|ом)\\s+квартал\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(3)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(4, year))))
            }),
        },
        Rule {
            name: "в 4 утра (ru)".to_string(),
            pattern: vec![regex("в\\s*(\\d{1,2})\\s+утра|\\b(\\d{1,2}):(\\d{2})\\s+утра")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => {
                        if let Some(h1) = rm.group(1) {
                            (h1.to_string(), "0".to_string())
                        } else {
                            (rm.group(2)?.to_string(), rm.group(3)?.to_string())
                        }
                    }
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                let minute: u32 = m.parse().ok()?;
                if hour == 0 || hour > 11 || minute > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
            }),
        },
        Rule {
            name: "в полночь (ru)".to_string(),
            pattern: vec![regex("в\\s+полночь|полночь")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(0, 0, false))))),
        },
        Rule {
            name: "midnight (ru)".to_string(),
            pattern: vec![regex("полночь")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(0, 0, false))))),
        },
        Rule {
            name: "в полдень (ru)".to_string(),
            pattern: vec![regex("в\\s+полдень|полдень")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(12, 0, false))))),
        },
        Rule {
            name: "noon (ru)".to_string(),
            pattern: vec![regex("полдень")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(12, 0, false))))),
        },
        Rule {
            name: "в 3 (ru)".to_string(),
            pattern: vec![regex("в\\s*(\\d{1,2})")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
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
            name: "3 часа (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+час(а|ов)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => rm.group(1)?,
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
            name: "в три (ru)".to_string(),
            pattern: vec![regex("в\\s+три")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 0, false))))),
        },
        Rule {
            name: "в пятнадцать часов (ru)".to_string(),
            pattern: vec![regex("в\\s+пятнадцать\\s+часов")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(15, 0, false))))),
        },
        Rule {
            name: "in <duration> (ru)".to_string(),
            pattern: vec![regex("через"), dim(DimensionKind::Duration)],
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
            name: "<duration> ago (ru)".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex("(тому )?назад")],
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
            name: "last n <cycle> (ru)".to_string(),
            pattern: vec![
                regex("последние"),
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
            name: "next n <cycle> (ru)".to_string(),
            pattern: vec![
                regex("следующие"),
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
            name: "last <cycle> of <time> (ru)".to_string(),
            pattern: vec![
                regex("последн(ий|юю|яя|его|ему)"),
                dim(DimensionKind::TimeGrain),
                regex("в"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::LastCycleOfTime {
                    grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "last <cycle> of <time> #2 (ru)".to_string(),
            pattern: vec![
                regex("последн(ий|юю|яя|его|ему)"),
                dim(DimensionKind::TimeGrain),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::LastCycleOfTime {
                    grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "last <day-of-week> of <time> (ru)".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex("последн(ий|юю|яя|его|ему)"),
                predicate(is_day_of_week),
            ],
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
            name: "<time> after next (ru)".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex("после\\s+следующ(ей|его)")],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[0].token_data)?.clone();
                td.direction = Some(Direction::FarFuture);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "nth <time> after <time> (ru)".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex("после"),
                dim(DimensionKind::Ordinal),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let td1 = time_data(&nodes[0].token_data)?.clone();
                let ord = match &nodes[2].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::NthClosestToTime {
                    n: (ord - 1) as i32,
                    target: Box::new(td1),
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "nth <time> of <time> (ru)".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                dim(DimensionKind::Ordinal),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let td1 = time_data(&nodes[0].token_data)?.clone();
                let ord = match &nodes[1].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                let td2 = time_data(&nodes[2].token_data)?.clone();
                match td1.form {
                    TimeForm::DayOfWeek(dow) => Some(TokenData::Time(TimeData::new(TimeForm::NthDOWOfTime {
                        n: (ord - 1) as i32,
                        dow,
                        base: Box::new(td2),
                    }))),
                    _ => Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                        Box::new(td2),
                        Box::new(td1),
                    )))),
                }
            }),
        },
        Rule {
            name: "<part-of-day> of <time> (ru)".to_string(),
            pattern: vec![predicate(is_part_of_day), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let p = time_data(&nodes[0].token_data)?.clone();
                let t = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t),
                    Box::new(p),
                ))))
            }),
        },
        Rule {
            name: "<ordinal> <cycle> of <time> (ru)".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                dim(DimensionKind::Ordinal),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let base = time_data(&nodes[0].token_data)?.clone();
                let ord = match &nodes[1].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime {
                    n: (ord - 1) as i32,
                    grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "<ordinal> quarter (ru)".to_string(),
            pattern: vec![dim(DimensionKind::Ordinal), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let ord = match &nodes[0].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                if grain != Grain::Quarter {
                    return None;
                }
                let q = ord as u32;
                if !(1..=4).contains(&q) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Quarter(q))))
            }),
        },
        Rule {
            name: "<ordinal> quarter <year> (ru)".to_string(),
            pattern: vec![dim(DimensionKind::Ordinal), dim(DimensionKind::TimeGrain), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let ord = match &nodes[0].token_data {
                    TokenData::Ordinal(o) => o.value,
                    _ => return None,
                };
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                if grain != Grain::Quarter {
                    return None;
                }
                let year = match &nodes[2].token_data {
                    TokenData::Time(TimeData {
                        form: TimeForm::Year(y),
                        ..
                    }) => *y,
                    _ => return None,
                };
                let q = ord as u32;
                if !(1..=4).contains(&q) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(q, year))))
            }),
        },
        Rule {
            name: "<duration> after <time> (ru)".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex("после"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                let base = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n: d.value,
                    grain: d.grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "<duration> before <time> (ru)".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex("перед"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let d = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                let base = time_data(&nodes[2].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n: -d.value,
                    grain: d.grain,
                    base: Box::new(base),
                })))
            }),
        },
        Rule {
            name: "intersect by 'of', 'from', 's (ru)".to_string(),
            pattern: vec![predicate(is_not_latent_time), regex("на"), predicate(is_not_latent_time)],
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
            name: "within <duration> (ru)".to_string(),
            pattern: vec![regex("в\\s+течение"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let d = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::Now)),
                    Box::new(TimeData::new(TimeForm::RelativeGrain {
                        n: d.value,
                        grain: d.grain,
                    })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "лето (ru)".to_string(),
            pattern: vec![regex("лет(о|а)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Season(1))))),
        },
        Rule {
            name: "весна (ru)".to_string(),
            pattern: vec![regex("весна")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Season(0))))),
        },
        Rule {
            name: "зима (ru)".to_string(),
            pattern: vec![regex("зима")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Season(3))))),
        },
        Rule {
            name: "рождество (ru)".to_string(),
            pattern: vec![regex("рождество(\\s+христово)?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                month: 1,
                day: 7,
                year: None,
            })))),
        },
        Rule {
            name: "новый год (ru)".to_string(),
            pattern: vec![regex("новый\\s+год")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                "new year".to_string(),
                None,
            ))))),
        },
        Rule {
            name: "сегодня вечером (ru)".to_string(),
            pattern: vec![regex("сегодня\\s+в\\s+вечер(а|ом)|сегодня\\s+вечером")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Today)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "завтра вечером (ru)".to_string(),
            pattern: vec![regex("завтра\\s+вечером")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Tomorrow)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "завтра в обед (ru)".to_string(),
            pattern: vec![regex("завтра\\s+в\\s+обед")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Tomorrow)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Lunch))),
            ))))),
        },
        Rule {
            name: "вчера вечером (ru)".to_string(),
            pattern: vec![regex("вчера\\s+вечером")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                Box::new(TimeData::new(TimeForm::Yesterday)),
                Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
            ))))),
        },
        Rule {
            name: "в эти выходные (ru)".to_string(),
            pattern: vec![regex("в\\s+эти\\s+выходные")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "13 - 15 июля (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s*\\-\\s*(\\d{1,2})\\s+июл[яь]")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None })),
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 7, day: day2, year: None })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "с 13 по 15 июля (ru)".to_string(),
            pattern: vec![regex("с\\s*(\\d{1,2})\\s+по\\s*(\\d{1,2})\\s+июл[яь]")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None })),
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 7, day: day2, year: None })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "13 июля - 15 июля (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+июл[яь]\\s*\\-\\s*(\\d{1,2})\\s+июл[яь]")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 7, day: day1, year: None })),
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 7, day: day2, year: None })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "8 авг - 12 авг (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+авг\\.?\\s*\\-\\s*(\\d{1,2})\\s+авг\\.?")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 8, day: day1, year: None })),
                    Box::new(TimeData::new(TimeForm::DateMDY { month: 8, day: day2, year: None })),
                    false,
                ))))
            }),
        },
        Rule {
            name: "до конца дня (ru)".to_string(),
            pattern: vec![regex("до\\s+кон(ец|ца)\\s+дня")],
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
            name: "до конца месяца (ru)".to_string(),
            pattern: vec![regex("до\\s+кон(ец|ца)\\s+месяца")],
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
            name: "после 14ч (ru)".to_string(),
            pattern: vec![regex("после\\s*(\\d{1,2})(\\s*ч|\\s*час(а|ов)?)")],
            production: Box::new(|nodes| {
                let h = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let hour: u32 = h.parse().ok()?;
                if hour > 23 {
                    return None;
                }
                let mut td = TimeData::new(TimeForm::HourMinute(hour, 0, false));
                td.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "18:30ч - 19:00ч (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2}):(\\d{2})ч\\s*(\\-|/)\\s*(\\d{1,2}):(\\d{2})ч")],
            production: Box::new(|nodes| {
                let (h1s, m1s, h2s, m2s) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?, m.group(4)?, m.group(5)?),
                    _ => return None,
                };
                let h1: u32 = h1s.parse().ok()?;
                let m1: u32 = m1s.parse().ok()?;
                let h2: u32 = h2s.parse().ok()?;
                let m2: u32 = m2s.parse().ok()?;
                if h1 > 23 || h2 > 23 || m1 > 59 || m2 > 59 {
                    return None;
                }
                let from = TimeData::new(TimeForm::HourMinute(h1, m1, false));
                let to = TimeData::new(TimeForm::HourMinute(h2, m2, false));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
            }),
        },
        Rule {
            name: "17ч10 (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})ч(\\d{2})")],
            production: Box::new(|nodes| {
                let (hs, ms) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(2)?),
                    _ => return None,
                };
                let h: u32 = hs.parse().ok()?;
                let m: u32 = ms.parse().ok()?;
                if h > 23 || m > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(h, m, false))))
            }),
        },
        Rule {
            name: "<hour-of-day> <integer> (as relative minutes) (ru)".to_string(),
            pattern: vec![regex("((?:[01]?\\d)|(?:2[0-3]))\\s+(\\d{1,2})\\s*мин(ут)?")],
            production: Box::new(|nodes| {
                let (h, m) = match &nodes[0].token_data {
                    TokenData::RegexMatch(rm) => (rm.group(1)?, rm.group(2)?),
                    _ => return None,
                };
                let hh: u32 = h.parse().ok()?;
                let mm: u32 = m.parse().ok()?;
                if hh > 23 || mm > 59 {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hh, mm, false))))
            }),
        },
        Rule {
            name: "18 февраля (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})(-го|-е)?\\s+феврал[яь]")],
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
            name: "восемнадцатое февраля (ru)".to_string(),
            pattern: vec![regex("восемнадцат(ое|ого)\\s+феврал[яь]")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 2,
                    day: 18,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "1 марта (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+март[а]?")],
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
                    month: 3,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "первое марта (ru)".to_string(),
            pattern: vec![regex("перв(ое|ого)\\s+март[а]?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
        Rule {
            name: "1-е мар. (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})-е\\s+мар\\.?")],
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
            name: "3-его марта 2015 (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})-(его|го)\\s+март[а]?\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let (d, y) = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => (m.group(1)?, m.group(3)?),
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
            name: "3 мар. 2015 (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+мар\\.?\\s+(\\d{4})")],
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
            name: "третьего марта 2015 (ru)".to_string(),
            pattern: vec![regex("третьего\\s+март[а]?\\s+(\\d{4})")],
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
            name: "третье марта 2015 (ru)".to_string(),
            pattern: vec![regex("третье\\s+март[а]?\\s+(\\d{4})")],
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
            name: "к третьему марта 2015 (ru)".to_string(),
            pattern: vec![regex("к\\s+третьему\\s+март[а]?\\s+(\\d{4})")],
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
            name: "15.2 (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\.(\\d{1,2})")],
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
            name: "<day-of-month> (ordinal) (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})-?(?:е|го|ого|ое)")],
            production: Box::new(|nodes| {
                let day_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = day_s.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "<day-of-month>(ordinal) <named-month> (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})-?(?:е|го|ого|ое)"), predicate(is_month)],
            production: Box::new(|nodes| {
                let day_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = day_s.parse().ok()?;
                let month = match &nodes[1].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "<named-month> <day-of-month> (ordinal) (ru)".to_string(),
            pattern: vec![predicate(is_month), regex("(\\d{1,2})-?(?:е|го|ого|ое)")],
            production: Box::new(|nodes| {
                let month = match &nodes[0].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                let day_s = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = day_s.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: None })))
            }),
        },
        Rule {
            name: "mm/dd/yyyy (ru)".to_string(),
            pattern: vec![regex("([012]?[1-9]|10|20|30|31)\\.(0?[1-9]|10|11|12)\\.(\\d{2,4})")],
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
            name: "<time-of-day>  o'clock (ru)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("час(у|а|ов)?|ч(?:[\\s'\"\\-_{}\\[\\]()]|$)")],
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
            name: "<day-of-month>(ordinal) <named-month> year (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})-?(?:е|го|ого|ое)"), predicate(is_month), regex("(\\d{4})")],
            production: Box::new(|nodes| {
                let day_s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let day: u32 = day_s.parse().ok()?;
                let month = match &nodes[1].token_data {
                    TokenData::Time(TimeData { form: TimeForm::Month(m), .. }) => *m,
                    _ => return None,
                };
                let year_s = match &nodes[2].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = year_s.parse().ok()?;
                if !(1..=31).contains(&day) {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year: Some(year) })))
            }),
        },
        Rule {
            name: "midnight|EOD|end of day (ru)".to_string(),
            pattern: vec![regex("полночь|кон(ец|ца)\\s+дня|eod|end\\s+of\\s+day")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(0, 0, false))))),
        },
        Rule {
            name: "<time> timezone (ru)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("\\b(YEKT|YEKST|YAKT|YAKST|WITA|WIT|WIB|WGT|WGST|WFT|WET|WEST|WAT|WAST|VUT|VLAT|VLAST|VET|UZT|UYT|UYST|UTC|ULAT|TVT|TMT|TLT|TKT|TJT|TFT|TAHT|SST|SRT|SGT|SCT|SBT|SAST|SAMT|RET|PYT|PYST|PWT|PST|PONT|PMST|PMDT|PKT|PHT|PHOT|PGT|PETT|PETST|PET|PDT|OMST|OMSST|NZST|NZDT|NUT|NST|NPT|NOVT|NOVST|NFT|NDT|NCT|MYT|MVT|MUT|MST|MSK|MSD|MMT|MHT|MDT|MAWT|MART|MAGT|MAGST|LINT|LHST|LHDT|KUYT|KST|KRAT|KRAST|KGT|JST|IST|IRST|IRKT|IRKST|IRDT|IOT|IDT|ICT|HOVT|HKT|GYT|GST|GMT|GILT|GFT|GET|GAMT|GALT|FNT|FKT|FKST|FJT|FJST|EST|EGT|EGST|EET|EEST|EDT|ECT|EAT|EAST|EASST|DAVT|ChST|CXT|CVT|CST|COT|CLT|CLST|CKT|CHAST|CHADT|CET|CEST|CDT|CCT|CAT|CAST|BTT|BST|BRT|BRST|BOT|BNT|AZT|AZST|AZOT|AZOST|AWST|AWDT|AST|ART|AQTT|ANAT|ANAST|AMT|AMST|ALMT|AKST|AKDT|AFT|AEST|AEDT|ADT|ACST|ACDT)\\b")],
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
            name: "15 Фев (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Фф]ев\\.?")],
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
            name: "15-го Фев (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})-го\\s+[Фф]ев\\.?")],
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
            name: "пятнадцатое февраля (ru)".to_string(),
            pattern: vec![regex("пятнадцат(ое|ого)\\s+феврал[яь]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 15, year: None })))),
        },
        Rule {
            name: "пятнадцатого фев (ru)".to_string(),
            pattern: vec![regex("пятнадцатого\\s+фев\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 15, year: None })))),
        },
        Rule {
            name: "8 августа (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+август[а]?")],
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
            name: "8 Авг (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+[Аа]вг\\.?")],
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
            name: "восьмое августа (ru)".to_string(),
            pattern: vec![regex("восьм(ое|ого)\\s+август[а]?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 8, day: 8, year: None })))),
        },
        Rule {
            name: "март (ru)".to_string(),
            pattern: vec![regex("март")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Month(3))))),
        },
        Rule {
            name: "Октябрь 2014 (ru)".to_string(),
            pattern: vec![regex("[Оо]ктябрь\\s*(\\d{4})")],
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
            name: "Ноябрь 2014 (ru)".to_string(),
            pattern: vec![regex("[Нн]оябрь\\s*(\\d{4})")],
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
            name: "тридцать первое октября 1974 (ru)".to_string(),
            pattern: vec![regex("тридцать первое октября\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 31, year: Some(year) })))
            }),
        },
        Rule {
            name: "31-ое октября 1974 (ru)".to_string(),
            pattern: vec![regex("31-ое октября\\s*(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 10, day: 31, year: Some(year) })))
            }),
        },
        Rule {
            name: "14 апреля 2015 (ru)".to_string(),
            pattern: vec![regex("(\\d{1,2})\\s+апреля\\s+(\\d{4})")],
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
            name: "Пт, 18 июля 2014 (ru)".to_string(),
            pattern: vec![regex("[Пп]т,?\\s*(\\d{1,2})\\s+июля\\s+(\\d{4})")],
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
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 7,
                    day,
                    year: Some(year),
                })))
            }),
        },
        Rule {
            name: "четырнадцатое апреля 2015 (ru)".to_string(),
            pattern: vec![regex("четырнадцат(ое|ого)\\s+апреля\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 4, day: 14, year: Some(year) })))
            }),
        },
        Rule {
            name: "двадцать четвертое фев (ru)".to_string(),
            pattern: vec![regex("двадцать\\s+четверт(ое|ого)\\s+фев\\.?")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 24, year: None })))),
        },
        Rule {
            name: "двадцать четвёртое февраля (ru)".to_string(),
            pattern: vec![regex("двадцать\\s+четв(е|ё)рт(ое|ого)\\s+феврал(я|ь)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 24, year: None })))),
        },
        Rule {
            name: "24-ого февраля (ru)".to_string(),
            pattern: vec![regex("24-ого\\s+феврал(я|ь)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 24, year: None })))),
        },
        Rule {
            name: "к двадцать четвёртому февраля (ru)".to_string(),
            pattern: vec![regex("к\\s+двадцать\\s+четв(е|ё)ртому\\s+феврал(я|ь)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 2, day: 24, year: None })))),
        },
        Rule {
            name: "тридцать первое мая 2015 (ru)".to_string(),
            pattern: vec![regex("тридцать\\s+перв(ое|ого)\\s+мая\\s+(\\d{4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let year: i32 = y.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 5, day: 31, year: Some(year) })))
            }),
        },
    ]);
    rules
}
