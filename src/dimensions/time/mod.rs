pub mod en;
pub mod helpers;

use chrono::{DateTime, Datelike, Timelike, Utc};
use crate::resolve::Context;
use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct TimeData {
    pub form: TimeForm,
    pub direction: Option<Direction>,
}

#[derive(Debug, Clone)]
pub enum TimeForm {
    DayOfWeek(u32),       // 0=Monday .. 6=Sunday
    Month(u32),           // 1..12
    DayOfMonth(u32),      // 1..31
    Hour(u32, bool),      // hour, is_12h_ambiguous
    HourMinute(u32, u32), // hour, minute
    Year(i32),
    Now,
    Today,
    Tomorrow,
    Yesterday,
    RelativeGrain { n: i64, grain: crate::dimensions::time_grain::Grain },
    DateMDY { month: u32, day: u32, year: Option<i32> },
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Past,
    Future,
}

impl TimeData {
    pub fn new(form: TimeForm) -> Self {
        TimeData {
            form,
            direction: None,
        }
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }
}

pub fn resolve(data: &TimeData, context: &Context) -> ResolvedValue {
    let ref_time = context.reference_time;
    let resolved = resolve_form(&data.form, ref_time, data.direction);

    ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": resolved.format("%Y-%m-%dT%H:%M:%S%:z").to_string(),
            "grain": grain_for_form(&data.form),
            "type": "value",
        }),
    }
}

fn grain_for_form(form: &TimeForm) -> &'static str {
    match form {
        TimeForm::DayOfWeek(_) => "day",
        TimeForm::Month(_) => "month",
        TimeForm::DayOfMonth(_) => "day",
        TimeForm::Hour(_, _) => "hour",
        TimeForm::HourMinute(_, _) => "minute",
        TimeForm::Year(_) => "year",
        TimeForm::Now => "second",
        TimeForm::Today => "day",
        TimeForm::Tomorrow => "day",
        TimeForm::Yesterday => "day",
        TimeForm::RelativeGrain { grain, .. } => grain.as_str(),
        TimeForm::DateMDY { .. } => "day",
    }
}

fn resolve_form(
    form: &TimeForm,
    ref_time: DateTime<Utc>,
    direction: Option<Direction>,
) -> DateTime<Utc> {
    use chrono::Duration;
    use crate::dimensions::time_grain::Grain;

    match form {
        TimeForm::Now => ref_time,
        TimeForm::Today => ref_time
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc(),
        TimeForm::Tomorrow => (ref_time + Duration::days(1))
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc(),
        TimeForm::Yesterday => (ref_time - Duration::days(1))
            .date_naive()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc(),
        TimeForm::DayOfWeek(dow) => {
            // Find next occurrence of this day of week
            let current_dow = ref_time.weekday().num_days_from_monday();
            let target = *dow;
            let days_ahead = if target > current_dow {
                target - current_dow
            } else if target < current_dow {
                7 - (current_dow - target)
            } else {
                7 // same day = next week
            };
            let days_ahead = match direction {
                Some(Direction::Past) => {
                    if days_ahead == 7 { 0 } else { (days_ahead as i64) - 7 }
                }
                _ => days_ahead as i64,
            };
            (ref_time + Duration::days(days_ahead))
                .date_naive()
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
        }
        TimeForm::Month(m) => {
            let year = if *m > ref_time.month() {
                ref_time.year()
            } else {
                ref_time.year() + 1
            };
            chrono::NaiveDate::from_ymd_opt(year, *m, 1)
                .unwrap_or(ref_time.date_naive())
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
        }
        TimeForm::DayOfMonth(d) => {
            let date = chrono::NaiveDate::from_ymd_opt(
                ref_time.year(),
                ref_time.month(),
                *d,
            );
            match date {
                Some(d) => d.and_hms_opt(0, 0, 0).unwrap().and_utc(),
                None => ref_time,
            }
        }
        TimeForm::Hour(h, _ambiguous) => {
            let hour = *h;
            ref_time
                .date_naive()
                .and_hms_opt(hour, 0, 0)
                .unwrap_or(ref_time.naive_utc())
                .and_utc()
        }
        TimeForm::HourMinute(h, m) => {
            ref_time
                .date_naive()
                .and_hms_opt(*h, *m, 0)
                .unwrap_or(ref_time.naive_utc())
                .and_utc()
        }
        TimeForm::Year(y) => {
            chrono::NaiveDate::from_ymd_opt(*y, 1, 1)
                .unwrap_or(ref_time.date_naive())
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
        }
        TimeForm::RelativeGrain { n, grain } => {
            match grain {
                Grain::Second => ref_time + Duration::seconds(*n),
                Grain::Minute => ref_time + Duration::minutes(*n),
                Grain::Hour => ref_time + Duration::hours(*n),
                Grain::Day => ref_time + Duration::days(*n),
                Grain::Week => ref_time + Duration::weeks(*n),
                Grain::Month => {
                    let total_months = ref_time.year() * 12 + ref_time.month() as i32 + *n as i32;
                    let year = (total_months - 1) / 12;
                    let month = ((total_months - 1) % 12 + 1) as u32;
                    chrono::NaiveDate::from_ymd_opt(year, month, ref_time.day().min(28))
                        .unwrap_or(ref_time.date_naive())
                        .and_hms_opt(ref_time.hour(), ref_time.minute(), ref_time.second())
                        .unwrap()
                        .and_utc()
                }
                Grain::Year => {
                    let year = ref_time.year() + *n as i32;
                    chrono::NaiveDate::from_ymd_opt(year, ref_time.month(), ref_time.day().min(28))
                        .unwrap_or(ref_time.date_naive())
                        .and_hms_opt(ref_time.hour(), ref_time.minute(), ref_time.second())
                        .unwrap()
                        .and_utc()
                }
                Grain::Quarter => {
                    let months = *n * 3;
                    let total_months = ref_time.year() * 12 + ref_time.month() as i32 + months as i32;
                    let year = (total_months - 1) / 12;
                    let month = ((total_months - 1) % 12 + 1) as u32;
                    chrono::NaiveDate::from_ymd_opt(year, month, ref_time.day().min(28))
                        .unwrap_or(ref_time.date_naive())
                        .and_hms_opt(ref_time.hour(), ref_time.minute(), ref_time.second())
                        .unwrap()
                        .and_utc()
                }
            }
        }
        TimeForm::DateMDY { month, day, year } => {
            let y = year.unwrap_or(ref_time.year());
            chrono::NaiveDate::from_ymd_opt(y, *month, *day)
                .unwrap_or(ref_time.date_naive())
                .and_hms_opt(0, 0, 0)
                .unwrap()
                .and_utc()
        }
    }
}
