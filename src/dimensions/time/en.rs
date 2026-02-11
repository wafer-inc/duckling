use crate::pattern::{dim, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::{Direction, TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    vec![
        // Days of week
        Rule {
            name: "day of week".to_string(),
            pattern: vec![regex(
                r#"(monday|tuesday|wednesday|thursday|friday|saturday|sunday|mon\.?|tue\.?|wed\.?|thu\.?|fri\.?|sat\.?|sun\.?)"#,
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let dow = match text.to_lowercase().trim_end_matches('.').as_ref() {
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
        // Months
        Rule {
            name: "month name".to_string(),
            pattern: vec![regex(
                r#"(january|february|march|april|may|june|july|august|september|october|november|december|jan\.?|feb\.?|mar\.?|apr\.?|jun\.?|jul\.?|aug\.?|sep\.?|oct\.?|nov\.?|dec\.?)"#,
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let month = match text.to_lowercase().trim_end_matches('.').as_ref() {
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
        // now
        Rule {
            name: "now".to_string(),
            pattern: vec![regex(r#"(now|right now|just now)"#)],
            production: Box::new(|_nodes| {
                Some(TokenData::Time(TimeData::new(TimeForm::Now)))
            }),
        },
        // today
        Rule {
            name: "today".to_string(),
            pattern: vec![regex(r#"today"#)],
            production: Box::new(|_nodes| {
                Some(TokenData::Time(TimeData::new(TimeForm::Today)))
            }),
        },
        // tomorrow
        Rule {
            name: "tomorrow".to_string(),
            pattern: vec![regex(r#"tomorrow"#)],
            production: Box::new(|_nodes| {
                Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))
            }),
        },
        // yesterday
        Rule {
            name: "yesterday".to_string(),
            pattern: vec![regex(r#"yesterday"#)],
            production: Box::new(|_nodes| {
                Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))
            }),
        },
        // last <day-of-week>
        Rule {
            name: "last <day-of-week>".to_string(),
            pattern: vec![
                regex(r#"last"#),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let time = match &nodes[1].token_data {
                    TokenData::Time(d) => d,
                    _ => return None,
                };
                match &time.form {
                    TimeForm::DayOfWeek(_) | TimeForm::Month(_) => {
                        let mut new_time = time.clone();
                        new_time.direction = Some(Direction::Past);
                        Some(TokenData::Time(new_time))
                    }
                    _ => None,
                }
            }),
        },
        // next <day-of-week>
        Rule {
            name: "next <day-of-week>".to_string(),
            pattern: vec![
                regex(r#"next"#),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                let time = match &nodes[1].token_data {
                    TokenData::Time(d) => d,
                    _ => return None,
                };
                match &time.form {
                    TimeForm::DayOfWeek(_) | TimeForm::Month(_) => {
                        let mut new_time = time.clone();
                        new_time.direction = Some(Direction::Future);
                        Some(TokenData::Time(new_time))
                    }
                    _ => None,
                }
            }),
        },
        // Clock time: HH:MM
        Rule {
            name: "time HH:MM".to_string(),
            pattern: vec![regex(r#"(\d{1,2}):(\d{2})"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let caps = regex::Regex::new(r"(\d{1,2}):(\d{2})")
                    .ok()?
                    .captures(text)?;
                let hour: u32 = caps.get(1)?.as_str().parse().ok()?;
                let minute: u32 = caps.get(2)?.as_str().parse().ok()?;
                if hour < 24 && minute < 60 {
                    Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(
                        hour, minute,
                    ))))
                } else {
                    None
                }
            }),
        },
        // <time> am/pm
        Rule {
            name: "<time> am/pm".to_string(),
            pattern: vec![
                dim(DimensionKind::Time),
                regex(r#"(a\.?m\.?|p\.?m\.?)"#),
            ],
            production: Box::new(|nodes| {
                let time = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let is_pm = time.to_lowercase().starts_with('p');
                let time_data = match &nodes[0].token_data {
                    TokenData::Time(d) => d,
                    _ => return None,
                };
                match &time_data.form {
                    TimeForm::Hour(h, _) => {
                        let hour = if is_pm && *h < 12 { h + 12 } else if !is_pm && *h == 12 { 0 } else { *h };
                        Some(TokenData::Time(TimeData::new(TimeForm::Hour(hour, false))))
                    }
                    TimeForm::HourMinute(h, m) => {
                        let hour = if is_pm && *h < 12 { h + 12 } else if !is_pm && *h == 12 { 0 } else { *h };
                        Some(TokenData::Time(TimeData::new(TimeForm::HourMinute(hour, *m))))
                    }
                    _ => None,
                }
            }),
        },
        // at <time>
        Rule {
            name: "at <time>".to_string(),
            pattern: vec![
                regex(r#"at"#),
                dim(DimensionKind::Time),
            ],
            production: Box::new(|nodes| {
                match &nodes[1].token_data {
                    TokenData::Time(d) => Some(TokenData::Time(d.clone())),
                    _ => None,
                }
            }),
        },
        // <integer> o'clock
        Rule {
            name: "<integer> o'clock".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex(r#"o'?\s?clock"#),
            ],
            production: Box::new(|nodes| {
                let num = match &nodes[0].token_data {
                    TokenData::Numeral(d) => d.value,
                    _ => return None,
                };
                if num >= 1.0 && num <= 12.0 && num == num.floor() {
                    Some(TokenData::Time(TimeData::new(TimeForm::Hour(
                        num as u32,
                        true,
                    ))))
                } else {
                    None
                }
            }),
        },
        // in <duration> (future)
        Rule {
            name: "in <duration>".to_string(),
            pattern: vec![
                regex(r#"in"#),
                dim(DimensionKind::Duration),
            ],
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
        // <duration> ago
        Rule {
            name: "<duration> ago".to_string(),
            pattern: vec![
                dim(DimensionKind::Duration),
                regex(r#"ago"#),
            ],
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
        // MM/DD/YYYY or MM/DD
        Rule {
            name: "date MM/DD(/YYYY)".to_string(),
            pattern: vec![regex(r#"(\d{1,2})/(\d{1,2})(?:/(\d{2,4}))?"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?,
                    _ => return None,
                };
                let caps = regex::Regex::new(r"(\d{1,2})/(\d{1,2})(?:/(\d{2,4}))?")
                    .ok()?
                    .captures(text)?;
                let month: u32 = caps.get(1)?.as_str().parse().ok()?;
                let day: u32 = caps.get(2)?.as_str().parse().ok()?;
                let year = caps.get(3).and_then(|m| {
                    let y: i32 = m.as_str().parse().ok()?;
                    if y < 100 { Some(y + 2000) } else { Some(y) }
                });
                if month >= 1 && month <= 12 && day >= 1 && day <= 31 {
                    Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                        month,
                        day,
                        year,
                    })))
                } else {
                    None
                }
            }),
        },
        // Year: 4-digit year
        Rule {
            name: "year (4 digits)".to_string(),
            pattern: vec![regex(r#"(19\d{2}|20\d{2})"#)],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let year: i32 = text.parse().ok()?;
                Some(TokenData::Time(TimeData::new(TimeForm::Year(year))))
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimensions::{duration, numeral, time_grain};
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::types::DimensionKind;
    fn all_rules() -> Vec<Rule> {
        let mut r = numeral::en::rules();
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
            "monday",
            "tuesday",
            "wednesday",
            "thursday",
            "friday",
            "saturday",
            "sunday",
        ] {
            let entities = engine::parse_and_resolve(
                day,
                &rules,
                &context,
                &options,
                &[DimensionKind::Time],
            );
            let found = entities.iter().any(|e| {
                e.dim == "time" && e.value.value.get("grain").and_then(|v| v.as_str()) == Some("day")
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
