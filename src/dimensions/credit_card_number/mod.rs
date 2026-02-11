pub mod rules;

use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct CreditCardNumberData {
    pub value: String,
    pub issuer: Option<String>,
}

impl CreditCardNumberData {
    pub fn new(value: &str) -> Self {
        let issuer = detect_issuer(value);
        CreditCardNumberData {
            value: value.to_string(),
            issuer,
        }
    }
}

fn detect_issuer(number: &str) -> Option<String> {
    let digits: String = number.chars().filter(|c| c.is_ascii_digit()).collect();
    if digits.len() < 13 {
        return None;
    }
    let first = digits.chars().next()?;
    let first2: String = digits.chars().take(2).collect();
    match first {
        '4' => Some("visa".to_string()),
        '5' => {
            let second = first2.chars().nth(1)?;
            if ('1'..='5').contains(&second) {
                Some("mastercard".to_string())
            } else {
                None
            }
        }
        '3' => {
            if first2 == "34" || first2 == "37" {
                Some("amex".to_string())
            } else {
                None
            }
        }
        '6' => {
            if first2 == "65" || digits.starts_with("6011") {
                Some("discover".to_string())
            } else {
                None
            }
        }
        _ => None,
    }
}

/// Luhn algorithm for credit card validation.
pub fn luhn_check(number: &str) -> bool {
    let digits: Vec<u32> = number
        .chars()
        .filter(|c| c.is_ascii_digit())
        .filter_map(|c| c.to_digit(10))
        .collect();

    if digits.len() < 13 {
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
    let mut json = serde_json::json!({
        "value": data.value,
        "type": "value",
    });
    if let Some(ref issuer) = data.issuer {
        json.as_object_mut()
            .unwrap()
            .insert("issuer".to_string(), serde_json::json!(issuer));
    }
    ResolvedValue {
        kind: "value".to_string(),
        value: json,
    }
}
