use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{luhn_check, CreditCardNumberData};

pub fn rules() -> Vec<Rule> {
    vec![
        // Credit card number with spaces or dashes
        Rule {
            name: "credit card number".to_string(),
            pattern: vec![regex(
                r#"(\d{4}[\s-]?\d{4}[\s-]?\d{4}[\s-]?\d{4}|\d{4}[\s-]?\d{6}[\s-]?\d{5})"#,
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                if luhn_check(text) {
                    Some(TokenData::CreditCardNumber(CreditCardNumberData::new(text)))
                } else {
                    None
                }
            }),
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luhn() {
        // Known valid test numbers
        assert!(luhn_check("4111111111111111")); // Visa
        assert!(luhn_check("5500000000000004")); // Mastercard
        assert!(!luhn_check("1234567890123456")); // Invalid
    }
}
