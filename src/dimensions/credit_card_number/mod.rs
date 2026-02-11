pub mod rules;

use crate::types::ResolvedValue;

pub const MIN_NUMBER_DIGITS: usize = 8;
pub const MAX_NUMBER_DIGITS: usize = 19;

#[derive(Debug, Clone)]
pub struct CreditCardNumberData {
    pub value: String,
    pub issuer: String,
}

impl CreditCardNumberData {
    pub fn new(value: &str, issuer: &str) -> Self {
        CreditCardNumberData {
            value: value.to_string(),
            issuer: issuer.to_string(),
        }
    }
}

pub fn detect_issuer(digits: &str) -> &'static str {
    let len = digits.len();
    if len >= 16 && digits.starts_with('4') {
        return "visa";
    }
    if len >= 15 && (digits.starts_with("34") || digits.starts_with("37")) {
        return "amex";
    }
    if len >= 16
        && (digits.starts_with("6011")
            || digits.starts_with("65")
            || digits.starts_with("64"))
    {
        return "discover";
    }
    if len >= 16 {
        if let Some(second) = digits.chars().nth(1) {
            if digits.starts_with('5') && ('1'..='5').contains(&second) {
                return "mastercard";
            }
        }
    }
    if len >= 14 {
        let first3: String = digits.chars().take(3).collect();
        if digits.starts_with("36") || digits.starts_with("38") {
            return "dinerclub";
        }
        if first3.starts_with("30") {
            if let Some(third) = digits.chars().nth(2) {
                if ('0'..='5').contains(&third) {
                    return "dinerclub";
                }
            }
        }
    }
    "other"
}

/// Luhn algorithm for credit card validation.
pub fn luhn_check(number: &str) -> bool {
    let digits: Vec<u32> = number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() < MIN_NUMBER_DIGITS || digits.len() > MAX_NUMBER_DIGITS {
        return false;
    }

    let mut sum = 0u32;
    let mut double = false;
    for &d in digits.iter().rev() {
        let mut val = d;
        if double {
            val *= 2;
            if val > 9 {
                val -= 9;
            }
        }
        sum += val;
        double = !double;
    }
    sum % 10 == 0
}

pub fn resolve(data: &CreditCardNumberData) -> ResolvedValue {
    ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": data.value,
            "issuer": data.issuer,
            "type": "value",
        }),
    }
}
