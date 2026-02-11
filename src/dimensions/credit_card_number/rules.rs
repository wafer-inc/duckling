use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{detect_issuer, luhn_check, CreditCardNumberData};

fn cc_rule(name: &str, pattern: &str, issuer: &str) -> Rule {
    let issuer = issuer.to_string();
    Rule {
        name: name.to_string(),
        pattern: vec![regex(pattern)],
        production: Box::new(move |nodes| {
            let text = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
            if luhn_check(&digits) {
                Some(TokenData::CreditCardNumber(CreditCardNumberData::new(
                    &digits, &issuer,
                )))
            } else {
                None
            }
        }),
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        // Visa: 4xxx-xxxx-xxxx-xxxx (16 digits)
        cc_rule(
            "visa credit card number",
            r"(4[0-9]{15}|4[0-9]{3}-[0-9]{4}-[0-9]{4}-[0-9]{4})",
            "visa",
        ),
        // Amex: 3[47]xx-xxxxxx-xxxxx (15 digits)
        cc_rule(
            "amex card number",
            r"(3[47][0-9]{13}|3[47][0-9]{2}-[0-9]{6}-[0-9]{5})",
            "amex",
        ),
        // Discover: 6011/64xx/65xx (16 digits)
        cc_rule(
            "discover card number",
            r"(6(?:011|[45][0-9]{2})[0-9]{12}|6(?:011|[45][0-9]{2})-[0-9]{4}-[0-9]{4}-[0-9]{4})",
            "discover",
        ),
        // Mastercard: 5[1-5]xx (16 digits)
        cc_rule(
            "mastercard card number",
            r"(5[1-5][0-9]{14}|5[1-5][0-9]{2}-[0-9]{4}-[0-9]{4}-[0-9]{4})",
            "mastercard",
        ),
        // Diner Club: 30[0-5]x/36xx/38xx (14 digits)
        cc_rule(
            "diner club card number",
            r"(3(?:0[0-5]|[68][0-9])[0-9]{11}|3(?:0[0-5]|[68][0-9])[0-9]-[0-9]{6}-[0-9]{4})",
            "dinerclub",
        ),
        // Other: any 8-19 digit number not matching above patterns.
        // Since we can't use negative lookaheads, we use detect_issuer in production.
        {
            Rule {
                name: "credit card number".to_string(),
                pattern: vec![regex(r"(\d{8,19})")],
                production: Box::new(|nodes| {
                    let text = match &nodes[0].token_data {
                        TokenData::RegexMatch(m) => m.group(1)?,
                        _ => return None,
                    };
                    let digits: String = text.chars().filter(|c| c.is_ascii_digit()).collect();
                    if !luhn_check(&digits) {
                        return None;
                    }
                    let issuer = detect_issuer(&digits);
                    Some(TokenData::CreditCardNumber(CreditCardNumberData::new(
                        &digits, issuer,
                    )))
                }),
            }
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_luhn() {
        assert!(luhn_check("4111111111111111")); // Visa
        assert!(luhn_check("5500000000000004")); // Mastercard
        assert!(luhn_check("30569309025904")); // Diners Club (14 digits)
        assert!(!luhn_check("1234567890123456")); // Invalid
    }
}
