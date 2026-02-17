#![allow(clippy::approx_constant)]

// Ported from Duckling/AmountOfMoney/EN/Corpus.hs
use duckling::{parse_en, DimensionKind, DimensionValue, MeasurementPoint, MeasurementValue};

fn check_money(text: &str, expected_val: f64, expected_unit: &str) {
    let entities = parse_en(text, &[DimensionKind::AmountOfMoney]);
    let found = entities.iter().any(|e| {
        match &e.value {
            DimensionValue::AmountOfMoney(mv) => match mv {
                MeasurementValue::Value { value, unit } => {
                    (*value - expected_val).abs() < 0.01 && unit == expected_unit
                }
                MeasurementValue::Interval { from, to } => {
                    // For intervals, check from value (matching original test behavior)
                    if let Some(MeasurementPoint { value, unit }) = from {
                        if (*value - expected_val).abs() < 0.01 && unit == expected_unit {
                            return true;
                        }
                    }
                    if let Some(MeasurementPoint { value, unit }) = to {
                        if (*value - expected_val).abs() < 0.01 && unit == expected_unit {
                            return true;
                        }
                    }
                    false
                }
            },
            _ => false,
        }
    });
    assert!(
        found,
        "Expected money {} {} for '{}', got: {:?}",
        expected_val,
        expected_unit,
        text,
        entities
            .iter()
            .map(|e| format!("{:?}={:?}", e.value.dim_kind(), e.value))
            .collect::<Vec<_>>()
    );
}

// simple Dollar 1
#[test]
fn test_money_1_dollar() {
    check_money("$1", 1.0, "USD");
    check_money("one dollar", 1.0, "USD");
    check_money("a dollar", 1.0, "USD");
}

// simple Dollar 10
#[test]
fn test_money_10_dollars() {
    check_money("$10", 10.0, "USD");
    check_money("$ 10", 10.0, "USD");
    check_money("10$", 10.0, "USD");
    check_money("10 dollars", 10.0, "USD");
    check_money("ten dollars", 10.0, "USD");
}

// simple Cent 10
#[test]
fn test_money_10_cents() {
    check_money("10 cent", 10.0, "cent");
    check_money("ten pennies", 10.0, "cent");
    check_money("ten cents", 10.0, "cent");
    check_money("10 c", 10.0, "cent");
    check_money("10¢", 10.0, "cent");
}

// simple Dollar 10000
#[test]
fn test_money_10k_dollars() {
    check_money("$10K", 10000.0, "USD");
    check_money("10k$", 10000.0, "USD");
    check_money("$10,000", 10000.0, "USD");
}

// simple USD 3.14
#[test]
fn test_money_usd_3_14() {
    check_money("USD3.14", 3.14, "USD");
    check_money("3.14US$", 3.14, "USD");
    check_money("US$ 3.14", 3.14, "USD");
    check_money("US$3 and fourteen", 3.14, "USD");
}

// simple EUR 20
#[test]
fn test_money_20_euros() {
    check_money("20\u{20ac}", 20.0, "EUR"); // €
    check_money("20 euros", 20.0, "EUR");
    check_money("20 Euro", 20.0, "EUR");
    check_money("20 Euros", 20.0, "EUR");
    check_money("EUR 20", 20.0, "EUR");
    check_money("EUR 20.0", 20.0, "EUR");
    check_money("20€", 20.0, "EUR");
    check_money("20 €ur", 20.0, "EUR");
}

// simple Pound 10
#[test]
fn test_money_10_pounds() {
    check_money("\u{00a3}10", 10.0, "GBP"); // £10
    check_money("ten pounds", 10.0, "GBP");
}

// simple INR 20
#[test]
fn test_money_20_inr() {
    check_money("Rs. 20", 20.0, "INR");
    check_money("Rs 20", 20.0, "INR");
    check_money("20 Rupees", 20.0, "INR");
    check_money("20Rs", 20.0, "INR");
    check_money("Rs20", 20.0, "INR");
}

// simple INR 20.43
#[test]
fn test_money_20_43_inr() {
    check_money("20 Rupees 43", 20.43, "INR");
    check_money("twenty rupees 43", 20.43, "INR");
}

// simple Dollar 20.43
#[test]
fn test_money_20_43_dollars() {
    check_money("$20 and 43c", 20.43, "USD");
    check_money("$20 43", 20.43, "USD");
    check_money("20 dollar 43c", 20.43, "USD");
    check_money("20 dollars 43 cents", 20.43, "USD");
    check_money("twenty dollar 43 cents", 20.43, "USD");
    check_money("20 dollar 43", 20.43, "USD");
    check_money("twenty dollar and 43", 20.43, "USD");
}

// simple GBP 3.01
#[test]
fn test_money_gbp_3_01() {
    check_money("GBP3.01", 3.01, "GBP");
    check_money("GBP 3.01", 3.01, "GBP");
    check_money("3 GBP 1 pence", 3.01, "GBP");
    check_money("3 GBP and one", 3.01, "GBP");
}

// simple CAD 3.03
#[test]
fn test_money_cad_3_03() {
    check_money("CAD3.03", 3.03, "CAD");
    check_money("CAD 3.03", 3.03, "CAD");
    check_money("3 CAD 3 cents", 3.03, "CAD");
}

// simple CHF 3.04
#[test]
fn test_money_chf_3_04() {
    check_money("CHF3.04", 3.04, "CHF");
    check_money("CHF 3.04", 3.04, "CHF");
    check_money("3 CHF 4 cents", 3.04, "CHF");
}

// simple CNY 3
#[test]
fn test_money_cny_3() {
    check_money("CNY3", 3.0, "CNY");
    check_money("CNY 3", 3.0, "CNY");
    check_money("3 CNY", 3.0, "CNY");
    check_money("3 yuan", 3.0, "CNY");
}

// simple Unnamed 42 (bucks)
#[test]
fn test_money_42_bucks() {
    check_money("42 bucks", 42.0, "USD");
    check_money("around 42 bucks", 42.0, "USD");
    check_money("exactly 42 bucks", 42.0, "USD");
}

// simple KWD 42
#[test]
fn test_money_42_kwd() {
    check_money("42 KWD", 42.0, "KWD");
    check_money("42 kuwaiti Dinar", 42.0, "KWD");
}

// simple LBP 42
#[test]
fn test_money_42_lbp() {
    check_money("42 LBP", 42.0, "LBP");
    check_money("42 Lebanese Pounds", 42.0, "LBP");
}

// simple EGP 42
#[test]
fn test_money_42_egp() {
    check_money("42 EGP", 42.0, "EGP");
    check_money("42 egyptianpound", 42.0, "EGP");
    check_money("42 LE", 42.0, "EGP");
    check_money("42 L.E", 42.0, "EGP");
    check_money("42 l.e.", 42.0, "EGP");
    check_money("42 le", 42.0, "EGP");
    check_money("42 geneh", 42.0, "EGP");
    check_money("42 genihat masriya", 42.0, "EGP");
}

// simple QAR 42
#[test]
fn test_money_42_qar() {
    check_money("42 QAR", 42.0, "QAR");
    check_money("42 qatari riyals", 42.0, "QAR");
}

// simple SAR 42
#[test]
fn test_money_42_sar() {
    check_money("42 SAR", 42.0, "SAR");
    check_money("42 Saudiriyal", 42.0, "SAR");
}

// simple BGN 42
#[test]
fn test_money_42_bgn() {
    check_money("42 BGN", 42.0, "BGN");
}

// simple MYR 42
#[test]
fn test_money_42_myr() {
    check_money("42 MYR", 42.0, "MYR");
    check_money("42 RM", 42.0, "MYR");
    check_money("RM 42", 42.0, "MYR");
    check_money("MYR 42", 42.0, "MYR");
    check_money("42MYR", 42.0, "MYR");
    check_money("42RM", 42.0, "MYR");
    check_money("RM42", 42.0, "MYR");
    check_money("MYR42", 42.0, "MYR");
    check_money("ringgit 42", 42.0, "MYR");
    check_money("42 ringgit", 42.0, "MYR");
    check_money("42 malaysia ringgit", 42.0, "MYR");
    check_money("malaysia ringgit 42", 42.0, "MYR");
    check_money("42 malaysian ringgit", 42.0, "MYR");
    check_money("malaysian ringgit 42", 42.0, "MYR");
    check_money("42 malaysia ringgits", 42.0, "MYR");
    check_money("malaysia ringgits 42", 42.0, "MYR");
    check_money("42 malaysian ringgits", 42.0, "MYR");
    check_money("malaysian ringgits 42", 42.0, "MYR");
}

// simple MYR 20.43
#[test]
fn test_money_myr_20_43() {
    check_money("20 ringgit and 43c", 20.43, "MYR");
    check_money("20 ringgit and 43 sen", 20.43, "MYR");
    check_money("twenty ringgit 43 sens", 20.43, "MYR");
    check_money("20 ringgit 43", 20.43, "MYR");
    check_money("twenty ringgit and 43", 20.43, "MYR");
}

// simple Dinar 10
#[test]
fn test_money_10_dinars() {
    check_money("10 dinars", 10.0, "dinar");
}

// simple ILS 10
#[test]
fn test_money_10_ils() {
    check_money("ten shekels", 10.0, "ILS");
    check_money("10 ILS", 10.0, "ILS");
}

// simple Riyal 10
#[test]
fn test_money_10_riyals() {
    check_money("ten riyals", 10.0, "riyal");
    check_money("10 riyals", 10.0, "riyal");
}

// simple Rial 10
#[test]
fn test_money_10_rials() {
    check_money("ten rials", 10.0, "rial");
    check_money("10 rials", 10.0, "rial");
}

// between Dollar (10, 20)
#[test]
fn test_money_between_10_20_dollars() {
    check_money("between 10 and 20 dollars", 10.0, "USD");
    check_money("from 10 dollars to 20 dollars", 10.0, "USD");
    check_money("around 10-20 dollars", 10.0, "USD");
    check_money("between 10 dollars and 20 dollars", 10.0, "USD");
    check_money("from 10 to 20 dollars", 10.0, "USD");
    check_money("about $10-$20", 10.0, "USD");
    check_money("10-20 dollars", 10.0, "USD");
}

// between Dollar (1.1, 1.3)
#[test]
fn test_money_between_1_1_1_3_dollars() {
    check_money("between 1.1 and 1.3 dollars", 1.1, "USD");
    check_money("from 1 point 1 and one point three dollars", 1.1, "USD");
}

// under EUR 7
#[test]
fn test_money_under_7_eur() {
    check_money("under seven euros", 7.0, "EUR");
    check_money("less than 7 EUR", 7.0, "EUR");
    check_money("lower than 7€", 7.0, "EUR");
    check_money("no more than 7 euros", 7.0, "EUR");
    check_money("at most 7€", 7.0, "EUR");
}

// above Dollar 1.42
#[test]
fn test_money_above_1_42_dollars() {
    check_money("more than 1 dollar and forty-two cents", 1.42, "USD");
    check_money("at least $1.42", 1.42, "USD");
    check_money("over 1.42 dollars", 1.42, "USD");
    check_money("above a dollar and 42 cents", 1.42, "USD");
    check_money("no less than $1.42", 1.42, "USD");
}

// simple INR 500000
#[test]
fn test_money_5_lakh_rupees() {
    check_money("5 lakh rupees", 500000.0, "INR");
    check_money("five lakhs rupees", 500000.0, "INR");
}

// between INR (7, 900000)
#[test]
fn test_money_7_9_lakh_rupees() {
    check_money("7-9 lakh rupees", 7.0, "INR");
}

// simple INR 40000000
#[test]
fn test_money_4_crore_rupees() {
    check_money("four crore rupees", 40000000.0, "INR");
    check_money("4 crores rupees", 40000000.0, "INR");
}

// simple MNT 10
#[test]
fn test_money_10_mnt() {
    check_money("ten tugriks", 10.0, "MNT");
    check_money("10 Tugrik", 10.0, "MNT");
    check_money("10MNT", 10.0, "MNT");
}

// simple USD 4.7e9
#[test]
fn test_money_usd_4_7_billion() {
    check_money("US$4.7 billion", 4700000000.0, "USD");
    check_money("a US$4.7 billion", 4700000000.0, "USD");
    check_money("a US$ 4.7 billion", 4700000000.0, "USD");
}

// simple UAH 3.04
#[test]
fn test_money_uah_3_04() {
    check_money("UAH3.04", 3.04, "UAH");
    check_money("UAH 3.04", 3.04, "UAH");
    check_money("3 UAH 4 kopiykas", 3.04, "UAH");
}
