use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "amount keywords (zh)".to_string(),
        pattern: vec![regex(
            "(人民币|人民幣|港幣|元|圆|块|蚊|個|分|仙|角|毛|毫|\\$|€|£)",
        )],
        production: Box::new(|nodes| {
            let m = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            let c = if m.contains("人民") {
                Currency::CNY
            } else if m.contains("港幣") {
                Currency::HKD
            } else if m.contains("分")
                || m.contains("仙")
                || m.contains("角")
                || m.contains("毛")
                || m.contains("毫")
            {
                Currency::Cent
            } else if m.contains('€') {
                Currency::EUR
            } else if m.contains('£') {
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
