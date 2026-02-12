// Ported from Duckling/Email/EN/Corpus.hs
use duckling::{parse_en, DimensionKind, DimensionValue};

fn check_email(text: &str, expected: &str) {
    let entities = parse_en(text, &[DimensionKind::Email]);
    let found = entities
        .iter()
        .any(|e| matches!(&e.value, DimensionValue::Email(v) if v == expected));
    assert!(
        found,
        "Expected email '{}' for '{}', got: {:?}",
        expected,
        text,
        entities
            .iter()
            .map(|e| format!("{:?}={:?}", e.value.dim_kind(), e.value))
            .collect::<Vec<_>>()
    );
}

fn check_no_email(text: &str) {
    let entities = parse_en(text, &[DimensionKind::Email]);
    let found = entities
        .iter()
        .any(|e| matches!(&e.value, DimensionValue::Email(_)));
    assert!(
        !found,
        "Expected NO email for '{}', but got: {:?}",
        text,
        entities
            .iter()
            .map(|e| format!("{:?}={:?}", e.value.dim_kind(), e.value))
            .collect::<Vec<_>>()
    );
}

// Positive examples
#[test]
fn test_email_alice_at_example() {
    check_email("alice at exAmple.io", "alice@exAmple.io");
}

#[test]
fn test_email_yo_plus_yo() {
    check_email("yo+yo at blah.org", "yo+yo@blah.org");
}

#[test]
fn test_email_1234_abc() {
    check_email("1234+abc at x.net", "1234+abc@x.net");
}

#[test]
fn test_email_jean_jacques() {
    check_email("jean-jacques at stuff.co.uk", "jean-jacques@stuff.co.uk");
}

#[test]
fn test_email_asdf_ab_dot_c() {
    check_email("asdf+ab dot c at gmail dot com", "asdf+ab.c@gmail.com");
}

#[test]
fn test_email_asdf_dot_k() {
    check_email("asdf dot k@fb dot com", "asdf.k@fb.com");
}

// Negative examples
#[test]
fn test_email_negative_fitness() {
    check_no_email("fitness at 6.40");
}

#[test]
fn test_email_negative_class() {
    check_no_email("class at 12.00");
}

#[test]
fn test_email_negative_tonight() {
    check_no_email("tonight at 9.15");
}

#[test]
fn test_email_negative_dot_before() {
    check_no_email(" dot 2@abci");
}

#[test]
fn test_email_negative_at_dot() {
    check_no_email("x@ dot x");
}

#[test]
fn test_email_negative_at_x_dot() {
    check_no_email("x@ x dot ");
}

#[test]
fn test_email_negative_abc_at_x_dot() {
    check_no_email("abc@x dot ");
}
