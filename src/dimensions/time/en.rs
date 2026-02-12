use crate::dimensions::numeral::helpers::{integer_value, numeral_data};
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{Direction, EarlyLate, IntervalDirection, PartOfDay, TimeData, TimeForm};

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

/// Create a Composed TimeData, propagating timezone from either inner token.
/// This mirrors Haskell's approach where shiftTimezone modifies the predicate
/// and carries through intersections automatically.
fn compose(t1: &TimeData, t2: &TimeData) -> TimeData {
    let mut td = TimeData::new(TimeForm::Composed(
        Box::new(t1.clone()),
        Box::new(t2.clone()),
    ));
    // Propagate timezone from inner tokens (like Haskell's predicate composition)
    td.timezone = t1.timezone.clone().or_else(|| t2.timezone.clone());
    td
}

/// Extract a day-of-month value from numeral or ordinal token data
fn get_dom_value(td: &TokenData) -> Option<u32> {
    match td {
        TokenData::Numeral(n) => {
            let v = n.value as u32;
            if v >= 1 && v <= 31 { Some(v) } else { None }
        }
        TokenData::Ordinal(o) => {
            let v = o.value as u32;
            if v >= 1 && v <= 31 { Some(v) } else { None }
        }
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        // ====================================================================
        // Days of week (with word boundaries)
        // ====================================================================
        Rule {
            name: "day of week".to_string(),
            pattern: vec![regex(
                r"\b(mondays?|tuesdays?|wednesdays?|thursdays?|fridays?|saturdays?|sundays?|mon|tue|wed|thu|fri|sat|sun)\b\.?",
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let text_lower = text.to_lowercase();
                let text_singular = text_lower.trim_end_matches('s');
                let dow = match text_singular {
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
            pattern: vec![regex(r"\b(now|right now|just now)\b")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "at this time".to_string(),
            pattern: vec![regex(r"\bat this time\b")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "ATM".to_string(),
            pattern: vec![regex(r"\b(at the moment|atm)\b")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "ASAP".to_string(),
            pattern: vec![regex(r"\b(asap|as soon as possible)\b")],
            production: Box::new(|_| {
                let mut td = TimeData::new(TimeForm::Now);
                td.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(td))
            }),
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
            pattern: vec![regex(r"\b(early ((in|hours of) the )?morning|morning)\b")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0).unwrap_or("").to_string(),
                    _ => return None,
                };
                let mut td = TimeData::latent(TimeForm::PartOfDay(PartOfDay::Morning));
                if text.contains("early") {
                    td.early_late = Some(EarlyLate::Early);
                }
                Some(TokenData::Time(td))
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
            name: "late/early/mid <time>".to_string(),
            pattern: vec![regex(r"\b(late|early|mid)\b[\s-]?"), predicate(is_time)],
            production: Box::new(|nodes| {
                let keyword = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let t = time_data(&nodes[1].token_data)?;
                let mut result = t.clone();
                result.early_late = Some(match keyword.as_str() {
                    "late" => EarlyLate::Late,
                    "mid" => EarlyLate::Mid,
                    _ => EarlyLate::Early,
                });
                Some(TokenData::Time(result))
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
                Some(TokenData::Time(TimeData::new(TimeForm::Season(99)))) // 99 = generic "season"
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
                        hour, minute, true,
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
                                hour, minute, false,
                            ))))
                        } else {
                            None
                        }
                    }
                    None => Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                        hour, 0, false,
                    )))),
                }
            }),
        },
        // 4-digit HHMM: 1030, 0730 (exclude year-like 19xx, 20xx)
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
                // Exclude year-like patterns (1900-2099)
                let full_num = hour * 100 + minute;
                if full_num >= 1900 && full_num <= 2099 {
                    return None;
                }
                Some(TokenData::Time(TimeData::latent(TimeForm::HourMinute(
                    hour, minute, false,
                ))))
            }),
        },
        // 3-4 digit time + "ish": "150ish" → 1:50 (Haskell: ruleHHMMLatent + approx)
        Rule {
            name: "time HMM-ish".to_string(),
            pattern: vec![regex(r"\b((?:[01]?\d)|(?:2[0-3]))([0-5]\d)\s?(?:ish|approximately|roughly)\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let hour: u32 = m.group(1)?.parse().ok()?;
                let minute: u32 = m.group(2)?.parse().ok()?;
                // Haskell: is12H = (h < 12)
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                    hour, minute, hour < 12,
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
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
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
        // H + a/p only (no "m") — latent, used in compositions (e.g., "9a to 11a")
        Rule {
            name: "H a/p (latent)".to_string(),
            pattern: vec![regex(r"\b(\d{1,2})([ap])\b")],
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
                Some(TokenData::Time(TimeData::latent(TimeForm::Hour(hour, false))))
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
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
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
                Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, false))))
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
                    TimeForm::Hour(_, _) | TimeForm::HourMinute(_, _, _) => {
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
                    TimeForm::HourMinute(h, m, _) => {
                        let hour = if is_pm && *h < 12 {
                            h + 12
                        } else {
                            *h
                        };
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                            hour, *m, false,
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
                    | TimeForm::Holiday(..) | TimeForm::Quarter(_) | TimeForm::QuarterYear(_, _) => {
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
        // in <integer> (implicit minutes, e.g., "in 15" → "in 15 minutes")
        Rule {
            name: "in <integer> (implicit minutes)".to_string(),
            pattern: vec![regex(r"\bin\b"), predicate(is_integer_between(0, 60))],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?.value as i64;
                Some(TokenData::Time(TimeData::new(TimeForm::RelativeGrain {
                    n,
                    grain: Grain::Minute,
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
            name: "<duration> from right now".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex(r"\bfrom right now\b")],
            production: Box::new(|nodes| {
                let dur = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                // "from right now" → Composed(RelativeGrain, Now) for exact ref_time + duration
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::RelativeGrain { n: dur.value, grain: dur.grain })),
                    Box::new(TimeData::new(TimeForm::Now)),
                ))))
            }),
        },
        Rule {
            name: "<duration> from today".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex(r"\bfrom today\b")],
            production: Box::new(|nodes| {
                let dur = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                // "from today" → Composed(Today, RelativeGrain) preserves day grain
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::Today)),
                    Box::new(TimeData::new(TimeForm::RelativeGrain { n: dur.value, grain: dur.grain })),
                ))))
            }),
        },
        Rule {
            name: "<duration> from now".to_string(),
            pattern: vec![dim(DimensionKind::Duration), regex(r"\bfrom now\b")],
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
        // after <duration> → open interval from (now + duration)
        Rule {
            name: "after <duration>".to_string(),
            pattern: vec![regex(r"\bafter\b"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let dur = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                let mut td = TimeData::new(TimeForm::RelativeGrain {
                    n: dur.value,
                    grain: dur.grain,
                });
                td.open_interval_direction = Some(IntervalDirection::After);
                Some(TokenData::Time(td))
            }),
        },
        // within <duration> → interval from now to N grains from now
        Rule {
            name: "within <duration>".to_string(),
            pattern: vec![regex(r"\bwithin\b"), dim(DimensionKind::Duration)],
            production: Box::new(|nodes| {
                let dur = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                let from = TimeData::new(TimeForm::Now);
                let to = TimeData::new(TimeForm::RelativeGrain {
                    n: dur.value,
                    grain: dur.grain,
                });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ))))
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
                    if yr < 100 {
                        // 2-digit year: 00-49 → 2000s, 50-99 → 1900s
                        if yr < 50 { Some(yr + 2000) } else { Some(yr + 1900) }
                    } else {
                        Some(yr)
                    }
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
        // MM/YYYY (e.g., "2/2013", "11/2014")
        Rule {
            name: "date MM/YYYY".to_string(),
            pattern: vec![regex(r"\b(\d{1,2})\s?/\s?(\d{4})\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let month: u32 = m.group(1)?.parse().ok()?;
                let year: i32 = m.group(2)?.parse().ok()?;
                if month >= 1 && month <= 12 {
                    // Compose Month + Year to get "month" grain
                    Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                        Box::new(TimeData::new(TimeForm::Month(month))),
                        Box::new(TimeData::new(TimeForm::Year(year))),
                    ))))
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
                        // YYYY-MM or YYYY/MM (e.g., "2014/10") - compose month with year
                        Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                            Box::new(TimeData::new(TimeForm::Month(month))),
                            Box::new(TimeData::new(TimeForm::Year(year))),
                        ))))
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
                let year = if year < 30 { year + 2000 } else if year < 100 { year + 1900 } else { year };
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
        // Spelled-out year from numeral dimension (e.g., "two thousand eighteen" → 2018)
        // Only matches 1900-2100 to avoid partial numeral compositions like "thousand ten" = 1010
        Rule {
            name: "spelled-out year (numeral)".to_string(),
            pattern: vec![predicate(is_integer_between(1900, 2100))],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let year = num.value as i32;
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
                    TimeForm::Year(y) => {
                        let suffix = match &nodes[1].token_data {
                            TokenData::RegexMatch(m) => m.group(1).unwrap_or("").to_lowercase(),
                            _ => String::new(),
                        };
                        let year = if suffix.contains('b') { -y.abs() } else { *y };
                        let mut result = TimeData::new(TimeForm::Year(year));
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
                let suffix = match &nodes[2].token_data {
                    TokenData::RegexMatch(m) => m.group(1).unwrap_or("").to_lowercase(),
                    _ => String::new(),
                };
                let year = if suffix.contains('b') { -year } else { year };
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
        // <month> <year> composition (allows both latent to compose)
        // "October 2018" → Composed(Month(10), Year(2018))
        Rule {
            name: "<month> <year>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Year(_)))),
            ],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?;
                let t2 = time_data(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(t1.clone()),
                    Box::new(t2.clone()),
                ))))
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
                    TimeForm::DayOfWeek(_) | TimeForm::Month(_) | TimeForm::Holiday(..)
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
                    TimeForm::DayOfWeek(_) | TimeForm::Month(_) | TimeForm::Holiday(..)
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
                    TimeForm::DayOfWeek(_) | TimeForm::Month(_) | TimeForm::Holiday(..)
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
                        new_t.direction = Some(Direction::FarFuture);
                        Some(TokenData::Time(new_t))
                    }
                    _ => None,
                }
            }),
        },
        // ====================================================================
        // Nth DOW of time (e.g., "first Monday of March", "second Tuesday of last month")
        // ====================================================================
        Rule {
            name: "<ordinal> <dow> of <time>".to_string(),
            pattern: vec![
                dim(DimensionKind::Ordinal),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::DayOfWeek(_)))),
                regex(r"\bof|in\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::Ordinal(d) => d.value as i32,
                    _ => return None,
                };
                let dow = match &nodes[1].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::DayOfWeek(d) => *d,
                        _ => return None,
                    },
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthDOWOfTime {
                    n,
                    dow,
                    base: Box::new(base.clone()),
                })))
            }),
        },
        // "last <dow> of <time>" (e.g., "last Friday of October")
        Rule {
            name: "last <dow> of <time>".to_string(),
            pattern: vec![
                regex(r"\blast\b"),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::DayOfWeek(_)))),
                regex(r"\bof|in\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let dow = match &nodes[1].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::DayOfWeek(d) => *d,
                        _ => return None,
                    },
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::LastDOWOfTime {
                    dow,
                    base: Box::new(base.clone()),
                })))
            }),
        },
        // "last <cycle> of <time>" (e.g., "last week of September")
        Rule {
            name: "last <grain> of <time>".to_string(),
            pattern: vec![
                regex(r"\blast\b"),
                dim(DimensionKind::TimeGrain),
                regex(r"\bof|in\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::LastCycleOfTime {
                    grain,
                    base: Box::new(base.clone()),
                })))
            }),
        },
        // "(the) <ordinal> last <grain> of <time>" (e.g., "2nd last week of October 2018")
        // Haskell: cycleNthAfter True grain (-n) $ cycleNthAfter True (timeGrain td) 1 td
        Rule {
            name: "<ordinal> last <grain> of <time>".to_string(),
            pattern: vec![
                regex(r"\b(the\s+)?"),
                dim(DimensionKind::Ordinal),
                regex(r"\blast\b"),
                dim(DimensionKind::TimeGrain),
                regex(r"\bof|in|from\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let n = match &nodes[1].token_data {
                    TokenData::Ordinal(d) => d.value as i32,
                    _ => return None,
                };
                let grain = match &nodes[3].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[5].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthLastCycleOfTime {
                    n,
                    grain,
                    base: Box::new(base.clone()),
                })))
            }),
        },
        // "last weekend of <month>" (e.g., "last weekend of October")
        Rule {
            name: "last weekend of <time>".to_string(),
            pattern: vec![
                regex(r"\blast\s+(week[\s-]?end|wkend)\s+(of|in)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let base = time_data(&nodes[1].token_data)?;
                // Resolve: find the last Saturday in the month, weekend = [Fri 18:00, Mon 00:00)
                Some(TokenData::Time(TimeData::new(TimeForm::LastDOWOfTime {
                    dow: 5, // Saturday
                    base: Box::new(base.clone()),
                })))
            }),
        },
        // "<integer> <dow> from <time>" (e.g., "2 Sundays from now", "3 Fridays from now")
        Rule {
            name: "<integer> <dow> from <time>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(_))),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::DayOfWeek(_)))),
                regex(r"\bfrom\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i32;
                let dow = match &nodes[1].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::DayOfWeek(d) => *d,
                        _ => return None,
                    },
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NDOWsFromTime {
                    n,
                    dow,
                    base: Box::new(base.clone()),
                })))
            }),
        },
        // "<integer> <dow> ago" (e.g., "2 Thursdays ago")
        Rule {
            name: "<integer> <dow> ago".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(_))),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::DayOfWeek(_)))),
                regex(r"\b(ago|back)\b"),
            ],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[0].token_data)?.value as i32;
                let dow = match &nodes[1].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::DayOfWeek(d) => *d,
                        _ => return None,
                    },
                    _ => return None,
                };
                Some(TokenData::Time(TimeData::new(TimeForm::NDOWsFromTime {
                    n: -n,
                    dow,
                    base: Box::new(TimeData::new(TimeForm::Now)),
                })))
            }),
        },
        // "<ordinal> <dow> after <time>" (e.g., "third Tuesday after Christmas 2014")
        Rule {
            name: "<ordinal> <time> after <time>".to_string(),
            pattern: vec![
                dim(DimensionKind::Ordinal),
                dim(DimensionKind::Time),
                regex(r"\bafter\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::Ordinal(d) => d.value as i32,
                    _ => return None,
                };
                let t1 = time_data(&nodes[1].token_data)?;
                let dow = match &t1.form {
                    TimeForm::DayOfWeek(d) => *d,
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NDOWsFromTime {
                    n,
                    dow,
                    base: Box::new(base.clone()),
                })))
            }),
        },
        // "(the) closest <dow> to <time>" (e.g., "the closest Monday to Oct 5th")
        // Haskell: predNthClosest 0 td1 td2
        Rule {
            name: "closest <time> to <time>".to_string(),
            pattern: vec![
                regex(r"\b(the\s+)?closest\b"),
                dim(DimensionKind::Time),
                regex(r"\bto\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let td1 = time_data(&nodes[1].token_data)?;
                let td2 = time_data(&nodes[3].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthClosestToTime {
                    n: 1,
                    target: Box::new(td1.clone()),
                    base: Box::new(td2.clone()),
                })))
            }),
        },
        // "(the) <ordinal> closest <time> to <time>"
        Rule {
            name: "<ordinal> closest <time> to <time>".to_string(),
            pattern: vec![
                regex(r"\b(the\s+)?"),
                dim(DimensionKind::Ordinal),
                regex(r"\bclosest\b"),
                dim(DimensionKind::Time),
                regex(r"\bto\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let n = match &nodes[1].token_data {
                    TokenData::Ordinal(d) => d.value as i32,
                    _ => return None,
                };
                let td1 = time_data(&nodes[3].token_data)?;
                let td2 = time_data(&nodes[5].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthClosestToTime {
                    n,
                    target: Box::new(td1.clone()),
                    base: Box::new(td2.clone()),
                })))
            }),
        },
        // "Nth <grain> of <time>" (e.g., "first week of October 2014")
        Rule {
            name: "<ordinal> <grain> of <time>".to_string(),
            pattern: vec![
                dim(DimensionKind::Ordinal),
                dim(DimensionKind::TimeGrain),
                regex(r"\bof|in\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::Ordinal(d) => d.value as i32,
                    _ => return None,
                };
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime {
                    n,
                    grain,
                    base: Box::new(base.clone()),
                })))
            }),
        },
        // "Nth last day of <month>" (e.g., "last day of October 2015", "5th last day of May")
        Rule {
            name: "last day of <time>".to_string(),
            pattern: vec![
                regex(r"\b(last day|last day) of\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let base = time_data(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthLastDayOfTime {
                    n: 1,
                    base: Box::new(base.clone()),
                })))
            }),
        },
        Rule {
            name: "(the) <ordinal> last day of <time>".to_string(),
            pattern: vec![
                regex(r"\b(the\s+)?"),
                dim(DimensionKind::Ordinal),
                regex(r"\blast day of\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let n = match &nodes[1].token_data {
                    TokenData::Ordinal(d) => d.value as i32,
                    _ => return None,
                };
                let base = time_data(&nodes[3].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthLastDayOfTime {
                    n,
                    base: Box::new(base.clone()),
                })))
            }),
        },
        // "Nth <day> of <month>" (e.g., "third day of October")
        Rule {
            name: "<ordinal> day of <time>".to_string(),
            pattern: vec![
                dim(DimensionKind::Ordinal),
                regex(r"\bday of\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let n = match &nodes[0].token_data {
                    TokenData::Ordinal(d) => d.value as i32,
                    _ => return None,
                };
                let base = time_data(&nodes[2].token_data)?;
                // Resolve: nth day of the period
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime {
                    n,
                    grain: Grain::Day,
                    base: Box::new(base.clone()),
                })))
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
        // "the week of <date>" → week containing that date
        Rule {
            name: "the week of <time>".to_string(),
            pattern: vec![
                regex(r"\b(the )?week of\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let base = time_data(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::NthGrainOfTime {
                    n: 1,
                    grain: Grain::Week,
                    base: Box::new(base.clone()),
                })))
            }),
        },
        // "the month" / "the week" / "the year" → current period (same as "this")
        Rule {
            name: "the <grain>".to_string(),
            pattern: vec![
                regex(r"\bthe\b"),
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
                    interval: true,
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
                    interval: true,
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
                    interval: false,
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
                    interval: false,
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
        // <month> + integer → date (e.g., "Feb 10", "Jul 18")
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
        // <dow>,? <month> <numeral/ordinal> (e.g., "Sunday, Feb 10", "Fri, Jul 18")
        Rule {
            name: "<dow> <month> <day>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::DayOfWeek(_)))),
                regex(r",?\s*"),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
                predicate(|td| {
                    matches!(td, TokenData::Numeral(_) | TokenData::Ordinal(_))
                }),
            ],
            production: Box::new(|nodes| {
                let dow = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::DayOfWeek(d) => *d,
                        _ => return None,
                    },
                    _ => return None,
                };
                let month = match &nodes[2].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Month(m) => *m,
                        _ => return None,
                    },
                    _ => return None,
                };
                let day = get_dom_value(&nodes[3].token_data)?;
                let date = TimeData::new(TimeForm::DateMDY { month, day, year: None });
                let dow_td = TimeData::new(TimeForm::DayOfWeek(dow));
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(dow_td),
                    Box::new(date),
                ))))
            }),
        },
        // <integer> of <month> (e.g., "15 of february")
        Rule {
            name: "<integer> of <month>".to_string(),
            pattern: vec![
                predicate(is_integer_between(1, 31)),
                regex(r"\b(of )\b"),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
            ],
            production: Box::new(|nodes| {
                let day = numeral_data(&nodes[0].token_data)?.value as u32;
                let month = match &nodes[2].token_data {
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
        // <ordinal/numeral> <month> <2-digit-year> (e.g., "14th April 15")
        Rule {
            name: "<ordinal> <month> <short-year>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Ordinal(_) | TokenData::Numeral(_))),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
                predicate(|td| {
                    if let TokenData::Numeral(n) = td {
                        n.value >= 0.0 && n.value < 100.0
                    } else {
                        false
                    }
                }),
            ],
            production: Box::new(|nodes| {
                let day = get_dom_value(&nodes[0].token_data)?;
                let month = match &nodes[1].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Month(m) => *m,
                        _ => return None,
                    },
                    _ => return None,
                };
                let y = match &nodes[2].token_data {
                    TokenData::Numeral(n) => n.value as i32,
                    _ => return None,
                };
                let year = if y < 30 { y + 2000 } else if y < 100 { y + 1900 } else { y };
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month,
                    day,
                    year: Some(year),
                })))
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
        // ordinal (latent day of month) - for "20th of this month" etc.
        Rule {
            name: "<ordinal> (latent day of month)".to_string(),
            pattern: vec![dim(DimensionKind::Ordinal)],
            production: Box::new(|nodes| {
                let day = match &nodes[0].token_data {
                    TokenData::Ordinal(d) => d.value as u32,
                    _ => return None,
                };
                if day >= 1 && day <= 31 {
                    Some(TokenData::Time(TimeData::latent(TimeForm::DayOfMonth(day))))
                } else {
                    None
                }
            }),
        },
        // "<numeral 1-31> of this/next/previous month" (e.g., "20 of this month")
        Rule {
            name: "<numeral> of <grain-offset-month>".to_string(),
            pattern: vec![
                predicate(|td| {
                    if let TokenData::Numeral(n) = td {
                        let v = n.value as u32;
                        v >= 1 && v <= 31 && n.value == (v as f64)
                    } else {
                        false
                    }
                }),
                regex(r"\b(of|day of)( the)?\b"),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::GrainOffset { grain: Grain::Month, .. }))),
            ],
            production: Box::new(|nodes| {
                let day = match &nodes[0].token_data {
                    TokenData::Numeral(n) => n.value as u32,
                    _ => return None,
                };
                let t = time_data(&nodes[2].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::DayOfMonth(day))),
                    Box::new(t.clone()),
                ))))
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
                regex(r"\b(at the )?(beginning|start) of( the| around)?\b"),
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
                regex(r"\b(at the )?(end) of( the| around)?\b"),
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
        // EOM / BOM / EOY / BOY / EOD (with optional "by" prefix)
        Rule {
            name: "EOM/BOM/EOY/BOY/EOD".to_string(),
            pattern: vec![regex(r"\b(by (the )?|(at )?the )?(eom|bom|eoy|boy|eod)\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let prefix = m.group(0)?.to_lowercase();
                let has_by = prefix.starts_with("by");
                let abbr = m.group(4)?.to_lowercase();

                if has_by {
                    // "by EOM/EOD/EOY" → interval from now to end of period
                    let grain = match abbr.as_str() {
                        "eom" | "bom" => Grain::Month,
                        "eoy" | "boy" => Grain::Year,
                        "eod" => Grain::Day,
                        _ => return None,
                    };
                    let end_form = TimeForm::BeginEnd {
                        begin: false,
                        target: Box::new(TimeForm::GrainOffset { grain, offset: 0 }),
                    };
                    Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                        Box::new(TimeData::new(TimeForm::Now)),
                        Box::new(TimeData::new(end_form)),
                        false,
                    ))))
                } else {
                    match abbr.as_str() {
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
        // "by end/eom/eoy" — creates interval [now, time)
        Rule {
            name: "by <time>".to_string(),
            pattern: vec![regex(r"\bby( the)?\b"), dim(DimensionKind::Time)],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[1].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(TimeData::new(TimeForm::Now)),
                    Box::new(t.clone()),
                    false,
                ))))
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
                let keyword = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let t = time_data(&nodes[1].token_data)?;
                // Accept latent Year tokens (context disambiguates: "since 2014")
                if t.latent && !matches!(t.form, TimeForm::Year(_)) {
                    return None;
                }
                let mut result = t.clone();
                result.latent = false;
                match keyword.as_str() {
                    "after" | "since" | "from" | "anytime after" => {
                        result.open_interval_direction = Some(IntervalDirection::After);
                    }
                    "before" | "until" | "through" | "sometimes before" => {
                        result.open_interval_direction = Some(IntervalDirection::Before);
                    }
                    _ => {}
                }
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
                let keyword = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let pod = time_data(&nodes[1].token_data)?;
                let mut result = pod.clone();
                result.latent = false;
                if keyword == "after" {
                    result.open_interval_direction = Some(IntervalDirection::After);
                } else {
                    result.open_interval_direction = Some(IntervalDirection::Before);
                }
                Some(TokenData::Time(result))
            }),
        },
        // "after lunch/work/school" - hardcoded intervals matching Haskell
        Rule {
            name: "after lunch/work/school".to_string(),
            pattern: vec![regex(r"\bafter[\s-]?(lunch|work|school)\b")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let keyword = m.group(1)?.to_lowercase();
                let (start_h, end_h) = match keyword.as_str() {
                    "lunch" => (13, 17),
                    "work" => (17, 21),
                    "school" => (15, 21),
                    _ => return None,
                };
                let from = TimeData::new(TimeForm::Hour(start_h, false));
                let to = TimeData::new(TimeForm::Hour(end_h, false));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    true, // open interval - don't adjust end
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
                    text.to_lowercase(), None,
                ))))
            }),
        },
        // <holiday> + year
        Rule {
            name: "<holiday> <year>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Holiday(..)))),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Year(_)))),
            ],
            production: Box::new(|nodes| {
                let holiday_name = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Holiday(name, _) => name.clone(),
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
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(holiday_name, Some(year)))))
            }),
        },
        // <holiday> in <year>
        Rule {
            name: "<holiday> in <year>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Holiday(..)))),
                regex(r"\bin\b"),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Year(_)))),
            ],
            production: Box::new(|nodes| {
                let holiday_name = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Holiday(name, _) => name.clone(),
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
                Some(TokenData::Time(TimeData::new(TimeForm::Holiday(holiday_name, Some(year)))))
            }),
        },
        // <duration> after/from <time> (e.g., "3 days after christmas", "15 min from 1pm")
        // Haskell: durationAfter — shifts each occurrence of time by duration
        Rule {
            name: "<duration> after <time>".to_string(),
            pattern: vec![
                dim(DimensionKind::Duration),
                regex(r"\b(after|from)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let dur = match &nodes[0].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                let t = time_data(&nodes[2].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::DurationAfter {
                    n: dur.value,
                    grain: dur.grain,
                    base: Box::new(t.clone()),
                })))
            }),
        },
        // ====================================================================
        // <latent-time> + <time> composition (e.g., "8 tonight", "9 tomorrow morning")
        // ====================================================================
        Rule {
            name: "<latent-time> <time> compose".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if d.latent && matches!(d.form, TimeForm::Hour(_, true) | TimeForm::HourMinute(_, _, _)))),
                predicate(is_not_latent_time),
            ],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?;
                let t2 = time_data(&nodes[1].token_data)?;
                Some(TokenData::Time(compose(t1, t2)))
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
                Some(TokenData::Time(compose(t1, t2)))
            }),
        },
        // <time> "of"/"from"/"for"/","/"'s" <time> compose
        Rule {
            name: "intersect by of/from/for/,/'s".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex(r"\b(of|from|for)\b( the)?|'s( the)?|,"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?;
                let t2 = time_data(&nodes[2].token_data)?;
                // At least one must be not latent
                if t1.latent && t2.latent {
                    return None;
                }
                Some(TokenData::Time(compose(t1, t2)))
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
                    false,
                ))))
            }),
        },
        // ====================================================================
        // Date range intervals with month context
        // ====================================================================
        // <dom> - <dom> <month> (e.g., "1-8 september", "19th to 21st Aug")
        Rule {
            name: "dd-dd <month> (interval)".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(_) | TokenData::Ordinal(_))),
                regex(r"\-|to( the)?|th?ru|through|(un)?til(l)?"),
                predicate(|td| matches!(td, TokenData::Numeral(_) | TokenData::Ordinal(_))),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
            ],
            production: Box::new(|nodes| {
                let d1 = get_dom_value(&nodes[0].token_data)?;
                let d2 = get_dom_value(&nodes[2].token_data)?;
                let month = match &nodes[3].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Month(m) => *m,
                        _ => return None,
                    },
                    _ => return None,
                };
                if d1 < 1 || d1 > 31 || d2 < 1 || d2 > 31 { return None; }
                let from = TimeData::new(TimeForm::DateMDY { month, day: d1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: d2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from), Box::new(to), false,
                ))))
            }),
        },
        // <month> <dom> - <dom> (e.g., "July 13 to 15", "August 27th to 29th")
        Rule {
            name: "<month> dd-dd (interval)".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
                predicate(|td| matches!(td, TokenData::Numeral(_) | TokenData::Ordinal(_))),
                regex(r"\-|to( the)?|th?ru|through|(un)?til(l)?"),
                predicate(|td| matches!(td, TokenData::Numeral(_) | TokenData::Ordinal(_))),
            ],
            production: Box::new(|nodes| {
                let month = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Month(m) => *m,
                        _ => return None,
                    },
                    _ => return None,
                };
                let d1 = get_dom_value(&nodes[1].token_data)?;
                let d2 = get_dom_value(&nodes[3].token_data)?;
                if d1 < 1 || d1 > 31 || d2 < 1 || d2 > 31 { return None; }
                let from = TimeData::new(TimeForm::DateMDY { month, day: d1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: d2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from), Box::new(to), false,
                ))))
            }),
        },
        // from (the)? <dom> to (the)? <dom> (of)? <month> (e.g., "from 13 to 15 of July")
        Rule {
            name: "from dd to dd <month> (interval)".to_string(),
            pattern: vec![
                regex(r"\bfrom( the)?\b"),
                predicate(|td| matches!(td, TokenData::Numeral(_) | TokenData::Ordinal(_))),
                regex(r"\-|to( the)?|th?ru|through|(un)?til(l)?"),
                predicate(|td| matches!(td, TokenData::Numeral(_) | TokenData::Ordinal(_))),
                regex(r"\b(of )?\b"),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
            ],
            production: Box::new(|nodes| {
                let d1 = get_dom_value(&nodes[1].token_data)?;
                let d2 = get_dom_value(&nodes[3].token_data)?;
                let month = match &nodes[5].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Month(m) => *m,
                        _ => return None,
                    },
                    _ => return None,
                };
                if d1 < 1 || d1 > 31 || d2 < 1 || d2 > 31 { return None; }
                let from = TimeData::new(TimeForm::DateMDY { month, day: d1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: d2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from), Box::new(to), false,
                ))))
            }),
        },
        // Haskell: ruleIntervalFromMonthDDDD — "from <month> dd-dd"
        // e.g., "from August 27th - 29th", "from July 13-15"
        Rule {
            name: "from <month> dd-dd (interval)".to_string(),
            pattern: vec![
                regex(r"\bfrom\b"),
                predicate(|td| matches!(td, TokenData::Time(d) if matches!(d.form, TimeForm::Month(_)))),
                predicate(|td| matches!(td, TokenData::Numeral(_) | TokenData::Ordinal(_))),
                regex(r"\-|to( the)?|th?ru|through|(un)?til(l)?"),
                predicate(|td| matches!(td, TokenData::Numeral(_) | TokenData::Ordinal(_))),
            ],
            production: Box::new(|nodes| {
                let month = match &nodes[1].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Month(m) => *m,
                        _ => return None,
                    },
                    _ => return None,
                };
                let d1 = get_dom_value(&nodes[2].token_data)?;
                let d2 = get_dom_value(&nodes[4].token_data)?;
                if d1 < 1 || d1 > 31 || d2 < 1 || d2 > 31 { return None; }
                let from = TimeData::new(TimeForm::DateMDY { month, day: d1, year: None });
                let to = TimeData::new(TimeForm::DateMDY { month, day: d2, year: None });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from), Box::new(to), false,
                ))))
            }),
        },
        // Haskell: ruleIntervalTODFrom — "later than/from/between <tod> before/to <tod>"
        Rule {
            name: "later than <tod> before <tod>".to_string(),
            pattern: vec![
                regex(r"\b(later than|from|(in[\s\-])?between)\b"),
                dim(DimensionKind::Time),
                regex(r"((but )?before)|\-|to|th?ru|through|(un)?til(l)?"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[1].token_data)?;
                let t2 = time_data(&nodes[3].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(t1.clone()),
                    Box::new(t2.clone()),
                    false,
                ))))
            }),
        },
        // from <time> to/till/- <time>
        Rule {
            name: "from <time> to <time>".to_string(),
            pattern: vec![
                regex(r"\b(from|between)\b"),
                dim(DimensionKind::Time),
                regex(r"\-|to|till|until|and|thru|through"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[1].token_data)?;
                let t2 = time_data(&nodes[3].token_data)?;
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(t1.clone()),
                    Box::new(t2.clone()),
                    false,
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
                    false,
                ))))
            }),
        },
        // Haskell: ruleIntervalTimeForDuration — "<time> for <duration>"
        // interval Closed td1 (durationAfter dd td1)
        Rule {
            name: "<time> for <duration>".to_string(),
            pattern: vec![
                predicate(is_not_latent_time),
                regex(r"\bfor\b"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?;
                let dur = match &nodes[2].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                let end = TimeData::new(TimeForm::DurationAfter {
                    n: dur.value, grain: dur.grain, base: Box::new(t.clone()),
                });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(t.clone()), Box::new(end), false,
                ))))
            }),
        },
        // Haskell: ruleIntervalForDurationFrom — "for <duration> from <time>"
        Rule {
            name: "for <duration> from <time>".to_string(),
            pattern: vec![
                regex(r"\bfor\b"),
                dim(DimensionKind::Duration),
                regex(r"\b(from|starting|starting from|beginning|after)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let dur = match &nodes[1].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                let t = time_data(&nodes[3].token_data)?;
                let end = TimeData::new(TimeForm::DurationAfter {
                    n: dur.value, grain: dur.grain, base: Box::new(t.clone()),
                });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(t.clone()), Box::new(end), false,
                ))))
            }),
        },
        // Haskell: ruleIntervalFromTimeForDuration — "from <time> for <duration>"
        Rule {
            name: "from <time> for <duration>".to_string(),
            pattern: vec![
                regex(r"\b(from|starting|starting from|beginning|after)\b"),
                dim(DimensionKind::Time),
                regex(r"\bfor\b"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let dur = match &nodes[3].token_data {
                    TokenData::Duration(d) => d,
                    _ => return None,
                };
                let t = time_data(&nodes[1].token_data)?;
                let end = TimeData::new(TimeForm::DurationAfter {
                    n: dur.value, grain: dur.grain, base: Box::new(t.clone()),
                });
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(t.clone()),
                    Box::new(end),
                    false,
                ))))
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
                    TimeForm::Hour(h, is_12h) => {
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(*h, 30, *is_12h))))
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
                    TimeForm::Hour(h, is_12h) => {
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(*h, 15, *is_12h))))
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
                    TimeForm::Hour(h, is_12h) => {
                        let hour = if *h == 0 { 23 } else { h - 1 };
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, 45, *is_12h))))
                    }
                    _ => None,
                }
            }),
        },
        // <integer> (minutes)? past/after <time> (e.g., "15 past 3pm", "20 minutes past 3pm")
        Rule {
            name: "<integer> past <time>".to_string(),
            pattern: vec![
                predicate(is_integer_between(1, 59)),
                regex(r"\b(minutes? )?(past|after|from)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let mins = numeral_data(&nodes[0].token_data)?.value as u32;
                let t = time_data(&nodes[2].token_data)?;
                match &t.form {
                    TimeForm::Hour(h, is_12h) => {
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(*h, mins, *is_12h))))
                    }
                    TimeForm::HourMinute(h, 0, is_12h) => {
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(*h, mins, *is_12h))))
                    }
                    _ => None,
                }
            }),
        },
        // <integer> (minutes)? to/before <time>
        Rule {
            name: "<integer> to <time>".to_string(),
            pattern: vec![
                predicate(is_integer_between(1, 59)),
                regex(r"\b(minutes? )?(to|before|of|til)\b"),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let mins = numeral_data(&nodes[0].token_data)?.value as u32;
                let t = time_data(&nodes[2].token_data)?;
                match &t.form {
                    TimeForm::Hour(h, is_12h) | TimeForm::HourMinute(h, 0, is_12h) => {
                        let hour = if *h == 0 { 23 } else { h - 1 };
                        let minute = 60 - mins;
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, minute, *is_12h))))
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
                let (h, is_12h, is_latent) = match &nodes[0].token_data {
                    TokenData::Time(d) => match &d.form {
                        TimeForm::Hour(h, amb) => (*h, *amb, d.latent),
                        _ => return None,
                    },
                    _ => return None,
                };
                let m = numeral_data(&nodes[1].token_data)?.value as u32;
                let td = if is_latent {
                    TimeData::latent(TimeForm::HourMinute(h, m, is_12h))
                } else {
                    TimeData::new(TimeForm::HourMinute(h, m, is_12h))
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
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let y1: i32 = m.group(1)?.parse().ok()?;
                let mo1: u32 = m.group(2)?.parse().ok()?;
                let d1: u32 = m.group(3)?.parse().ok()?;
                let h1: u32 = m.group(4)?.parse().ok()?;
                let mi1: u32 = m.group(5)?.parse().ok()?;
                let s1: u32 = m.group(6)?.parse().ok()?;
                let y2: i32 = m.group(7)?.parse().ok()?;
                let mo2: u32 = m.group(8)?.parse().ok()?;
                let d2: u32 = m.group(9)?.parse().ok()?;
                let h2: u32 = m.group(10)?.parse().ok()?;
                let mi2: u32 = m.group(11)?.parse().ok()?;
                let s2: u32 = m.group(12)?.parse().ok()?;
                let from = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::HourMinuteSecond(h1, mi1, s1))),
                    Box::new(TimeData::new(TimeForm::DateMDY { month: mo1, day: d1, year: Some(y1) })),
                ));
                let to = TimeData::new(TimeForm::Composed(
                    Box::new(TimeData::new(TimeForm::HourMinuteSecond(h2, mi2, s2))),
                    Box::new(TimeData::new(TimeForm::DateMDY { month: mo2, day: d2, year: Some(y2) })),
                ));
                Some(TokenData::Time(TimeData::new(TimeForm::Interval(
                    Box::new(from), Box::new(to), false,
                ))))
            }),
        },
        // ====================================================================
        // (N DOWs from now is handled by the "<integer> <dow> from <time>" rule above)
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
        // Timezone: "<time> CET/GMT/EST/etc." — applies timezone offset
        // Haskell: inTimezone → shiftTimezone: result += (contextOffset - providedOffset) minutes
        // ====================================================================
        Rule {
            name: "<time> <timezone>".to_string(),
            pattern: vec![
                predicate(is_not_latent_time),
                regex(r"(?i)\b(cet|cest|gmt|utc|est|edt|cst|cdt|mst|mdt|pst|pdt|eet|eest|wet|west|bst|ist|jst|kst|hkt|sgt|aest|aedt|acst|acdt|awst|nzst|nzdt)\b"),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?;
                let tz_name = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_uppercase(),
                    _ => return None,
                };
                let mut new_t = t.clone();
                new_t.timezone = Some(tz_name);
                new_t.latent = false;
                Some(TokenData::Time(new_t))
            }),
        },
        // Haskell: ruleTimezoneBracket — "<time> (CET)" with parentheses
        Rule {
            name: "<time> (timezone)".to_string(),
            pattern: vec![
                predicate(is_not_latent_time),
                regex(r"\((cet|cest|gmt|utc|est|edt|cst|cdt|mst|mdt|pst|pdt|eet|eest|wet|west|bst|ist|jst|kst|hkt|sgt|aest|aedt|acst|acdt|awst|nzst|nzdt)\)"),
            ],
            production: Box::new(|nodes| {
                let t = time_data(&nodes[0].token_data)?;
                let tz_name = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_uppercase(),
                    _ => return None,
                };
                let mut new_t = t.clone();
                new_t.timezone = Some(tz_name);
                new_t.latent = false;
                Some(TokenData::Time(new_t))
            }),
        },
        // Haskell: ruleIntervalDashTimezone — "9:30 - 11:00 CST"
        // Applies timezone to both endpoints of a time interval
        Rule {
            name: "<time> - <time> timezone".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex(r"\s*[\-\u{2013}]\s*|to|th?ru|through|(un)?til(l)?"),
                dim(DimensionKind::Time),
                regex(r"\b(cet|cest|gmt|utc|est|edt|cst|cdt|mst|mdt|pst|pdt|eet|eest|wet|west|bst|ist|jst|kst|hkt|sgt|aest|aedt|acst|acdt|awst|nzst|nzdt)\b"),
            ],
            production: Box::new(|nodes| {
                let t1 = time_data(&nodes[0].token_data)?;
                let t2 = time_data(&nodes[2].token_data)?;
                let tz_name = match &nodes[3].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_uppercase(),
                    _ => return None,
                };
                let mut from = t1.clone();
                from.timezone = Some(tz_name.clone());
                let mut to = t2.clone();
                to.timezone = Some(tz_name.clone());
                let mut iv = TimeData::new(TimeForm::Interval(
                    Box::new(from),
                    Box::new(to),
                    false,
                ));
                iv.timezone = Some(tz_name);
                Some(TokenData::Time(iv))
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
        TimeForm::HourMinute(h, m, _) => {
            let hour = if is_pm && *h < 12 {
                h + 12
            } else if !is_pm && *h == 12 {
                0
            } else {
                *h
            };
            Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                hour, *m, false,
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
        r"orthodox\s+(easter|good\s+friday|great\s+friday)",
        r"(orthodox\s+)?(ash|clean|green|pure|shrove)\s+monday",
        r"monday\s+of\s+lent",
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
        r"chhathi?",
        r"chhath\s+(parv|puja)",
        r"dala\s+(chhath|puja)",
        r"surya\s+shashthi",
        r"navaratri",
        r"durga\s+puja",
        r"karva\s+chauth",
        r"ratha[\-\s]yatra",
        r"rakhi",
        r"raksha\s+bandhan",
        r"(mahavir|mahaveer)\s+(jayanti|janma\s+kalyanak)",
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
        r"y?ugadi",
        r"samvatsaradi",
        r"chaitra\s+sukh?ladi",
        r"pongal",
        r"makara?\s+sankranth?i",
        r"makar\s+sankranti",
        r"maghi",
        r"[bv]aisakhi",
        r"vaisakhadi",
        r"vasakhi",
        r"vaishakhi",
        r"mesadi",
        r"(thiru(v|\s+))?onam",
        r"vasant\s+panchami",
        r"basant\s+panchami",
        r"naraka?\s+(nivaran\s+)?chaturdashi",
        r"(kali|roop)\s+chaudas",
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
        r"rabindra(nath)?\s+jayanti",
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
