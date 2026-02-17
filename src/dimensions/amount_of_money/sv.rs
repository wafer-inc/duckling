use crate::dimensions::numeral::helpers::{is_natural, is_positive, numeral_data};
use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

fn money_data(td: &TokenData) -> Option<&AmountOfMoneyData> {
    if let TokenData::AmountOfMoney(d) = td {
        Some(d)
    } else {
        None
    }
}
fn is_currency_only(td: &TokenData) -> bool {
    matches!(td,TokenData::AmountOfMoney(d) if d.value.is_none()&&d.min_value.is_none()&&d.max_value.is_none())
}
fn is_money_with_value(td: &TokenData) -> bool {
    matches!(td,TokenData::AmountOfMoney(d) if d.value.is_some()||d.min_value.is_some()||d.max_value.is_some())
}
fn is_without_cents(td: &TokenData) -> bool {
    match td {
        TokenData::AmountOfMoney(d) => {
            d.currency != Currency::Cent && d.value.map(|v| v == v.floor()).unwrap_or(false)
        }
        _ => false,
    }
}
fn is_cents(td: &TokenData) -> bool {
    matches!(td,TokenData::AmountOfMoney(d) if d.currency==Currency::Cent&&d.value.is_some())
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<unit> <amount>".to_string(),
            pattern: vec![predicate(is_currency_only), predicate(is_positive)],
            production: Box::new(|n| {
                let c = money_data(&n[0].token_data)?.currency;
                let v = numeral_data(&n[1].token_data)?.value;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(c).with_value(v),
                ))
            }),
        },
        Rule {
            name: "intersect (and number)".to_string(),
            pattern: vec![
                predicate(is_without_cents),
                regex("och"),
                predicate(is_natural),
            ],
            production: Box::new(|n| {
                let d = money_data(&n[0].token_data)?;
                let c = numeral_data(&n[2].token_data)?.value;
                Some(TokenData::AmountOfMoney(d.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "about <amount-of-money>".to_string(),
            pattern: vec![
                regex("omkring|cirka|runt|ca"),
                predicate(is_money_with_value),
            ],
            production: Box::new(|n| Some(n[1].token_data.clone())),
        },
        Rule {
            name: "cent".to_string(),
            pattern: vec![regex("cents?|penn(y|ies)|öre")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Cent,
                )))
            }),
        },
        Rule {
            name: "exactly <amount-of-money>".to_string(),
            pattern: vec![regex("exakt|precis"), predicate(is_money_with_value)],
            production: Box::new(|n| Some(n[1].token_data.clone())),
        },
        Rule {
            name: "intersect (X cents)".to_string(),
            pattern: vec![predicate(is_without_cents), predicate(is_cents)],
            production: Box::new(|n| {
                let d = money_data(&n[0].token_data)?;
                let c = money_data(&n[1].token_data)?.value?;
                Some(TokenData::AmountOfMoney(d.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "NOK".to_string(),
            pattern: vec![regex("norska kronor|nkr")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::NOK,
                )))
            }),
        },
        Rule {
            name: "£".to_string(),
            pattern: vec![regex("pund?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Pound,
                )))
            }),
        },
        Rule {
            name: "intersect (and X cents)".to_string(),
            pattern: vec![
                predicate(is_without_cents),
                regex("och"),
                predicate(is_cents),
            ],
            production: Box::new(|n| {
                let d = money_data(&n[0].token_data)?;
                let c = money_data(&n[2].token_data)?.value?;
                Some(TokenData::AmountOfMoney(d.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "intersect".to_string(),
            pattern: vec![predicate(is_without_cents), predicate(is_natural)],
            production: Box::new(|n| {
                let d = money_data(&n[0].token_data)?;
                let c = numeral_data(&n[1].token_data)?.value;
                Some(TokenData::AmountOfMoney(d.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "SEK".to_string(),
            pattern: vec![regex("kr(onor)?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::SEK,
                )))
            }),
        },
        Rule {
            name: "AED".to_string(),
            pattern: vec![regex("dirhams?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::AED,
                )))
            }),
        },
    ]
}
