use crate::dimensions::numeral::helpers::{is_natural, is_positive, numeral_data};
use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

fn money_data(td: &TokenData) -> Option<&AmountOfMoneyData> {
    match td {
        TokenData::AmountOfMoney(d) => Some(d),
        _ => None,
    }
}

fn is_currency_only(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::AmountOfMoney(d)
            if d.value.is_none() && d.min_value.is_none() && d.max_value.is_none()
    )
}

fn is_money_with_value(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::AmountOfMoney(d)
            if d.value.is_some() || d.min_value.is_some() || d.max_value.is_some()
    )
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
    matches!(td, TokenData::AmountOfMoney(d) if d.currency == Currency::Cent && d.value.is_some())
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<unit> <amount>".to_string(),
            pattern: vec![predicate(is_currency_only), predicate(is_positive)],
            production: Box::new(|nodes| {
                let c = money_data(&nodes[0].token_data)?.currency;
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(c).with_value(v),
                ))
            }),
        },
        Rule {
            name: "$".to_string(),
            pattern: vec![regex("n?dh?oll?ai?rs?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Dollar,
                )))
            }),
        },
        Rule {
            name: "cent".to_string(),
            pattern: vec![regex("cents?|g?ch?eint(eanna)?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Cent,
                )))
            }),
        },
        Rule {
            name: "thart ar <amount-of-money>".to_string(),
            pattern: vec![
                regex("thart( ar)?|beagnach|breis (is|agus)"),
                predicate(is_money_with_value),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "intersect (X cents)".to_string(),
            pattern: vec![predicate(is_without_cents), predicate(is_cents)],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[0].token_data)?;
                let c = money_data(&nodes[1].token_data)?.value?;
                Some(TokenData::AmountOfMoney(d.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "£".to_string(),
            pattern: vec![regex("pounds?|b?ph?unt")],
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
                regex("agus|is"),
                predicate(is_cents),
            ],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[0].token_data)?;
                let c = money_data(&nodes[2].token_data)?.value?;
                Some(TokenData::AmountOfMoney(d.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "intersect".to_string(),
            pattern: vec![predicate(is_without_cents), predicate(is_natural)],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[0].token_data)?;
                let c = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::AmountOfMoney(d.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "INR".to_string(),
            pattern: vec![regex("r(ú|u)pa(í|i)")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::INR,
                )))
            }),
        },
        Rule {
            name: "intersect (agus number)".to_string(),
            pattern: vec![
                predicate(is_without_cents),
                regex("agus|is"),
                predicate(is_natural),
            ],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[0].token_data)?;
                let c = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::AmountOfMoney(d.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "<amount-of-money> glan".to_string(),
            pattern: vec![
                predicate(is_money_with_value),
                regex("glan|baileach|(go )?d(í|i)reach"),
            ],
            production: Box::new(|nodes| Some(nodes[0].token_data.clone())),
        },
    ]
}
