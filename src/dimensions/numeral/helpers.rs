use crate::dimensions::numeral::NumeralData;
use crate::types::TokenData;

pub fn numeral_data(token: &TokenData) -> Option<&NumeralData> {
    match token {
        TokenData::Numeral(data) => Some(data),
        _ => None,
    }
}

/// Matches non-negative numbers (v >= 0).
/// Equivalent to Haskell Duckling's `isPositive`.
pub fn is_positive(token: &TokenData) -> bool {
    match token {
        TokenData::Numeral(data) => data.value >= 0.0,
        _ => false,
    }
}

/// Matches positive integers (v > 0 and v is integer).
/// Equivalent to Haskell Duckling's `isNatural`.
pub fn is_natural(token: &TokenData) -> bool {
    match token {
        TokenData::Numeral(data) => data.value > 0.0 && data.value == data.value.floor(),
        _ => false,
    }
}

pub fn is_multipliable(token: &TokenData) -> bool {
    match token {
        TokenData::Numeral(data) => data.multipliable,
        _ => false,
    }
}

/// Convert a number to its decimal form: 5 → 0.5, 25 → 0.25, etc.
/// Finds the smallest power of 10 greater than x and divides by it.
pub fn decimals_to_double(x: f64) -> f64 {
    let mut multiplier = 1.0;
    for _ in 0..10 {
        if x < multiplier {
            return x / multiplier;
        }
        multiplier *= 10.0;
    }
    0.0
}

pub fn integer_value(token: &TokenData) -> Option<i64> {
    match token {
        TokenData::Numeral(data) => {
            if data.value == data.value.floor() {
                Some(data.value as i64)
            } else {
                None
            }
        }
        _ => None,
    }
}
