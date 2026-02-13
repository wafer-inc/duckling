use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "amount keywords (he)".to_string(),
        pattern: vec![regex("(\\$|€|£|₪|ש\"?ח|ש״ח|שקל|שנקל|אגורה|אגורות|דולר|יורו|אירו|פאונד|לירה|שטרלינג|GBP|EUR|USD)")],
        production: Box::new(|nodes| {
            let m = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            let c = if m.contains('₪') || m.contains("שקל") || m.contains("ש\"ח") || m.contains("ש״ח") {
                Currency::ILS
            } else if m.contains("אגור") {
                Currency::Cent
            } else if m.contains("€") || m.contains("יורו") || m.contains("אירו") || m.contains("EUR") {
                Currency::EUR
            } else if m.contains('£') || m.contains("פאונד") || m.contains("לירה") || m.contains("GBP") {
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
