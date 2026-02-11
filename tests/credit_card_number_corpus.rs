// Ported from Duckling/CreditCardNumber/Corpus.hs
use duckling::{parse_en, DimensionKind};

fn check_cc(text: &str, expected_issuer: &str) {
    let entities = parse_en(text, &[DimensionKind::CreditCardNumber]);
    let found = entities.iter().any(|e| {
        e.dim == "credit-card-number"
            && e.value.value.get("issuer").and_then(|v| v.as_str()) == Some(expected_issuer)
    });
    assert!(
        found,
        "Expected credit card issuer '{}' for '{}', got: {:?}",
        expected_issuer, text,
        entities.iter().map(|e| format!("{}={:?}", e.dim, e.value)).collect::<Vec<_>>()
    );
}

fn check_cc_any(text: &str) {
    let entities = parse_en(text, &[DimensionKind::CreditCardNumber]);
    let found = entities.iter().any(|e| e.dim == "credit-card-number");
    assert!(
        found,
        "Expected credit card for '{}', got: {:?}",
        text,
        entities.iter().map(|e| format!("{}={:?}", e.dim, e.value)).collect::<Vec<_>>()
    );
}

fn check_no_cc(text: &str) {
    let entities = parse_en(text, &[DimensionKind::CreditCardNumber]);
    let found = entities.iter().any(|e| e.dim == "credit-card-number");
    assert!(
        !found,
        "Expected NO credit card for '{}', but got: {:?}",
        text,
        entities.iter().map(|e| format!("{}={:?}", e.dim, e.value)).collect::<Vec<_>>()
    );
}

// Visa
#[test]
fn test_cc_visa() {
    check_cc("4111111111111111", "visa");
    check_cc("4111-1111-1111-1111", "visa");
}

// Amex
#[test]
fn test_cc_amex() {
    check_cc("371449635398431", "amex");
    check_cc("3714-496353-98431", "amex");
}

// Discover
#[test]
fn test_cc_discover() {
    check_cc("6011111111111117", "discover");
    check_cc("6011-1111-1111-1117", "discover");
}

// Mastercard
#[test]
fn test_cc_mastercard() {
    check_cc("5555555555554444", "mastercard");
    check_cc("5555-5555-5555-4444", "mastercard");
}

// DinerClub
#[test]
fn test_cc_diners_club() {
    check_cc_any("30569309025904");
    check_cc_any("3056-930902-5904");
}

// Other (JCB)
#[test]
fn test_cc_other() {
    check_cc_any("3530111333300000");
}

// Negative examples - invalid Luhn
#[test]
fn test_cc_negative_invalid_luhn() {
    check_no_cc("4111111111111110");
    check_no_cc("371449635398430");
    check_no_cc("6011111111111110");
    check_no_cc("5555555555554440");
    check_no_cc("30569309025900");
}

// Negative - wrong format (wrong grouping)
#[test]
fn test_cc_negative_wrong_format() {
    check_no_cc("41111111-1111-1111");
    check_no_cc("3714496353-98431");
    check_no_cc("60111111-1111-1117");
    check_no_cc("55555555-5555-4444");
    check_no_cc("3056930902-5904");
}

// Negative - too short / too long / invalid
#[test]
fn test_cc_negative_other() {
    check_no_cc("invalid");
}
