use chrono::{TimeZone, Utc};
use duckling::{parse, Context, DimensionKind, DimensionValue, Lang, Locale, Options};

fn context_en() -> Context {
    Context {
        reference_time: Utc.with_ymd_and_hms(2013, 2, 12, 4, 30, 0).unwrap(),
        locale: Locale::new(Lang::EN, None),
        timezone_offset_minutes: -120,
    }
}

fn parse_no_panic(text: &str, dims: &[DimensionKind]) -> Vec<duckling::Entity> {
    let locale = Locale::new(Lang::EN, None);
    let ctx = context_en();
    let options = Options::default();
    std::panic::catch_unwind(|| parse(text, &locale, dims, &ctx, &options))
        .unwrap_or_else(|_| panic!("parse panicked for input: {text:?}"))
}

#[test]
fn test_extreme_inputs_do_not_panic() {
    let cases = [
        "9999999999",
        "-9999999999",
        "temperature is 9999999999 c",
        "temperature is -9999999999 f",
        "January 1, 999999",
        "January 1, -999999",
        "year 9999999999",
        "in 999999999 months",
        "in 9999999999999999 days",
        "9999999999 years from now",
        "Tuesday, March 11, 2025 at 8:15 PM\n(773) 348-8886\nlocation: 2300 N. Lincoln Park West  Chicago, IL United States 60614",
    ];
    let dims = [
        DimensionKind::Numeral,
        DimensionKind::Temperature,
        DimensionKind::Time,
    ];
    for text in cases {
        let _ = parse_no_panic(text, &dims);
    }
}

#[test]
fn test_extreme_inputs_do_not_panic_all_dimensions() {
    let cases = [
        "999999999999999999999999999999999999999",
        "-999999999999999999999999999999999999999",
        "in 9999999999999999 days",
        "9999999999 years from now",
        "temperature is 1e309 c",
        "pay 999999999999999999999 dollars",
        "(999) 999-999999999999999999",
        "https://example.com/999999999999999999999999999",
    ];
    let dims = [
        DimensionKind::Numeral,
        DimensionKind::Ordinal,
        DimensionKind::Temperature,
        DimensionKind::Distance,
        DimensionKind::Volume,
        DimensionKind::Quantity,
        DimensionKind::AmountOfMoney,
        DimensionKind::Email,
        DimensionKind::PhoneNumber,
        DimensionKind::Url,
        DimensionKind::CreditCardNumber,
        DimensionKind::TimeGrain,
        DimensionKind::Duration,
        DimensionKind::Time,
    ];
    for text in cases {
        let _ = parse_no_panic(text, &dims);
    }
}

#[test]
fn test_overflowing_time_inputs_return_no_time_entities() {
    let cases = [
        "in 9999999999999999 days",
    ];
    for text in cases {
        let entities = parse_no_panic(text, &[DimensionKind::Time]);
        let has_time = entities
            .iter()
            .any(|e| matches!(e.value, DimensionValue::Time(_)));
        assert!(!has_time, "expected no Time entity for {text:?}, got {entities:?}");
    }
}
