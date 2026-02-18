//! Profiling harness — exercise all major parsing paths repeatedly.
//! Run with: cargo flamegraph --example profile

use duckling::{parse, Context, DimensionKind, Lang, Locale, Options};

fn main() {
    let locale = Locale::new(Lang::EN, None);
    let context = Context::default();
    let options = Options::default();
    let options_latent = Options { with_latent: true };

    // Representative inputs for each dimension
    let cases: Vec<(&[DimensionKind], &[&str])> = vec![
        (
            &[DimensionKind::Time],
            &[
                "today",
                "tomorrow at 3pm",
                "next Tuesday",
                "in 3 days",
                "March 15, 2024",
                "the day after tomorrow",
                "last week",
                "this Friday at noon",
                "10/31/1974",
                "morning",
                "tonight",
                "3pm",
                "at 8 in the evening",
                "within 2 weeks",
                "between 3 and 5 pm",
                "from 9am to 5pm",
                "the 3rd of March",
                "next month",
                "last Christmas",
                "Monday morning",
                "February 12th at 4:30",
            ],
        ),
        (
            &[DimensionKind::Numeral],
            &[
                "forty-two",
                "3",
                "one hundred",
                "1.5 million",
                "a dozen",
                "twenty one",
                "100K",
                "three thousand",
                "0.5",
                "a couple",
            ],
        ),
        (
            &[DimensionKind::Temperature],
            &["80 degrees fahrenheit", "3 degrees", "100°F", "-5 celsius"],
        ),
        (
            &[DimensionKind::AmountOfMoney],
            &["$42.50", "ten dollars", "a grand", "€100", "$3.50"],
        ),
        (
            &[DimensionKind::Duration],
            &[
                "3 days",
                "an hour",
                "two weeks",
                "45 minutes",
                "a year and a half",
            ],
        ),
        (
            &[DimensionKind::Ordinal],
            &["the 3rd", "first", "31st", "twenty-second"],
        ),
        (
            &[DimensionKind::Distance],
            &["5 miles", "100 meters", "3 km"],
        ),
        (
            &[DimensionKind::Volume],
            &["2 gallons", "500 ml", "a liter"],
        ),
        (
            &[DimensionKind::Email],
            &["user@example.com", "test.user@domain.co.uk"],
        ),
        (
            &[DimensionKind::Url],
            &["https://www.example.com/path", "http://foo.bar"],
        ),
        // All dimensions at once (worst case)
        (
            &[],
            &[
                "tomorrow at 3pm for $50",
                "I need 3 degrees celsius by next Tuesday",
                "the meeting is at 10am and costs $200",
            ],
        ),
    ];

    let iterations = 100;

    for _ in 0..iterations {
        for (dims, inputs) in &cases {
            for input in *inputs {
                let _ = parse(input, &locale, dims, &context, &options);
            }
        }
        // A few latent parses too
        for input in &["morning", "evening", "noon", "3", "Tuesday"] {
            let _ = parse(
                input,
                &locale,
                &[DimensionKind::Time],
                &context,
                &options_latent,
            );
        }
    }
}
