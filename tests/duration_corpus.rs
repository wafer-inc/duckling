// Ported from Duckling/Duration/EN/Corpus.hs
use duckling::{parse_en, DimensionKind, DimensionValue, Grain};

fn check_duration(text: &str, expected_val: i64, expected_unit: &str) {
    let expected_grain = Grain::from_str(expected_unit);
    let entities = parse_en(text, &[DimensionKind::Duration]);
    let found = entities.iter().any(|e| {
        matches!(&e.value, DimensionValue::Duration { value, grain, .. }
            if *value == expected_val && *grain == expected_grain)
    });
    assert!(
        found,
        "Expected duration {} {} for '{}', got: {:?}",
        expected_val,
        expected_unit,
        text,
        entities
            .iter()
            .map(|e| format!("{:?}={:?}", e.value.dim_kind(), e.value))
            .collect::<Vec<_>>()
    );
}

fn check_no_duration(text: &str) {
    let entities = parse_en(text, &[DimensionKind::Duration]);
    let found = entities
        .iter()
        .any(|e| matches!(&e.value, DimensionValue::Duration { .. }));
    assert!(
        !found,
        "Expected NO duration for '{}', but got: {:?}",
        text,
        entities
            .iter()
            .map(|e| format!("{:?}={:?}", e.value.dim_kind(), e.value))
            .collect::<Vec<_>>()
    );
}

// DurationData 1 Second
#[test]
fn test_duration_1_second() {
    check_duration("one sec", 1, "second");
    check_duration("1 second", 1, "second");
    check_duration("1\"", 1, "second");
}

// DurationData 2 Minute
#[test]
fn test_duration_2_minutes() {
    check_duration("2 mins", 2, "minute");
    check_duration("two minutes", 2, "minute");
    check_duration("2'", 2, "minute");
    check_duration("2 more minutes", 2, "minute");
    check_duration("two additional minutes", 2, "minute");
    check_duration("2 extra minutes", 2, "minute");
    check_duration("2 less minutes", 2, "minute");
    check_duration("2 fewer minutes", 2, "minute");
    check_duration("2m", 2, "minute");
    check_duration("2 m", 2, "minute");
}

// DurationData 30 Day
#[test]
fn test_duration_30_days() {
    check_duration("30 days", 30, "day");
}

// DurationData 7 Week
#[test]
fn test_duration_7_weeks() {
    check_duration("seven weeks", 7, "week");
}

// DurationData 1 Month
#[test]
fn test_duration_1_month() {
    check_duration("1 month", 1, "month");
    check_duration("a month", 1, "month");
}

// DurationData 3 Quarter
#[test]
fn test_duration_3_quarters() {
    check_duration("3 quarters", 3, "quarter");
}

// DurationData 2 Year
#[test]
fn test_duration_2_years() {
    check_duration("2 years", 2, "year");
}

// DurationData 30 Minute (half an hour)
#[test]
fn test_duration_half_hour() {
    check_duration("half an hour", 30, "minute");
    check_duration("half hour", 30, "minute");
    check_duration("1/2 hour", 30, "minute");
    check_duration("1/2h", 30, "minute");
    check_duration("1/2 h", 30, "minute");
}

// DurationData 12 Hour (half a day)
#[test]
fn test_duration_half_day() {
    check_duration("half a day", 12, "hour");
    check_duration("half day", 12, "hour");
    check_duration("1/2 day", 12, "hour");
}

// DurationData 90 Minute (an hour and a half)
#[test]
fn test_duration_90_minutes() {
    check_duration("an hour and a half", 90, "minute");
    check_duration("one hour and half", 90, "minute");
    check_duration("1 hour thirty", 90, "minute");
    check_duration("1 hour and thirty", 90, "minute");
    check_duration("1.5 hours", 90, "minute");
    check_duration("1.5 hrs", 90, "minute");
    check_duration("one and two quarter hour", 90, "minute");
    check_duration("one and two quarters hour", 90, "minute");
    check_duration("one and two quarter of hour", 90, "minute");
    check_duration("one and two quarters of hour", 90, "minute");
}

// DurationData 75 Minute
#[test]
fn test_duration_75_minutes() {
    check_duration("1 hour fifteen", 75, "minute");
    check_duration("1 hour and fifteen", 75, "minute");
    check_duration("one and quarter hour", 75, "minute");
    check_duration("one and a quarter hour", 75, "minute");
    check_duration("one and one quarter hour", 75, "minute");
    check_duration("one and quarter of hour", 75, "minute");
    check_duration("one and a quarter of hour", 75, "minute");
    check_duration("one and one quarter of hour", 75, "minute");
}

// DurationData 130 Minute
#[test]
fn test_duration_130_minutes() {
    check_duration("2 hours ten", 130, "minute");
    check_duration("2 hour and 10", 130, "minute");
}

// DurationData 3615 Second
#[test]
fn test_duration_3615_seconds() {
    check_duration("1 hour fifteen seconds", 3615, "second");
    check_duration("1 hour and fifteen seconds", 3615, "second");
}

// DurationData 45 Day (a month and a half)
#[test]
fn test_duration_45_days() {
    check_duration("a month and a half", 45, "day");
    check_duration("one month and half", 45, "day");
}

// DurationData 27 Month
#[test]
fn test_duration_27_months() {
    check_duration("2 years and 3 months", 27, "month");
    check_duration("2 years, 3 months", 27, "month");
    check_duration("2 years 3 months", 27, "month");
}

// DurationData 31719604 Second
#[test]
fn test_duration_complex_seconds() {
    check_duration("1 year, 2 days, 3 hours and 4 seconds", 31719604, "second");
    check_duration("1 year 2 days 3 hours and 4 seconds", 31719604, "second");
}

// DurationData 330 Second (5 and a half minutes)
#[test]
fn test_duration_330_seconds() {
    check_duration("5 and a half minutes", 330, "second");
    check_duration("five and half min", 330, "second");
    check_duration("5 and an half minute", 330, "second");
}

// DurationData 105 Minute (one and three quarter hour)
#[test]
fn test_duration_105_minutes() {
    check_duration("one and three quarter hour", 105, "minute");
    check_duration("one and three quarters hour", 105, "minute");
    check_duration("one and three quarter of hour", 105, "minute");
    check_duration("one and three quarters of hour", 105, "minute");
    check_duration("one and three quarter of hours", 105, "minute");
    check_duration("one and three quarters of hours", 105, "minute");
}

// DurationData 135 Minute
#[test]
fn test_duration_135_minutes() {
    check_duration("two and quarter hour", 135, "minute");
    check_duration("two and a quarter of hour", 135, "minute");
    check_duration("two and quarter of hours", 135, "minute");
    check_duration("two and a quarter of hours", 135, "minute");
}

// DurationData 105 Minute (an hour and 45 minutes)
#[test]
fn test_duration_hour_45_minutes() {
    check_duration("an hour and 45 minutes", 105, "minute");
    check_duration("one hour and 45 minutes", 105, "minute");
}

// DurationData 90 Second (a minute and 30 seconds)
#[test]
fn test_duration_90_seconds() {
    check_duration("a minute and 30 seconds", 90, "second");
    check_duration("one minute and 30 seconds", 90, "second");
}

// DurationData 3630 Second (an hour and 30 seconds)
#[test]
fn test_duration_3630_seconds() {
    check_duration("an hour and 30 seconds", 3630, "second");
}

// DurationData 930 Second (15.5 minutes)
#[test]
fn test_duration_930_seconds() {
    check_duration("15.5 minutes", 930, "second");
    check_duration("15.5 minute", 930, "second");
    check_duration("15.5 mins", 930, "second");
    check_duration("15.5 min", 930, "second");
}

// Negative corpus
#[test]
fn test_duration_negative_for_months() {
    check_no_duration("for months");
}

#[test]
fn test_duration_negative_in_days() {
    check_no_duration("in days");
}

#[test]
fn test_duration_negative_secretary() {
    check_no_duration("secretary");
}

#[test]
fn test_duration_negative_minutes_alone() {
    check_no_duration("minutes");
}

#[test]
fn test_duration_negative_i_second_that() {
    check_no_duration("I second that");
}
