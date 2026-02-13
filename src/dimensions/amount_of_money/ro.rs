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
            name: "<amount >= 20> de <unit>".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value >= 20.0)),
                regex("de"),
                predicate(is_currency_only),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let c = money_data(&nodes[2].token_data)?.currency;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(c).with_value(v),
                ))
            }),
        },
        Rule {
            name: "AED".to_string(),
            pattern: vec![regex("dirhami?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::AED,
                )))
            }),
        },
        Rule {
            name: "cent|bani".to_string(),
            pattern: vec![regex("bani?|cen(t|ț)i?|c|¢")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Cent,
                )))
            }),
        },
        Rule {
            name: "$".to_string(),
            pattern: vec![regex("dolari?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Dollar,
                )))
            }),
        },
        Rule {
            name: "INR".to_string(),
            pattern: vec![regex("rupii?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::INR,
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
                regex("(s|ș)i"),
                predicate(is_natural),
            ],
            production: Box::new(|nodes| {
                let m = money_data(&nodes[0].token_data)?;
                let c = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::AmountOfMoney(m.clone().with_cents(c)))
            }),
        },
        Rule {
            name: "intersect (and X cents)".to_string(),
            pattern: vec![predicate(is_without_cents), regex("(s|ș)i"), predicate(is_cents)],
            production: Box::new(|nodes| {
                let m = money_data(&nodes[0].token_data)?;
                let c = money_data(&nodes[2].token_data)?.value?;
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
            name: "KWD".to_string(),
            pattern: vec![regex("dinar kuweitian")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::KWD,
                )))
            }),
        },
        Rule {
            name: "other pounds".to_string(),
            pattern: vec![regex("lir(a|ă) (egiptian|libanez)(a|ă)")],
            production: Box::new(|nodes| {
                let kind = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?.to_lowercase(),
                    _ => return None,
                };
                let c = match kind.as_str() {
                    "egiptian" => Currency::EGP,
                    "libanez" => Currency::LBP,
                    _ => return None,
                };
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(c)))
            }),
        },
        Rule {
            name: "£".to_string(),
            pattern: vec![regex("lire?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Pound,
                )))
            }),
        },
        Rule {
            name: "about/exactly <amount-of-money>".to_string(),
            pattern: vec![
                regex("exact|cam|aprox(\\.|imativ)?|(aproape|(i|î)n jur)( de)?"),
                predicate(is_money_with_value),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "riyals".to_string(),
            pattern: vec![regex("rial (saudit?|qatarian?)")],
            production: Box::new(|nodes| {
                let kind = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let c = match kind.as_str() {
                    "saudi" | "saudit" => Currency::SAR,
                    "qataria" | "qatarian" => Currency::QAR,
                    _ => return None,
                };
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(c)))
            }),
        },
        Rule {
            name: "RON".to_string(),
            pattern: vec![regex("roni|lei")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::RON,
                )))
            }),
        },
        Rule {
            name: "between|from <amount-of-money> to|and <amount-of-money>".to_string(),
            pattern: vec![
                regex("intre|de la"),
                predicate(is_simple_money),
                regex("[sș]i|la"),
                predicate(is_simple_money),
            ],
            production: Box::new(|nodes| {
                let d1 = money_data(&nodes[1].token_data)?;
                let d2 = money_data(&nodes[3].token_data)?;
                let from = d1.value?;
                let to = d2.value?;
                if from >= to || d1.currency != d2.currency {
                    return None;
                }
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d1.currency).with_interval(from, to),
                ))
            }),
        },
        Rule {
            name: "between|from <numeral> to|and <amount-of-money>".to_string(),
            pattern: vec![
                regex("intre|de la"),
                predicate(is_positive),
                regex("[sș]i|la"),
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
            name: "<amount-of-money> - <amount-of-money>".to_string(),
            pattern: vec![predicate(is_simple_money), regex("-"), predicate(is_simple_money)],
            production: Box::new(|nodes| {
                let d1 = money_data(&nodes[0].token_data)?;
                let d2 = money_data(&nodes[2].token_data)?;
                let from = d1.value?;
                let to = d2.value?;
                if from >= to || d1.currency != d2.currency {
                    return None;
                }
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d1.currency).with_interval(from, to),
                ))
            }),
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
            name: "under/less/lower/no more than <amount-of-money>".to_string(),
            pattern: vec![
                regex("sub|mai (pu[tț]|ieft)in de|nu chiar|nici macar|cel mult"),
                predicate(is_simple_money),
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
                regex("peste|mai (mult|scump) de|cel pu[tț]in"),
                predicate(is_simple_money),
            ],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[1].token_data)?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_min(d.value?),
                ))
            }),
        },
    ]
}
