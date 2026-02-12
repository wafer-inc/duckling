// Ported from Duckling/Ordinal/EN/Corpus.hs
use duckling::{parse_en, DimensionKind, DimensionValue};

fn check_ordinal(text: &str, expected: i64) {
    let entities = parse_en(text, &[DimensionKind::Ordinal]);
    let found = entities.iter().any(|e| {
        matches!(&e.value, DimensionValue::Ordinal(v) if *v == expected)
    });
    assert!(
        found,
        "Expected ordinal {} for '{}', got: {:?}",
        expected,
        text,
        entities
            .iter()
            .map(|e| format!("{:?}={:?}", e.value.dim_kind(), e.value))
            .collect::<Vec<_>>()
    );
}

// OrdinalData 1
#[test]
fn test_ordinal_first() {
    check_ordinal("first", 1);
    check_ordinal("1st", 1);
}

// OrdinalData 2
#[test]
fn test_ordinal_second() {
    check_ordinal("second", 2);
    check_ordinal("2nd", 2);
}

// OrdinalData 3
#[test]
fn test_ordinal_third() {
    check_ordinal("third", 3);
    check_ordinal("3rd", 3);
}

// OrdinalData 4
#[test]
fn test_ordinal_fourth() {
    check_ordinal("fourth", 4);
    check_ordinal("4th", 4);
}

// OrdinalData 8
#[test]
fn test_ordinal_eighth() {
    check_ordinal("eighth", 8);
    check_ordinal("8th", 8);
}

// OrdinalData 25
#[test]
fn test_ordinal_twenty_fifth() {
    check_ordinal("twenty-fifth", 25);
    check_ordinal("twenty\u{2014}fifth", 25); // em dash
    check_ordinal("twenty fifth", 25);
    check_ordinal("twentyfifth", 25);
    check_ordinal("25th", 25);
}

// OrdinalData 31
#[test]
fn test_ordinal_thirty_first() {
    check_ordinal("thirty-first", 31);
    check_ordinal("thirty\u{2014}first", 31); // em dash
    check_ordinal("thirty first", 31);
    check_ordinal("thirtyfirst", 31);
    check_ordinal("31st", 31);
}

// OrdinalData 42
#[test]
fn test_ordinal_forty_second() {
    check_ordinal("forty-second", 42);
    check_ordinal("forty\u{2014}second", 42); // em dash
    check_ordinal("forty second", 42);
    check_ordinal("fortysecond", 42);
    check_ordinal("42nd", 42);
}

// OrdinalData 73
#[test]
fn test_ordinal_seventy_third() {
    check_ordinal("seventy-third", 73);
    check_ordinal("seventy\u{2014}third", 73); // em dash
    check_ordinal("seventy third", 73);
    check_ordinal("seventythird", 73);
    check_ordinal("73rd", 73);
}

// OrdinalData 90
#[test]
fn test_ordinal_ninetieth() {
    check_ordinal("ninetieth", 90);
    check_ordinal("90th", 90);
}
