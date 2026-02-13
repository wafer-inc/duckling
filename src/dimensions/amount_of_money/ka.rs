use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "amount keywords (ka)".to_string(),
        pattern: vec![regex("(\\$|€|£|¢|x00a3&|დოლარ|ცენტ|ევრო|ფუნტ|დინარ|ლირ|რიალ|ლარი|ლარ|KWD|LBP|EGP|QAR|SAR|USD|EUR|GBP)")],
        production: Box::new(|nodes| {
            let m = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            let c = if m.contains("ევრო") || m.contains("EUR") || m.contains('€') {
                Currency::EUR
            } else if m.contains("ცენტ") || m.contains('¢') {
                Currency::Cent
            } else if m.contains("ფუნტ") || m.contains("GBP") || m.contains('£') || m.contains("x00a3&") {
                Currency::GBP
            } else if m.contains("KWD") || m.contains("დინარ") {
                Currency::Dinar
            } else if m.contains("QAR") || m.contains("SAR") || m.contains("რიალ") {
                Currency::Riyal
            } else if m.contains("LBP") || m.contains("ლირ") {
                Currency::LBP
            } else if m.contains("EGP") {
                Currency::EGP
            } else {
                Currency::Dollar
            };
            Some(TokenData::AmountOfMoney(
                AmountOfMoneyData::currency_only(c).with_value(1.0),
            ))
        }),
    }]
}
