use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "amount keywords (vi)".to_string(),
        pattern: vec![regex("(\\$|€|£|đồng|đô( la)?|xen|xu|vnd|vnđ|vn\\$|rupees?|rs\\.?|pounds?|aed|dirhams?|usd|eur|gbp)")],
        production: Box::new(|nodes| {
            let m = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                _ => return None,
            };
            let c = if m.contains("đồng") || m.contains("vnd") || m.contains("vnđ") || m.contains("vn$") {
                Currency::VND
            } else if m.contains("xen") || m.contains("xu") {
                Currency::Cent
            } else if m.contains("eur") || m.contains('€') {
                Currency::EUR
            } else if m.contains("gbp") || m.contains("pound") || m.contains('£') {
                Currency::GBP
            } else if m.contains("aed") || m.contains("dirham") {
                Currency::AED
            } else if m.contains("rs") || m.contains("rupee") {
                Currency::INR
            } else {
                Currency::Dollar
            };
            Some(TokenData::AmountOfMoney(
                AmountOfMoneyData::currency_only(c).with_value(1.0),
            ))
        }),
    }]
}
