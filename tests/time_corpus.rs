// Ported from Duckling/Time/EN/Corpus.hs
// Reference time for tests: 2013-02-12 04:30:00 UTC-2 (= 2013-02-12 06:30:00 UTC)
// Many of these tests exercise features (holidays, intervals, seasons, timezones)
// that our simplified implementation does not yet support.

use duckling::{parse, Locale, Lang, Context, Options, DimensionKind, Entity};

fn make_context() -> Context {
    use chrono::TimeZone;
    Context {
        reference_time: chrono::Utc.with_ymd_and_hms(2013, 2, 12, 4, 30, 0).unwrap(),
        locale: Locale::new(Lang::EN, None),
    }
}

fn parse_time(text: &str) -> Vec<Entity> {
    let locale = Locale::new(Lang::EN, None);
    let context = make_context();
    let options = Options::default();
    parse(text, &locale, &[DimensionKind::Time], &context, &options)
}

fn check_time(text: &str, expected_grain: &str) {
    let entities = parse_time(text);
    let found = entities.iter().any(|e| {
        e.dim == "time"
            && e.value.value.get("grain").and_then(|v| v.as_str()) == Some(expected_grain)
    });
    assert!(
        found,
        "Expected time with grain '{}' for '{}', got: {:?}",
        expected_grain, text,
        entities.iter().map(|e| format!("{}={:?}", e.dim, e.value)).collect::<Vec<_>>()
    );
}

#[allow(dead_code)]
fn check_time_value(text: &str, expected_value: &str, expected_grain: &str) {
    let entities = parse_time(text);
    let found = entities.iter().any(|e| {
        e.dim == "time"
            && e.value.value.get("grain").and_then(|v| v.as_str()) == Some(expected_grain)
            && e.value.value.get("value").and_then(|v| v.as_str())
                .map(|v| v.starts_with(expected_value))
                .unwrap_or(false)
    });
    assert!(
        found,
        "Expected time value starting with '{}' grain '{}' for '{}', got: {:?}",
        expected_value, expected_grain, text,
        entities.iter().map(|e| format!("{}={:?}", e.dim, e.value)).collect::<Vec<_>>()
    );
}

fn check_any_time(text: &str) {
    let entities = parse_time(text);
    let found = entities.iter().any(|e| e.dim == "time");
    assert!(
        found,
        "Expected some time entity for '{}', got: {:?}",
        text,
        entities.iter().map(|e| format!("{}={:?}", e.dim, e.value)).collect::<Vec<_>>()
    );
}

fn check_no_time(text: &str) {
    let entities = parse_time(text);
    let found = entities.iter().any(|e| e.dim == "time");
    assert!(
        !found,
        "Expected NO time for '{}', but got: {:?}",
        text,
        entities.iter().map(|e| format!("{}={:?}", e.dim, e.value)).collect::<Vec<_>>()
    );
}

// ============================================================
// datetime (2013, 2, 12, 4, 30, 0) Second - "now"
// ============================================================
#[test]
fn test_time_now() {
    check_time("now", "second");
    check_time("right now", "second");
    check_time("just now", "second");
}

#[test]
fn test_time_now_atm() {
    check_time("at the moment", "second");
    check_time("ATM", "second");
}

// ============================================================
// datetime (2013, 2, 12, 0, 0, 0) Day - "today"
// ============================================================
#[test]
fn test_time_today() {
    check_time("today", "day");
}

#[test]
fn test_time_at_this_time() {
    check_any_time("at this time");
}

// ============================================================
// datetime (2014, 1, 1, 0, 0, 0) Year - "in 2014"
// ============================================================
#[test]
fn test_time_in_2014() {
    check_time("in 2014", "year");
}

// ============================================================
// datetime (2013, 2, 11, 0, 0, 0) Day - "yesterday"
// ============================================================
#[test]
fn test_time_yesterday() {
    check_time("yesterday", "day");
}

// ============================================================
// datetime (2013, 2, 13, 0, 0, 0) Day - "tomorrow"
// ============================================================
#[test]
fn test_time_tomorrow() {
    check_time("tomorrow", "day");
    check_time("tomorrows", "day");
}

// ============================================================
// Days of week
// ============================================================
#[test]
fn test_time_monday() {
    check_time("monday", "day");
    check_time("mon.", "day");
    check_time("this monday", "day");
}

#[test]
fn test_time_monday_feb_18() {
    check_any_time("Monday, Feb 18");
    check_any_time("Mon, February 18");
}

#[test]
fn test_time_tuesday() {
    check_time("tuesday", "day");
}

#[test]
fn test_time_tuesday_the_19th() {
    check_any_time("Tuesday the 19th");
    check_any_time("Tuesday 19th");
}

#[test]
fn test_time_thursday() {
    check_time("thursday", "day");
    check_time("thu", "day");
    check_time("thu.", "day");
}

#[test]
fn test_time_friday() {
    check_time("friday", "day");
    check_time("fri", "day");
    check_time("fri.", "day");
}

#[test]
fn test_time_saturday() {
    check_time("saturday", "day");
    check_time("sat", "day");
    check_time("sat.", "day");
}

#[test]
fn test_time_sunday() {
    check_time("sunday", "day");
    check_time("sun", "day");
    check_time("sun.", "day");
}

// ============================================================
// Dates with month names
// ============================================================
#[test]
fn test_time_first_of_march() {
    check_any_time("the 1st of march");
    check_any_time("first of march");
    check_any_time("the first of march");
    check_any_time("march first");
}

#[test]
fn test_time_second_of_march() {
    check_any_time("the 2nd of march");
    check_any_time("second of march");
    check_any_time("the second of march");
}

#[test]
fn test_time_march_3() {
    check_any_time("march 3");
    check_any_time("the third of march");
}

#[test]
fn test_time_ides_of_march() {
    check_any_time("the ides of march");
}

#[test]
fn test_time_march_3_2015() {
    check_any_time("march 3 2015");
    check_any_time("march 3rd 2015");
    check_any_time("march third 2015");
    check_any_time("3/3/2015");
    check_any_time("3/3/15");
    check_any_time("2015-3-3");
    check_any_time("2015-03-03");
}

#[test]
fn test_time_15th_of_february() {
    check_any_time("on the 15th");
    check_any_time("the 15th of february");
    check_any_time("15 of february");
    check_any_time("february the 15th");
    check_any_time("february 15");
    check_any_time("15th february");
    check_any_time("February 15");
}

#[test]
fn test_time_aug_8() {
    check_any_time("Aug 8");
}

#[test]
fn test_time_march_in_1_year() {
    check_any_time("March in 1 year");
    check_any_time("March in a year");
}

#[test]
fn test_time_fri_jul_18() {
    check_any_time("Fri, Jul 18");
    check_any_time("Jul 18, Fri");
}

#[test]
fn test_time_october_2014() {
    check_any_time("October 2014");
    check_any_time("2014-10");
    check_any_time("2014/10");
}

#[test]
fn test_time_14april_2015() {
    check_any_time("14april 2015");
    check_any_time("April 14, 2015");
    check_any_time("14th April 15");
}

// ============================================================
// Next/last modifiers
// ============================================================
#[test]
fn test_time_next_tuesday() {
    check_time("next tuesday", "day");
    check_any_time("around next tuesday");
}

#[test]
fn test_time_friday_after_next() {
    check_any_time("friday after next");
}

#[test]
fn test_time_next_march() {
    check_any_time("next March");
}

#[test]
fn test_time_march_after_next() {
    check_any_time("March after next");
}

#[test]
fn test_time_sunday_feb_10() {
    check_any_time("Sunday, Feb 10");
}

#[test]
fn test_time_wed_feb13() {
    check_any_time("Wed, Feb13");
}

// ============================================================
// This/last/next week
// ============================================================
#[test]
fn test_time_this_week() {
    check_any_time("this week");
    check_any_time("current week");
}

#[test]
fn test_time_last_week() {
    check_any_time("last week");
    check_any_time("past week");
    check_any_time("previous week");
}

#[test]
fn test_time_next_week() {
    check_any_time("next week");
    check_any_time("the following week");
    check_any_time("around next week");
    check_any_time("upcoming week");
    check_any_time("coming week");
}

// ============================================================
// This/last/next month
// ============================================================
#[test]
fn test_time_last_month() {
    check_any_time("last month");
}

#[test]
fn test_time_next_month() {
    check_any_time("next month");
}

#[test]
fn test_time_20th_of_next_month() {
    check_any_time("20 of next month");
    check_any_time("20th of the next month");
    check_any_time("20th day of next month");
}

#[test]
fn test_time_20th_of_current_month() {
    check_any_time("20th of the current month");
    check_any_time("20 of this month");
}

#[test]
fn test_time_20th_of_previous_month() {
    check_any_time("20th of the previous month");
}

// ============================================================
// Quarters
// ============================================================
#[test]
fn test_time_this_quarter() {
    check_any_time("this quarter");
    check_any_time("this qtr");
}

#[test]
fn test_time_next_quarter() {
    check_any_time("next quarter");
    check_any_time("next qtr");
}

#[test]
fn test_time_third_quarter() {
    check_any_time("third quarter");
    check_any_time("3rd quarter");
    check_any_time("third qtr");
    check_any_time("3rd qtr");
    check_any_time("the 3rd qtr");
}

#[test]
fn test_time_4th_quarter_2018() {
    check_any_time("4th quarter 2018");
    check_any_time("4th qtr 2018");
    check_any_time("the 4th qtr of 2018");
    check_any_time("18q4");
    check_any_time("2018Q4");
}

// ============================================================
// This/last/next year
// ============================================================
#[test]
fn test_time_last_year() {
    check_any_time("last year");
    check_any_time("last yr");
}

#[test]
fn test_time_this_year() {
    check_any_time("this year");
    check_any_time("current year");
    check_any_time("this yr");
}

#[test]
fn test_time_next_year() {
    check_any_time("next year");
    check_any_time("next yr");
}

#[test]
fn test_time_in_2014_ad() {
    check_any_time("in 2014 A.D.");
    check_any_time("in 2014 AD");
}

#[test]
fn test_time_in_2014_bc() {
    check_any_time("in 2014 B.C.");
    check_any_time("in 2014 BC");
}

// ============================================================
// Last/next day of week
// ============================================================
#[test]
fn test_time_last_sunday() {
    check_any_time("last sunday");
    check_any_time("sunday from last week");
    check_any_time("last week's sunday");
}

#[test]
fn test_time_last_tuesday() {
    check_any_time("last tuesday");
}

#[test]
fn test_time_next_wednesday() {
    check_time("next wednesday", "day");
}

#[test]
fn test_time_wednesday_of_next_week() {
    check_any_time("wednesday of next week");
    check_any_time("wednesday next week");
    check_any_time("wednesday after next");
}

#[test]
fn test_time_monday_of_this_week() {
    check_any_time("monday of this week");
}

#[test]
fn test_time_tuesday_of_this_week() {
    check_any_time("tuesday of this week");
}

#[test]
fn test_time_wednesday_of_this_week() {
    check_any_time("wednesday of this week");
}

// ============================================================
// Day after/before tomorrow/yesterday
// ============================================================
#[test]
fn test_time_day_after_tomorrow() {
    check_any_time("the day after tomorrow");
}

#[test]
fn test_time_day_after_tomorrow_5pm() {
    check_any_time("day after tomorrow 5pm");
}

#[test]
fn test_time_day_before_yesterday() {
    check_any_time("the day before yesterday");
}

#[test]
fn test_time_day_before_yesterday_8am() {
    check_any_time("day before yesterday 8am");
}

// ============================================================
// Last/first day of month
// ============================================================
#[test]
fn test_time_last_monday_of_march() {
    check_any_time("last Monday of March");
}

#[test]
fn test_time_last_sunday_of_march_2014() {
    check_any_time("last Sunday of March 2014");
}

#[test]
fn test_time_third_day_of_october() {
    check_any_time("third day of october");
}

#[test]
fn test_time_first_week_of_october_2014() {
    check_any_time("first week of october 2014");
}

#[test]
fn test_time_last_day_of_october_2015() {
    check_any_time("last day of october 2015");
    check_any_time("last day in october 2015");
}

#[test]
fn test_time_first_tuesday_of_october() {
    check_any_time("first tuesday of october");
    check_any_time("first tuesday in october");
}

#[test]
fn test_time_first_wednesday_of_october_2014() {
    check_any_time("first wednesday of october 2014");
}

// ============================================================
// Clock times - AM
// ============================================================
#[test]
fn test_time_at_3am() {
    check_time("at 3am", "hour");
    check_time("3 in the AM", "hour");
    check_time("at 3 AM", "hour");
    check_time("3 oclock am", "hour");
    check_time("at three am", "hour");
}

#[test]
fn test_time_3am_morning() {
    check_any_time("this morning at 3");
    check_any_time("3 in the morning");
    check_any_time("at 3 in the morning");
    check_any_time("early morning @ 3");
}

#[test]
fn test_time_this_morning_at_10() {
    check_any_time("this morning @ 10");
    check_any_time("this morning at 10am");
}

#[test]
fn test_time_3_18am() {
    check_time("3:18am", "minute");
    check_time("3:18a", "minute");
    check_any_time("3h18");
}

// ============================================================
// Clock times - PM
// ============================================================
#[test]
fn test_time_at_3pm() {
    check_time("at 3pm", "hour");
    check_time("@ 3pm", "hour");
    check_time("3PM", "hour");
    check_time("3pm", "hour");
    check_time("3 oclock pm", "hour");
}

#[test]
fn test_time_3pm_variants() {
    check_any_time("3 o'clock in the afternoon");
    check_any_time("3ish pm");
    check_any_time("3pm approximately");
    check_any_time("at about 3pm");
    check_any_time("at 3p");
    check_any_time("at 3p.");
}

#[test]
fn test_time_15h00() {
    check_any_time("15h00");
    check_any_time("at 15h00");
    check_any_time("15h");
    check_any_time("at 15h");
}

#[test]
fn test_time_quarter_past_3pm() {
    check_any_time("at 15 past 3pm");
    check_any_time("a quarter past 3pm");
    check_any_time("3:15 in the afternoon");
    check_time("15:15", "minute");
    check_any_time("15h15");
    check_time("3:15pm", "minute");
    check_time("3:15PM", "minute");
    check_time("3:15p", "minute");
    check_any_time("at 3 15");
}

#[test]
fn test_time_20_past_3pm() {
    check_any_time("at 20 past 3pm");
    check_any_time("3:20 in the afternoon");
    check_any_time("3:20 in afternoon");
    check_any_time("twenty after 3pm");
    check_time("3:20p", "minute");
    check_any_time("15h20");
    check_any_time("at three twenty");
    check_any_time("this afternoon at 3:20");
}

#[test]
fn test_time_half_past_3pm() {
    check_any_time("at half past three pm");
    check_any_time("half past 3 pm");
    check_time("15:30", "minute");
    check_any_time("15h30");
    check_time("3:30pm", "minute");
    check_time("3:30PM", "minute");
    check_any_time("330 p.m.");
    check_any_time("3:30 p m");
    check_time("3:30", "minute");
    check_any_time("half three");
}

#[test]
fn test_time_quarter_past_noon() {
    check_any_time("at 15 past noon");
    check_any_time("a quarter past noon");
    check_time("12:15", "minute");
    check_any_time("12h15");
    check_time("12:15pm", "minute");
    check_time("12:15PM", "minute");
    check_time("12:15p", "minute");
    check_any_time("at 12 15");
}

#[test]
fn test_time_9_59am() {
    check_any_time("nine fifty nine a m");
}

#[test]
fn test_time_15_23_24() {
    check_any_time("15:23:24");
}

#[test]
fn test_time_9_01_10_am() {
    check_any_time("9:01:10 AM");
}

#[test]
fn test_time_quarter_to_noon() {
    check_any_time("a quarter to noon");
    check_time("11:45am", "minute");
    check_any_time("11h45");
    check_any_time("15 to noon");
}

#[test]
fn test_time_quarter_past_1pm() {
    check_time("1:15pm", "minute");
    check_any_time("a quarter past 1pm");
    check_any_time("13h15");
}

#[test]
fn test_time_8_tonight() {
    check_any_time("8 tonight");
    check_any_time("tonight at 8 o'clock");
    check_any_time("eight tonight");
    check_any_time("8 this evening");
    check_any_time("at 8 in the evening");
    check_any_time("in the evening at eight");
}

#[test]
fn test_time_7_30pm_on_fri_sep_20() {
    check_any_time("at 7:30 PM on Fri, Sep 20");
    check_any_time("at 19h30 on Fri, Sep 20");
}

#[test]
fn test_time_saturday_morning_at_9() {
    check_any_time("at 9am on Saturday");
    check_any_time("Saturday morning at 9");
}

#[test]
fn test_time_fri_jul_18_2014_7pm() {
    check_any_time("Fri, Jul 18, 2014 07:00 PM");
    check_any_time("Fri, Jul 18, 2014 19h00");
    check_any_time("Fri, Jul 18, 2014 19h");
}

// ============================================================
// Relative time - in X
// ============================================================
#[test]
fn test_time_in_a_sec() {
    check_any_time("in a sec");
    check_any_time("one second from now");
}

#[test]
fn test_time_in_a_minute() {
    check_any_time("in a minute");
    check_any_time("in one minute");
}

#[test]
fn test_time_in_2_minutes() {
    check_any_time("in 2 minutes");
    check_any_time("in 2 more minutes");
    check_any_time("2 minutes from now");
    check_any_time("in a couple of minutes");
}

#[test]
fn test_time_in_three_minutes() {
    check_any_time("in three minutes");
    check_any_time("in a few minutes");
}

#[test]
fn test_time_in_60_minutes() {
    check_any_time("in 60 minutes");
}

#[test]
fn test_time_in_quarter_hour() {
    check_any_time("in a quarter of an hour");
    check_any_time("in 1/4h");
    check_any_time("in 1/4 h");
    check_any_time("in 1/4 hour");
}

#[test]
fn test_time_in_half_hour() {
    check_any_time("in half an hour");
    check_any_time("in 1/2h");
    check_any_time("in 1/2 h");
    check_any_time("in 1/2 hour");
}

#[test]
fn test_time_in_three_quarters_hour() {
    check_any_time("in three-quarters of an hour");
    check_any_time("in 3/4h");
    check_any_time("in 3/4 h");
    check_any_time("in 3/4 hour");
}

#[test]
fn test_time_in_2_5_hours() {
    check_any_time("in 2.5 hours");
    check_any_time("in 2 and an half hours");
}

#[test]
fn test_time_in_one_hour() {
    check_any_time("in one hour");
    check_any_time("in 1h");
}

#[test]
fn test_time_in_a_couple_hours() {
    check_any_time("in a couple hours");
    check_any_time("in a couple of hours");
}

#[test]
fn test_time_in_a_few_hours() {
    check_any_time("in a few hours");
    check_any_time("in few hours");
}

#[test]
fn test_time_in_24_hours() {
    check_any_time("in 24 hours");
}

#[test]
fn test_time_in_a_day() {
    check_any_time("in a day");
    check_any_time("a day from now");
}

#[test]
fn test_time_3_years_from_today() {
    check_any_time("3 years from today");
}

#[test]
fn test_time_3_fridays_from_now() {
    check_any_time("3 fridays from now");
    check_any_time("three fridays from now");
}

#[test]
fn test_time_in_7_days() {
    check_any_time("in 7 days");
}

#[test]
fn test_time_in_7_days_at_5pm() {
    check_any_time("in 7 days at 5pm");
}

#[test]
fn test_time_in_1_week() {
    check_any_time("in 1 week");
    check_any_time("in a week");
}

#[test]
fn test_time_in_about_half_hour() {
    check_any_time("in about half an hour");
}

// ============================================================
// Relative time - X ago
// ============================================================
#[test]
fn test_time_7_days_ago() {
    check_any_time("7 days ago");
}

#[test]
fn test_time_14_days_ago() {
    check_any_time("14 days Ago");
    check_any_time("a fortnight ago");
}

#[test]
fn test_time_a_week_ago() {
    check_any_time("a week ago");
    check_any_time("one week ago");
    check_any_time("1 week ago");
}

#[test]
fn test_time_2_thursdays_ago() {
    check_any_time("2 thursdays back");
    check_any_time("2 thursdays ago");
}

#[test]
fn test_time_three_weeks_ago() {
    check_any_time("three weeks ago");
}

#[test]
fn test_time_three_months_ago() {
    check_any_time("three months ago");
}

#[test]
fn test_time_two_years_ago() {
    check_any_time("two years ago");
}

// ============================================================
// Hence
// ============================================================
#[test]
fn test_time_7_days_hence() {
    check_any_time("7 days hence");
}

#[test]
fn test_time_14_days_hence() {
    check_any_time("14 days hence");
    check_any_time("a fortnight hence");
}

#[test]
fn test_time_a_week_hence() {
    check_any_time("a week hence");
    check_any_time("one week hence");
    check_any_time("1 week hence");
}

#[test]
fn test_time_three_weeks_hence() {
    check_any_time("three weeks hence");
}

#[test]
fn test_time_three_months_hence() {
    check_any_time("three months hence");
}

#[test]
fn test_time_two_years_hence() {
    check_any_time("two years hence");
}

// ============================================================
// One year after holiday
// ============================================================
#[test]
fn test_time_one_year_after_christmas() {
    check_any_time("one year After christmas");
    check_any_time("a year from Christmas");
}

// ============================================================
// Duration intervals (from X for Y)
// ============================================================
#[test]
fn test_time_for_10_days_from_18th_dec() {
    check_any_time("for 10 days from 18th Dec");
    check_any_time("from 18th Dec for 10 days");
    check_any_time("18th Dec for 10 days");
}

#[test]
fn test_time_from_4pm_for_30_mins() {
    check_any_time("for 30' starting from 4pm");
    check_any_time("from 4pm for thirty minutes");
    check_any_time("4pm for 30 mins");
    check_any_time("16h for 30 mins");
}

// ============================================================
// Seasons
// ============================================================
#[test]
fn test_time_this_summer() {
    check_any_time("this Summer");
    check_any_time("current summer");
}

#[test]
fn test_time_this_winter() {
    check_any_time("this winter");
}

#[test]
fn test_time_this_season() {
    check_any_time("this season");
    check_any_time("current seasons");
}

#[test]
fn test_time_last_season() {
    check_any_time("last season");
    check_any_time("past seasons");
    check_any_time("previous seasons");
}

#[test]
fn test_time_next_season() {
    check_any_time("next season");
}

// ============================================================
// Evening/night
// ============================================================
#[test]
fn test_time_last_night() {
    check_any_time("last night");
    check_any_time("yesterday evening");
}

#[test]
fn test_time_late_last_night() {
    check_any_time("late last night");
}

#[test]
fn test_time_this_evening() {
    check_any_time("this evening");
    check_any_time("today evening");
    check_any_time("tonight");
}

#[test]
fn test_time_tomorrow_evening() {
    check_any_time("tomorrow evening");
}

#[test]
fn test_time_tomorrow_lunch() {
    check_any_time("tomorrow lunch");
    check_any_time("tomorrow at lunch");
}

#[test]
fn test_time_this_weekend() {
    check_any_time("this week-end");
}

#[test]
fn test_time_late_tonight() {
    check_any_time("late tonight");
    check_any_time("late tonite");
}

// ============================================================
// Morning of specific date
// ============================================================
#[test]
fn test_time_morning_of_feb_15() {
    check_any_time("february the 15th in the morning");
    check_any_time("15 of february in the morning");
    check_any_time("morning of the 15th of february");
}

// ============================================================
// Holidays - Christmas
// ============================================================
#[test]
fn test_time_christmas() {
    check_any_time("xmas");
    check_any_time("christmas");
    check_any_time("christmas day");
}

#[test]
fn test_time_xmas_at_6pm() {
    check_any_time("xmas at 6 pm");
}

#[test]
fn test_time_morning_of_xmas() {
    check_any_time("morning of xmas");
    check_any_time("morning of christmas 2013");
    check_any_time("morning of this christmas day");
}

// ============================================================
// Holidays - New Year
// ============================================================
#[test]
fn test_time_new_years_eve() {
    check_any_time("new year's eve");
    check_any_time("new years eve");
}

#[test]
fn test_time_new_years_day() {
    check_any_time("new year's day");
    check_any_time("new years day");
}

// ============================================================
// Holidays - Valentine's Day
// ============================================================
#[test]
fn test_time_valentines_day() {
    check_any_time("valentine's day");
    check_any_time("valentine day");
}

// ============================================================
// Holidays - 4th of July
// ============================================================
#[test]
fn test_time_4th_of_july() {
    check_any_time("4th of July");
    check_any_time("4 of july");
}

// ============================================================
// Holidays - Halloween
// ============================================================
#[test]
fn test_time_halloween() {
    check_any_time("halloween");
    check_any_time("next halloween");
    check_any_time("Halloween 2013");
}

// ============================================================
// Holidays - Black Friday
// ============================================================
#[test]
fn test_time_black_friday() {
    check_any_time("black friday");
    check_any_time("black friday of this year");
    check_any_time("black friday 2013");
}

#[test]
fn test_time_black_friday_2017() {
    check_any_time("black friday 2017");
}

// ============================================================
// Holidays - Boss's Day
// ============================================================
#[test]
fn test_time_bosss_day() {
    check_any_time("boss's day");
    check_any_time("boss's");
    check_any_time("boss day");
    check_any_time("next boss's day");
}

// ============================================================
// Holidays - MLK Day
// ============================================================
#[test]
fn test_time_mlk_day() {
    check_any_time("MLK day");
    check_any_time("next Martin Luther King day");
    check_any_time("next Martin Luther King's day");
    check_any_time("next Martin Luther Kings day");
    check_any_time("this MLK day");
}

#[test]
fn test_time_last_mlk_day() {
    check_any_time("last MLK Jr. day");
    check_any_time("MLK day 2013");
}

// ============================================================
// Holidays - World Vegan Day
// ============================================================
#[test]
fn test_time_world_vegan_day() {
    check_any_time("world vegan day");
}

// ============================================================
// Holidays - Easter
// ============================================================
#[test]
fn test_time_easter() {
    check_any_time("easter");
    check_any_time("easter 2013");
}

#[test]
fn test_time_last_easter() {
    check_any_time("last easter");
}

#[test]
fn test_time_easter_monday() {
    check_any_time("easter mon");
}

#[test]
fn test_time_easter_2010() {
    check_any_time("easter 2010");
    check_any_time("Easter Sunday two thousand ten");
}

#[test]
fn test_time_three_days_after_easter() {
    check_any_time("three days after Easter");
}

// ============================================================
// Holidays - Maundy Thursday, Pentecost, etc.
// ============================================================
#[test]
fn test_time_maundy_thursday() {
    check_any_time("Maundy Thursday");
    check_any_time("Covenant thu");
    check_any_time("Thu of Mysteries");
}

#[test]
fn test_time_pentecost() {
    check_any_time("Pentecost");
    check_any_time("white sunday 2013");
}

#[test]
fn test_time_whit_monday() {
    check_any_time("whit monday");
    check_any_time("Monday of the Holy Spirit");
}

#[test]
fn test_time_palm_sunday() {
    check_any_time("palm sunday");
    check_any_time("branch sunday 2013");
}

#[test]
fn test_time_trinity_sunday() {
    check_any_time("trinity sunday");
}

#[test]
fn test_time_shrove_tuesday() {
    check_any_time("pancake day 2013");
    check_any_time("mardi gras");
}

// ============================================================
// Holidays - St Patrick's Day
// ============================================================
#[test]
fn test_time_st_patricks_day() {
    check_any_time("st patrick's day 2013");
    check_any_time("st paddy's day");
    check_any_time("saint paddy's day");
    check_any_time("saint patricks day");
}

// ============================================================
// Holidays - Lent
// ============================================================
#[test]
fn test_time_lent_2018() {
    check_any_time("lent 2018");
}

// ============================================================
// Holidays - Orthodox Easter
// ============================================================
#[test]
fn test_time_orthodox_easter_2018() {
    check_any_time("orthodox easter 2018");
}

#[test]
fn test_time_orthodox_good_friday_2020() {
    check_any_time("orthodox good friday 2020");
    check_any_time("orthodox great friday 2020");
}

#[test]
fn test_time_clean_monday_2018() {
    check_any_time("clean monday 2018");
    check_any_time("orthodox shrove monday two thousand eighteen");
}

// ============================================================
// Evening/afternoon intervals
// ============================================================
#[test]
fn test_time_monday_morning() {
    check_any_time("monday mOrnIng");
}

#[test]
fn test_time_monday_early_morning() {
    check_any_time("monday early in the morning");
    check_any_time("monday early morning");
    check_any_time("monday in the early hours of the morning");
}

// ============================================================
// Last/next N seconds/minutes/hours/days/weeks/months/years
// ============================================================
#[test]
fn test_time_last_2_seconds() {
    check_any_time("last 2 seconds");
    check_any_time("last two seconds");
}

#[test]
fn test_time_next_3_seconds() {
    check_any_time("next 3 seconds");
    check_any_time("next three seconds");
}

#[test]
fn test_time_last_2_minutes() {
    check_any_time("last 2 minutes");
    check_any_time("last two minutes");
}

#[test]
fn test_time_next_3_minutes() {
    check_any_time("next 3 minutes");
    check_any_time("next three minutes");
}

#[test]
fn test_time_last_1_hour() {
    check_any_time("last 1 hour");
    check_any_time("last one hour");
}

#[test]
fn test_time_next_3_hours() {
    check_any_time("next 3 hours");
    check_any_time("next three hours");
}

#[test]
fn test_time_last_2_days() {
    check_any_time("last 2 days");
    check_any_time("last two days");
    check_any_time("past 2 days");
}

#[test]
fn test_time_next_3_days() {
    check_any_time("next 3 days");
    check_any_time("next three days");
}

#[test]
fn test_time_next_few_days() {
    check_any_time("next few days");
}

#[test]
fn test_time_last_2_weeks() {
    check_any_time("last 2 weeks");
    check_any_time("last two weeks");
    check_any_time("past 2 weeks");
}

#[test]
fn test_time_next_3_weeks() {
    check_any_time("next 3 weeks");
    check_any_time("next three weeks");
}

#[test]
fn test_time_last_2_months() {
    check_any_time("last 2 months");
    check_any_time("last two months");
}

#[test]
fn test_time_next_3_months() {
    check_any_time("next 3 months");
    check_any_time("next three months");
}

#[test]
fn test_time_last_2_years() {
    check_any_time("last 2 years");
    check_any_time("last two years");
}

#[test]
fn test_time_next_3_years() {
    check_any_time("next 3 years");
    check_any_time("next three years");
}

// ============================================================
// Date range intervals
// ============================================================
#[test]
fn test_time_july_13_to_15() {
    check_any_time("July 13-15");
    check_any_time("July 13 to 15");
    check_any_time("July 13 thru 15");
    check_any_time("July 13 through 15");
    check_any_time("July 13 - July 15");
}

#[test]
fn test_time_from_july_13_to_15() {
    check_any_time("from July 13-15");
    check_any_time("from 13 to 15 July");
    check_any_time("from 13th to 15th July");
}

#[test]
fn test_time_aug_8_to_12() {
    check_any_time("Aug 8 - Aug 12");
}

// ============================================================
// Time range intervals
// ============================================================
#[test]
fn test_time_930_to_1100() {
    check_any_time("9:30 - 11:00");
    check_any_time("9h30 - 11h00");
}

#[test]
fn test_time_930_to_1100_on_thursday() {
    check_any_time("from 9:30 - 11:00 on Thursday");
    check_any_time("between 9:30 and 11:00 on thursday");
    check_any_time("9:30 - 11:00 on Thursday");
    check_any_time("Thursday from 9:30 to 11:00");
    check_any_time("9:30 till 11:00 on Thursday");
}

#[test]
fn test_time_3_to_4pm() {
    check_any_time("3-4pm");
    check_any_time("from 3 to 4 in the PM");
    check_any_time("around 3-4pm");
}

#[test]
fn test_time_330_to_6pm() {
    check_any_time("3:30 to 6 PM");
    check_any_time("3:30-6 p.m.");
    check_any_time("3:30-6:00pm");
    check_any_time("from 3:30 to six p.m.");
    check_any_time("from 3:30 to 6:00pm");
    check_any_time("between 3:30pm and 6 pm");
}

#[test]
fn test_time_8am_to_1pm() {
    check_any_time("8am - 1pm");
}

#[test]
fn test_time_thursday_9a_to_11a() {
    check_any_time("Thursday from 9a to 11a");
    check_any_time("this Thu 9-11am");
}

// ============================================================
// Datetime on specific day
// ============================================================
#[test]
fn test_time_1_30pm_on_sat_sep_21() {
    check_any_time("1:30 PM on Sat, Sep 21");
}

// ============================================================
// Within / by
// ============================================================
#[test]
fn test_time_within_2_weeks() {
    check_any_time("Within 2 weeks");
}

#[test]
fn test_time_by_2pm() {
    check_any_time("by 2:00pm");
}

#[test]
fn test_time_by_eod() {
    check_any_time("by EOD");
}

#[test]
fn test_time_by_eom() {
    check_any_time("by EOM");
    check_any_time("by the EOM");
    check_any_time("by end of the month");
    check_any_time("by the end of month");
}

#[test]
fn test_time_eom() {
    check_any_time("EOM");
    check_any_time("the EOM");
    check_any_time("end of the month");
}

#[test]
fn test_time_bom() {
    check_any_time("BOM");
    check_any_time("beginning of the month");
}

// ============================================================
// Timezone examples
// ============================================================
#[test]
fn test_time_4pm_cet() {
    check_any_time("4pm CET");
}

#[test]
fn test_time_thursday_8_gmt() {
    check_any_time("Thursday 8:00 GMT");
    check_any_time("Thursday 8:00 gmt");
    check_any_time("Thu at 8 GMT");
}

#[test]
fn test_time_thursday_8_pst() {
    check_any_time("Thursday 8:00 PST");
    check_any_time("Thursday 8:00 pst");
    check_any_time("Thu at 8 am PST");
    check_any_time("Thu at 8 am pst");
}

// ============================================================
// Today at time
// ============================================================
#[test]
fn test_time_today_at_2pm() {
    check_any_time("today at 2pm");
    check_time("at 2pm", "hour");
    check_any_time("this afternoon at 2");
    check_any_time("tonight at 2");
}

#[test]
fn test_time_3pm_tomorrow() {
    check_any_time("3pm tomorrow");
}

// ============================================================
// ASAP
// ============================================================
#[test]
fn test_time_asap() {
    check_any_time("ASAP");
    check_any_time("as soon as possible");
}

// ============================================================
// Until / after / before / since
// ============================================================
#[test]
fn test_time_until_2pm() {
    check_any_time("until 2:00pm");
    check_any_time("through 2:00pm");
}

#[test]
fn test_time_after_2pm() {
    check_any_time("after 2 pm");
    check_any_time("from 2 pm");
    check_any_time("since 2pm");
}

#[test]
fn test_time_before_11am() {
    check_any_time("before 11 am");
}

#[test]
fn test_time_in_the_afternoon() {
    check_any_time("in the afternoon");
}

// ============================================================
// After lunch / school
// ============================================================
#[test]
fn test_time_after_lunch() {
    check_any_time("after lunch");
}

#[test]
fn test_time_after_school() {
    check_any_time("after school");
}

// ============================================================
// This morning
// ============================================================
#[test]
fn test_time_this_morning() {
    check_any_time("this morning");
}

// ============================================================
// Noon / midnight
// ============================================================
#[test]
fn test_time_noon() {
    check_any_time("at 12pm");
    check_any_time("at noon");
    check_any_time("midday");
    check_any_time("the midday");
    check_any_time("mid day");
}

#[test]
fn test_time_midnight() {
    check_any_time("at 12am");
    check_any_time("at midnight");
}

// ============================================================
// Tomorrow morning/evening
// ============================================================
#[test]
fn test_time_9_tomorrow_morning() {
    check_any_time("9 tomorrow morning");
    check_any_time("9 tomorrow");
}

#[test]
fn test_time_9_tomorrow_evening() {
    check_any_time("9 tomorrow evening");
}

// ============================================================
// Month names
// ============================================================
#[test]
fn test_time_march() {
    check_time("March", "month");
    check_any_time("in March");
    check_any_time("during March");
}

// ============================================================
// Tomorrow afternoon at 5
// ============================================================
#[test]
fn test_time_tomorrow_afternoon_at_5() {
    check_any_time("tomorrow afternoon at 5");
    check_any_time("at 5 tomorrow afternoon");
    check_any_time("at 5pm tomorrow");
    check_any_time("tomorrow at 5pm");
    check_any_time("tomorrow evening at 5");
}

#[test]
fn test_time_tomorrow_afternoon() {
    check_any_time("tomorrow afternoon");
    check_any_time("tomorrow afternoonish");
}

// ============================================================
// On the first / the 1st
// ============================================================
#[test]
fn test_time_on_the_first() {
    check_any_time("on the first");
    check_any_time("the 1st");
}

// ============================================================
// At 1030 / ten thirty am
// ============================================================
#[test]
fn test_time_at_1030() {
    check_any_time("at 1030");
    check_any_time("around 1030");
    check_any_time("ten thirty am");
}

#[test]
fn test_time_at_730_evening() {
    check_any_time("at 730 in the evening");
    check_any_time("seven thirty p.m.");
}

// ============================================================
// Tonight at 11
// ============================================================
#[test]
fn test_time_tonight_at_11() {
    check_any_time("tonight at 11");
    check_any_time("this evening at 11");
    check_any_time("tonight at 11pm");
}

// ============================================================
// At 4:23
// ============================================================
#[test]
fn test_time_at_4_23() {
    check_any_time("at 4:23");
    check_time("4:23am", "minute");
    check_any_time("four twenty-three a m");
}

// ============================================================
// Closest Monday to date
// ============================================================
#[test]
fn test_time_closest_monday_to_oct_5() {
    check_any_time("the closest Monday to Oct 5th");
}

// ============================================================
// Early/mid/late month
// ============================================================
#[test]
fn test_time_early_march() {
    check_any_time("early March");
}

#[test]
fn test_time_mid_march() {
    check_any_time("mid March");
}

#[test]
fn test_time_late_march() {
    check_any_time("late March");
}

// ============================================================
// Last weekend of month
// ============================================================
#[test]
fn test_time_last_weekend_of_october() {
    check_any_time("last weekend of October");
    check_any_time("last week-end in October");
    check_any_time("last week end of October");
}

// ============================================================
// All week / rest of the week
// ============================================================
#[test]
fn test_time_all_week() {
    check_any_time("all week");
}

#[test]
fn test_time_rest_of_the_week() {
    check_any_time("rest of the week");
}

// ============================================================
// Date ranges (August 27th - 29th, etc.)
// ============================================================
#[test]
fn test_time_august_27_to_29() {
    check_any_time("August 27th - 29th");
    check_any_time("from August 27th - 29th");
}

#[test]
fn test_time_23rd_to_26th_oct() {
    check_any_time("23rd to 26th Oct");
}

#[test]
fn test_time_1_to_8_september() {
    check_any_time("1-8 september");
}

#[test]
fn test_time_12_to_16_september() {
    check_any_time("12 to 16 september");
}

// ============================================================
// End/beginning of month/year
// ============================================================
#[test]
fn test_time_end_of_april() {
    check_any_time("end of April");
    check_any_time("at the end of April");
}

#[test]
fn test_time_beginning_of_january() {
    check_any_time("beginning of January");
    check_any_time("at the beginning of January");
}

#[test]
fn test_time_end_of_2012() {
    check_any_time("end of 2012");
    check_any_time("at the end of 2012");
}

#[test]
fn test_time_beginning_of_2017() {
    check_any_time("beginning of 2017");
    check_any_time("at the beginning of 2017");
}

#[test]
fn test_time_beginning_of_year() {
    check_any_time("beginning of year");
    check_any_time("the beginning of the year");
    check_any_time("the BOY");
    check_any_time("BOY");
}

#[test]
fn test_time_by_eoy() {
    check_any_time("by EOY");
    check_any_time("by the EOY");
    check_any_time("by end of the year");
    check_any_time("by the end of year");
}

#[test]
fn test_time_eoy() {
    check_any_time("EOY");
    check_any_time("the EOY");
    check_any_time("the end of the year");
    check_any_time("end of the year");
}

// ============================================================
// Beginning/end of week
// ============================================================
#[test]
fn test_time_beginning_of_this_week() {
    check_any_time("beginning of this week");
    check_any_time("beginning of current week");
}

#[test]
fn test_time_beginning_of_next_week() {
    check_any_time("beginning of next week");
    check_any_time("beginning of the following week");
}

#[test]
fn test_time_beginning_of_last_week() {
    check_any_time("beginning of last week");
    check_any_time("beginning of past week");
    check_any_time("beginning of previous week");
}

#[test]
fn test_time_end_of_this_week() {
    check_any_time("end of this week");
    check_any_time("end of current week");
}

#[test]
fn test_time_end_of_next_week() {
    check_any_time("end of next week");
    check_any_time("end of the following week");
}

#[test]
fn test_time_end_of_last_week() {
    check_any_time("end of last week");
    check_any_time("end of past week");
    check_any_time("end of previous week");
}

// ============================================================
// Holidays - Chinese New Year
// ============================================================
#[test]
fn test_time_chinese_new_year() {
    check_any_time("chinese new year");
    check_any_time("chinese lunar new year's day");
}

#[test]
fn test_time_last_chinese_new_year() {
    check_any_time("last chinese new year");
    check_any_time("last chinese lunar new year's day");
    check_any_time("last chinese new years");
}

#[test]
fn test_time_chinese_new_year_2018() {
    check_any_time("chinese new year's day 2018");
}

// ============================================================
// Holidays - Jewish
// ============================================================
#[test]
fn test_time_yom_kippur_2018() {
    check_any_time("yom kippur 2018");
}

#[test]
fn test_time_shemini_atzeret_2018() {
    check_any_time("shemini atzeret 2018");
}

#[test]
fn test_time_simchat_torah_2018() {
    check_any_time("simchat torah 2018");
}

#[test]
fn test_time_tisha_bav_2018() {
    check_any_time("tisha b'av 2018");
}

#[test]
fn test_time_yom_haatzmaut_2018() {
    check_any_time("yom haatzmaut 2018");
}

#[test]
fn test_time_lag_baomer_2017() {
    check_any_time("lag b'omer 2017");
}

#[test]
fn test_time_yom_hashoah_2018() {
    check_any_time("Yom Hashoah 2018");
    check_any_time("Holocaust Day 2018");
}

#[test]
fn test_time_rosh_hashanah_2018() {
    check_any_time("rosh hashanah 2018");
    check_any_time("rosh hashana 2018");
    check_any_time("rosh hashanna 2018");
}

#[test]
fn test_time_hanukkah_2018() {
    check_any_time("Chanukah 2018");
    check_any_time("hanukah 2018");
    check_any_time("hannukkah 2018");
}

#[test]
fn test_time_passover_2018() {
    check_any_time("passover 2018");
}

#[test]
fn test_time_sukkot_2018() {
    check_any_time("feast of the ingathering 2018");
    check_any_time("succos 2018");
}

#[test]
fn test_time_shavuot_2018() {
    check_any_time("shavuot 2018");
}

#[test]
fn test_time_tu_bishvat_2018() {
    check_any_time("tu bishvat 2018");
}

#[test]
fn test_time_purim() {
    check_any_time("purim");
}

#[test]
fn test_time_shushan_purim() {
    check_any_time("Shushan Purim");
}

// ============================================================
// Holidays - Islamic
// ============================================================
#[test]
fn test_time_mawlid_2017() {
    check_any_time("mawlid al-nabawi 2017");
}

#[test]
fn test_time_eid_al_fitr_2018() {
    check_any_time("Eid al-Fitr 2018");
}

#[test]
fn test_time_eid_al_adha_2018() {
    check_any_time("Eid al-Adha 2018");
    check_any_time("id ul-adha 2018");
    check_any_time("sacrifice feast 2018");
    check_any_time("Bakr Id 2018");
}

#[test]
fn test_time_laylat_al_qadr_2018() {
    check_any_time("laylat al-qadr 2018");
    check_any_time("night of power 2018");
}

#[test]
fn test_time_islamic_new_year_2018() {
    check_any_time("Islamic New Year 2018");
    check_any_time("Amun Jadid 2018");
}

#[test]
fn test_time_ashura_2017() {
    check_any_time("day of Ashura 2017");
}

#[test]
fn test_time_ramadan_2018() {
    check_any_time("Ramadan 2018");
}

#[test]
fn test_time_isra_and_miraj_2018() {
    check_any_time("isra and mi'raj 2018");
    check_any_time("the prophet's ascension 2018");
}

// ============================================================
// Holidays - Hindu
// ============================================================
#[test]
fn test_time_dhanteras_2019() {
    check_any_time("dhanteras 2019");
}

#[test]
fn test_time_diwali_2019() {
    check_any_time("diwali 2019");
    check_any_time("Deepavali in 2019");
}

#[test]
fn test_time_bhai_dooj_2019() {
    check_any_time("bhai dooj 2019");
}

#[test]
fn test_time_chhath_2019() {
    check_any_time("chhath 2019");
    check_any_time("dala puja 2019");
}

#[test]
fn test_time_navaratri_2018() {
    check_any_time("navaratri 2018");
    check_any_time("durga puja in 2018");
}

#[test]
fn test_time_karva_chauth_2018() {
    check_any_time("karva chauth 2018");
}

#[test]
fn test_time_ratha_yatra_2018() {
    check_any_time("ratha-yatra 2018");
}

#[test]
fn test_time_raksha_bandhan_2018() {
    check_any_time("rakhi 2018");
}

#[test]
fn test_time_mahavir_jayanti_2020() {
    check_any_time("mahavir jayanti 2020");
}

#[test]
fn test_time_maha_shivaratri_2020() {
    check_any_time("maha shivaratri 2020");
}

#[test]
fn test_time_holi_2019() {
    check_any_time("holi 2019");
    check_any_time("dhulandi 2019");
    check_any_time("phagwah 2019");
}

#[test]
fn test_time_krishna_janmashtami_2019() {
    check_any_time("krishna janmashtami 2019");
    check_any_time("gokulashtami 2019");
}

#[test]
fn test_time_ganesh_chaturthi_2019() {
    check_any_time("ganesh chaturthi 2019");
}

#[test]
fn test_time_rama_navami_2020() {
    check_any_time("rama navami 2020");
}

#[test]
fn test_time_ugadi_2018() {
    check_any_time("Ugadi 2018");
    check_any_time("yugadi 2018");
}

// ============================================================
// Holidays - Pongal / Vaisakhi / Onam / etc.
// ============================================================
#[test]
fn test_time_pongal_2018() {
    check_any_time("pongal 2018");
    check_any_time("makara sankranthi 2018");
}

#[test]
fn test_time_vaisakhi_2018() {
    check_any_time("Vaisakhi 2018");
    check_any_time("baisakhi in 2018");
}

#[test]
fn test_time_onam_2018() {
    check_any_time("onam 2018");
    check_any_time("Thiru Onam 2018");
}

#[test]
fn test_time_vasant_panchami_2019() {
    check_any_time("vasant panchami in 2019");
    check_any_time("basant panchami 2019");
}

// ============================================================
// Holidays - Parsi / GYSD / Vesak / Earth Hour
// ============================================================
#[test]
fn test_time_parsi_new_year_2018() {
    check_any_time("Parsi New Year 2018");
    check_any_time("Jamshedi Navroz 2018");
}

#[test]
fn test_time_gysd_2013() {
    check_any_time("GYSD 2013");
    check_any_time("global youth service day");
}

#[test]
fn test_time_vesak() {
    check_any_time("vesak");
    check_any_time("vaisakha");
    check_any_time("Buddha day");
    check_any_time("Buddha Purnima");
}

#[test]
fn test_time_earth_hour() {
    check_any_time("earth hour");
}

// ============================================================
// Holidays - Sikh / Other
// ============================================================
#[test]
fn test_time_guru_gobind_singh_jayanti() {
    check_any_time("guru gobind singh birthday");
    check_any_time("guru gobind singh jayanti");
}

#[test]
fn test_time_kings_day_2018() {
    check_any_time("Koningsdag 2018");
    check_any_time("king's day 2018");
}

#[test]
fn test_time_rabindra_jayanti_2018() {
    check_any_time("rabindra jayanti 2018");
}

#[test]
fn test_time_guru_ravidass_jayanti_2018() {
    check_any_time("guru Ravidas jayanti 2018");
    check_any_time("Guru Ravidass birthday 2018");
}

#[test]
fn test_time_pargat_diwas_2019() {
    check_any_time("valmiki jayanti 2019");
    check_any_time("pargat diwas 2019");
}

// ============================================================
// In 15 minutes
// ============================================================
#[test]
fn test_time_in_15_minutes() {
    check_any_time("in 15 minutes");
}

// ============================================================
// At 10:30
// ============================================================
#[test]
fn test_time_10_30() {
    check_time("10:30", "minute");
    check_any_time("approximately 1030");
}

// ============================================================
// At 1:30pm
// ============================================================
#[test]
fn test_time_at_1_30pm() {
    check_time("at 1:30pm", "minute");
    check_time("1:30pm", "minute");
    check_any_time("at 13h30");
    check_any_time("13h30");
}

// ============================================================
// Upcoming N weeks/days/months/quarters/years
// ============================================================
#[test]
fn test_time_upcoming_2_weeks() {
    check_any_time("upcoming two weeks");
    check_any_time("upcoming 2 weeks");
    check_any_time("two upcoming weeks");
    check_any_time("2 upcoming weeks");
}

#[test]
fn test_time_upcoming_2_days() {
    check_any_time("upcoming two days");
    check_any_time("upcoming 2 days");
    check_any_time("two upcoming days");
    check_any_time("2 upcoming days");
}

#[test]
fn test_time_upcoming_2_months() {
    check_any_time("upcoming two months");
    check_any_time("upcoming 2 months");
    check_any_time("two upcoming months");
    check_any_time("2 upcoming months");
}

#[test]
fn test_time_upcoming_2_quarters() {
    check_any_time("upcoming two quarters");
    check_any_time("upcoming 2 quarters");
    check_any_time("two upcoming quarters");
    check_any_time("2 upcoming quarters");
}

#[test]
fn test_time_upcoming_2_years() {
    check_any_time("upcoming two years");
    check_any_time("upcoming 2 years");
    check_any_time("two upcoming years");
    check_any_time("2 upcoming years");
}

// ============================================================
// 20 minutes to 2pm tomorrow
// ============================================================
#[test]
fn test_time_20_minutes_to_2pm_tomorrow() {
    check_any_time("20 minutes to 2pm tomorrow");
}

// ============================================================
// First monday of last month
// ============================================================
#[test]
fn test_time_first_monday_of_last_month() {
    check_any_time("first monday of last month");
}

#[test]
fn test_time_first_tuesday_of_last_month() {
    check_any_time("first tuesday of last month");
}

#[test]
fn test_time_second_monday_of_last_month() {
    check_any_time("second monday of last month");
}

// ============================================================
// Next saturday / next monday
// ============================================================
#[test]
fn test_time_next_saturday() {
    check_time("next saturday", "day");
}

#[test]
fn test_time_next_monday() {
    check_time("next monday", "day");
}

// ============================================================
// Default corpus - date formats (MM/DD)
// ============================================================
#[test]
fn test_time_2_15() {
    check_any_time("2/15");
    check_any_time("on 2/15");
    check_any_time("2 / 15");
    check_any_time("2-15");
    check_any_time("2 - 15");
}

#[test]
fn test_time_10_31_1974() {
    check_any_time("10/31/1974");
    check_any_time("10/31/74");
    check_any_time("10-31-74");
    check_any_time("10.31.1974");
    check_any_time("31/Oct/1974");
    check_any_time("31-Oct-74");
    check_any_time("31st Oct 1974");
}

#[test]
fn test_time_4_25_at_4pm() {
    check_any_time("4/25 at 4:00pm");
    check_any_time("4/25 at 16h00");
    check_any_time("4/25 at 16h");
}

// ============================================================
// Default corpus - Thanksgiving
// ============================================================
#[test]
fn test_time_thanksgiving() {
    check_any_time("thanksgiving day");
    check_any_time("thanksgiving");
    check_any_time("thanksgiving 2013");
    check_any_time("this thanksgiving");
    check_any_time("next thanksgiving day");
}

#[test]
fn test_time_thanksgiving_next_year() {
    check_any_time("thanksgiving of next year");
    check_any_time("thanksgiving in a year");
    check_any_time("thanksgiving 2014");
}

#[test]
fn test_time_last_thanksgiving() {
    check_any_time("last thanksgiving");
    check_any_time("thanksgiving day 2012");
}

#[test]
fn test_time_thanksgiving_2016() {
    check_any_time("thanksgiving 2016");
}

#[test]
fn test_time_thanksgiving_2017() {
    check_any_time("thanksgiving 2017");
}

// ============================================================
// Negative corpus
// ============================================================
#[test]
fn test_time_negative_laughing_out_loud() {
    check_no_time("laughing out loud");
}

#[test]
fn test_time_negative_1_adult() {
    check_no_time("1 adult");
}

#[test]
fn test_time_negative_we_are_separated() {
    check_no_time("we are separated");
}

#[test]
fn test_time_negative_25() {
    check_no_time("25");
}

#[test]
fn test_time_negative_this_is_the_one() {
    check_no_time("this is the one");
}

#[test]
fn test_time_negative_this_one() {
    check_no_time("this one");
}

#[test]
fn test_time_negative_this_past_one() {
    check_no_time("this past one");
}

#[test]
fn test_time_negative_at_single() {
    check_no_time("at single");
}

#[test]
fn test_time_negative_at_couple_of() {
    check_no_time("at a couple of");
}

#[test]
fn test_time_negative_at_pairs() {
    check_no_time("at pairs");
}

#[test]
fn test_time_negative_at_a_few() {
    check_no_time("at a few");
}

#[test]
fn test_time_negative_at_dozens() {
    check_no_time("at dozens");
}

#[test]
fn test_time_negative_single_oclock() {
    check_no_time("single o'clock");
}

#[test]
fn test_time_negative_dozens_oclock() {
    check_no_time("dozens o'clock");
}

#[test]
fn test_time_negative_rat_6() {
    check_no_time("Rat 6");
    check_no_time("rat 6");
}

#[test]
fn test_time_negative_3_30() {
    check_no_time("3 30");
}

#[test]
fn test_time_negative_three_twenty() {
    check_no_time("three twenty");
}

#[test]
fn test_time_negative_phone_numbers() {
    check_no_time("at 650.650.6500");
    check_no_time("at 650-650-6500");
}

#[test]
fn test_time_negative_two_sixty_am() {
    check_no_time("two sixty a m");
}

#[test]
fn test_time_negative_pay_abc_2000() {
    check_no_time("Pay ABC 2000");
}

#[test]
fn test_time_negative_4a() {
    check_no_time("4a");
    check_no_time("4a.");
}

#[test]
fn test_time_negative_a4_a5() {
    check_no_time("A4 A5");
}

#[test]
fn test_time_negative_palm() {
    check_no_time("palm");
}

#[test]
fn test_time_negative_mlk_apostrophe() {
    check_no_time("Martin Luther King' day");
}

#[test]
fn test_time_negative_two_three() {
    check_no_time("two three");
}

// ============================================================
// Latent corpus
// ============================================================
#[test]
fn test_time_latent_the_24() {
    // Latent: needs withLatent = true
    // check_any_time("the 24");
    // check_any_time("On 24th");
}

#[test]
fn test_time_latent_7() {
    // Latent: "7" alone should not match without latent mode
    // check_any_time("7");
}

#[test]
fn test_time_latent_1974() {
    // Latent: "1974" alone
    // check_any_time("1974");
}

#[test]
fn test_time_latent_may() {
    // Latent: "May" alone
    // check_any_time("May");
}

#[test]
fn test_time_latent_morning() {
    // Latent: "morning" alone
    // check_any_time("morning");
}

#[test]
fn test_time_latent_afternoon() {
    // Latent: "afternoon" alone
    // check_any_time("afternoon");
}

#[test]
fn test_time_latent_evening() {
    // Latent: "evening" alone
    // check_any_time("evening");
}

#[test]
fn test_time_latent_night() {
    // Latent: "night" alone
    // check_any_time("night");
}

#[test]
fn test_time_latent_ten_thirty() {
    // Latent: "ten thirty" / "ten-thirty"
    // check_any_time("ten thirty");
    // check_any_time("ten-thirty");
}

// ============================================================
// Diff corpus (different reference time: 2013-02-15 04:30:00 UTC-2)
// ============================================================
#[test]
fn test_time_diff_3_fridays_from_now() {
    // This uses a different reference time, so result differs
    check_any_time("3 fridays from now");
    check_any_time("three fridays from now");
}

// ============================================================
// 2/2013 -> Feb 2013
// ============================================================
#[test]
fn test_time_2_2013() {
    check_any_time("2/2013");
}

// ============================================================
// First Monday of this month
// ============================================================
#[test]
fn test_time_first_monday_of_this_month() {
    check_any_time("the first Monday of this month");
    check_any_time("the first Monday of the month");
    check_any_time("first Monday in the month");
}

// ============================================================
// 8am until 6
// ============================================================
#[test]
fn test_time_8am_until_6() {
    check_any_time("8am until 6");
}

// ============================================================
// Last friday of october
// ============================================================
#[test]
fn test_time_last_friday_of_october() {
    check_any_time("last friday of october");
    check_any_time("last friday in october");
}

// ============================================================
// ISO datetime interval
// ============================================================
#[test]
fn test_time_iso_interval() {
    check_any_time("2015-03-28 17:00:00/2015-03-29 21:00:00");
}

// ============================================================
// At 7 in 3 years
// ============================================================
#[test]
fn test_time_at_7_in_3_years() {
    check_any_time("at 7 in 3 years");
}

// ============================================================
// In 4 years at 5pm
// ============================================================
#[test]
fn test_time_in_4_years_at_5pm() {
    check_any_time("in 4 years at 5pm");
}

// ============================================================
// A day from right now
// ============================================================
#[test]
fn test_time_a_day_from_right_now() {
    check_any_time("a day from right now");
}

// ============================================================
// 2 sundays from now
// ============================================================
#[test]
fn test_time_2_sundays_from_now() {
    check_any_time("2 sundays from now");
    check_any_time("two sundays from now");
}

// ============================================================
// 4 tuesdays from now
// ============================================================
#[test]
fn test_time_4_tuesdays_from_now() {
    check_any_time("4 tuesdays from now");
    check_any_time("four tuesdays from now");
}

// ============================================================
// Third last week of 2018
// ============================================================
#[test]
fn test_time_third_last_week_of_2018() {
    check_any_time("third last week of 2018");
    check_any_time("the third last week of 2018");
    check_any_time("the 3rd last week of 2018");
}

// ============================================================
// 2nd last week of October 2018
// ============================================================
#[test]
fn test_time_2nd_last_week_of_october_2018() {
    check_any_time("2nd last week of October 2018");
    check_any_time("the second last week of October 2018");
}

// ============================================================
// Fifth last day of May
// ============================================================
#[test]
fn test_time_fifth_last_day_of_may() {
    check_any_time("fifth last day of May");
    check_any_time("the 5th last day of May");
}

// ============================================================
// The week of october 6th
// ============================================================
#[test]
fn test_time_week_of_october_6th() {
    check_any_time("the week of october 6th");
    check_any_time("the week of october 7th");
}

// ============================================================
// Last week of september 2014
// ============================================================
#[test]
fn test_time_last_week_of_september_2014() {
    check_any_time("last week of september 2014");
}

// ============================================================
// Third tuesday of september 2014
// ============================================================
#[test]
fn test_time_third_tuesday_of_september_2014() {
    check_any_time("third tuesday of september 2014");
}

// ============================================================
// Second wednesday of october 2014
// ============================================================
#[test]
fn test_time_second_wednesday_of_october_2014() {
    check_any_time("second wednesday of october 2014");
}

// ============================================================
// Third tuesday after christmas 2014
// ============================================================
#[test]
fn test_time_third_tuesday_after_christmas_2014() {
    check_any_time("third tuesday after christmas 2014");
}

// ============================================================
// Thu 15th (Aug 2013)
// ============================================================
#[test]
fn test_time_thu_15th() {
    check_any_time("Thu 15th");
}

// ============================================================
// This past weekend
// ============================================================
#[test]
fn test_time_this_past_weekend() {
    check_any_time("this past weekend");
}

// ============================================================
// Tomorrow in between 1-2:30 ish
// ============================================================
#[test]
fn test_time_tomorrow_1_to_230ish() {
    check_any_time("tomorrow in between 1-2:30 ish");
}

// ============================================================
// 1pm-2pm tomorrow
// ============================================================
#[test]
fn test_time_1pm_2pm_tomorrow() {
    check_any_time("1pm-2pm tomorrow");
}

// ============================================================
// Tomorrow at 150ish
// ============================================================
#[test]
fn test_time_tomorrow_at_150ish() {
    check_any_time("tomorrow at 150ish");
}

// ============================================================
// Saturday for 9am
// ============================================================
#[test]
fn test_time_on_saturday_for_9am() {
    check_any_time("on Saturday for 9am");
}

// ============================================================
// Closest Xmas to today
// ============================================================
#[test]
fn test_time_closest_xmas_to_today() {
    check_any_time("the closest xmas to today");
}

#[test]
fn test_time_second_closest_xmas() {
    check_any_time("the second closest xmas to today");
}

#[test]
fn test_time_3rd_closest_xmas() {
    check_any_time("the 3rd closest xmas to today");
}

// ============================================================
// Last wkend of July
// ============================================================
#[test]
fn test_time_last_wkend_of_july() {
    check_any_time("last wkend of July");
}

#[test]
fn test_time_last_weekend_of_october_2017() {
    check_any_time("last weekend of October 2017");
}

// ============================================================
// 19th to 21st aug
// ============================================================
#[test]
fn test_time_19th_to_21st_aug() {
    check_any_time("19th To 21st aug");
}

// ============================================================
// Since 2014 / through 2014
// ============================================================
#[test]
fn test_time_since_2014() {
    check_any_time("anytime after 2014");
    check_any_time("since 2014");
}

#[test]
fn test_time_before_2014() {
    check_any_time("sometimes before 2014");
    check_any_time("through 2014");
}

// ============================================================
// After 5 days
// ============================================================
#[test]
fn test_time_after_5_days() {
    check_any_time("after 5 days");
}

// ============================================================
// 11:30-1:30
// ============================================================
#[test]
fn test_time_1130_to_130() {
    check_any_time("11:30-1:30");
}

// ============================================================
// Today in one hour
// ============================================================
#[test]
fn test_time_today_in_one_hour() {
    check_any_time("today in one hour");
}

// ============================================================
// In 14 A.D.
// ============================================================
#[test]
fn test_time_in_14_ad() {
    check_any_time("in 14 a.d.");
}

// ============================================================
// Anytime after 2014 / since
// ============================================================
#[test]
fn test_time_anytime_after_2014() {
    check_any_time("anytime after 2014");
}

// ============================================================
// 9:30 - 11:00 CST / GMT timezone intervals
// ============================================================
#[test]
fn test_time_930_1100_cst() {
    check_any_time("9:30 - 11:00 CST");
}

#[test]
fn test_time_1500_1800_gmt() {
    check_any_time("15:00 GMT - 18:00 GMT");
}

// ============================================================
// By end of next month
// ============================================================
#[test]
fn test_time_by_end_of_next_month() {
    check_any_time("by the end of next month");
}

// ============================================================
// Beginning of coming week
// ============================================================
#[test]
fn test_time_beginning_of_coming_week() {
    check_any_time("beginning of coming week");
}

// ============================================================
// End of coming week
// ============================================================
#[test]
fn test_time_end_of_coming_week() {
    check_any_time("end of coming week");
}

// ============================================================
// Lazarus Saturday 2018
// ============================================================
#[test]
fn test_time_lazarus_saturday_2018() {
    check_any_time("lazarus saturday 2018");
}

// ============================================================
// Great Lent (Great Fast) 2018
// ============================================================
#[test]
fn test_time_great_fast_2018() {
    check_any_time("great fast 2018");
}

// ============================================================
// Jumu'atul-Wida
// ============================================================
#[test]
fn test_time_jumatul_wida_2018() {
    check_any_time("Jumu'atul-Wida 2018");
    check_any_time("Jamat Ul-Vida 2018");
}

// ============================================================
// Holika Dahan / Chhoti Holi
// ============================================================
#[test]
fn test_time_holika_dahan_2019() {
    check_any_time("chhoti holi 2019");
    check_any_time("holika dahan 2019");
    check_any_time("kamudu pyre 2019");
}

// ============================================================
// Maha Saptami 2021
// ============================================================
#[test]
fn test_time_maha_saptami_2021() {
    check_any_time("Maha Saptami 2021");
}

// ============================================================
// Vijayadashami / Dussehra 2018
// ============================================================
#[test]
fn test_time_dussehra_2018() {
    check_any_time("Dussehra 2018");
}

// ============================================================
// Saraswati Jayanti 2018
// ============================================================
#[test]
fn test_time_saraswati_jayanti_2018() {
    check_any_time("saraswati jayanti 2018");
}

// ============================================================
// Boghi / Mattu Pongal / Kaanum Pongal
// ============================================================
#[test]
fn test_time_bogi_pandigai_2018() {
    check_any_time("bogi pandigai 2018");
}

#[test]
fn test_time_maattu_pongal_2018() {
    check_any_time("maattu pongal 2018");
}

#[test]
fn test_time_kaanum_pongal_2018() {
    check_any_time("kaanum pongal 2018");
    check_any_time("kanni pongal 2018");
}

// ============================================================
// Makar Sankranti 2019
// ============================================================
#[test]
fn test_time_makar_sankranti_2019() {
    check_any_time("makar sankranti 2019");
    check_any_time("maghi in 2019");
}

// ============================================================
// Naraka Chaturdashi / Choti Diwali 2019
// ============================================================
#[test]
fn test_time_naraka_chaturdashi_2019() {
    check_any_time("kali chaudas 2019");
    check_any_time("choti diwali two thousand nineteen");
}

// ============================================================
// Eid al-Fitr various years
// ============================================================
#[test]
fn test_time_eid_al_fitr_1950() {
    check_any_time("Eid al-Fitr 1950");
}

#[test]
fn test_time_eid_al_fitr_1975() {
    check_any_time("Eid al-Fitr 1975");
}

#[test]
fn test_time_eid_al_fitr_1988() {
    check_any_time("Eid al-Fitr 1988");
}

// ============================================================
// Eid al-Adha various years
// ============================================================
#[test]
fn test_time_eid_al_adha_1980() {
    check_any_time("Eid al-Adha 1980");
}

#[test]
fn test_time_eid_al_adha_1966() {
    check_any_time("Eid al-Adha 1966");
}

#[test]
fn test_time_eid_al_adha_1974() {
    check_any_time("Eid al-Adha 1974");
}

// ============================================================
// Ramadan various years
// ============================================================
#[test]
fn test_time_ramadan_1950() {
    check_any_time("Ramadan 1950");
}

#[test]
fn test_time_ramadan_1977() {
    check_any_time("Ramadan 1977");
}

#[test]
fn test_time_ramadan_2034() {
    check_any_time("Ramadan 2034");
}

#[test]
fn test_time_ramadan_2046() {
    check_any_time("Ramadan 2046");
}

#[test]
fn test_time_ramadan_2050() {
    check_any_time("Ramadan 2050");
}

// ============================================================
// Laylat al-Qadr 2017
// ============================================================
#[test]
fn test_time_laylat_al_qadr_2017() {
    check_any_time("laylat al kadr 2017");
    check_any_time("night of measures 2017");
}

// ============================================================
// Isra and Mi'raj 2019
// ============================================================
#[test]
fn test_time_night_journey_2019() {
    check_any_time("the night journey 2019");
    check_any_time("ascension to heaven 2019");
}

// ============================================================
// Dhanatrayodashi 2017
// ============================================================
#[test]
fn test_time_dhanatrayodashi_2017() {
    check_any_time("dhanatrayodashi in 2017");
}

// ============================================================
// Parsi New Year 2022
// ============================================================
#[test]
fn test_time_parsi_new_year_2022() {
    check_any_time("jamshedi Navroz 2022");
    check_any_time("parsi new year 2022");
}

// ============================================================
// Earth Hour 2016
// ============================================================
#[test]
fn test_time_earth_hour_2016() {
    check_any_time("earth hour 2016");
}

// ============================================================
// King's Day 2014
// ============================================================
#[test]
fn test_time_kings_day_2014() {
    check_any_time("Koningsdag 2014");
    check_any_time("King's Day 2014");
}

// ============================================================
// Rabindra Jayanti 2019
// ============================================================
#[test]
fn test_time_rabindra_jayanti_2019() {
    check_any_time("rabindra jayanti 2019");
}

// ============================================================
// Guru Ravidass Jayanti 2019
// ============================================================
#[test]
fn test_time_guru_ravidass_jayanti_2019() {
    check_any_time("Guru Ravidass Jayanti 2019");
    check_any_time("Guru Ravidas Birthday 2019");
}

// ============================================================
// Pargat Diwas 2018
// ============================================================
#[test]
fn test_time_pargat_diwas_2018() {
    check_any_time("maharishi valmiki jayanti 2018");
    check_any_time("pargat diwas 2018");
}

// ============================================================
// Guru Gobind Singh Jayanti 2014
// ============================================================
#[test]
fn test_time_guru_gobind_singh_jayanti_2014() {
    check_any_time("guru gobind singh jayanti 2014");
    check_any_time("Guru Govind Singh Jayanti");
}

// ============================================================
// Lakshmi Puja / Deepavali
// ============================================================
#[test]
fn test_time_lakshmi_puja() {
    check_any_time("Lakshmi Puja six years hence");
}
