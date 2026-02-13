use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "amount keywords (id)".to_string(),
        pattern: vec![regex("(\\$|€|£|¥|dolar|rupiah|rp\\.?|idr|euro|pound|sterling|yen|USD|EUR|GBP|IDR|JPY)")],
        production: Box::new(|nodes| {
            let m = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                _ => return None,
            };
            let c = if m.contains("idr") || m.contains("rupiah") || m.contains("rp") {
                Currency::IDR
            } else if m.contains("eur") || m.contains("euro") || m.contains('€') {
                Currency::EUR
            } else if m.contains("gbp") || m.contains("pound") || m.contains('£') {
                Currency::GBP
            } else if m.contains("yen") || m.contains("jpy") || m.contains('¥') {
                Currency::JPY
            } else {
                Currency::Dollar
            };
            Some(TokenData::AmountOfMoney(
                AmountOfMoneyData::currency_only(c).with_value(1.0),
            ))
        }),
    }]
}
