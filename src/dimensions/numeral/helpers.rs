use crate::types::TokenData;
use crate::dimensions::numeral::NumeralData;

pub fn numeral_data(token: &TokenData) -> Option<&NumeralData> {
    match token {
        TokenData::Numeral(data) => Some(data),
        _ => None,
    }
}

pub fn is_numeral(token: &TokenData) -> bool {
    matches!(token, TokenData::Numeral(_))
}

pub fn is_integer(token: &TokenData) -> bool {
    match token {
        TokenData::Numeral(data) => data.value == data.value.floor(),
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

pub fn number_between(low: f64, high: f64) -> Box<dyn Fn(&TokenData) -> bool + Send + Sync> {
    Box::new(move |token: &TokenData| {
        if let TokenData::Numeral(data) = token {
            data.value >= low && data.value <= high
        } else {
            false
        }
    })
}

pub fn number_with(
    low: f64,
    high: f64,
) -> Box<dyn Fn(&TokenData) -> bool + Send + Sync> {
    number_between(low, high)
}

pub fn is_multipliable(token: &TokenData) -> bool {
    match token {
        TokenData::Numeral(data) => data.multipliable,
        _ => false,
    }
}

pub fn double_value(token: &TokenData) -> Option<f64> {
    match token {
        TokenData::Numeral(data) => Some(data.value),
        _ => None,
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
