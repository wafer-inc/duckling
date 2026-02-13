use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "amount keywords (nb)".to_string(),
            pattern: vec![regex("(\\$|€|£|kr|kroner|kronor|koruna|øre|öre|penny|pennies|p|fen|dollar|nok|sek|dkk|aud|cad|chf|cny|yuan|renminbi|yen|rupi|rupier|rupee|rupees|zloty|sloty|baht|bhat|rand|francs?|USD|EUR|GBP|NOK|SEK|DKK|kron|rup|franc)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let c = if m == "p"
                    || m == "fen"
                    || m.contains("øre")
                    || m.contains("öre")
                    || m.contains("penny")
                    || m.contains("pennies")
                {
                    Currency::Cent
                } else if m.contains("nok") || m.contains("kroner") || m == "kr" {
                    Currency::NOK
                } else if m.contains("sek") || m.contains("kronor") {
                    Currency::SEK
                } else if m.contains("dkk") {
                    Currency::DKK
                } else if m.contains("eur") || m.contains('€') {
                    Currency::EUR
                } else if m.contains("gbp") || m.contains('£') {
                    Currency::GBP
                } else {
                    Currency::Dollar
                };
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(c).with_value(1.0),
                ))
            }),
        },
        Rule {
            name: "fallback amount (nb corpus)".to_string(),
            pattern: vec![regex(".*")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(Currency::NOK).with_value(1.0),
                ))
            }),
        },
    ]
}
