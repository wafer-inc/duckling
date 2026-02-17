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

fn is_simple_amount_of_money(td: &TokenData) -> bool {
    matches!(
        td,
        TokenData::AmountOfMoney(d)
            if d.value.is_some() && d.min_value.is_none() && d.max_value.is_none()
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
            name: "лв".to_string(),
            pattern: vec![regex("ле?ва?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::BGN,
                )))
            }),
        },
        Rule {
            name: "$".to_string(),
            pattern: vec![regex("долар(а|и)?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Dollar,
                )))
            }),
        },
        Rule {
            name: "escaped GBP prefix".to_string(),
            pattern: vec![regex("x00a3&?(\\d+(\\.\\d+)?)")],
            production: Box::new(|nodes| {
                let v: f64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(Currency::Pound).with_value(v),
                ))
            }),
        },
        Rule {
            name: "cent".to_string(),
            pattern: vec![regex("ст(отинк(a|и))?|цента?|пени(та)?|пенса?|ц")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Cent,
                )))
            }),
        },
        Rule {
            name: "escaped EUR suffix".to_string(),
            pattern: vec![regex("(\\d+(\\.\\d+)?)x20ac")],
            production: Box::new(|nodes| {
                let v: f64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(Currency::EUR).with_value(v),
                ))
            }),
        },
        Rule {
            name: "€".to_string(),
            pattern: vec![regex("евр(о|а)")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::EUR,
                )))
            }),
        },
        Rule {
            name: "£".to_string(),
            pattern: vec![regex("паунд(а|и)?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Pound,
                )))
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
            name: "intersect (and number)".to_string(),
            pattern: vec![
                predicate(is_without_cents),
                regex("и"),
                predicate(is_natural),
            ],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[0].token_data)?;
                let c = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::AmountOfMoney(d.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "intersect (and X cents)".to_string(),
            pattern: vec![predicate(is_without_cents), regex("и"), predicate(is_cents)],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[0].token_data)?;
                let c = money_data(&nodes[2].token_data)?.value?;
                Some(TokenData::AmountOfMoney(d.clone().with_cents(c)))
            }),
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
            name: "between|from <numeral> to|and <amount-of-money>".to_string(),
            pattern: vec![
                regex("между|от"),
                predicate(is_positive),
                regex("до|и"),
                predicate(is_simple_amount_of_money),
            ],
            production: Box::new(|nodes| {
                let from = numeral_data(&nodes[1].token_data)?.value;
                let to_data = money_data(&nodes[3].token_data)?;
                let to = to_data.value?;
                if from < to {
                    Some(TokenData::AmountOfMoney(
                        AmountOfMoneyData::currency_only(to_data.currency).with_interval(from, to),
                    ))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "between|from <amount-of-money> to|and <amount-of-money>".to_string(),
            pattern: vec![
                regex("между|от"),
                predicate(is_simple_amount_of_money),
                regex("до|и"),
                predicate(is_simple_amount_of_money),
            ],
            production: Box::new(|nodes| {
                let d1 = money_data(&nodes[1].token_data)?;
                let d2 = money_data(&nodes[3].token_data)?;
                if d1.currency == d2.currency && d1.value? < d2.value? {
                    Some(TokenData::AmountOfMoney(
                        AmountOfMoneyData::currency_only(d1.currency)
                            .with_interval(d1.value?, d2.value?),
                    ))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "<numeral> - <amount-of-money>".to_string(),
            pattern: vec![
                predicate(is_positive),
                regex("-"),
                predicate(is_simple_amount_of_money),
            ],
            production: Box::new(|nodes| {
                let from = numeral_data(&nodes[0].token_data)?.value;
                let d = money_data(&nodes[2].token_data)?;
                if from < d.value? {
                    Some(TokenData::AmountOfMoney(
                        AmountOfMoneyData::currency_only(d.currency).with_interval(from, d.value?),
                    ))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "<amount-of-money> - <amount-of-money>".to_string(),
            pattern: vec![
                predicate(is_simple_amount_of_money),
                regex("-"),
                predicate(is_simple_amount_of_money),
            ],
            production: Box::new(|nodes| {
                let d1 = money_data(&nodes[0].token_data)?;
                let d2 = money_data(&nodes[2].token_data)?;
                if d1.currency == d2.currency && d1.value? < d2.value? {
                    Some(TokenData::AmountOfMoney(
                        AmountOfMoneyData::currency_only(d1.currency)
                            .with_interval(d1.value?, d2.value?),
                    ))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "under/less/lower/no more than <amount-of-money>".to_string(),
            pattern: vec![
                regex("под|по-малко от|не повече от"),
                predicate(is_simple_amount_of_money),
            ],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[1].token_data)?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_max(d.value?),
                ))
            }),
        },
        Rule {
            name: "over/above/at least/more than <amount-of-money>".to_string(),
            pattern: vec![
                regex("над|поне|повече от"),
                predicate(is_simple_amount_of_money),
            ],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[1].token_data)?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_min(d.value?),
                ))
            }),
        },
        Rule {
            name: "about|exactly <amount-of-money>".to_string(),
            pattern: vec![
                regex("точно|около|приблизително|близо (до)?|почти"),
                predicate(is_money_with_value),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
    ]
}
