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
    matches!(td, TokenData::AmountOfMoney(d) if d.value.is_none() && d.min_value.is_none() && d.max_value.is_none())
}

fn is_simple_money(td: &TokenData) -> bool {
    matches!(td, TokenData::AmountOfMoney(d) if d.value.is_some() && d.min_value.is_none() && d.max_value.is_none())
}

fn is_money_with_value(td: &TokenData) -> bool {
    matches!(td, TokenData::AmountOfMoney(d) if d.value.is_some() || d.min_value.is_some() || d.max_value.is_some())
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "<unit> <amount>".to_string(),
            pattern: vec![predicate(is_currency_only), predicate(is_positive)],
            production: Box::new(|nodes| {
                let c = money_data(&nodes[0].token_data)?.currency;
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(c).with_value(v)))
            }),
        },
        Rule {
            name: "төг".to_string(),
            pattern: vec![regex("төг(рөг(ийн)?)?")],
            production: Box::new(|_| Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(Currency::MNT)))),
        },
        Rule {
            name: "£".to_string(),
            pattern: vec![regex("фунт(аар|тай|аас)?|£|x00a3\\&?")],
            production: Box::new(|_| Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(Currency::Pound)))),
        },
        Rule {
            name: "Mongolian GBP".to_string(),
            pattern: vec![regex("Английн\\s+фунт")],
            production: Box::new(|_| Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(Currency::GBP)))),
        },
        Rule {
            name: "$".to_string(),
            pattern: vec![regex("доллар(ын|оор|оос|той)?")],
            production: Box::new(|_| Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(Currency::Dollar)))),
        },
        Rule {
            name: "cent".to_string(),
            pattern: vec![regex("цент(ийн|ээс|ээр|тэй)?|пени|пенс(ээр|гээр|тэй|ээс|гээс)?|ц")],
            production: Box::new(|_| Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(Currency::Cent)))),
        },
        Rule {
            name: "€".to_string(),
            pattern: vec![regex("евро")],
            production: Box::new(|_| Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(Currency::EUR)))),
        },
        Rule {
            name: "bucks".to_string(),
            pattern: vec![regex("бакс(аар|тай|аас)?")],
            production: Box::new(|_| Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(Currency::Unnamed)))),
        },
        Rule {
            name: "about|exactly <amount-of-money>".to_string(),
            pattern: vec![regex("яг|ойролцоогоор|бараг"), predicate(is_money_with_value)],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "between|from <numeral> to <amount-of-money>".to_string(),
            pattern: vec![predicate(is_positive), regex("-c"), predicate(is_simple_money), regex("(-н\\s+)?(хооронд|хүртэл)")],
            production: Box::new(|nodes| {
                let from = numeral_data(&nodes[0].token_data)?.value;
                let d = money_data(&nodes[2].token_data)?;
                let to = d.value?;
                if from >= to { return None; }
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(d.currency).with_interval(from, to)))
            }),
        },
        Rule {
            name: "between|from <amount-of-money> to <numeral>".to_string(),
            pattern: vec![predicate(is_simple_money), regex("-c"), predicate(is_natural), regex("(-н\\s+)?(хооронд|хүртэл)")],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[0].token_data)?;
                let from = d.value?;
                let to = numeral_data(&nodes[2].token_data)?.value;
                if from >= to { return None; }
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(d.currency).with_interval(from, to)))
            }),
        },
        Rule {
            name: "between|from <amount-of-money> to <amount-of-money>".to_string(),
            pattern: vec![predicate(is_simple_money), regex("-c"), predicate(is_simple_money), regex("(-н\\s+)?(хооронд|хүртэл)")],
            production: Box::new(|nodes| {
                let d1 = money_data(&nodes[0].token_data)?;
                let d2 = money_data(&nodes[2].token_data)?;
                if d1.currency != d2.currency || d1.value? >= d2.value? { return None; }
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(d1.currency).with_interval(d1.value?, d2.value?)))
            }),
        },
        Rule {
            name: "<numeral> - <amount-of-money>".to_string(),
            pattern: vec![predicate(is_positive), regex("-"), predicate(is_simple_money)],
            production: Box::new(|nodes| {
                let from = numeral_data(&nodes[0].token_data)?.value;
                let d = money_data(&nodes[2].token_data)?;
                if from >= d.value? { return None; }
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(d.currency).with_interval(from, d.value?)))
            }),
        },
        Rule {
            name: "<amount-of-money> - <amount-of-money>".to_string(),
            pattern: vec![predicate(is_simple_money), regex("-"), predicate(is_simple_money)],
            production: Box::new(|nodes| {
                let d1 = money_data(&nodes[0].token_data)?;
                let d2 = money_data(&nodes[2].token_data)?;
                if d1.currency != d2.currency || d1.value? >= d2.value? { return None; }
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(d1.currency).with_interval(d1.value?, d2.value?)))
            }),
        },
        Rule {
            name: "interval max".to_string(),
            pattern: vec![predicate(is_simple_money), regex("-c\\s+(бага|доогуур|ихгүй)")],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[0].token_data)?;
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(d.currency).with_max(d.value?)))
            }),
        },
        Rule {
            name: "interval min".to_string(),
            pattern: vec![predicate(is_simple_money), regex("-c\\s+(их|дээгүүр|илүү)")],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[0].token_data)?;
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(d.currency).with_min(d.value?)))
            }),
        },
    ]
}
