use crate::dimensions::numeral::helpers::{integer_value, numeral_data};
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{Direction, IntervalDirection, PartOfDay, TimeData, TimeForm};

fn time_data(td: &TokenData) -> Option<&TimeData> {
    match td {
        TokenData::Time(d) => Some(d),
        _ => None,
    }
}

fn is_day_of_week(td: &TokenData) -> bool {
    matches!(td, TokenData::Time(TimeData { form: TimeForm::DayOfWeek(_), .. }))
}

fn is_month(td: &TokenData) -> bool {
    matches!(td, TokenData::Time(TimeData { form: TimeForm::Month(_), .. }))
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

fn is_integer_between(lo: i64, hi: i64) -> impl Fn(&TokenData) -> bool {
    move |td| integer_value(td).is_some_and(|v| v >= lo && v <= hi)
}

fn is_dom_token(td: &TokenData) -> bool {
    match td {
        TokenData::Numeral(n) => {
            let v = n.value as i64;
            (1..=31).contains(&v) && (n.value - v as f64).abs() < f64::EPSILON
        }
        TokenData::Ordinal(o) => (1..=31).contains(&o.value),
        _ => false,
    }
}

fn dom_value(td: &TokenData) -> Option<u32> {
    match td {
        TokenData::Numeral(n) if (n.value - n.value.floor()).abs() < f64::EPSILON => {
            let v = n.value as i64;
            if (1..=31).contains(&v) {
                Some(v as u32)
            } else {
                None
            }
        }
        TokenData::Ordinal(o) if (1..=31).contains(&o.value) => Some(o.value as u32),
        _ => None,
    }
}

fn month_value(td: &TokenData) -> Option<u32> {
    match td {
        TokenData::Time(TimeData {
            form: TimeForm::Month(m),
            ..
        }) => Some(*m),
        _ => None,
    }
}

fn parse_year_2_or_4(y: &str) -> Option<i32> {
    let yr: i32 = y.parse().ok()?;
    if y.len() == 2 {
        if yr < 50 {
            Some(yr + 2000)
        } else {
            Some(yr + 1900)
        }
    } else {
        Some(yr)
    }
}

fn to_h24_for_part(hour: u32, pod: PartOfDay) -> u32 {
    match pod {
        PartOfDay::Morning => {
            if hour == 12 {
                0
            } else {
                hour
            }
        }
        PartOfDay::Afternoon | PartOfDay::Evening | PartOfDay::Night => {
            if hour < 12 {
                hour + 12
            } else {
                hour
            }
        }
        PartOfDay::Lunch => 12,
    }
}

fn apply_pod_to_time(t: &TimeData, pod: PartOfDay) -> Option<TimeData> {
    let form = match t.form {
        TimeForm::Hour(h, _) => TimeForm::Hour(to_h24_for_part(h, pod), false),
        TimeForm::HourMinute(h, m, _) => TimeForm::HourMinute(to_h24_for_part(h, pod), m, false),
        TimeForm::HourMinuteSecond(h, m, s) => {
            TimeForm::HourMinuteSecond(to_h24_for_part(h, pod), m, s)
        }
        _ => return None,
    };
    Some(TimeData::new(form))
}

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (bg)".to_string(),
            pattern: vec![regex("((точно\\s+)?сега)|веднага")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (bg)".to_string(),
            pattern: vec![regex("днес|(по това време)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (bg)".to_string(),
            pattern: vec![regex("утре")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (bg)".to_string(),
            pattern: vec![regex("вчера")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "after tomorrow (bg)".to_string(),
            pattern: vec![regex("(в\\s*)?другиден")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))
            }),
        },
        Rule {
            name: "before yesterday (bg)".to_string(),
            pattern: vec![regex("(оня ден)|завчера")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))
            }),
        },
        Rule {
            name: "day of week (bg)".to_string(),
            pattern: vec![regex(
                "(понеделник|пон\\.?|вторник|сряда|ср\\.?|четвъртък|чет\\.?|петък|пет\\.?|събота|съб\\.?|неделя|нед\\.?)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let s = s.trim_end_matches('.');
                let dow = if s.starts_with("пон") {
                    0
                } else if s.starts_with("втор") {
                    1
                } else if s.starts_with("ср") {
                    2
                } else if s.starts_with("чет") {
                    3
                } else if s.starts_with("пет") {
                    4
                } else if s.starts_with("съб") {
                    5
                } else if s.starts_with("нед") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "month (bg)".to_string(),
            pattern: vec![regex(
                "(януари|февруари|март|април|май|юни|юли|август|септември|октомври|ноември|декември)",
            )],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let m = match s.as_str() {
                    "януари" => 1,
                    "февруари" => 2,
                    "март" => 3,
                    "април" => 4,
                    "май" => 5,
                    "юни" => 6,
                    "юли" => 7,
                    "август" => 8,
                    "септември" => 9,
                    "октомври" => 10,
                    "ноември" => 11,
                    "декември" => 12,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Month(m))))
            }),
        },
        Rule {
            name: "on <day> (bg)".to_string(),
            pattern: vec![regex("на"), predicate(is_dom_token)],
            production: Box::new(|nodes| {
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(dom_value(
                    &nodes[1].token_data,
                )?))))
            }),
        },
        Rule {
            name: "<dom> <month> (bg)".to_string(),
            pattern: vec![predicate(is_dom_token), predicate(is_month)],
            production: Box::new(|nodes| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: month_value(&nodes[1].token_data)?,
                    day: dom_value(&nodes[0].token_data)?,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "<month> <dom> (bg)".to_string(),
            pattern: vec![predicate(is_month), predicate(is_dom_token)],
            production: Box::new(|nodes| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: month_value(&nodes[0].token_data)?,
                    day: dom_value(&nodes[1].token_data)?,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "<month> <year> (bg)".to_string(),
            pattern: vec![predicate(is_month), regex("(\\d{2,4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => parse_year_2_or_4(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: month_value(&nodes[0].token_data)?,
                    day: 1,
                    year: Some(y),
                })))
            }),
        },
        Rule {
            name: "<dom> <month> <year> (bg)".to_string(),
            pattern: vec![predicate(is_dom_token), predicate(is_month), regex("(\\d{2,4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[2].token_data {
                    TokenData::RegexMatch(m) => parse_year_2_or_4(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: month_value(&nodes[1].token_data)?,
                    day: dom_value(&nodes[0].token_data)?,
                    year: Some(y),
                })))
            }),
        },
        Rule {
            name: "<month> <dom>, <year> (bg)".to_string(),
            pattern: vec![predicate(is_month), predicate(is_dom_token), regex(","), regex("(\\d{2,4})")],
            production: Box::new(|nodes| {
                let y = match &nodes[3].token_data {
                    TokenData::RegexMatch(m) => parse_year_2_or_4(m.group(1)?)?,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: month_value(&nodes[0].token_data)?,
                    day: dom_value(&nodes[1].token_data)?,
                    year: Some(y),
                })))
            }),
        },
        Rule {
            name: "numeric date d.m.y (bg)".to_string(),
            pattern: vec![regex("(\\d{1,2})[./](\\d{1,2})[./](\\d{2,4})")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let day: u32 = m.group(1)?.parse().ok()?;
                let month: u32 = m.group(2)?.parse().ok()?;
                if !(1..=31).contains(&day) || !(1..=12).contains(&month) {
                    return None;
                }
                let year = parse_year_2_or_4(m.group(3)?)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: Some(year),
                })))
            }),
        },
        Rule {
            name: "this|next <day-of-week> (bg)".to_string(),
            pattern: vec![regex("(т(о|а)зи)|следващ((ия(т)?)|ата)"), predicate(is_day_of_week)],
            production: Box::new(|nodes| {
                let which = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(if which.contains("следващ") {
                    Direction::FarFuture
                } else {
                    Direction::Future
                });
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "after next <day-of-week> (bg)".to_string(),
            pattern: vec![regex("по(\\-|\\s+)следващ((ия(т)?)|ата|ото)"), predicate(is_day_of_week)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::FarFuture);
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "next month name (bg)".to_string(),
            pattern: vec![regex("следващ((ия(т)?)|ата|ото)"), predicate(is_month)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Future);
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "this week (bg)".to_string(),
            pattern: vec![regex("тази седмица")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: Grain::Week,
                    offset: 0,
                })))
            }),
        },
        Rule {
            name: "last week (bg)".to_string(),
            pattern: vec![regex("последната седмица")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: Grain::Week,
                    offset: -1,
                })))
            }),
        },
        Rule {
            name: "next week (bg)".to_string(),
            pattern: vec![regex("следващата седмица")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: Grain::Week,
                    offset: 1,
                })))
            }),
        },
        Rule {
            name: "last month (bg)".to_string(),
            pattern: vec![regex("последния месец")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: Grain::Month,
                    offset: -1,
                })))
            }),
        },
        Rule {
            name: "year offsets (bg)".to_string(),
            pattern: vec![regex("миналата година|тази година|следващата година")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let offset = if s.contains("минал") {
                    -1
                } else if s.contains("тази") {
                    0
                } else {
                    1
                };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: Grain::Year,
                    offset,
                })))
            }),
        },
        Rule {
            name: "at <time> (bg)".to_string(),
            pattern: vec![regex("в"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "<time> o'clock (bg)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("часа")],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[0].token_data)?.clone();
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "<time> morning (bg)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("сутрин(та)?")],
            production: Box::new(|nodes| {
                Some(TokenData::Time(apply_pod_to_time(
                    time_data(&nodes[0].token_data)?,
                    PartOfDay::Morning,
                )?))
            }),
        },
        Rule {
            name: "<time> afternoon/evening (bg)".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("вечер(та)?|след об(е|я)д")],
            production: Box::new(|nodes| {
                Some(TokenData::Time(apply_pod_to_time(
                    time_data(&nodes[0].token_data)?,
                    PartOfDay::Afternoon,
                )?))
            }),
        },
        Rule {
            name: "morning (latent, bg)".to_string(),
            pattern: vec![regex("сутрин(та)?")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(
                    PartOfDay::Morning,
                ))))
            }),
        },
        Rule {
            name: "evening (latent, bg)".to_string(),
            pattern: vec![regex("вечер(та)?")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(
                    PartOfDay::Evening,
                ))))
            }),
        },
        Rule {
            name: "this <part-of-day> (bg)".to_string(),
            pattern: vec![
                regex("т(а|о)зи|това"),
                predicate(|td| {
                    matches!(
                        td,
                        TokenData::Time(TimeData {
                            form: TimeForm::PartOfDay(_),
                            ..
                        })
                    )
                }),
            ],
            production: Box::new(|nodes| {
                let pod = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(pod),
                ))))
            }),
        },
        Rule {
            name: "after <n> <time-grain> (bg)".to_string(),
            pattern: vec![regex("след"), predicate(is_integer_between(1, 99999)), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?.value as i64;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let mut td = TimeData::new(TimeForm::RelativeGrain { n, grain });
                td.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "this summer/winter (bg)".to_string(),
            pattern: vec![regex("това|тази|този"), regex("лято|зима")],
            production: Box::new(|nodes| {
                let s = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let idx = if s.contains("лято") { 1 } else { 3 };
                let mut td = TimeData::new(TimeForm::Season(idx));
                td.direction = Some(Direction::Future);
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "christmas (bg)".to_string(),
            pattern: vec![regex("коледа")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "christmas".to_string(),
                    None,
                ))))
            }),
        },
    ]);
    rules
}
