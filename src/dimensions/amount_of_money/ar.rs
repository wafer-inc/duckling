use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "amount keywords (ar)".to_string(),
        pattern: vec![regex("(\\$|€|£|¢|دولار|سنت|سينت|يورو|اورو|أورو|جنيه|دينار|دنانير|ليرة|ليرات|ريال|شيقل|شيكل|شواقل|KWD|LBP|EGP|QAR|SAR|JOD|ILS|ج[.]م[.]?)")],
        production: Box::new(|nodes| {
            let m = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            let c = if m.contains("€") || m.contains("يورو") || m.contains("اورو") || m.contains("أورو") {
                Currency::EUR
            } else if m.contains('£') || m.contains("جنيه") {
                Currency::Pound
            } else if m.contains("سنت") || m.contains("سينت") || m.contains('¢') {
                Currency::Cent
            } else if m.contains("دينار") || m.contains("KWD") || m.contains("JOD") {
                Currency::Dinar
            } else if m.contains("ريال") || m.contains("QAR") || m.contains("SAR") {
                Currency::Riyal
            } else if m.contains("ليرة") || m.contains("LBP") {
                Currency::LBP
            } else if m.contains("شيقل") || m.contains("ILS") {
                Currency::ILS
            } else if m.contains("EGP") || m.contains("ج.م") {
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
