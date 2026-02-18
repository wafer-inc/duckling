use chrono::{NaiveDate, TimeZone, Utc};
use duckling::{
    parse, Context, DimensionKind, DimensionValue, Entity, Grain, Lang, Locale, MeasurementValue,
    Options, TimePoint, TimeValue,
};

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
    let cases = ["in 9999999999999999 days"];
    for text in cases {
        let entities = parse_no_panic(text, &[DimensionKind::Time]);
        let has_time = entities
            .iter()
            .any(|e| matches!(e.value, DimensionValue::Time(_)));
        assert!(
            !has_time,
            "expected no Time entity for {text:?}, got {entities:?}"
        );
    }
}

#[test]
fn test_real_world_event_listing_exact_entities() {
    let text = "created_by: chris@wafer.systems\n\
        description: Mon Ami Gabi\n\
        Tuesday, March 11, 2025 at 8:15 PM\n\
        Honoring classic French cuisine, Mon Ami Gabi is a traditional French bistro \
        located in the heart of Lincoln Park. Our menu features simple French food \
        including Steak Frites, fresh seafood, exquisite bites, and French rolling \
        wine carts. Valet is located at the corner of Lincoln Park West and Belden \
        for $17 cash or Zelle. During the holiday season, for all parties of 9 or \
        more, we ask our guests to call the restaurant directly to book a reservation.\n\
        \n\
        Did you know that you can use Lettuce gift cards for carryout and delivery? \
        Just place your order through Lettuce website, the LettuceEats app or any \
        Lettuce restaurant website.\n\
        (773) 348-8886\n\
        \n\
        https://www.google.com/maps/search/?api=1&query=41.924216%2C-87.6370405\n\
        \n\
        location: 2300 N. Lincoln Park West  Chicago, IL United States 60614\n\
        name: Mon Ami Gabi";

    let locale = Locale::new(Lang::EN, None);
    let ctx = Context {
        reference_time: Utc.with_ymd_and_hms(2025, 3, 5, 12, 0, 0).unwrap(),
        locale: Locale::new(Lang::EN, None),
        timezone_offset_minutes: -360,
    };
    let options = Options::default();
    let dims = [
        DimensionKind::Time,
        DimensionKind::Email,
        DimensionKind::PhoneNumber,
        DimensionKind::Url,
        DimensionKind::AmountOfMoney,
        DimensionKind::Numeral,
        DimensionKind::Ordinal,
    ];

    let entities = parse(text, &locale, &dims, &ctx, &options);

    fn e(
        body: &str,
        start: usize,
        end: usize,
        value: DimensionValue,
        latent: Option<bool>,
    ) -> Entity {
        Entity {
            body: body.to_string(),
            start,
            end,
            value,
            latent,
        }
    }
    fn ndt(y: i32, m: u32, d: u32, h: u32, mi: u32, s: u32) -> chrono::NaiveDateTime {
        NaiveDate::from_ymd_opt(y, m, d)
            .unwrap()
            .and_hms_opt(h, mi, s)
            .unwrap()
    }
    fn money(value: f64, unit: &str) -> DimensionValue {
        DimensionValue::AmountOfMoney(MeasurementValue::Value {
            value,
            unit: unit.to_string(),
        })
    }
    fn num(v: f64) -> DimensionValue {
        DimensionValue::Numeral(v)
    }
    #[allow(clippy::too_many_arguments)]
    fn time_single(
        y: i32,
        m: u32,
        d: u32,
        h: u32,
        mi: u32,
        s: u32,
        grain: Grain,
        extra_values: &[(i32, u32, u32, u32, u32, u32)],
    ) -> DimensionValue {
        let point = TimePoint::Naive {
            value: ndt(y, m, d, h, mi, s),
            grain,
        };
        let mut values = vec![point.clone()];
        for &(ey, em, ed, eh, emi, es) in extra_values {
            values.push(TimePoint::Naive {
                value: ndt(ey, em, ed, eh, emi, es),
                grain,
            });
        }
        DimensionValue::Time(TimeValue::Single {
            value: point,
            values,
            holiday: None,
        })
    }
    let l = Some(false);

    let expected = vec![
        e("cr", 0, 2, num(10_000_000.0), l),
        e(
            "chris@wafer.systems",
            12,
            31,
            DimensionValue::Email("chris@wafer.systems".into()),
            l,
        ),
        e("cr", 35, 37, num(10_000_000.0), l),
        e(
            "Mon",
            45,
            48,
            time_single(
                2025,
                3,
                10,
                0,
                0,
                0,
                Grain::Day,
                &[(2025, 3, 17, 0, 0, 0), (2025, 3, 24, 0, 0, 0)],
            ),
            l,
        ),
        e(
            "Tuesday, March 11, 2025 at 8:15 PM",
            58,
            92,
            time_single(2025, 3, 11, 20, 15, 0, Grain::Minute, &[]),
            l,
        ),
        e("cl", 102, 104, money(100_000.0, "cent"), l),
        e(
            "Mon",
            126,
            129,
            time_single(
                2025,
                3,
                10,
                0,
                0,
                0,
                Grain::Day,
                &[(2025, 3, 17, 0, 0, 0), (2025, 3, 24, 0, 0, 0)],
            ),
            l,
        ),
        e("l", 154, 155, num(100_000.0), l),
        e("l", 170, 171, num(100_000.0), l),
        e("L", 194, 195, num(100_000.0), l),
        e("l", 199, 200, num(100_000.0), l),
        e("l", 230, 231, num(100_000.0), l),
        e("cl", 247, 249, money(100_000.0, "cent"), l),
        e("l", 314, 315, num(100_000.0), l),
        e("l", 315, 316, num(100_000.0), l),
        e("alet", 333, 337, money(1.0, "EGP"), l),
        e("l", 341, 342, num(100_000.0), l),
        e("L", 366, 367, num(100_000.0), l),
        e("l", 371, 372, num(100_000.0), l),
        e("l", 390, 391, num(100_000.0), l),
        e("$17", 399, 402, money(17.0, "USD"), l),
        e("17 c", 400, 404, money(17.0, "cent"), l),
        e("l", 413, 414, num(100_000.0), l),
        e("l", 414, 415, num(100_000.0), l),
        e("l", 431, 432, num(100_000.0), l),
        e("l", 450, 451, num(100_000.0), l),
        e("l", 451, 452, num(100_000.0), l),
        e("9", 464, 465, num(9.0), l),
        e("l", 498, 499, num(100_000.0), l),
        e("l", 499, 500, num(100_000.0), l),
        e("l", 522, 523, num(100_000.0), l),
        e("L", 579, 580, num(100_000.0), l),
        e("l", 617, 618, num(100_000.0), l),
        e("lac", 631, 634, num(100_000.0), l),
        e("L", 655, 656, num(100_000.0), l),
        e("L", 676, 677, num(100_000.0), l),
        e("L", 699, 700, num(100_000.0), l),
        e(
            "(773) 348-8886",
            727,
            741,
            DimensionValue::PhoneNumber("7733488886".into()),
            l,
        ),
        e(
            "https://www.google.com/maps/search/?api=1&query=41.924216%2C-87.6370405",
            743,
            814,
            DimensionValue::Url {
                value: "https://www.google.com/maps/search/?api=1&query=41.924216%2C-87.6370405"
                    .into(),
                domain: "google.com".into(),
            },
            l,
        ),
        e("-87.6370405\n\nl", 803, 817, num(-8_763_704.049_999_999), l),
        e("2300", 826, 830, num(2300.0), l),
        e("L", 834, 835, num(100_000.0), l),
        e("l", 839, 840, num(100_000.0), l),
        e("L", 863, 864, num(100_000.0), l),
        e("60614", 879, 884, num(60614.0), l),
        e(
            "Mon",
            891,
            894,
            time_single(
                2025,
                3,
                10,
                0,
                0,
                0,
                Grain::Day,
                &[(2025, 3, 17, 0, 0, 0), (2025, 3, 24, 0, 0, 0)],
            ),
            l,
        ),
    ];

    assert_eq!(entities, expected);
}

#[test]
fn test_email_like_marketing_text_no_time_entities_1() {
    let text = "body: Dear Andre Popovitch, We'd love to learn more about why you chose AgelessRx as your partner in longevity. Your feedback may help inspire others to add healthy years to their life. How did we do? ○\nfrom: <noreply.invitations@trustpilotmail.com>\nsubject: Inspire others to add healthy years to their life ⭐⭐⭐⭐⭐";

    let locale = Locale::new(Lang::EN, None);
    let ctx = Context {
        reference_time: Utc.with_ymd_and_hms(2025, 3, 5, 12, 0, 0).unwrap(),
        locale: Locale::new(Lang::EN, None),
        timezone_offset_minutes: -360,
    };
    let options = Options::default();
    let dims = [
        DimensionKind::Time,
        DimensionKind::Email,
        DimensionKind::PhoneNumber,
        DimensionKind::Url,
        DimensionKind::AmountOfMoney,
        DimensionKind::Numeral,
        DimensionKind::Ordinal,
    ];
    let entities = parse(text, &locale, &dims, &ctx, &options);
    let has_time = entities
        .iter()
        .any(|e| matches!(e.value, DimensionValue::Time(_)));
    assert!(
        !has_time,
        "expected no Time entity for {:?}, got {:?}",
        text, entities
    );
}

#[test]
fn test_email_like_marketing_text_no_time_entities_2() {
    let text = "body: Flowers all season long, from $24.95\nfrom: Fast-Growing-Trees.com <plantexperts@fast-growing-trees.com>\nsubject: 💌 A gift for you";

    let locale = Locale::new(Lang::EN, None);
    let ctx = Context {
        reference_time: Utc.with_ymd_and_hms(2025, 3, 5, 12, 0, 0).unwrap(),
        locale: Locale::new(Lang::EN, None),
        timezone_offset_minutes: -360,
    };
    let options = Options::default();
    let dims = [
        DimensionKind::Time,
        DimensionKind::Email,
        DimensionKind::PhoneNumber,
        DimensionKind::Url,
        DimensionKind::AmountOfMoney,
        DimensionKind::Numeral,
        DimensionKind::Ordinal,
    ];
    let entities = parse(text, &locale, &dims, &ctx, &options);
    let has_time = entities
        .iter()
        .any(|e| matches!(e.value, DimensionValue::Time(_)));
    assert!(
        !has_time,
        "expected no Time entity for {:?}, got {:?}",
        text, entities
    );
}

#[test]
fn test_iso_date_in_sentence_() {
    let text = "On 2018-04-01 we met.";

    let locale = Locale::new(Lang::EN, None);
    let ctx = Context {
        reference_time: Utc.with_ymd_and_hms(2025, 3, 5, 12, 0, 0).unwrap(),
        locale: Locale::new(Lang::EN, None),
        timezone_offset_minutes: -360,
    };
    let options = Options::default();
    let dims = [
        DimensionKind::Time,
        DimensionKind::Email,
        DimensionKind::PhoneNumber,
        DimensionKind::Url,
        DimensionKind::AmountOfMoney,
        DimensionKind::Numeral,
        DimensionKind::Ordinal,
    ];

    let entities = parse(text, &locale, &dims, &ctx, &options);

    let expected = vec![Entity {
        body: "On 2018-04-01".to_string(),
        start: 0,
        end: 13,
        value: {
            let point = TimePoint::Naive {
                value: NaiveDate::from_ymd_opt(2018, 4, 1)
                    .unwrap()
                    .and_hms_opt(0, 0, 0)
                    .unwrap(),
                grain: Grain::Day,
            };
            DimensionValue::Time(TimeValue::Single {
                value: point.clone(),
                values: vec![point],
                holiday: None,
            })
        },
        latent: Some(false),
    }];

    assert_eq!(entities, expected);
}
