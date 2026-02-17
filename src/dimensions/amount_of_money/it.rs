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

fn is_simple_money(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::AmountOfMoney(d)
            if d.value.is_some() && d.min_value.is_none() && d.max_value.is_none()
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
            name: "cent".to_string(),
            pattern: vec![regex("cent(esim)i?o?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Cent,
                )))
            }),
        },
        Rule {
            name: "£, $".to_string(),
            pattern: vec![regex("(dollari|sterline)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let c = match m.as_str() {
                    "dollari" => Currency::Dollar,
                    "sterline" => Currency::Pound,
                    _ => return None,
                };
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    c,
                )))
            }),
        },
        Rule {
            name: "intersect".to_string(),
            pattern: vec![predicate(is_without_cents), predicate(is_natural)],
            production: Box::new(|nodes| {
                let m = money_data(&nodes[0].token_data)?;
                let c = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::AmountOfMoney(m.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "intersect (and number)".to_string(),
            pattern: vec![
                predicate(is_without_cents),
                regex("e"),
                predicate(is_natural),
            ],
            production: Box::new(|nodes| {
                let m = money_data(&nodes[0].token_data)?;
                let c = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::AmountOfMoney(m.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "intersect (X cents)".to_string(),
            pattern: vec![predicate(is_without_cents), predicate(is_cents)],
            production: Box::new(|nodes| {
                let m = money_data(&nodes[0].token_data)?;
                let c = money_data(&nodes[1].token_data)?.value?;
                Some(TokenData::AmountOfMoney(m.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "intersect (and X cents)".to_string(),
            pattern: vec![predicate(is_without_cents), regex("e"), predicate(is_cents)],
            production: Box::new(|nodes| {
                let m = money_data(&nodes[0].token_data)?;
                let c = money_data(&nodes[2].token_data)?.value?;
                Some(TokenData::AmountOfMoney(m.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "between|from <numeral> to|and <amount-of-money>".to_string(),
            pattern: vec![
                regex("tra"),
                predicate(is_positive),
                regex("e"),
                predicate(is_simple_money),
            ],
            production: Box::new(|nodes| {
                let from = numeral_data(&nodes[1].token_data)?.value;
                let d = money_data(&nodes[3].token_data)?;
                let to = d.value?;
                if from >= to {
                    return None;
                }
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "between|from <amount-of-money> to|and <amount-of-money>".to_string(),
            pattern: vec![
                regex("tra"),
                predicate(is_simple_money),
                regex("e"),
                predicate(is_simple_money),
            ],
            production: Box::new(|nodes| {
                let d1 = money_data(&nodes[1].token_data)?;
                let d2 = money_data(&nodes[3].token_data)?;
                let from = d1.value?;
                let to = d2.value?;
                if d1.currency != d2.currency || from >= to {
                    return None;
                }
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d1.currency).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "<numeral> - <amount-of-money>".to_string(),
            pattern: vec![
                predicate(is_natural),
                regex("-"),
                predicate(is_simple_money),
            ],
            production: Box::new(|nodes| {
                let from = numeral_data(&nodes[0].token_data)?.value;
                let d = money_data(&nodes[2].token_data)?;
                let to = d.value?;
                if from >= to {
                    return None;
                }
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "<amount-of-money> - <amount-of-money>".to_string(),
            pattern: vec![
                predicate(is_simple_money),
                regex("-"),
                predicate(is_simple_money),
            ],
            production: Box::new(|nodes| {
                let d1 = money_data(&nodes[0].token_data)?;
                let d2 = money_data(&nodes[2].token_data)?;
                let from = d1.value?;
                let to = d2.value?;
                if d1.currency != d2.currency || from >= to {
                    return None;
                }
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d1.currency).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "under/less/lower/no more than <amount-of-money>".to_string(),
            pattern: vec![regex("(meno|non più) di"), predicate(is_simple_money)],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[1].token_data)?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_max(d.value?),
                ))
            }),
        },
        Rule {
            name: "over/above/at least/more than <amount-of-money>".to_string(),
            pattern: vec![regex("più di|almeno"), predicate(is_simple_money)],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[1].token_data)?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_min(d.value?),
                ))
            }),
        },
        Rule {
            name: "precision".to_string(),
            pattern: vec![
                regex("esattamente|quasi|più o meno|circa"),
                predicate(is_money_with_value),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
    ]
}
