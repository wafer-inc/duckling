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
            name: "a <currency>".to_string(),
            pattern: vec![regex("jed(an|na|no)"), predicate(is_currency_only)],
            production: Box::new(|nodes| {
                let c = money_data(&nodes[1].token_data)?.currency;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(c).with_value(1.0),
                ))
            }),
        },
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
            name: "intersect (and number)".to_string(),
            pattern: vec![predicate(is_without_cents), regex("i"), predicate(is_natural)],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[0].token_data)?;
                let c = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::AmountOfMoney(d.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "SAR".to_string(),
            pattern: vec![regex("saudijskirijal|saudi rijal?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::SAR,
                )))
            }),
        },
        Rule {
            name: "$".to_string(),
            pattern: vec![regex("dolar(a|i|e)?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Dollar,
                )))
            }),
        },
        Rule {
            name: "cent".to_string(),
            pattern: vec![regex("cent(i|a)?|penij(i|a)?|c|¢|lp|lip(a|e)")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Cent,
                )))
            }),
        },
        Rule {
            name: "intersect (i X lipa)".to_string(),
            pattern: vec![predicate(is_without_cents), regex("i"), predicate(is_cents)],
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
            name: "£".to_string(),
            pattern: vec![regex("funt(a|e|i)?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Pound,
                )))
            }),
        },
        Rule {
            name: "HRK".to_string(),
            pattern: vec![regex("kn|(hrvatsk(a|ih|e) )?kun(a|e)")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::HRK,
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
            name: "other pounds".to_string(),
            pattern: vec![regex("(egipatska|libanonska) ?funta")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let c = match t.as_str() {
                    "egipatska" => Currency::EGP,
                    "libanonska" => Currency::LBP,
                    _ => return None,
                };
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(c)))
            }),
        },
        Rule {
            name: "INR".to_string(),
            pattern: vec![regex("rupija?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::INR,
                )))
            }),
        },
        Rule {
            name: "KWD".to_string(),
            pattern: vec![regex("kuvajtski ?dinar")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::KWD,
                )))
            }),
        },
        Rule {
            name: "QAR".to_string(),
            pattern: vec![regex("katarski(i| )rijal")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::QAR,
                )))
            }),
        },
        Rule {
            name: "AED".to_string(),
            pattern: vec![regex("drahma?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::AED,
                )))
            }),
        },
        Rule {
            name: "about|exactly <amount-of-money>".to_string(),
            pattern: vec![regex("oko|otprilike|u blizini|skoro|približno"), predicate(is_money_with_value)],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "<numeral> - <amount-of-money>".to_string(),
            pattern: vec![predicate(is_natural), regex("-"), predicate(is_simple_money)],
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
            pattern: vec![predicate(is_simple_money), regex("-"), predicate(is_simple_money)],
            production: Box::new(|nodes| {
                let d1 = money_data(&nodes[0].token_data)?;
                let d2 = money_data(&nodes[2].token_data)?;
                if d1.currency != d2.currency || d1.value? >= d2.value? {
                    return None;
                }
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d1.currency).with_interval(d1.value?, d2.value?),
                ))
            }),
        },
        Rule {
            name: "between|from <numeral> to|and <amount-of-money>".to_string(),
            pattern: vec![regex("od|otprilike|približno"), predicate(is_positive), regex("do"), predicate(is_simple_money)],
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
            name: "between|from <numeral> and <amount-of-money>".to_string(),
            pattern: vec![regex("izmedju"), predicate(is_positive), regex("i"), predicate(is_simple_money)],
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
            pattern: vec![regex("od|otprilike"), predicate(is_simple_money), regex("do"), predicate(is_simple_money)],
            production: Box::new(|nodes| {
                let d1 = money_data(&nodes[1].token_data)?;
                let d2 = money_data(&nodes[3].token_data)?;
                if d1.currency != d2.currency || d1.value? >= d2.value? {
                    return None;
                }
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d1.currency).with_interval(d1.value?, d2.value?),
                ))
            }),
        },
        Rule {
            name: "between <amount-of-money> and <amount-of-money>".to_string(),
            pattern: vec![regex("izmedju"), predicate(is_simple_money), regex("i"), predicate(is_simple_money)],
            production: Box::new(|nodes| {
                let d1 = money_data(&nodes[1].token_data)?;
                let d2 = money_data(&nodes[3].token_data)?;
                if d1.currency != d2.currency || d1.value? >= d2.value? {
                    return None;
                }
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d1.currency).with_interval(d1.value?, d2.value?),
                ))
            }),
        },
        Rule {
            name: "less/no more than <amount-of-money>".to_string(),
            pattern: vec![regex("manje od|ispod"), predicate(is_simple_money)],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[1].token_data)?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_max(d.value?),
                ))
            }),
        },
        Rule {
            name: "more than <amount-of-money>".to_string(),
            pattern: vec![regex("više od|najmanje|preko|iznad"), predicate(is_simple_money)],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[1].token_data)?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_min(d.value?),
                ))
            }),
        },
    ]
}
