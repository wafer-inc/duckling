use crate::dimensions::numeral::helpers::{integer_value, numeral_data};
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{Direction, PartOfDay, TimeData, TimeForm};

fn is_integer_between(lo: i64, hi: i64) -> Box<dyn Fn(&TokenData) -> bool + Send + Sync> {
    Box::new(move |td: &TokenData| {
        if let Some(v) = integer_value(td) {
            v >= lo && v <= hi
        } else {
            false
        }
    })
}

fn time_data(td: &TokenData) -> Option<&TimeData> {
    match td {
        TokenData::Time(d) => Some(d),
        _ => None,
    }
}

fn is_time(td: &TokenData) -> bool {
    matches!(td, TokenData::Time(_))
}

fn is_part_of_day(td: &TokenData) -> bool {
    matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::PartOfDay(_)))
}

fn is_not_latent_time(td: &TokenData) -> bool {
    matches!(td, TokenData::Time(d) if !d.latent)
}

pub fn rules() -> Vec<Rule> {
    vec![
        // ====================================================================
        // Days of week (with word boundaries)
        // ====================================================================
        Rule {
            name: "day of week".to_string(),
            pattern: vec![regex(
                r"\b(monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon|tue|wed|thu|fri|sat|sun)\b\.?",
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let dow = match text.to_lowercase().as_ref() {
                    "monday" | "mon" => 0,
                    "tuesday" | "tue" => 1,
                    "wednesday" | "wed" => 2,
                    "thursday" | "thu" => 3,
                    "friday" | "fri" => 4,
                    "saturday" | "sat" => 5,
                    "sunday" | "sun" => 6,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        // ====================================================================
        // Month names (with word boundaries)
        // ====================================================================
        Rule {
            name: "month name".to_string(),
            pattern: vec![regex(
                r"\b(january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sep|oct|nov|dec)\b\.?",
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let month = match text.to_lowercase().as_ref() {
                    "january" | "jan" => 1,
                    "february" | "feb" => 2,
                    "march" | "mar" => 3,
                    "april" | "apr" => 4,
                    "may" => 5,
                    "june" | "jun" => 6,
                    "july" | "jul" => 7,
                    "august" | "aug" => 8,
                    "september" | "sep" => 9,
                    "october" | "oct" => 10,
                    "november" | "nov" => 11,
                    "december" | "dec" => 12,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
            }),
        },
        // ====================================================================
        // Now / ATM / ASAP / at this time
        // ====================================================================
        Rule {
            name: "now".to_string(),
            pattern: vec![regex(r"\b(now|right now|just now|at this time)\b")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "ATM".to_string(),
            pattern: vec![regex(r"\b(at the moment|atm)\b")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "ASAP".to_string(),
            pattern: vec![regex(r"\b(asap|as soon as possible)\b")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        // ====================================================================
        // Today / Tomorrow / Yesterday
        // ====================================================================
        Rule {
            name: "today".to_string(),
            pattern: vec![regex(r"\btoday\b")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow".to_string(),
            pattern: vec![regex(r"\btomorrows?\b")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday".to_string(),
            pattern: vec![regex(r"\byesterday\b")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        // ====================================================================
        // Day after tomorrow / Day before yesterday
        // ====================================================================
        Rule {
            name: "day after tomorrow".to_string(),
            pattern: vec![regex(r"\b(the )?day after tomorrow\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))
            }),
        },
        Rule {
            name: "day before yesterday".to_string(),
            pattern: vec![regex(r"\b(the )?day before yesterday\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))
            }),
        },
        // ====================================================================
        // Noon / Midnight
        // ====================================================================
        Rule {
            name: "noon".to_string(),
            pattern: vec![regex(r"\b(noon|(the )?midday|mid day)\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Hour(12, false))))
            }),
        },
        Rule {
            name: "midnight".to_string(),
            pattern: vec![regex(r"\bmidnight\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Hour(0, false))))
            }),
        },
        // ====================================================================
        // Part of day keywords (latent - need context to resolve)
        // ====================================================================
        Rule {
            name: "morning (latent)".to_string(),
            pattern: vec![regex(r"\b(morning|early morning)\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(
                    PartOfDay::Morning,
                ))))
            }),
        },
        Rule {
            name: "afternoon (latent)".to_string(),
            pattern: vec![regex(r"\bafternoon(ish)?\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(
                    PartOfDay::Afternoon,
                ))))
            }),
        },
        Rule {
            name: "evening (latent)".to_string(),
            pattern: vec![regex(r"\bevening\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(
                    PartOfDay::Evening,
                ))))
            }),
        },
        Rule {
            name: "night (latent)".to_string(),
            pattern: vec![regex(r"\bnight\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(
                    PartOfDay::Night,
                ))))
            }),
        },
        Rule {
            name: "lunch (latent)".to_string(),
            pattern: vec![regex(r"\blunch\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::latent(TimeForm::PartOfDay(
                    PartOfDay::Lunch,
                ))))
            }),
        },
        // ====================================================================
        // tonight / tonite
        // ====================================================================
        Rule {
            name: "tonight".to_string(),
            pattern: vec![regex(r"\b(tonight|tonite)\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(TimeData::new(TimeForm::PartOfDay(PartOfDay::Evening))),
                ))))
            }),
        },
        // ====================================================================
        // this/today + <part of day>
        // ====================================================================
        Rule {
            name: "this <part-of-day>".to_string(),
            pattern: vec![regex(r"\b(this|today)\b"), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let pod = time_data(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(pod.clone()),
                ))))
            }),
        },
        // in the <part of day>
        Rule {
            name: "in the <part-of-day>".to_string(),
            pattern: vec![regex(r"\bin the\b"), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let pod = time_data(&nodes[1].token_data)?;
                let mut result = pod.clone();
                result.latent = false;
                Some(TokenData::Time(result))
            }),
        },
        // last night / yesterday evening
        Rule {
            name: "last night".to_string(),
            pattern: vec![regex(r"\b(last|yesterday)\b"), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let pod = time_data(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Yesterday)),
                    Box::new(pod.clone()),
                ))))
            }),
        },
        // tomorrow + <part of day>
        Rule {
            name: "tomorrow <part-of-day>".to_string(),
            pattern: vec![regex(r"\btomorrow\b"), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let pod = time_data(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Tomorrow)),
                    Box::new(pod.clone()),
                ))))
            }),
        },
        // late tonight / late last night
        Rule {
            name: "late <time>".to_string(),
            pattern: vec![regex(r"\b(late|early)\b"), predicate(is_time)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                Some(TokenData::Time(t.clone()))
            }),
        },
        // ====================================================================
        // Weekend
        // ====================================================================
        Rule {
            name: "weekend".to_string(),
            pattern: vec![regex(r"\bweek[\s-]?ends?\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))
            }),
        },
        // wkend
        Rule {
            name: "wkend".to_string(),
            pattern: vec![regex(r"\bwkends?\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Weekend)))
            }),
        },
        // this past weekend
        Rule {
            name: "this past weekend".to_string(),
            pattern: vec![regex(r"\bthis past\b"), predicate(is_time)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                let mut new_t = t.clone();
                new_t.direction = Some(Direction::Past);
                Some(TokenData::Time(new_t))
            }),
        },
        // ====================================================================
        // Season words
        // ====================================================================
        Rule {
            name: "season word".to_string(),
            pattern: vec![regex(r"\b(spring|summer|fall|autumn|winter)\b")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let s = match text.to_lowercase().as_ref() {
                    "spring" => 0,
                    "summer" => 1,
                    "fall" | "autumn" => 2,
                    "winter" => 3,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Season(s))))
            }),
        },
        // season (generic keyword)
        Rule {
            name: "season (generic)".to_string(),
            pattern: vec![regex(r"\bseasons?\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Season(0))))
            }),
        },
        // ====================================================================
        // Clock times
        // ====================================================================
        // HH:MM
        Rule {
            name: "time HH:MM".to_string(),
            pattern: vec![regex(r"\b(\d{1,2}):(\d{2})\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let hour: u32 = m.group(1)?.parse().ok()?;
                let minute: u32 = m.group(2)?.parse().ok()?;
                if hour < 24 && minute < 60 {
                    Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                        hour, minute,
                    ))))
                } else {
                    None
                }
            }),
        },
        // HH:MM:SS
        Rule {
            name: "time HH:MM:SS".to_string(),
            pattern: vec![regex(r"\b(\d{1,2}):(\d{2}):(\d{2})\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let hour: u32 = m.group(1)?.parse().ok()?;
                let minute: u32 = m.group(2)?.parse().ok()?;
                let second: u32 = m.group(3)?.parse().ok()?;
                if hour < 24 && minute < 60 && second < 60 {
                    Some(TokenData::Time(TimeData::new(TimeForm::HourMinuteSecond(
                        hour, minute, second,
                    ))))
                } else {
                    None
                }
            }),
        },
        // XhYY / Xh format: 15h00, 3h18, 15h
        Rule {
            name: "time Xh(YY)".to_string(),
            pattern: vec![regex(r"\b(\d{1,2})h(\d{2})?\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let hour: u32 = m.group(1)?.parse().ok()?;
                if hour >= 24 {
                    return None;
                }
                match m.group(2) {
                    Some(min_str) => {
                        let minute: u32 = min_str.parse().ok()?;
                        if minute < 60 {
                            Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                                hour, minute,
                            ))))
                        } else {
                            None
                        }
                    }
                    None => Some(TokenData::Time(TimeData::new(TimeForm::Hour(
                        hour, false,
                    )))),
                }
            }),
        },
        // 4-digit HHMM: 1030, 0730
        Rule {
            name: "time HHMM (4-digit)".to_string(),
            pattern: vec![regex(r"\b([01]\d|2[0-3])([0-5]\d)\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let hour: u32 = m.group(1)?.parse().ok()?;
                let minute: u32 = m.group(2)?.parse().ok()?;
                Some(TokenData::Time(TimeData::latent(TimeForm::HourMinute(
                    hour, minute,
                ))))
            }),
        },
        // ====================================================================
        // Combined digit + AM/PM regex rules (handle "3pm", "3:18am", etc.)
        // These fire in the regex phase and don't need word-boundary tricks.
        // ====================================================================
        // HH:MM:SS + am/pm
        Rule {
            name: "HH:MM:SS ampm".to_string(),
            pattern: vec![regex(r"\b(\d{1,2}):(\d{2}):(\d{2})\s?([ap])\.?(\s?m\.?)?")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let hour: u32 = m.group(1)?.parse().ok()?;
                let minute: u32 = m.group(2)?.parse().ok()?;
                let second: u32 = m.group(3)?.parse().ok()?;
                let ap = m.group(4)?;
                if hour > 12 || minute >= 60 || second >= 60 {
                    return None;
                }
                let is_pm = ap.to_lowercase().starts_with('p');
                let hour = if is_pm && hour < 12 { hour + 12 } else if !is_pm && hour == 12 { 0 } else { hour };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinuteSecond(hour, minute, second))))
            }),
        },
        // HH:MM + am/pm
        Rule {
            name: "HH:MM ampm".to_string(),
            pattern: vec![regex(r"\b(\d{1,2}):(\d{2})\s?([ap])\.?(\s?m\.?)?")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let hour: u32 = m.group(1)?.parse().ok()?;
                let minute: u32 = m.group(2)?.parse().ok()?;
                let ap = m.group(3)?;
                if hour > 12 || minute >= 60 {
                    return None;
                }
                let is_pm = ap.to_lowercase().starts_with('p');
                let hour = if is_pm && hour < 12 { hour + 12 } else if !is_pm && hour == 12 { 0 } else { hour };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute))))
            }),
        },
        // H + am/pm (e.g., "3pm", "12am", "3 p.m.", "3p", "3p.")
        Rule {
            name: "H ampm".to_string(),
            pattern: vec![regex(r"\b(\d{1,2})\s?([ap])\.?\s?m\.?")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let hour: u32 = m.group(1)?.parse().ok()?;
                let ap = m.group(2)?;
                if hour > 12 || hour == 0 {
                    return None;
                }
                let is_pm = ap.to_lowercase().starts_with('p');
                let hour = if is_pm && hour < 12 { hour + 12 } else if !is_pm && hour == 12 { 0 } else { hour };
                Some(TokenData::Time(TimeData::new(TimeForm::Hour(hour, false))))
            }),
        },
        // 3-digit HMM + am/pm (e.g., "330 p.m.")
        Rule {
            name: "HMM ampm".to_string(),
            pattern: vec![regex(r"\b([1-9])([0-5]\d)\s?([ap])\.?(\s?m\.?)?")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let hour: u32 = m.group(1)?.parse().ok()?;
                let minute: u32 = m.group(2)?.parse().ok()?;
                let ap = m.group(3)?;
                if hour > 12 || minute >= 60 {
                    return None;
                }
                let is_pm = ap.to_lowercase().starts_with('p');
                let hour = if is_pm && hour < 12 { hour + 12 } else if !is_pm && hour == 12 { 0 } else { hour };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute))))
            }),
        },
        // 3-digit HMM + "in the morning/afternoon/evening" (e.g., "730 in the evening")
        Rule {
            name: "HMM in the <pod>".to_string(),
            pattern: vec![regex(r"\b([1-9])([0-5]\d)\s+in the (morning|afternoon|evening)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let hour: u32 = m.group(1)?.parse().ok()?;
                let minute: u32 = m.group(2)?.parse().ok()?;
                let pod = m.group(3)?;
                if hour > 12 || minute >= 60 {
                    return None;
                }
                let is_pm = matches!(pod.to_lowercase().as_ref(), "afternoon" | "evening");
                let hour = if is_pm && hour < 12 { hour + 12 } else { hour };
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute))))
            }),
        },
        // ====================================================================
        // Integer (1-12) → latent Hour (for word numerals like "three")
        // ====================================================================
        Rule {
            name: "integer as latent hour".to_string(),
            pattern: vec![predicate(|td| {
                if let Some(data) = numeral_data(td) {
                    let v = data.value;
                    v >= 1.0 && v <= 12.0 && v == v.floor() && !data.quantifier
                } else {
                    false
                }
            })],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let hour = num.value as u32;
                Some(TokenData::Time(TimeData::latent(TimeForm::Hour(hour, true))))
            }),
        },
        // ====================================================================
        // AM/PM
        // ====================================================================
        // <time> am/pm (extended: a.m., p.m., AM, PM)
        Rule {
            name: "<time> am/pm".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex(r"\b(a\.?\s?m\.?|p\.?\s?m\.?)"),
            ],
            production: Box::new(|nodes| {
                let ampm = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let is_pm = ampm.to_lowercase().starts_with('p');
                let td = time_data(&nodes[0].token_data)?;
                apply_ampm(&td.form, is_pm)
            }),
        },
        // <time> + single a/p: "3p", "3a" (requires non-latent time to prevent "4a" standalone)
        Rule {
            name: "<time> a/p suffix".to_string(),
            pattern: vec![
                predicate(is_not_latent_time),
                regex(r"([ap])\.?(?:\b|\s|$)"),
            ],
            production: Box::new(|nodes| {
                let ampm = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let is_pm = ampm.to_lowercase() == "p";
                let td = time_data(&nodes[0].token_data)?;
                // Only apply to hour/minute forms, not general time
                match &td.form {
                    TimeForm::Hour(_, _) | TimeForm::HourMinute(_, _) => {
                        apply_ampm(&td.form, is_pm)
                    }
                    _ => None,
                }
            }),
        },
        // <time> in the morning/afternoon/evening
        Rule {
            name: "<time> in the <part-of-day>".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex(r"\bin (the )?(morning|afternoon|evening)\b"),
            ],
            production: Box::new(|nodes| {
                let pod_text = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let is_pm = match pod_text.to_lowercase().as_ref() {
                    "afternoon" | "evening" => true,
                    _ => false,
                };
                let td = time_data(&nodes[0].token_data)?;
                match &td.form {
                    TimeForm::Hour(h, true) => {
                        let hour = if is_pm && *h < 12 {
                            h + 12
                        } else {
                            *h
                        };
                        Some(TokenData::Time(TimeData::new(TimeForm::Hour(
                            hour, false,
                        ))))
                    }
                    TimeForm::HourMinute(h, m) => {
                        let hour = if is_pm && *h < 12 {
                            h + 12
                        } else {
                            *h
                        };
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                            hour, *m,
                        ))))
                    }
                    _ => {
                        // Compose time with part of day
                        let pod = if is_pm {
                            PartOfDay::Afternoon
                        } else {
                            PartOfDay::Morning
                        };
                        Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                            Box::new(td.clone()),
                            Box::new(TimeData::new(TimeForm::PartOfDay(pod))),
                        ))))
                    }
                }
            }),
        },
        // in the AM/PM
        Rule {
            name: "<time> in the AM/PM".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex(r"\bin the (am|pm)\b"),
            ],
            production: Box::new(|nodes| {
                let ampm = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let is_pm = ampm.to_lowercase() == "pm";
                let td = time_data(&nodes[0].token_data)?;
                apply_ampm(&td.form, is_pm)
            }),
        },
        // ====================================================================
        // <number> o'clock (regex-based to exclude "single", "dozens", etc.)
        // ====================================================================
        Rule {
            name: "<number> o'clock".to_string(),
            pattern: vec![regex(
                r"\b(one|two|three|four|five|six|seven|eight|nine|ten|eleven|twelve|\d{1,2})\s+o[' ]?clock\b",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let text = m.group(1)?;
                let hour: u32 = match text.to_lowercase().as_ref() {
                    "one" | "1" => 1,
                    "two" | "2" => 2,
                    "three" | "3" => 3,
                    "four" | "4" => 4,
                    "five" | "5" => 5,
                    "six" | "6" => 6,
                    "seven" | "7" => 7,
                    "eight" | "8" => 8,
                    "nine" | "9" => 9,
                    "ten" | "10" => 10,
                    "eleven" | "11" => 11,
                    "twelve" | "12" => 12,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Hour(hour, true))))
            }),
        },
        // ====================================================================
        // at/@ <time>
        // ====================================================================
        Rule {
            name: "at <time>".to_string(),
            pattern: vec![regex(r"(\bat\b|@)"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                let mut result = t.clone();
                result.latent = false;
                Some(TokenData::Time(result))
            }),
        },
        // ====================================================================
        // on/during/in <time> (passthrough)
        // ====================================================================
        Rule {
            name: "on <time>".to_string(),
            pattern: vec![regex(r"\b(on|during)\b"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                Some(TokenData::Time(t.clone()))
            }),
        },
        // "in March", "in 2014" - contextual passthrough
        Rule {
            name: "in <time>".to_string(),
            pattern: vec![regex(r"\bin\b"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                match &t.form {
                    TimeForm::Month(_) | TimeForm::Year(_) | TimeForm::Season(_)
                    | TimeForm::Holiday(_) | TimeForm::Quarter(_) | TimeForm::QuarterYear(_, _) => {
                        let mut result = t.clone();
                        result.latent = false;
                        Some(TokenData::Time(result))
                    }
                    _ => None,
                }
            }),
        },
        // ====================================================================
        // around/about/approximately <time>
        // ====================================================================
        Rule {
            name: "around <time>".to_string(),
            pattern: vec![
                regex(r"\b(around|about|approximately|roughly)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                let mut result = t.clone();
                result.latent = false;
                Some(TokenData::Time(result))
            }),
        },
        // <time> approximately
        Rule {
            name: "<time> approximately".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex(r"\b(approximately|roughly|ish)\b"),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?;
                Some(TokenData::Time(t.clone()))
            }),
        },
        // ====================================================================
        // in <duration> (future)
        // ====================================================================
        Rule {
            name: "in <duration>".to_string(),
            pattern: vec![regex(r"\bin\b"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let dur = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: dur.value,
                    grain: dur.grain,
                })))
            }),
        },
        // ====================================================================
        // <duration> ago
        // ====================================================================
        Rule {
            name: "<duration> ago".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex(r"\bago\b")],
            production: Box::new(|nodes| {
                let dur = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: -dur.value,
                    grain: dur.grain,
                })))
            }),
        },
        // ====================================================================
        // <duration> hence
        // ====================================================================
        Rule {
            name: "<duration> hence".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex(r"\bhence\b")],
            production: Box::new(|nodes| {
                let dur = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: dur.value,
                    grain: dur.grain,
                })))
            }),
        },
        // ====================================================================
        // <duration> from now / back
        // ====================================================================
        Rule {
            name: "<duration> from now".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex(r"\b(from now|from today|from right now)\b")],
            production: Box::new(|nodes| {
                let dur = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: dur.value,
                    grain: dur.grain,
                })))
            }),
        },
        Rule {
            name: "<duration> back".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex(r"\bback\b")],
            production: Box::new(|nodes| {
                let dur = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: -dur.value,
                    grain: dur.grain,
                })))
            }),
        },
        // after <duration> (= in <duration>)
        Rule {
            name: "after <duration>".to_string(),
            pattern: vec![regex(r"\bafter\b"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let dur = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: dur.value,
                    grain: dur.grain,
                })))
            }),
        },
        // within <duration>
        Rule {
            name: "within <duration>".to_string(),
            pattern: vec![regex(r"\bwithin\b"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let dur = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: dur.value,
                    grain: dur.grain,
                })))
            }),
        },
        // ====================================================================
        // Date formats: MM/DD, MM/DD/YYYY, MM-DD, YYYY-MM-DD, DD.MM.YYYY
        // ====================================================================
        Rule {
            name: "date MM/DD(/YYYY)".to_string(),
            pattern: vec![regex(r"\b(\d{1,2})\s?[/\-]\s?(\d{1,2})(?:\s?[/\-]\s?(\d{2,4}))?\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let v1: u32 = m.group(1)?.parse().ok()?;
                let v2: u32 = m.group(2)?.parse().ok()?;
                let year = m.group(3).and_then(|y| {
                    let yr: i32 = y.parse().ok()?;
                    if yr < 100 { Some(yr + 2000) } else { Some(yr) }
                });
                if v1 >= 1 && v1 <= 12 && v2 >= 1 && v2 <= 31 {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                        month: v1,
                        day: v2,
                        year,
                    })))
                } else {
                    None
                }
            }),
        },
        // YYYY-MM-DD
        Rule {
            name: "date YYYY-MM-DD".to_string(),
            pattern: vec![regex(r"\b(\d{4})\s?[/\-]\s?(\d{1,2})(?:\s?[/\-]\s?(\d{1,2}))?\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let year: i32 = m.group(1)?.parse().ok()?;
                let month: u32 = m.group(2)?.parse().ok()?;
                if month < 1 || month > 12 {
                    return None;
                }
                match m.group(3) {
                    Some(d) => {
                        let day: u32 = d.parse().ok()?;
                        if day >= 1 && day <= 31 {
                            Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                                month,
                                day,
                                year: Some(year),
                            })))
                        } else {
                            None
                        }
                    }
                    None => {
                        // YYYY-MM or YYYY/MM (e.g., "2014/10")
                        Some(TokenData::Time(TimeData::new(TimeForm::Month(month))))
                    }
                }
            }),
        },
        // DD.MM.YYYY
        Rule {
            name: "date DD.MM.YYYY".to_string(),
            pattern: vec![regex(r"\b(\d{1,2})\.(\d{1,2})\.(\d{4})\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let v1: u32 = m.group(1)?.parse().ok()?;
                let v2: u32 = m.group(2)?.parse().ok()?;
                let year: i32 = m.group(3)?.parse().ok()?;
                // Try MM.DD.YYYY first (American), fallback to DD.MM.YYYY
                if v1 >= 1 && v1 <= 12 && v2 >= 1 && v2 <= 31 {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                        month: v1,
                        day: v2,
                        year: Some(year),
                    })))
                } else {
                    None
                }
            }),
        },
        // DD/Mon/YYYY: 31/Oct/1974
        Rule {
            name: "date DD/Mon/YYYY".to_string(),
            pattern: vec![regex(
                r"\b(\d{1,2})\s?[/\-]\s?(jan|feb|mar|apr|may|jun|jul|aug|sep|oct|nov|dec)\s?[/\-]\s?(\d{2,4})\b",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let day: u32 = m.group(1)?.parse().ok()?;
                let month_name = m.group(2)?;
                let year_str = m.group(3)?;
                let month = month_name_to_num(month_name)?;
                let year: i32 = year_str.parse().ok()?;
                let year = if year < 100 { year + 1900 } else { year };
                if day >= 1 && day <= 31 {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                        month,
                        day,
                        year: Some(year),
                    })))
                } else {
                    None
                }
            }),
        },
        // ====================================================================
        // Year (4 digits) - latent to avoid matching "Pay ABC 2000"
        // ====================================================================
        Rule {
            name: "year (4 digits)".to_string(),
            pattern: vec![regex(r"\b(1\d{3}|20\d{2})\b")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = text.parse().ok()?;
                Some(TokenData::Time(TimeData::latent(TimeForm::Year(year))))
            }),
        },
        // in <year> (makes year non-latent)
        Rule {
            name: "in <year>".to_string(),
            pattern: vec![regex(r"\bin\b"), predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Year(_))))],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                let mut result = t.clone();
                result.latent = false;
                Some(TokenData::Time(result))
            }),
        },
        // <year> AD/BC
        Rule {
            name: "<year> AD/BC".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex(r"\b(a\.?d\.?|b\.?c\.?)\b"),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?;
                match &t.form {
                    TimeForm::Year(_) => {
                        let mut result = t.clone();
                        result.latent = false;
                        Some(TokenData::Time(result))
                    }
                    _ => None,
                }
            }),
        },
        // in <integer> A.D. (e.g., "in 14 a.d.")
        Rule {
            name: "in <integer> AD".to_string(),
            pattern: vec![
                regex(r"\bin\b"),
                predicate(is_integer_between(1, 9999)),
                regex(r"\b(a\.?d\.?|b\.?c\.?)\b"),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[1].token_data)?;
                let year = num.value as i32;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        // ====================================================================
        // last/next <day-of-week or month>
        // ====================================================================
        Rule {
            name: "last <time>".to_string(),
            pattern: vec![regex(r"\b(last|past|previous)\b"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                match &t.form {
                    TimeForm::DayOfWeek(_) | TimeForm::Month(_) | TimeForm::Holiday(_)
                    | TimeForm::Season(_) | TimeForm::Weekend => {
                        let mut new_t = t.clone();
                        new_t.direction = Some(Direction::Past);
                        new_t.latent = false;
                        Some(TokenData::Time(new_t))
                    }
                    _ => None,
                }
            }),
        },
        Rule {
            name: "next <time>".to_string(),
            pattern: vec![
                regex(r"\b(next|following|upcoming|coming)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                match &t.form {
                    TimeForm::DayOfWeek(_) | TimeForm::Month(_) | TimeForm::Holiday(_)
                    | TimeForm::Season(_) | TimeForm::Weekend => {
                        let mut new_t = t.clone();
                        new_t.direction = Some(Direction::Future);
                        new_t.latent = false;
                        Some(TokenData::Time(new_t))
                    }
                    _ => None,
                }
            }),
        },
        // this <dow/month/season/weekend/holiday>
        Rule {
            name: "this <time>".to_string(),
            pattern: vec![regex(r"\b(this|current)\b"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                match &t.form {
                    TimeForm::DayOfWeek(_) | TimeForm::Month(_) | TimeForm::Holiday(_)
                    | TimeForm::Season(_) | TimeForm::Weekend => {
                        let mut new_t = t.clone();
                        new_t.latent = false;
                        Some(TokenData::Time(new_t))
                    }
                    _ => None,
                }
            }),
        },
        // <time> after next (e.g., "friday after next")
        Rule {
            name: "<time> after next".to_string(),
            pattern: vec![dim(DimensionKind::Time), regex(r"\bafter next\b")],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?;
                match &t.form {
                    TimeForm::DayOfWeek(_) | TimeForm::Month(_) => {
                        let mut new_t = t.clone();
                        new_t.direction = Some(Direction::Future);
                        Some(TokenData::Time(new_t))
                    }
                    _ => None,
                }
            }),
        },
        // ====================================================================
        // This/last/next week/month/year/quarter
        // ====================================================================
        Rule {
            name: "this <grain>".to_string(),
            pattern: vec![
                regex(r"\b(this|current)\b"),
                regex(r"\b(week|month|year|yr|quarter|qtr)\b"),
            ],
            production: Box::new(|nodes| {
                let grain_text = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = text_to_grain(grain_text)?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: 0,
                })))
            }),
        },
        Rule {
            name: "last <grain>".to_string(),
            pattern: vec![
                regex(r"\b(last|past|previous)\b"),
                regex(r"\b(week|month|year|yr|quarter|qtr)\b"),
            ],
            production: Box::new(|nodes| {
                let grain_text = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = text_to_grain(grain_text)?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: -1,
                })))
            }),
        },
        Rule {
            name: "next <grain>".to_string(),
            pattern: vec![
                regex(r"\b(next|following|upcoming|coming)\b"),
                regex(r"\b(week|month|year|yr|quarter|qtr)\b"),
            ],
            production: Box::new(|nodes| {
                let grain_text = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = text_to_grain(grain_text)?;
                Some(TokenData::Time(TimeData::new(TimeForm::GrainOffset {
                    grain,
                    offset: 1,
                })))
            }),
        },
        // ====================================================================
        // Last/next N <grain>
        // ====================================================================
        Rule {
            name: "last <integer> <grain>".to_string(),
            pattern: vec![
                regex(r"\b(last|past)\b"),
                predicate(is_integer_between(1, 9999)),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[1].token_data)?;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: num.value as i64,
                    grain,
                    past: true,
                })))
            }),
        },
        Rule {
            name: "next <integer> <grain>".to_string(),
            pattern: vec![
                regex(r"\b(next)\b"),
                predicate(is_integer_between(1, 9999)),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[1].token_data)?;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: num.value as i64,
                    grain,
                    past: false,
                })))
            }),
        },
        // upcoming <integer> <grain>
        Rule {
            name: "upcoming <integer> <grain>".to_string(),
            pattern: vec![
                regex(r"\b(upcoming)\b"),
                predicate(is_integer_between(1, 9999)),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[1].token_data)?;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: num.value as i64,
                    grain,
                    past: false,
                })))
            }),
        },
        // <integer> upcoming <grain>
        Rule {
            name: "<integer> upcoming <grain>".to_string(),
            pattern: vec![
                predicate(is_integer_between(1, 9999)),
                regex(r"\b(upcoming)\b"),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrain {
                    n: num.value as i64,
                    grain,
                    past: false,
                })))
            }),
        },
        // ====================================================================
        // Ordinal + month name → date
        // ====================================================================
        Rule {
            name: "ordinal of <month>".to_string(),
            pattern: vec![
                dim(DimensionKind::Ordinal),
                regex(r"\b(of )?\b"),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
            ],
            production: Box::new(|nodes| {
                let ord = match &nodes[0].token_data {
                    TokenData::Ordinal(d) => d.value as u32,
                    _ => return None,
                };
                let month = match &nodes[2].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Month(m) => *m,
                        _ => return None,
                    },
                    _ => return None,
                };
                if ord >= 1 && ord <= 31 {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                        month,
                        day: ord,
                        year: None,
                    })))
                } else {
                    None
                }
            }),
        },
        // <month> + ordinal/integer → date
        Rule {
            name: "<month> <ordinal>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
                dim(DimensionKind::Ordinal),
            ],
            production: Box::new(|nodes| {
                let month = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Month(m) => *m,
                        _ => return None,
                    },
                    _ => return None,
                };
                let day = match &nodes[1].token_data {
                    TokenData::Ordinal(d) => d.value as u32,
                    _ => return None,
                };
                if day >= 1 && day <= 31 {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                        month,
                        day,
                        year: None,
                    })))
                } else {
                    None
                }
            }),
        },
        // <month> <integer> (e.g., "march 3", "Aug 8")
        Rule {
            name: "<month> <integer>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
                predicate(is_integer_between(1, 31)),
            ],
            production: Box::new(|nodes| {
                let month = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Month(m) => *m,
                        _ => return None,
                    },
                    _ => return None,
                };
                let day = numeral_data(&nodes[1].token_data)?.value as u32;
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: None,
                })))
            }),
        },
        // <integer> <month> (e.g., "15 of february", "14april")
        Rule {
            name: "<integer> <month>".to_string(),
            pattern: vec![
                predicate(is_integer_between(1, 31)),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
            ],
            production: Box::new(|nodes| {
                let day = numeral_data(&nodes[0].token_data)?.value as u32;
                let month = match &nodes[1].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Month(m) => *m,
                        _ => return None,
                    },
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: None,
                })))
            }),
        },
        // ordinal + <month> (e.g., "15th february", "1st of march")
        Rule {
            name: "<ordinal> <month>".to_string(),
            pattern: vec![
                dim(DimensionKind::Ordinal),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
            ],
            production: Box::new(|nodes| {
                let day = match &nodes[0].token_data {
                    TokenData::Ordinal(d) => d.value as u32,
                    _ => return None,
                };
                let month = match &nodes[1].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Month(m) => *m,
                        _ => return None,
                    },
                    _ => return None,
                };
                if day >= 1 && day <= 31 {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                        month,
                        day,
                        year: None,
                    })))
                } else {
                    None
                }
            }),
        },
        // the ides of march
        Rule {
            name: "the ides of <month>".to_string(),
            pattern: vec![
                regex(r"\b(the )?ides of\b"),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
            ],
            production: Box::new(|nodes| {
                let month = match &nodes[1].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Month(m) => *m,
                        _ => return None,
                    },
                    _ => return None,
                };
                let day = if month == 3 || month == 5 || month == 7 || month == 10 { 15 } else { 13 };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: None,
                })))
            }),
        },
        // the Nth (day of month)
        Rule {
            name: "the <ordinal> (day of month)".to_string(),
            pattern: vec![regex(r"\b(the|on the)\b"), dim(DimensionKind::Ordinal)],
            production: Box::new(|nodes| {
                let day = match &nodes[1].token_data {
                    TokenData::Ordinal(d) => d.value as u32,
                    _ => return None,
                };
                if day >= 1 && day <= 31 {
                    Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
                } else {
                    None
                }
            }),
        },
        // on the first (word ordinal)
        Rule {
            name: "on the <ordinal-word>".to_string(),
            pattern: vec![regex(r"\bon (the )?\b"), dim(DimensionKind::Ordinal)],
            production: Box::new(|nodes| {
                let day = match &nodes[1].token_data {
                    TokenData::Ordinal(d) => d.value as u32,
                    _ => return None,
                };
                if day >= 1 && day <= 31 {
                    Some(TokenData::Time(TimeData::new(TimeForm::DayOfMonth(day))))
                } else {
                    None
                }
            }),
        },
        // <date> + year: "march 3 2015", "April 14, 2015"
        Rule {
            name: "<date> <year>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::DateMDY { year: None, .. }))),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Year(_)))),
            ],
            production: Box::new(|nodes| {
                let (month, day) = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::DateMDY { month, day, .. } => (*month, *day),
                        _ => return None,
                    },
                    _ => return None,
                };
                let year = match &nodes[1].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Year(y) => *y,
                        _ => return None,
                    },
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: Some(year),
                })))
            }),
        },
        // ====================================================================
        // Quarters
        // ====================================================================
        // Nth quarter: "third quarter", "3rd quarter", "3rd qtr"
        Rule {
            name: "<ordinal> quarter".to_string(),
            pattern: vec![
                dim(DimensionKind::Ordinal),
                regex(r"\b(quarter|qtr)\b"),
            ],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data {
                    TokenData::Ordinal(d) => d.value as u32,
                    _ => return None,
                };
                if q >= 1 && q <= 4 {
                    Some(TokenData::Time(TimeData::new(TimeForm::Quarter(q))))
                } else {
                    None
                }
            }),
        },
        // "the 3rd qtr" - with "the" prefix
        Rule {
            name: "the <ordinal> quarter".to_string(),
            pattern: vec![
                regex(r"\bthe\b"),
                dim(DimensionKind::Ordinal),
                regex(r"\b(quarter|qtr)\b"),
            ],
            production: Box::new(|nodes| {
                let q = match &nodes[1].token_data {
                    TokenData::Ordinal(d) => d.value as u32,
                    _ => return None,
                };
                if q >= 1 && q <= 4 {
                    Some(TokenData::Time(TimeData::new(TimeForm::Quarter(q))))
                } else {
                    None
                }
            }),
        },
        // <quarter> + year: "4th quarter 2018", "4th qtr 2018"
        Rule {
            name: "<quarter> <year>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Quarter(_)))),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Year(_)))),
            ],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Quarter(q) => *q,
                        _ => return None,
                    },
                    _ => return None,
                };
                let year = match &nodes[1].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Year(y) => *y,
                        _ => return None,
                    },
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(q, year))))
            }),
        },
        // <quarter> of <year>
        Rule {
            name: "<quarter> of <year>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Quarter(_)))),
                regex(r"\bof\b"),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Year(_)))),
            ],
            production: Box::new(|nodes| {
                let q = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Quarter(q) => *q,
                        _ => return None,
                    },
                    _ => return None,
                };
                let year = match &nodes[2].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Year(y) => *y,
                        _ => return None,
                    },
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(q, year))))
            }),
        },
        // 18q4, 2018Q4
        Rule {
            name: "YYqN / YYYYqN".to_string(),
            pattern: vec![regex(r"\b(\d{2,4})q([1-4])\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let year_str = m.group(1)?;
                let q: u32 = m.group(2)?.parse().ok()?;
                let year: i32 = year_str.parse().ok()?;
                let year = if year < 100 { year + 2000 } else { year };
                Some(TokenData::Time(TimeData::new(TimeForm::QuarterYear(q, year))))
            }),
        },
        // ====================================================================
        // All week / rest of the week
        // ====================================================================
        Rule {
            name: "all week".to_string(),
            pattern: vec![regex(r"\ball (week|month|year)\b")],
            production: Box::new(|nodes| {
                let grain_text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = text_to_grain(grain_text)?;
                Some(TokenData::Time(TimeData::new(TimeForm::AllGrain(grain))))
            }),
        },
        Rule {
            name: "rest of the week".to_string(),
            pattern: vec![regex(r"\brest of (the )?(week|month|year)\b")],
            production: Box::new(|nodes| {
                let grain_text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let grain = text_to_grain(grain_text)?;
                Some(TokenData::Time(TimeData::new(TimeForm::RestOfGrain(grain))))
            }),
        },
        // ====================================================================
        // Beginning/end of week/month/year
        // ====================================================================
        Rule {
            name: "beginning of <grain>".to_string(),
            pattern: vec![
                regex(r"\b(beginning|start) of( the| this| current)?\b"),
                regex(r"\b(week|month|year)\b"),
            ],
            production: Box::new(|nodes| {
                let grain_text = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = text_to_grain(grain_text)?;
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: true,
                    target: Box::new(TimeForm::GrainOffset { grain, offset: 0 }),
                })))
            }),
        },
        Rule {
            name: "end of <grain>".to_string(),
            pattern: vec![
                regex(r"\b(end) of( the| this| current)?\b"),
                regex(r"\b(week|month|year)\b"),
            ],
            production: Box::new(|nodes| {
                let grain_text = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = text_to_grain(grain_text)?;
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::GrainOffset { grain, offset: 0 }),
                })))
            }),
        },
        // beginning/end of next/last/coming/past/previous/following week
        Rule {
            name: "beginning of <modifier> <grain>".to_string(),
            pattern: vec![
                regex(r"\b(beginning|start) of( the)?\b"),
                regex(r"\b(next|last|past|previous|coming|following)\b"),
                regex(r"\b(week|month|year)\b"),
            ],
            production: Box::new(|nodes| {
                let modifier = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain_text = match &nodes[2].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = text_to_grain(grain_text)?;
                let offset = modifier_to_offset(modifier);
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: true,
                    target: Box::new(TimeForm::GrainOffset { grain, offset }),
                })))
            }),
        },
        Rule {
            name: "end of <modifier> <grain>".to_string(),
            pattern: vec![
                regex(r"\b(end) of( the)?\b"),
                regex(r"\b(next|last|past|previous|coming|following)\b"),
                regex(r"\b(week|month|year)\b"),
            ],
            production: Box::new(|nodes| {
                let modifier = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain_text = match &nodes[2].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = text_to_grain(grain_text)?;
                let offset = modifier_to_offset(modifier);
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::GrainOffset { grain, offset }),
                })))
            }),
        },
        // beginning/end of <month/year named>
        Rule {
            name: "beginning of <time>".to_string(),
            pattern: vec![
                regex(r"\b(beginning|start|end) of( the| this)?\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let begin_text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let begin = begin_text.to_lowercase() != "end";
                let t = time_data(&nodes[1].token_data)?;
                match &t.form {
                    TimeForm::Month(_) | TimeForm::Year(_) => {
                        Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                            begin,
                            target: Box::new(t.form.clone()),
                        })))
                    }
                    _ => None,
                }
            }),
        },
        // EOM / BOM / EOY / BOY / EOD
        Rule {
            name: "EOM/BOM/EOY/BOY/EOD".to_string(),
            pattern: vec![regex(r"\b(the )?(eom|bom|eoy|boy|eod)\b")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                match text.to_lowercase().as_ref() {
                    "eom" => Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                        begin: false,
                        target: Box::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 0 }),
                    }))),
                    "bom" => Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                        begin: true,
                        target: Box::new(TimeForm::GrainOffset { grain: Grain::Month, offset: 0 }),
                    }))),
                    "eoy" => Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                        begin: false,
                        target: Box::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 0 }),
                    }))),
                    "boy" => Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                        begin: true,
                        target: Box::new(TimeForm::GrainOffset { grain: Grain::Year, offset: 0 }),
                    }))),
                    "eod" => Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                        begin: false,
                        target: Box::new(TimeForm::GrainOffset { grain: Grain::Day, offset: 0 }),
                    }))),
                    _ => None,
                }
            }),
        },
        // "end of the month" / "beginning of the month" / "end of the year" / etc.
        Rule {
            name: "end/beginning of the month/year".to_string(),
            pattern: vec![regex(r"\b(beginning|end) of (the )?(month|year)\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let begin = m.group(1)?.to_lowercase() != "end";
                let grain_text = m.group(3)?;
                let grain = text_to_grain(grain_text)?;
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin,
                    target: Box::new(TimeForm::GrainOffset { grain, offset: 0 }),
                })))
            }),
        },
        // "the beginning of the year" / "the end of the year"
        Rule {
            name: "the beginning/end of the year".to_string(),
            pattern: vec![regex(r"\bthe (beginning|end) of (the )?(month|year)\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let begin = m.group(1)?.to_lowercase() != "end";
                let grain_text = m.group(3)?;
                let grain = text_to_grain(grain_text)?;
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin,
                    target: Box::new(TimeForm::GrainOffset { grain, offset: 0 }),
                })))
            }),
        },
        // "by end/eom/eoy"
        Rule {
            name: "by <time>".to_string(),
            pattern: vec![regex(r"\bby( the)?\b"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                Some(TokenData::Time(t.clone()))
            }),
        },
        // ====================================================================
        // until/through/before/since/after + <time>
        // ====================================================================
        Rule {
            name: "until/before/since/after <time>".to_string(),
            pattern: vec![
                regex(r"\b(until|through|before|since|after|from|anytime after|sometimes before)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                // Accept latent Year tokens (context disambiguates: "since 2014")
                if t.latent && !matches!(t.form, TimeForm::Year(_)) {
                    return None;
                }
                let mut result = t.clone();
                result.latent = false;
                Some(TokenData::Time(result))
            }),
        },
        // ====================================================================
        // after lunch / after school
        // ====================================================================
        Rule {
            name: "after <part-of-day>".to_string(),
            pattern: vec![regex(r"\b(after|before)\b"), predicate(is_part_of_day)],
            production: Box::new(|nodes| {
                let pod = time_data(&nodes[1].token_data)?;
                let mut result = pod.clone();
                result.latent = false;
                Some(TokenData::Time(result))
            }),
        },
        // "after school" - direct
        Rule {
            name: "after school".to_string(),
            pattern: vec![regex(r"\bafter school\b")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::PartOfDay(
                    PartOfDay::Afternoon,
                ))))
            }),
        },
        // ====================================================================
        // Holidays
        // ====================================================================
        Rule {
            name: "holidays".to_string(),
            pattern: vec![regex(&holidays_regex())],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(
                    text.to_lowercase(),
                ))))
            }),
        },
        // <holiday> + year
        Rule {
            name: "<holiday> <year>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Holiday(_)))),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Year(_)))),
            ],
            production: Box::new(|nodes| {
                let holiday_name = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Holiday(name) => name.clone(),
                        _ => return None,
                    },
                    _ => return None,
                };
                // Just keep as holiday - year context
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(holiday_name))))
            }),
        },
        // <holiday> in <year>
        Rule {
            name: "<holiday> in <year>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Holiday(_)))),
                regex(r"\bin\b"),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Year(_)))),
            ],
            production: Box::new(|nodes| {
                let holiday_name = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Holiday(name) => name.clone(),
                        _ => return None,
                    },
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(holiday_name))))
            }),
        },
        // <duration> after/from <holiday>
        Rule {
            name: "<duration> after <time>".to_string(),
            pattern: vec![
                dim(DimensionKind::Duration),
                regex(r"\b(after|from)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[2].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t.clone()),
                    Box::new(TimeData::new(TimeForm::Now)),
                ))))
            }),
        },
        // ====================================================================
        // <time> + <time> composition (generic)
        // ====================================================================
        // <time> + <time> for general composition (e.g., "tomorrow at 5pm", "Monday morning")
        Rule {
            name: "<time> <time> compose".to_string(),
            pattern: vec![
                predicate(is_not_latent_time),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?;
                let t2 = time_data(&nodes[1].token_data)?;
                // Don't compose two of the same type
                if std::mem::discriminant(&t1.form) == std::mem::discriminant(&t2.form) {
                    return None;
                }
                // Don't compose two latent tokens
                if t1.latent && t2.latent {
                    return None;
                }
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t1.clone()),
                    Box::new(t2.clone()),
                ))))
            }),
        },
        // ====================================================================
        // Time intervals
        // ====================================================================
        // <time> - <time> (e.g., "9:30 - 11:00", "3-4pm", "8am - 1pm")
        Rule {
            name: "<time> - <time> interval".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex(r"\s*[\-\u{2013}]\s*"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?;
                let t2 = time_data(&nodes[2].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(t1.clone()),
                    Box::new(t2.clone()),
                ))))
            }),
        },
        // from <time> to/till <time>
        Rule {
            name: "from <time> to <time>".to_string(),
            pattern: vec![
                regex(r"\b(from|between)\b"),
                dim(DimensionKind::Time),
                regex(r"\b(to|till|until|and|thru|through)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[1].token_data)?;
                let t2 = time_data(&nodes[3].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(t1.clone()),
                    Box::new(t2.clone()),
                ))))
            }),
        },
        // <time> to/till <time>
        Rule {
            name: "<time> to <time>".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex(r"\b(to|till|until|thru|through)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?;
                let t2 = time_data(&nodes[2].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(t1.clone()),
                    Box::new(t2.clone()),
                ))))
            }),
        },
        // <time> for <duration>
        Rule {
            name: "<time> for <duration>".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex(r"\bfor\b"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?;
                Some(TokenData::Time(t.clone()))
            }),
        },
        // for <duration> from/starting <time>
        Rule {
            name: "for <duration> from <time>".to_string(),
            pattern: vec![
                regex(r"\bfor\b"),
                dim(DimensionKind::Duration),
                regex(r"\b(from|starting|starting from)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[3].token_data)?;
                Some(TokenData::Time(t.clone()))
            }),
        },
        // from <time> for <duration>
        Rule {
            name: "from <time> for <duration>".to_string(),
            pattern: vec![
                regex(r"\bfrom\b"),
                dim(DimensionKind::Time),
                regex(r"\bfor\b"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                Some(TokenData::Time(t.clone()))
            }),
        },
        // ====================================================================
        // Half past / quarter past / quarter to / N past/to
        // ====================================================================
        Rule {
            name: "half past <time>".to_string(),
            pattern: vec![
                regex(r"\bhalf (past|after)?\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                match &t.form {
                    TimeForm::Hour(h, _) => {
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(*h, 30))))
                    }
                    _ => None,
                }
            }),
        },
        Rule {
            name: "quarter past <time>".to_string(),
            pattern: vec![
                regex(r"\b(a )?quarter (past|after)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                match &t.form {
                    TimeForm::Hour(h, _) => {
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(*h, 15))))
                    }
                    _ => None,
                }
            }),
        },
        Rule {
            name: "quarter to <time>".to_string(),
            pattern: vec![
                regex(r"\b(a )?quarter (to|before|of)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                match &t.form {
                    TimeForm::Hour(h, _) => {
                        let hour = if *h == 0 { 23 } else { h - 1 };
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 45))))
                    }
                    _ => None,
                }
            }),
        },
        // <integer> past/after <time> (e.g., "15 past 3pm", "20 past 3pm")
        Rule {
            name: "<integer> past <time>".to_string(),
            pattern: vec![
                predicate(is_integer_between(1, 59)),
                regex(r"\b(past|after)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let mins = numeral_data(&nodes[0].token_data)?.value as u32;
                let t = time_data(&nodes[2].token_data)?;
                match &t.form {
                    TimeForm::Hour(h, _) => {
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(*h, mins))))
                    }
                    _ => None,
                }
            }),
        },
        // <integer> to/before <time>
        Rule {
            name: "<integer> to <time>".to_string(),
            pattern: vec![
                predicate(is_integer_between(1, 59)),
                regex(r"\b(to|before|of|til)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let mins = numeral_data(&nodes[0].token_data)?.value as u32;
                let t = time_data(&nodes[2].token_data)?;
                match &t.form {
                    TimeForm::Hour(h, _) => {
                        let hour = if *h == 0 { 23 } else { h - 1 };
                        let minute = 60 - mins;
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute))))
                    }
                    _ => None,
                }
            }),
        },
        // <time> <integer> (e.g., "at 3 15" → 3:15)
        Rule {
            name: "<time:hour> <integer:minute>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Hour(_, _)))),
                predicate(is_integer_between(0, 59)),
            ],
            production: Box::new(|nodes| {
                let (h, is_latent) = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Hour(h, _) => (*h, d.latent),
                        _ => return None,
                    },
                    _ => return None,
                };
                let m = numeral_data(&nodes[1].token_data)?.value as u32;
                let td = if is_latent {
                    TimeData::latent(TimeForm::HourMinute(h, m))
                } else {
                    TimeData::new(TimeForm::HourMinute(h, m))
                };
                Some(TokenData::Time(td))
            }),
        },
        // ====================================================================
        // ISO datetime interval: "2015-03-28 17:00:00/2015-03-29 21:00:00"
        // ====================================================================
        Rule {
            name: "ISO interval".to_string(),
            pattern: vec![regex(
                r"\b(\d{4})-(\d{2})-(\d{2})\s+(\d{2}):(\d{2}):(\d{2})\s*/\s*(\d{4})-(\d{2})-(\d{2})\s+(\d{2}):(\d{2}):(\d{2})\b",
            )],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::Now)))
            }),
        },
        // ====================================================================
        // <N> <DOW>s from now (e.g., "3 fridays from now")
        // ====================================================================
        Rule {
            name: "<N> <DOW>s from now".to_string(),
            pattern: vec![
                predicate(is_integer_between(1, 52)),
                regex(r"\b(mondays?|tuesdays?|wednesdays?|thursdays?|fridays?|saturdays?|sundays?) (from now|from today|hence|ago|back)\b"),
            ],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i64;
                let m = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let direction_text = m.group(2)?;
                let n = if direction_text.to_lowercase().contains("ago") || direction_text.to_lowercase().contains("back") {
                    -n
                } else {
                    n
                };
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n: n * 7,
                    grain: Grain::Day,
                })))
            }),
        },
        // (3-digit HMM removed - handled by context-specific combined rules above)
        // ====================================================================
        // Digit + month name (no space): "14april", "3jan"
        // ====================================================================
        Rule {
            name: "digit<month> (no space)".to_string(),
            pattern: vec![regex(
                r"\b(\d{1,2})(january|february|march|april|may|june|july|august|september|october|november|december|jan|feb|mar|apr|jun|jul|aug|sep|oct|nov|dec)\b",
            )],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let day: u32 = m.group(1)?.parse().ok()?;
                let month_str = m.group(2)?;
                let month = month_name_to_num(month_str)?;
                if day >= 1 && day <= 31 {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                        month,
                        day,
                        year: None,
                    })))
                } else {
                    None
                }
            }),
        },
        // ====================================================================
        // M/YYYY format: "2/2013"
        // ====================================================================
        Rule {
            name: "M/YYYY".to_string(),
            pattern: vec![regex(r"\b(\d{1,2})/(\d{4})\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let month: u32 = m.group(1)?.parse().ok()?;
                let year: i32 = m.group(2)?.parse().ok()?;
                if month >= 1 && month <= 12 {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                        month,
                        day: 1,
                        year: Some(year),
                    })))
                } else {
                    None
                }
            }),
        },
        // ====================================================================
        // Timezone passthrough: "<time> CET/GMT/EST/etc."
        // ====================================================================
        Rule {
            name: "<time> <timezone>".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex(r"\b(cet|cest|gmt|utc|est|edt|cst|cdt|mst|mdt|pst|pdt|eet|eest|wet|west|bst|ist|jst|kst|hkt|sgt|aest|aedt|acst|acdt|awst|nzst|nzdt)\b"),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?;
                Some(TokenData::Time(t.clone()))
            }),
        },
    ]
}

// ====================================================================
// Helper functions
// ====================================================================

fn apply_ampm(form: &TimeForm, is_pm: bool) -> Option<TokenData> {
    match form {
        TimeForm::Hour(h, _) => {
            let hour = if is_pm && *h < 12 {
                h + 12
            } else if !is_pm && *h == 12 {
                0
            } else {
                *h
            };
            Some(TokenData::Time(TimeData::new(TimeForm::Hour(hour, false))))
        }
        TimeForm::HourMinute(h, m) => {
            let hour = if is_pm && *h < 12 {
                h + 12
            } else if !is_pm && *h == 12 {
                0
            } else {
                *h
            };
            Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                hour, *m,
            ))))
        }
        TimeForm::HourMinuteSecond(h, m, s) => {
            let hour = if is_pm && *h < 12 {
                h + 12
            } else if !is_pm && *h == 12 {
                0
            } else {
                *h
            };
            Some(TokenData::Time(TimeData::new(TimeForm::HourMinuteSecond(
                hour, *m, *s,
            ))))
        }
        _ => None,
    }
}

fn text_to_grain(text: &str) -> Option<Grain> {
    match text.to_lowercase().as_ref() {
        "second" | "seconds" => Some(Grain::Second),
        "minute" | "minutes" => Some(Grain::Minute),
        "hour" | "hours" => Some(Grain::Hour),
        "day" | "days" => Some(Grain::Day),
        "week" | "weeks" => Some(Grain::Week),
        "month" | "months" => Some(Grain::Month),
        "quarter" | "quarters" | "qtr" | "qtrs" => Some(Grain::Quarter),
        "year" | "years" | "yr" | "yrs" => Some(Grain::Year),
        _ => None,
    }
}

fn modifier_to_offset(text: &str) -> i32 {
    match text.to_lowercase().as_ref() {
        "last" | "past" | "previous" => -1,
        "next" | "following" | "upcoming" | "coming" => 1,
        "this" | "current" => 0,
        _ => 0,
    }
}

fn month_name_to_num(name: &str) -> Option<u32> {
    match name.to_lowercase().as_ref() {
        "jan" | "january" => Some(1),
        "feb" | "february" => Some(2),
        "mar" | "march" => Some(3),
        "apr" | "april" => Some(4),
        "may" => Some(5),
        "jun" | "june" => Some(6),
        "jul" | "july" => Some(7),
        "aug" | "august" => Some(8),
        "sep" | "september" => Some(9),
        "oct" | "october" => Some(10),
        "nov" | "november" => Some(11),
        "dec" | "december" => Some(12),
        _ => None,
    }
}

fn holidays_regex() -> String {
    let holidays = [
        // Christmas
        r"christmas(\s+day)?",
        r"xmas(\s+day)?",
        // New Year
        r"new\s+year'?s?\s+(eve|day)",
        r"new\s+years?\s+(eve|day)",
        // Valentine's
        r"valentine'?s?\s+day",
        r"valentine\s+day",
        // Halloween
        r"halloween",
        // Black Friday
        r"black\s+friday",
        // Thanksgiving
        r"thanksgiving(\s+day)?",
        // Boss's Day
        r"boss'?s?(\s+day)?",
        // MLK Day (from Haskell: (MLK|Martin Luther King('?s)?,?)( Jr\.?| Junior)? day)
        r"(mlk|martin\s+luther\s+king('?s)?,?)(\s+jr\.?|\s+junior)?\s+day",
        r"(civil|idaho\s+human)\s+rights\s+day",
        // St Patrick's Day (from Haskell: (saint|st\.?) (patrick|paddy)'?s day)
        r"(saint|st\.?)\s+(patrick|paddy)'?s\s+day",
        // World Vegan Day
        r"world\s+vegan\s+day",
        // Easter and related
        r"easter(\s+(sunday|mon(day)?))?",
        r"good\s+friday",
        r"palm\s+sunday",
        r"branch\s+sunday",
        r"maundy\s+thursday",
        r"covenant\s+thu(rsday)?",
        r"thu(rsday)?\s+of\s+mysteries",
        r"pentecost",
        r"white\s+sunday",
        r"whit\s+monday",
        r"monday\s+of\s+the\s+holy\s+spirit",
        r"trinity\s+sunday",
        r"pancake\s+day",
        r"mardi\s+gras",
        r"shrove\s+tuesday",
        r"lent",
        r"ash\s+wednesday",
        // Orthodox
        r"orthodox\s+(easter|good\s+friday|great\s+friday|shrove\s+monday)",
        r"clean\s+monday",
        r"lazarus\s+saturday",
        r"great\s+fast",
        // Chinese New Year
        r"chinese(\s+lunar)?\s+new\s+year'?s?(\s+day)?",
        // Jewish holidays
        r"yom\s+kippur",
        r"shemini\s+atzeret",
        r"simchat\s+torah",
        r"tisha\s+b'?av",
        r"yom\s+haatzmaut",
        r"lag\s+b'?omer",
        r"yom\s+hashoah",
        r"holocaust\s+day",
        r"rosh\s+hashann?ah?",
        r"yom\s+teruah",
        r"chanukah",
        r"hanuk(k)?ah",
        r"hannuk(k)?ah",
        r"passover",
        r"(feast\s+of\s+the\s+ingathering|succos|sukkot)",
        r"shavuot",
        r"tu\s+bishvat",
        r"purim",
        r"shushan\s+purim",
        // Islamic holidays
        r"mawlid(\s+al[\-\s]nabawi)?",
        r"eid\s+al[\-\s]fitr",
        r"eid\s+al[\-\s]adha",
        r"id\s+ul[\-\s]adha",
        r"sacrifice\s+feast",
        r"bakr\s+id",
        r"laylat\s+al[\-\s](qadr|kadr)",
        r"night\s+of\s+(power|measures)",
        r"islamic\s+new\s+year",
        r"amun\s+jadid",
        r"(day\s+of\s+)?ashura",
        r"ramadan",
        r"isra\s+and\s+mi'?raj",
        r"the\s+prophet'?s\s+ascension",
        r"the\s+night\s+journey",
        r"ascension\s+to\s+heaven",
        r"jumu'?atul[\-\s]wida",
        r"jamat\s+ul[\-\s]vida",
        // Hindu holidays
        r"dhanteras",
        r"dhanatrayodashi",
        r"diwali",
        r"deepavali",
        r"bhai\s+dooj",
        r"chhath",
        r"dala\s+puja",
        r"navaratri",
        r"durga\s+puja",
        r"karva\s+chauth",
        r"ratha[\-\s]yatra",
        r"rakhi",
        r"raksha\s+bandhan",
        r"mahavir\s+jayanti",
        r"maha\s+shivaratri",
        r"holi",
        r"dhulandi",
        r"phagwah",
        r"chhoti\s+holi",
        r"holika\s+dahan",
        r"kamudu\s+pyre",
        r"krishna\s+janmashtami",
        r"gokulashtami",
        r"ganesh\s+chaturthi",
        r"rama\s+navami",
        r"ugadi",
        r"yugadi",
        r"pongal",
        r"makara?\s+sankranth?i",
        r"makar\s+sankranti",
        r"maghi",
        r"vaisakhi",
        r"baisakhi",
        r"onam",
        r"thiru\s+onam",
        r"vasant\s+panchami",
        r"basant\s+panchami",
        r"naraka\s+chaturdashi",
        r"kali\s+chaudas",
        r"choti\s+diwali",
        r"maha\s+saptami",
        r"dussehra",
        r"vijayadashami",
        r"saraswati\s+jayanti",
        r"bogi\s+pandigai",
        r"maattu\s+pongal",
        r"kaanum\s+pongal",
        r"kanni\s+pongal",
        // Sikh
        r"guru\s+gobind\s+singh\s+(birthday|jayanti)",
        r"guru\s+govind\s+singh\s+jayanti",
        r"guru\s+ravida?s(s)?\s+(jayanti|birthday)",
        r"valmiki\s+jayanti",
        r"maharishi\s+valmiki\s+jayanti",
        r"pargat\s+diwas",
        r"rabindra\s+jayanti",
        // Other
        r"parsi\s+new\s+year",
        r"jamshedi\s+navroz",
        r"gysd",
        r"global\s+youth\s+service\s+day",
        r"vesak",
        r"vaisakha",
        r"buddha\s+(day|purnima)",
        r"earth\s+hour",
        r"koningsdag",
        r"king's\s+day",
        r"lakshmi\s+puja",
    ];
    format!(r"\b({})\b", holidays.join("|"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimensions::{duration, numeral, ordinal, time_grain};
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::types::DimensionKind;

    fn all_rules() -> Vec<Rule> {
        let mut r = numeral::en::rules();
        r.extend(ordinal::en::rules());
        r.extend(time_grain::en::rules());
        r.extend(duration::en::rules());
        r.extend(rules());
        r
    }

    #[test]
    fn test_days_of_week() {
        let rules = all_rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for day in &[
            "monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday",
        ] {
            let entities = engine::parse_and_resolve(
                day,
                &rules,
                &context,
                &options,
                &[DimensionKind::Time],
            );
            let found = entities.iter().any(|e| {
                e.dim == "time"
                    && e.value.value.get("grain").and_then(|v| v.as_str()) == Some("day")
            });
            assert!(found, "Expected time for '{}', got: {:?}", day, entities);
        }
    }

    #[test]
    fn test_today_tomorrow() {
        let rules = all_rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for text in &["today", "tomorrow", "yesterday"] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::Time],
            );
            let found = entities.iter().any(|e| e.dim == "time");
            assert!(found, "Expected time for '{}', got: {:?}", text, entities);
        }
    }

    #[test]
    fn test_clock_time() {
        let rules = all_rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        let entities = engine::parse_and_resolve(
            "3:30",
            &rules,
            &context,
            &options,
            &[DimensionKind::Time],
        );
        let found = entities.iter().any(|e| {
            e.dim == "time"
                && e.value.value.get("grain").and_then(|v| v.as_str()) == Some("minute")
        });
        assert!(found, "Expected time for '3:30', got: {:?}", entities);
    }

    #[test]
    fn test_in_duration() {
        let rules = all_rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        let entities = engine::parse_and_resolve(
            "in 3 days",
            &rules,
            &context,
            &options,
            &[DimensionKind::Time],
        );
        let found = entities.iter().any(|e| e.dim == "time");
        assert!(found, "Expected time for 'in 3 days', got: {:?}", entities);
    }
}
