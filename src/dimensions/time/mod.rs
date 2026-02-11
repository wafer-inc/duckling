pub mod en;
pub mod helpers;

use chrono::{DateTime, Datelike, Timelike, Utc};
use crate::dimensions::time_grain::Grain;
use crate::resolve::Context;
use crate::types::ResolvedValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartOfDay {
    Morning,
    Afternoon,
    Evening,
    Night,
    Lunch,
}

#[derive(Debug, Clone)]
pub struct TimeData {
    pub form: TimeForm,
    pub direction: Option<Direction>,
    pub latent: bool,
}

#[derive(Debug, Clone)]
pub enum TimeForm {
    DayOfWeek(u32),       // 0=Monday .. 6=Sunday
    Month(u32),           // 1..12
    DayOfMonth(u32),      // 1..31
    Hour(u32, bool),      // hour, is_12h_ambiguous
    HourMinute(u32, u32), // hour, minute
    HourMinuteSecond(u32, u32, u32),
    Year(i32),
    Now,
    Today,
    Tomorrow,
    Yesterday,
    RelativeGrain { n: i64, grain: Grain },
    DateMDY { month: u32, day: u32, year: Option<i32> },
    // Part of day
    PartOfDay(PartOfDay),
    // Weekend
    Weekend,
    // Season: 0=spring, 1=summer, 2=fall, 3=winter
    Season(u32),
    // Named holiday (placeholder resolution)
    Holiday(String),
    // this(0)/last(-1)/next(1) week/month/year/quarter/season
    GrainOffset { grain: Grain, offset: i32 },
    // last/next N days/weeks/months
    NthGrain { n: i64, grain: Grain, past: bool },
    // Specific quarter (1-4)
    Quarter(u32),
    // Quarter + year
    QuarterYear(u32, i32),
    // Day after tomorrow / before yesterday
    DayAfterTomorrow,
    DayBeforeYesterday,
    // Beginning/end of a time period
    BeginEnd { begin: bool, target: Box<TimeForm> },
    // Time interval (from, to)
    Interval(Box<TimeData>, Box<TimeData>),
    // All week / rest of week
    AllGrain(Grain),
    RestOfGrain(Grain),
    // Composed: two time expressions intersected (e.g., Monday + morning)
    Composed(Box<TimeData>, Box<TimeData>),
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
            latent: false,
        }
    }

    pub fn latent(form: TimeForm) -> Self {
        TimeData {
            form,
            direction: None,
            latent: true,
        }
    }

    pub fn with_direction(mut self, direction: Direction) -> Self {
        self.direction = Some(direction);
        self
    }
}

pub fn resolve(data: &TimeData, context: &Context, with_latent: bool) -> Option<ResolvedValue> {
    if data.latent && !with_latent {
        return None;
    }

    let ref_time = context.reference_time;
    let grain = grain_for_form(&data.form);
    let resolved = resolve_form(&data.form, ref_time, data.direction);

    Some(ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": resolved.format("%Y-%m-%dT%H:%M:%S%:z").to_string(),
            "grain": grain,
            "type": "value",
        }),
    })
}

fn grain_for_form(form: &TimeForm) -> &'static str {
    match form {
        TimeForm::DayOfWeek(_) => "day",
        TimeForm::Month(_) => "month",
        TimeForm::DayOfMonth(_) => "day",
        TimeForm::Hour(_, _) => "hour",
        TimeForm::HourMinute(_, _) => "minute",
        TimeForm::HourMinuteSecond(_, _, _) => "second",
        TimeForm::Year(_) => "year",
        TimeForm::Now => "second",
        TimeForm::Today => "day",
        TimeForm::Tomorrow => "day",
        TimeForm::Yesterday => "day",
        TimeForm::RelativeGrain { grain, .. } => grain.as_str(),
        TimeForm::DateMDY { .. } => "day",
        TimeForm::PartOfDay(_) => "hour",
        TimeForm::Weekend => "week",
        TimeForm::Season(_) => "month",
        TimeForm::Holiday(_) => "day",
        TimeForm::GrainOffset { grain, .. } => grain.as_str(),
        TimeForm::NthGrain { grain, .. } => grain.as_str(),
        TimeForm::Quarter(_) => "quarter",
        TimeForm::QuarterYear(_, _) => "quarter",
        TimeForm::DayAfterTomorrow => "day",
        TimeForm::DayBeforeYesterday => "day",
        TimeForm::BeginEnd { target, .. } => grain_for_form(target),
        TimeForm::Interval(from, _) => grain_for_form(&from.form),
        TimeForm::AllGrain(g) | TimeForm::RestOfGrain(g) => g.as_str(),
        TimeForm::Composed(primary, _) => grain_for_form(&primary.form),
    }
}

fn resolve_form(
    form: &TimeForm,
    ref_time: DateTime<Utc>,
    direction: Option<Direction>,
) -> DateTime<Utc> {
    use chrono::Duration;

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
            ref_time
                .date_naive()
                .and_hms_opt(*h, 0, 0)
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
        TimeForm::HourMinuteSecond(h, m, s) => {
            ref_time
                .date_naive()
                .and_hms_opt(*h, *m, *s)
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
        TimeForm::PartOfDay(_) => {
            ref_time.date_naive().and_hms_opt(12, 0, 0).unwrap().and_utc()
        }
        TimeForm::Weekend => {
            let dow = ref_time.weekday().num_days_from_monday();
            let days_to_sat = ((5i64 - dow as i64) + 7) % 7;
            let days_to_sat = if days_to_sat == 0 { 7 } else { days_to_sat };
            (ref_time + Duration::days(days_to_sat))
                .date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc()
        }
        TimeForm::Season(_) | TimeForm::Holiday(_) => {
            ref_time.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc()
        }
        TimeForm::GrainOffset { grain, offset } => {
            resolve_form(
                &TimeForm::RelativeGrain { n: *offset as i64, grain: *grain },
                ref_time,
                direction,
            )
        }
        TimeForm::NthGrain { n, grain, past } => {
            let signed_n = if *past { -(*n) } else { *n };
            resolve_form(
                &TimeForm::RelativeGrain { n: signed_n, grain: *grain },
                ref_time,
                direction,
            )
        }
        TimeForm::Quarter(q) => {
            let month = (*q - 1) * 3 + 1;
            chrono::NaiveDate::from_ymd_opt(ref_time.year(), month, 1)
                .unwrap_or(ref_time.date_naive())
                .and_hms_opt(0, 0, 0).unwrap().and_utc()
        }
        TimeForm::QuarterYear(q, y) => {
            let month = (*q - 1) * 3 + 1;
            chrono::NaiveDate::from_ymd_opt(*y, month, 1)
                .unwrap_or(ref_time.date_naive())
                .and_hms_opt(0, 0, 0).unwrap().and_utc()
        }
        TimeForm::DayAfterTomorrow => {
            (ref_time + Duration::days(2))
                .date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc()
        }
        TimeForm::DayBeforeYesterday => {
            (ref_time - Duration::days(2))
                .date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc()
        }
        TimeForm::BeginEnd { target, .. } => {
            resolve_form(target, ref_time, direction)
        }
        TimeForm::Interval(from, _) => {
            resolve_form(&from.form, ref_time, from.direction)
        }
        TimeForm::AllGrain(g) | TimeForm::RestOfGrain(g) => {
            resolve_form(
                &TimeForm::GrainOffset { grain: *g, offset: 0 },
                ref_time,
                direction,
            )
        }
        TimeForm::Composed(primary, _) => {
            resolve_form(&primary.form, ref_time, primary.direction)
        }
    }
}
