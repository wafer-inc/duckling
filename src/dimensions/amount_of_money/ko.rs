use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "amount keywords (ko)".to_string(),
        pattern: vec![regex(
            "(\\$|€|£|₩|원|달러|불|센트|유로|파운드|EUR|GBP|KRW|USD)",
        )],
        production: Box::new(|nodes| {
            let m = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            let c = if m.contains('₩') || m.contains("원") || m.contains("KRW") {
                Currency::KRW
            } else if m.contains("센트") {
                Currency::Cent
            } else if m.contains("유로") || m.contains("EUR") || m.contains('€') {
                Currency::EUR
            } else if m.contains("파운드") || m.contains("GBP") || m.contains('£') {
                Currency::GBP
            } else {
                Currency::Dollar
            };
            Some(TokenData::AmountOfMoney(
                AmountOfMoneyData::currency_only(c).with_value(1.0),
            ))
        }),
    }]
}
