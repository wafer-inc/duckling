// Ported from Duckling/PhoneNumber/Corpus.hs
use duckling::{parse_en, DimensionKind, DimensionValue};

fn check_phone(text: &str, expected_value: &str) {
    let entities = parse_en(text, &[DimensionKind::PhoneNumber]);
    let found = entities.iter().any(|e| {
        matches!(&e.value, DimensionValue::PhoneNumber(v) if v == expected_value)
    });
    assert!(
        found,
        "Expected phone number value '{}' for '{}', got: {:?}",
        expected_value, text,
        entities.iter().map(|e| format!("{:?}={:?}", e.value.dim_kind(), e.value)).collect::<Vec<_>>()
    );
}

fn check_no_phone(text: &str) {
    let entities = parse_en(text, &[DimensionKind::PhoneNumber]);
    let found = entities.iter().any(|e| matches!(&e.value, DimensionValue::PhoneNumber(_)));
    assert!(
        !found,
        "Expected NO phone number for '{}', but got: {:?}",
        text,
        entities.iter().map(|e| format!("{:?}={:?}", e.value.dim_kind(), e.value)).collect::<Vec<_>>()
    );
}

// Positive corpus examples
#[test]
fn test_phone_650_701_8887() {
    check_phone("650-701-8887", "6507018887");
}

#[test]
fn test_phone_plus1_650() {
    check_phone("(+1)650-701-8887", "(+1) 6507018887");
    check_phone("(+1)   650 - 701  8887", "(+1) 6507018887");
    check_phone("(+1) 650-701-8887", "(+1) 6507018887");
    check_phone("+1 6507018887", "(+1) 6507018887");
}

#[test]
fn test_phone_plus33() {
    check_phone("+33 1 46647998", "(+33) 146647998");
}

#[test]
fn test_phone_06_2070() {
    check_phone("06 2070 2220", "0620702220");
}

#[test]
fn test_phone_with_ext() {
    check_phone("(650)-701-8887 ext 897", "6507018887 ext 897");
}

#[test]
fn test_phone_plus1_202() {
    check_phone("+1-202-555-0121", "(+1) 2025550121");
    check_phone("+1 202.555.0121", "(+1) 2025550121");
}

#[test]
fn test_phone_dots() {
    check_phone("4.8.6.6.8.2.7", "4866827");
}

#[test]
fn test_phone_plain_digits() {
    check_phone("06354640807", "06354640807");
    check_phone("18998078030", "18998078030");
}

#[test]
fn test_phone_brazil_format() {
    check_phone("61 - 9 9285-2776", "61992852776");
    check_phone("(19) 997424919", "19997424919");
    check_phone("+55 19992842606", "(+55) 19992842606");
}

// Negative corpus examples
#[test]
fn test_phone_negative_short() {
    check_no_phone("12345");
}

#[test]
fn test_phone_negative_too_long() {
    check_no_phone("1234567890123456777777");
    check_no_phone("12345678901234567");
}
