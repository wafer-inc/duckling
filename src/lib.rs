pub mod dimensions;
pub mod document;
pub mod engine;
pub mod lang;
pub mod locale;
pub mod pattern;
pub mod ranking;
pub mod resolve;
pub mod stash;
pub mod testing;
pub mod types;

// Re-exports for convenience
pub use locale::{Lang, Region, Locale};
pub use resolve::{Context, Options};
pub use types::{DimensionKind, Entity, ResolvedValue};

/// Parse natural language text and return structured entities.
///
/// # Arguments
/// * `text` - The input text to parse
/// * `locale` - The locale (language + optional region)
/// * `dims` - Which dimensions to extract (empty = all)
/// * `context` - Reference time and locale context
/// * `options` - Parsing options (e.g., whether to include latent matches)
///
/// # Example
/// ```
/// use duckling::{parse, Locale, Lang, Context, Options, DimensionKind};
///
/// let context = Context::default();
/// let options = Options::default();
/// let locale = Locale::new(Lang::EN, None);
///
/// let entities = parse("I need 3 degrees celsius", &locale, &[DimensionKind::Temperature], &context, &options);
/// assert!(!entities.is_empty());
/// ```
pub fn parse(
    text: &str,
    locale: &Locale,
    dims: &[DimensionKind],
    context: &Context,
    options: &Options,
) -> Vec<Entity> {
    let rules = lang::rules_for(locale.lang, dims);
    let mut entities = engine::parse_and_resolve(text, rules, context, options, dims);
    ranking::rank(&mut entities);
    ranking::remove_overlapping(entities)
}

/// Convenience function to parse text with default settings for English.
pub fn parse_en(text: &str, dims: &[DimensionKind]) -> Vec<Entity> {
    let locale = Locale::new(Lang::EN, None);
    let context = Context::default();
    let options = Options::default();
    parse(text, &locale, dims, &context, &options)
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_parse_numeral() {
        let entities = parse_en("thirty three", &[DimensionKind::Numeral]);
        let found = entities.iter().any(|e| {
            e.dim == "number"
                && e.value
                    .value
                    .get("value")
                    .and_then(|v| v.as_f64())
                    .map(|v| (v - 33.0).abs() < 0.01)
                    .unwrap_or(false)
        });
        assert!(found, "Expected 33, got: {:?}", entities);
    }

    #[test]
    fn test_parse_100k() {
        let entities = parse_en("100K", &[DimensionKind::Numeral]);
        let found = entities.iter().any(|e| {
            e.dim == "number"
                && e.value
                    .value
                    .get("value")
                    .and_then(|v| v.as_f64())
                    .map(|v| (v - 100_000.0).abs() < 0.01)
                    .unwrap_or(false)
        });
        assert!(found, "Expected 100000, got: {:?}", entities);
    }

    #[test]
    fn test_parse_temperature() {
        let entities = parse_en("80 degrees fahrenheit", &[DimensionKind::Temperature]);
        let found = entities.iter().any(|e| {
            e.dim == "temperature"
                && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(80.0)
                && e.value.value.get("unit").and_then(|v| v.as_str()) == Some("fahrenheit")
        });
        assert!(found, "Expected 80F, got: {:?}", entities);
    }

    #[test]
    fn test_parse_email() {
        let entities = parse_en("user@example.com", &[DimensionKind::Email]);
        let found = entities.iter().any(|e| {
            e.dim == "email"
                && e.value.value.get("value").and_then(|v| v.as_str())
                    == Some("user@example.com")
        });
        assert!(found, "Expected email, got: {:?}", entities);
    }

    #[test]
    fn test_parse_mixed_numeral_and_temperature() {
        let entities = parse_en(
            "it's 3 degrees outside",
            &[DimensionKind::Numeral, DimensionKind::Temperature],
        );

        let has_numeral = entities.iter().any(|e| {
            e.dim == "number"
                && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(3.0)
        });

        let has_temp = entities.iter().any(|e| {
            e.dim == "temperature"
                && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(3.0)
        });

        assert!(
            has_numeral || has_temp,
            "Expected numeral(3) and/or temperature(3), got: {:?}",
            entities
        );
    }

    #[test]
    fn test_parse_url() {
        let entities = parse_en(
            "visit https://www.example.com/path",
            &[DimensionKind::Url],
        );
        let found = entities.iter().any(|e| e.dim == "url");
        assert!(found, "Expected URL, got: {:?}", entities);
    }

    #[test]
    fn test_parse_money() {
        let entities = parse_en("$42.50", &[DimensionKind::AmountOfMoney]);
        let found = entities.iter().any(|e| {
            e.dim == "amount-of-money"
                && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(42.5)
                && e.value.value.get("unit").and_then(|v| v.as_str()) == Some("USD")
        });
        assert!(found, "Expected $42.50, got: {:?}", entities);
    }

    #[test]
    fn test_parse_ordinal() {
        let entities = parse_en("the 3rd", &[DimensionKind::Ordinal]);
        let found = entities.iter().any(|e| {
            e.dim == "ordinal"
                && e.value.value.get("value").and_then(|v| v.as_i64()) == Some(3)
        });
        assert!(found, "Expected 3rd, got: {:?}", entities);
    }

    #[test]
    fn test_parse_duration() {
        let entities = parse_en("3 days", &[DimensionKind::Duration]);
        let found = entities.iter().any(|e| {
            e.dim == "duration"
                && e.value.value.get("value").and_then(|v| v.as_i64()) == Some(3)
                && e.value.value.get("unit").and_then(|v| v.as_str()) == Some("day")
        });
        assert!(found, "Expected 3 days, got: {:?}", entities);
    }

    #[test]
    fn test_parse_time_today() {
        let entities = parse_en("today", &[DimensionKind::Time]);
        let found = entities.iter().any(|e| e.dim == "time");
        assert!(found, "Expected time for 'today', got: {:?}", entities);
    }

    #[test]
    fn test_parse_distance() {
        let entities = parse_en("5 miles", &[DimensionKind::Distance]);
        let found = entities.iter().any(|e| {
            e.dim == "distance"
                && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(5.0)
                && e.value.value.get("unit").and_then(|v| v.as_str()) == Some("mile")
        });
        assert!(found, "Expected 5 miles, got: {:?}", entities);
    }

    #[test]
    fn test_parse_volume() {
        let entities = parse_en("2 gallons", &[DimensionKind::Volume]);
        let found = entities.iter().any(|e| {
            e.dim == "volume"
                && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(2.0)
                && e.value.value.get("unit").and_then(|v| v.as_str()) == Some("gallon")
        });
        assert!(found, "Expected 2 gallons, got: {:?}", entities);
    }

    #[test]
    fn test_parse_quantity() {
        let entities = parse_en("5 pounds", &[DimensionKind::Quantity]);
        let found = entities.iter().any(|e| {
            e.dim == "quantity"
                && e.value.value.get("value").and_then(|v| v.as_f64()) == Some(5.0)
        });
        assert!(found, "Expected 5 pounds, got: {:?}", entities);
    }

    #[test]
    fn test_all_dimensions_at_once() {
        // Should handle parsing with all dimensions enabled
        let entities = parse_en("tomorrow at 3pm for $50", &[]);
        assert!(!entities.is_empty(), "Expected some entities, got none");
    }
}
