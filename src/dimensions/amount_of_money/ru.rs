use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "amount keywords (ru)".to_string(),
        pattern: vec![regex("(\\$|€|£|x00a3&|₽|¢|руб|доллар|цент|пени|евро|фунт|бакс|грн|USD|EUR|GBP|RUB|UAH|KWD|LBP|EGP|QAR|SAR|BGN|MYR|RM)")],
        production: Box::new(|nodes| {
            let m = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?,
                _ => return None,
            };
            let c = if m.contains('₽') || m.contains("руб") || m.contains("RUB") {
                Currency::RUB
            } else if m.contains("грн") || m.contains("UAH") {
                Currency::UAH
            } else if m.contains("цент") || m.contains("пени") || m.contains('¢') {
                Currency::Cent
            } else if m.contains("евро") || m.contains("EUR") || m.contains('€') {
                Currency::EUR
            } else if m.contains("фунт") || m.contains("GBP") || m.contains('£') || m.contains("x00a3&") {
                Currency::GBP
            } else if m.contains("KWD") {
                Currency::KWD
            } else if m.contains("LBP") {
                Currency::LBP
            } else if m.contains("EGP") {
                Currency::EGP
            } else if m.contains("QAR") {
                Currency::QAR
            } else if m.contains("SAR") {
                Currency::SAR
            } else if m.contains("BGN") {
                Currency::BGN
            } else if m.contains("MYR") || m.contains("RM") {
                Currency::MYR
            } else if m.contains("бакс") {
                Currency::Unnamed
            } else {
                Currency::Dollar
            };
            Some(TokenData::AmountOfMoney(
                AmountOfMoneyData::currency_only(c).with_value(1.0),
            ))
        }),
    }]
}
