use crate::dimensions::numeral::helpers::integer_value;
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{Direction, PartOfDay, TimeData, TimeForm};

fn time_data(td: &TokenData) -> Option<&TimeData> {
    match td {
        TokenData::Time(d) => Some(d),
        _ => None,
    }
}

fn is_month(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Time(TimeData {
            form: TimeForm::Month(_),
            ..
        })
    )
}

fn is_day_of_week(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::Time(TimeData {
            form: TimeForm::DayOfWeek(_),
            ..
        })
    )
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
        TokenData::Time(t) => match t.form {
            TimeForm::Month(m) => Some(m),
            _ => None,
        },
        _ => None,
    }
}

fn parse_year_2_or_4(y: &str) -> Option<i32> {
    let yr: i32 = y.parse().ok()?;
    if y.len() == 2 {
        if yr < 50 {
            Some(yr.checked_add(2000)?)
        } else {
            Some(yr.checked_add(1900)?)
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
                hour.saturating_add(12)
            } else {
                hour
            }
        }
        PartOfDay::Lunch => 12,
    }
}

pub fn rules() -> Vec<Rule> {
    // Reuse existing generic/clock and composition rules, then extend with ES lexical rules.
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "ahora/ya".to_string(),
            pattern: vec![regex("ahor(it)?a|ya|cuanto antes|en\\s?seguida")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "hoy".to_string(),
            pattern: vec![regex("(hoy)|(en este momento)")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "ayer".to_string(),
            pattern: vec![regex("ayer")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "anteayer/antier".to_string(),
            pattern: vec![regex("anteayer|antier|antes de (ayer|anoche)")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))
            }),
        },
        Rule {
            name: "pasado manana".to_string(),
            pattern: vec![regex("pasado\\s?ma(n|ñ)ana")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))
            }),
        },
        Rule {
            name: "manana (tomorrow)".to_string(),
            pattern: vec![regex("ma(n|ñ)ana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "fin de semana".to_string(),
            pattern: vec![regex("week[ -]?end|fin de semana")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))),
        },
        Rule {
            name: "navidad".to_string(),
            pattern: vec![regex("(la )?navidad")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "christmas".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "nochevieja".to_string(),
            pattern: vec![regex("nochevieja")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "new year's eve".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "ano nuevo".to_string(),
            pattern: vec![regex("a(n|ñ)o nuevo")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    "new year's day".to_string(),
                    None,
                ))))
            }),
        },
        Rule {
            name: "season (es)".to_string(),
            pattern: vec![regex("verano|oto(ñ|n)o|invierno|primavera")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let idx = if s.contains("verano") {
                    1
                } else if s.contains("oto") {
                    2
                } else if s.contains("invierno") {
                    3
                } else if s.contains("primavera") {
                    0
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(idx))))
            }),
        },
        Rule {
            name: "este <season>".to_string(),
            pattern: vec![regex("est(e|a)"), predicate(|td| {
                matches!(td, TokenData::Time(TimeData { form: TimeForm::Season(_), .. }))
            })],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(Direction::Future);
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "dia mundial de la lengua arabe".to_string(),
            pattern: vec![regex("d(í|i)a mundial de la lengua (á|a)rabe")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 12,
                    day: 18,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "dia de la cero discriminacion".to_string(),
            pattern: vec![regex("d(í|i)a de la cero discriminaci(ó|o)n")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "dia internacional de las cooperativas 2019".to_string(),
            pattern: vec![regex("d(í|i)a internacional de las cooperativas( del 2019)?")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 7,
                    day: 6,
                    year: Some(2019),
                })))
            }),
        },
        Rule {
            name: "dia de la prematuridad mundial".to_string(),
            pattern: vec![regex(
                "d(í|i)a de la prematuridad mundial|d(í|i)a mundial del (ni(ñ|n)o )?prematuro",
            )],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 11,
                    day: 17,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "dia de los inocentes de abril".to_string(),
            pattern: vec![regex(
                "d(í|i)a de los inocentes( de abril)?|d(í|i)a de las bromas( de abril)?",
            )],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 4,
                    day: 1,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "noon (es)".to_string(),
            pattern: vec![regex("medio(\\s*)d(í|i)a")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Hour(12, false))))),
        },
        Rule {
            name: "midnight (es)".to_string(),
            pattern: vec![regex("medianoche")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Hour(0, false))))),
        },
        Rule {
            name: "morning (latent, es)".to_string(),
            pattern: vec![regex("ma(ñ|n)ana")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(
                    PartOfDay::Morning,
                ))))
            }),
        },
        Rule {
            name: "afternoon (latent, es)".to_string(),
            pattern: vec![regex("tarde")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(
                    PartOfDay::Afternoon,
                ))))
            }),
        },
        Rule {
            name: "night (latent, es)".to_string(),
            pattern: vec![regex("noche")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(
                    PartOfDay::Evening,
                ))))
            }),
        },
        Rule {
            name: "this <part-of-day> (es)".to_string(),
            pattern: vec![regex("est(e|a)"), predicate(|td| {
                matches!(td, TokenData::Time(TimeData { form: TimeForm::PartOfDay(_), .. }))
            })],
            production: Box::new(|nodes| {
                let pod = time_data(&nodes[1].token_data)?.clone();
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(pod),
                ))))
            }),
        },
        Rule {
            name: "lunes..domingo".to_string(),
            pattern: vec![regex(
                "(lunes|lu|lun\\.?|martes|ma\\.?|mar\\.?|mi(e|é)\\.?(rcoles)?|mx|mier\\.?|jueves|jue\\.?|viernes|vie\\.?|s(á|a)bado|s(á|a)b\\.?|domingo|dom\\.?)",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let s = m.to_lowercase();
                let s = s.trim_end_matches('.');
                let dow = if s == "lu" || s.starts_with("lun") {
                    0
                } else if s == "ma" || s.starts_with("mar") {
                    1
                } else if s.starts_with("mi") || s == "mx" {
                    2
                } else if s.starts_with("jue") {
                    3
                } else if s.starts_with("vie") {
                    4
                } else if s.starts_with("sá") || s.starts_with("sa") {
                    5
                } else if s.starts_with("dom") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "months (es)".to_string(),
            pattern: vec![regex(
                "(enero|ene\\.?|febrero|feb\\.?|marzo|mar\\.?|abril|abr\\.?|mayo?\\.?|junio|jun\\.?|julio|jul\\.?|agosto|ago\\.?|septiembre|sept?\\.?|octubre|oct\\.?|noviembre|nov\\.?|diciembre|dic\\.?)",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let s = m.to_lowercase();
                let month = if s.starts_with("ene") {
                    1
                } else if s.starts_with("feb") {
                    2
                } else if s.starts_with("mar") {
                    3
                } else if s.starts_with("abr") {
                    4
                } else if s.starts_with("may") {
                    5
                } else if s.starts_with("jun") {
                    6
                } else if s.starts_with("jul") {
                    7
                } else if s.starts_with("ago") {
                    8
                } else if s.starts_with("sep") {
                    9
                } else if s.starts_with("oct") {
                    10
                } else if s.starts_with("nov") {
                    11
                } else if s.starts_with("dic") {
                    12
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        Rule {
            name: "this|next <day-of-week> (es)".to_string(),
            pattern: vec![regex("(este|pr(o|ó)ximo)"), predicate(is_day_of_week)],
            production: Box::new(|nodes| {
                let which = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.direction = Some(if which == "este" {
                    Direction::Future
                } else {
                    Direction::FarFuture
                });
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "esta semana".to_string(),
            pattern: vec![regex("esta semana")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: Grain::Week,
                    offset: 0,
                })))
            }),
        },
        Rule {
            name: "semana pasada".to_string(),
            pattern: vec![regex("la semana pasada")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: Grain::Week,
                    offset: -1,
                })))
            }),
        },
        Rule {
            name: "semana que viene".to_string(),
            pattern: vec![regex(
                "(la )?(pr(o|ó)xima|siguiente) semana|semana que viene|pr(o|ó)ximas? semana",
            )],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: Grain::Week,
                    offset: 1,
                })))
            }),
        },
        Rule {
            name: "mes pasado/proximo".to_string(),
            pattern: vec![regex("el pasado mes|el mes que viene|el proximo mes")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let offset = if text.contains("pasado") { -1 } else { 1 };
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain: Grain::Month,
                    offset,
                })))
            }),
        },
        Rule {
            name: "ano pasado/este/proximo".to_string(),
            pattern: vec![regex("el año pasado|este ano|el año que viene|el proximo ano")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let offset = if text.contains("pasado") {
                    -1
                } else if text.contains("este") {
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
            name: "a las <time>".to_string(),
            pattern: vec![regex("((al?)( las?)?|las?)"), predicate(is_time_of_day)],
            production: Box::new(|nodes| {
                let mut td = time_data(&nodes[1].token_data)?.clone();
                td.latent = false;
                Some(TokenData::Time(td))
            }),
        },
        Rule {
            name: "las <integer> (hour)".to_string(),
            pattern: vec![regex("(a\\s+)?las?"), predicate(is_integer_between(1, 23))],
            production: Box::new(|nodes| {
                let h = integer_value(&nodes[1].token_data)? as u32;
                Some(TokenData::Time(TimeData::new(TimeForm::Hour(h, h <= 12))))
            }),
        },
        Rule {
            name: "<hour> y <minutes>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(TimeData { form: TimeForm::Hour(..), .. }))),
                regex("y"),
                predicate(is_integer_between(1, 59)),
            ],
            production: Box::new(|nodes| {
                let (h, is12h) = match time_data(&nodes[0].token_data)?.form {
                    TimeForm::Hour(h, is12h) => (h, is12h),
                    _ => return None,
                };
                let m = integer_value(&nodes[2].token_data)? as u32;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(h, m, is12h))))
            }),
        },
        Rule {
            name: "<hour> y cuarto/media".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(TimeData { form: TimeForm::Hour(..), .. }))),
                regex("y cuarto|y media|y treinta"),
            ],
            production: Box::new(|nodes| {
                let (h, is12h) = match time_data(&nodes[0].token_data)?.form {
                    TimeForm::Hour(h, is12h) => (h, is12h),
                    _ => return None,
                };
                let text = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let m = if text.contains("cuarto") { 15 } else { 30 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(h, m, is12h))))
            }),
        },
        Rule {
            name: "<hour> menos <minutes>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(TimeData { form: TimeForm::Hour(..), .. }))),
                regex("menos"),
                predicate(is_integer_between(1, 59)),
            ],
            production: Box::new(|nodes| {
                let (h, is12h) = match time_data(&nodes[0].token_data)?.form {
                    TimeForm::Hour(h, is12h) => (h, is12h),
                    _ => return None,
                };
                let minus = integer_value(&nodes[2].token_data)? as u32;
                if minus >= 60 || h == 0 {
                    return None;
                }
                let out_h = if h == 0 { 23 } else { h.checked_sub(1)? };
                let out_m = 60_u32.checked_sub(minus)?;
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                    out_h, out_m, is12h,
                ))))
            }),
        },
        Rule {
            name: "<hour> menos cuarto/media".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(TimeData { form: TimeForm::Hour(..), .. }))),
                regex("menos cuarto|menos media"),
            ],
            production: Box::new(|nodes| {
                let (h, is12h) = match time_data(&nodes[0].token_data)?.form {
                    TimeForm::Hour(h, is12h) => (h, is12h),
                    _ => return None,
                };
                if h == 0 {
                    return None;
                }
                let text = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let minus = if text.contains("cuarto") { 15 } else { 30 };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                    h.checked_sub(1)?,
                    60_u32.checked_sub(minus)?,
                    is12h,
                ))))
            }),
        },
        Rule {
            name: "<time> de la manana".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("(a|en|de|por) la ma(ñ|n)ana")],
            production: Box::new(|nodes| {
                let td = time_data(&nodes[0].token_data)?;
                let out = match td.form {
                    TimeForm::Hour(h, _) => TimeForm::Hour(to_h24_for_part(h, PartOfDay::Morning), false),
                    TimeForm::HourMinute(h, m, _) => {
                        TimeForm::HourMinute(to_h24_for_part(h, PartOfDay::Morning), m, false)
                    }
                    TimeForm::HourMinuteSecond(h, m, s) => {
                        TimeForm::HourMinuteSecond(to_h24_for_part(h, PartOfDay::Morning), m, s)
                    }
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(out)))
            }),
        },
        Rule {
            name: "<time> de la tarde".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("(a|en|de|por) la tarde|del mediod(í|i)a")],
            production: Box::new(|nodes| {
                let td = time_data(&nodes[0].token_data)?;
                let out = match td.form {
                    TimeForm::Hour(h, _) => TimeForm::Hour(to_h24_for_part(h, PartOfDay::Afternoon), false),
                    TimeForm::HourMinute(h, m, _) => {
                        TimeForm::HourMinute(to_h24_for_part(h, PartOfDay::Afternoon), m, false)
                    }
                    TimeForm::HourMinuteSecond(h, m, s) => {
                        TimeForm::HourMinuteSecond(to_h24_for_part(h, PartOfDay::Afternoon), m, s)
                    }
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(out)))
            }),
        },
        Rule {
            name: "<time> de la noche".to_string(),
            pattern: vec![predicate(is_time_of_day), regex("(a|en|de|por) la noche")],
            production: Box::new(|nodes| {
                let td = time_data(&nodes[0].token_data)?;
                let out = match td.form {
                    TimeForm::Hour(h, _) => TimeForm::Hour(to_h24_for_part(h, PartOfDay::Evening), false),
                    TimeForm::HourMinute(h, m, _) => {
                        TimeForm::HourMinute(to_h24_for_part(h, PartOfDay::Evening), m, false)
                    }
                    TimeForm::HourMinuteSecond(h, m, s) => {
                        TimeForm::HourMinuteSecond(to_h24_for_part(h, PartOfDay::Evening), m, s)
                    }
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(out)))
            }),
        },
        Rule {
            name: "day de month".to_string(),
            pattern: vec![predicate(is_dom_token), regex("de"), predicate(is_month)],
            production: Box::new(|nodes| {
                let day = dom_value(&nodes[0].token_data)?;
                let month = month_value(&nodes[2].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "el day de month".to_string(),
            pattern: vec![
                regex("el"),
                predicate(is_dom_token),
                regex("de"),
                predicate(is_month),
            ],
            production: Box::new(|nodes| {
                let day = dom_value(&nodes[1].token_data)?;
                let month = month_value(&nodes[3].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "month day (es)".to_string(),
            pattern: vec![predicate(is_month), predicate(is_dom_token)],
            production: Box::new(|nodes| {
                let month = month_value(&nodes[0].token_data)?;
                let day = dom_value(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: None,
                })))
            }),
        },
        Rule {
            name: "day/month(/year) (es dmy)".to_string(),
            pattern: vec![regex("(3[01]|[12]\\d|0?[1-9])[/.\\-](0?[1-9]|1[0-2])(?:[/.\\-](\\d{2,4}))?")],
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
                let year = m.group(3).and_then(parse_year_2_or_4);
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month, day, year })))
            }),
        },
        Rule {
            name: "el <day-of-month>".to_string(),
            pattern: vec![regex("el"), predicate(is_dom_token)],
            production: Box::new(|nodes| {
                let day = dom_value(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "dia <day-of-month>".to_string(),
            pattern: vec![regex("d(í|i)a"), predicate(is_dom_token)],
            production: Box::new(|nodes| {
                let day = dom_value(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
            }),
        },
        Rule {
            name: "<date> del year".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(TimeData { form: TimeForm::DateMDY { year: None, .. }, .. }))),
                regex("de(l)?"),
                predicate(is_integer_between(0, 9999)),
            ],
            production: Box::new(|nodes| {
                let base = time_data(&nodes[0].token_data)?;
                let year = integer_value(&nodes[2].token_data)? as i32;
                match base.form {
                    TimeForm::DateMDY { month, day, .. } => Some(TokenData::Time(TimeData::new(
                        TimeForm::DateMDY {
                            month,
                            day,
                            year: Some(year),
                        },
                    ))),
                    _ => None,
                }
            }),
        },
        Rule {
            name: "year (es)".to_string(),
            pattern: vec![predicate(is_integer_between(1000, 2100))],
            production: Box::new(|nodes| {
                let year = integer_value(&nodes[0].token_data)? as i32;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        Rule {
            name: "year by adding three numbers (es)".to_string(),
            pattern: vec![
                regex("mil"),
                predicate(is_integer_between(100, 1000)),
                predicate(is_integer_between(1, 100)),
            ],
            production: Box::new(|nodes| {
                let v1 = integer_value(&nodes[1].token_data)? as i32;
                let v2 = integer_value(&nodes[2].token_data)? as i32;
                let year = 1000i32.checked_add(v1)?.checked_add(v2)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        Rule {
            name: "en <duration> (es)".to_string(),
            pattern: vec![regex("en"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Duration(d) => Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: d.value,
                    grain: d.grain,
                }))),
                _ => None,
            }),
        },
        Rule {
            name: "hace <duration>".to_string(),
            pattern: vec![regex("hace"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Duration(d) => Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: d.value.checked_neg()?,
                    grain: d.grain,
                }))),
                _ => None,
            }),
        },
        Rule {
            name: "dentro de <duration>".to_string(),
            pattern: vec![regex("dentro de"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::Duration(d) => Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: d.value,
                    grain: d.grain,
                }))),
                _ => None,
            }),
        },
        Rule {
            name: "proximos n <cycle>".to_string(),
            pattern: vec![
                regex("pr(ó|o)xim(o|a)s?"),
                predicate(is_integer_between(1, 9999)),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let n = integer_value(&nodes[1].token_data)?;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain })))
            }),
        },
        Rule {
            name: "n pasados <cycle>".to_string(),
            pattern: vec![
                predicate(is_integer_between(1, 9999)),
                regex("pasad(a|o)s?"),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let n = integer_value(&nodes[0].token_data)?;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: n.checked_neg()?,
                    grain,
                })))
            }),
        },
        Rule {
            name: "n proximos <cycle>".to_string(),
            pattern: vec![
                predicate(is_integer_between(1, 9999)),
                regex("pr(ó|o)xim(o|a)s?|que vienen?|siguientes?"),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let n = integer_value(&nodes[0].token_data)?;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain { n, grain })))
            }),
        },
    ]);
    rules
}
