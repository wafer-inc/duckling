use crate::dimensions::numeral::helpers::{is_natural, is_positive, numeral_data};
use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::{AmountOfMoneyData, Currency};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn money_data(td: &TokenData) -> Option<&AmountOfMoneyData> {
    match td {
        TokenData::AmountOfMoney(d) => Some(d),
        _ => None,
    }
}

fn is_currency_only(td: &TokenData) -> bool {
    match td {
        TokenData::AmountOfMoney(d) => {
            d.value.is_none() && d.min_value.is_none() && d.max_value.is_none()
        }
        _ => false,
    }
}

fn is_simple_money(td: &TokenData) -> bool {
    match td {
        TokenData::AmountOfMoney(d) => {
            d.value.is_some() && d.min_value.is_none() && d.max_value.is_none()
        }
        _ => false,
    }
}

fn is_money_with_value(td: &TokenData) -> bool {
    match td {
        TokenData::AmountOfMoney(d) => {
            d.value.is_some() || d.min_value.is_some() || d.max_value.is_some()
        }
        _ => false,
    }
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
    match td {
        TokenData::AmountOfMoney(d) => d.currency == Currency::Cent && d.value.is_some(),
        _ => false,
    }
}

fn lookup_currency(s: &str) -> Option<Currency> {
    match s {
        "aed" => Some(Currency::AED),
        "aud" => Some(Currency::AUD),
        "bgn" => Some(Currency::BGN),
        "brl" => Some(Currency::BRL),
        "byn" => Some(Currency::BYN),
        "cad" => Some(Currency::CAD),
        "\u{00a2}" | "c" => Some(Currency::Cent),
        "chf" => Some(Currency::CHF),
        "cny" | "rmb" | "yuan" => Some(Currency::CNY),
        "czk" => Some(Currency::CZK),
        "$" => Some(Currency::Dollar),
        "dinar" | "dinars" => Some(Currency::Dinar),
        "dkk" => Some(Currency::DKK),
        "dollar" | "dollars" => Some(Currency::Dollar),
        "egp" => Some(Currency::EGP),
        "\u{20ac}" | "x20ac" | "eur" | "euro" | "euros" | "eurs" | "\u{20ac}ur" | "\u{20ac}uro"
        | "\u{20ac}uros" | "\u{20ac}urs" => Some(Currency::EUR),
        "gbp" => Some(Currency::GBP),
        "gel" | "lari" | "\u{20be}" => Some(Currency::GEL),
        "hkd" => Some(Currency::HKD),
        "hrk" => Some(Currency::HRK),
        "idr" => Some(Currency::IDR),
        "ils" | "\u{20aa}" | "nis" | "shekel" | "shekels" => Some(Currency::ILS),
        "inr" | "rs" | "rs." | "rupee" | "rupees" => Some(Currency::INR),
        "iqd" => Some(Currency::IQD),
        "jmd" => Some(Currency::JMD),
        "jod" => Some(Currency::JOD),
        "\u{00a5}" | "jpy" | "yen" => Some(Currency::JPY),
        "krw" => Some(Currency::KRW),
        "kwd" => Some(Currency::KWD),
        "lbp" => Some(Currency::LBP),
        "mad" => Some(Currency::MAD),
        "\u{20ae}" | "mnt" | "tugrik" | "tugriks" => Some(Currency::MNT),
        "myr" | "rm" => Some(Currency::MYR),
        "nok" => Some(Currency::NOK),
        "nzd" => Some(Currency::NZD),
        "\u{00a3}" => Some(Currency::Pound),
        "pkr" => Some(Currency::PKR),
        "pln" => Some(Currency::PLN),
        "pt" | "pts" | "pta" | "ptas" => Some(Currency::PTS),
        "qar" => Some(Currency::QAR),
        "rial" | "rials" => Some(Currency::Rial),
        "riyal" | "riyals" => Some(Currency::Riyal),
        "ron" => Some(Currency::RON),
        "\u{20bd}" | "rub" => Some(Currency::RUB),
        "sar" => Some(Currency::SAR),
        "sek" => Some(Currency::SEK),
        "sgd" => Some(Currency::SGD),
        "thb" => Some(Currency::THB),
        "ttd" => Some(Currency::TTD),
        "\u{20b4}" | "uah" => Some(Currency::UAH),
        "usd" | "us$" => Some(Currency::USD),
        "vnd" => Some(Currency::VND),
        "zar" => Some(Currency::ZAR),
        "tl" | "lira" | "\u{20ba}" => Some(Currency::TRY),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Rules
// ---------------------------------------------------------------------------

pub fn rules() -> Vec<Rule> {
    vec![
        // === Common rules (from AmountOfMoney/Rules.hs) ===

        // currencies: matches all currency symbols and codes
        Rule {
            name: "currencies".to_string(),
            pattern: vec![regex(
                r"(aed|aud|bgn|brl|byn|\u{00a2}|cad|chf|cny|czk|c|\$|dinars?|dkk|dollars?|egp|(e|\u{20ac}|x20ac)uro?s?|\u{20ac}|x20ac|gbp|gel|\u{20be}|hkd|hrk|idr|ils|\u{20aa}|inr|iqd|jmd|jod|\u{00a5}|jpy|lari|krw|kwd|lbp|mad|\u{20ae}|mnt|tugriks?|myr|rm|nis|nok|nzd|\u{00a3}|pkr|pln|pta?s?|qar|\u{20bd}|rs\.?|riy?als?|ron|rub|rupees?|sar|sek|sgd|shekels?|thb|ttd|\u{20b4}|uah|us(d|\$)|vnd|yen|yuan|zar|tl|lira|\u{20ba})",
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let c = lookup_currency(&text.to_lowercase())?;
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    c,
                )))
            }),
        },
        // <amount> <unit>: "10 dollars", "3.14 EUR"
        Rule {
            name: "<amount> <unit>".to_string(),
            pattern: vec![predicate(is_positive), predicate(is_currency_only)],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let d = money_data(&nodes[1].token_data)?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_value(num.value),
                ))
            }),
        },
        // <unit> <amount>: "$10", "EUR 20"
        Rule {
            name: "<unit> <amount>".to_string(),
            pattern: vec![predicate(is_currency_only), predicate(is_positive)],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[0].token_data)?;
                let num = numeral_data(&nodes[1].token_data)?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_value(num.value),
                ))
            }),
        },
        // === EN-specific rules (from AmountOfMoney/EN/Rules.hs) ===

        // pounds: "pound", "pounds"
        Rule {
            name: "pounds".to_string(),
            pattern: vec![regex(r"pounds?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Pound,
                )))
            }),
        },
        // other pounds: "egyptian pound", "lebanese pounds"
        Rule {
            name: "other pounds".to_string(),
            pattern: vec![regex(r"(egyptian|lebanese) ?pounds?")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let kind = m.group(1)?.to_lowercase();
                let c = match kind.as_str() {
                    "egyptian" => Currency::EGP,
                    "lebanese" => Currency::LBP,
                    _ => return None,
                };
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    c,
                )))
            }),
        },
        // EGP abbreviation: "LE", "L.E", "l.e."
        Rule {
            name: "EGP abbreviation".to_string(),
            pattern: vec![regex(r"[lL].?[eE].?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::EGP,
                )))
            }),
        },
        // EGP arabizi: "geneh", "genihat masriya"
        Rule {
            name: "EGP arabizi".to_string(),
            pattern: vec![regex(r"[Gg][eiy]*n[eiy]*h(at)?( m[aiey]?sr[eiy]+a?)?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::EGP,
                )))
            }),
        },
        // riyals: "qatari riyal", "saudi riyals"
        Rule {
            name: "riyals".to_string(),
            pattern: vec![regex(r"(qatari|saudi) ?riyals?")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let kind = m.group(1)?.to_lowercase();
                let c = match kind.as_str() {
                    "qatari" => Currency::QAR,
                    "saudi" => Currency::SAR,
                    _ => return None,
                };
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    c,
                )))
            }),
        },
        // dinars: "kuwaiti dinar"
        Rule {
            name: "dinars".to_string(),
            pattern: vec![regex(r"(kuwaiti) ?dinars?")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let kind = m.group(1)?.to_lowercase();
                let c = match kind.as_str() {
                    "kuwaiti" => Currency::KWD,
                    _ => return None,
                };
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    c,
                )))
            }),
        },
        // dirham: "dirhams"
        Rule {
            name: "dirham".to_string(),
            pattern: vec![regex(r"dirhams?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::AED,
                )))
            }),
        },
        // ringgit: "ringgit", "malaysian ringgit", "malaysia ringgits"
        Rule {
            name: "ringgit".to_string(),
            pattern: vec![regex(r"(malaysian? )?ringgits?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::MYR,
                )))
            }),
        },
        // hryvnia
        Rule {
            name: "hryvnia".to_string(),
            pattern: vec![regex(r"hryvnia")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::UAH,
                )))
            }),
        },
        // cent: "cents", "penny", "pennies", "pence", "sen"
        Rule {
            name: "cent".to_string(),
            pattern: vec![regex(r"cents?|penn(y|ies)|pence|sens?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Cent,
                )))
            }),
        },
        // kopiyka
        Rule {
            name: "kopiyka".to_string(),
            pattern: vec![regex(r"kopiy(ok|kas?)")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Cent,
                )))
            }),
        },
        // bucks
        Rule {
            name: "bucks".to_string(),
            pattern: vec![regex(r"bucks?")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(AmountOfMoneyData::currency_only(
                    Currency::Unnamed,
                )))
            }),
        },
        // a grand
        Rule {
            name: "a grand".to_string(),
            pattern: vec![regex(r"a grand")],
            production: Box::new(|_| {
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(Currency::Unnamed).with_value(1000.0),
                ))
            }),
        },
        // <amount> grand
        Rule {
            name: "<amount> grand".to_string(),
            pattern: vec![predicate(is_positive), regex(r"grand")],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(Currency::Unnamed)
                        .with_value(1000.0 * num.value),
                ))
            }),
        },
        // dollar coin
        Rule {
            name: "dollar coin".to_string(),
            pattern: vec![regex(r"(nickel|dime|quarter)s?")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let coin = m.group(1)?.to_lowercase();
                let value = match coin.as_str() {
                    "nickel" => 0.05,
                    "dime" => 0.1,
                    "quarter" => 0.25,
                    _ => return None,
                };
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(Currency::Dollar).with_value(value),
                ))
            }),
        },
        // a <currency>: "a dollar", "an euro"
        Rule {
            name: "a <currency>".to_string(),
            pattern: vec![regex(r"an?"), predicate(is_currency_only)],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[1].token_data)?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_value(1.0),
                ))
            }),
        },
        // a <amount-of-money>: absorbs "a" before simple money
        Rule {
            name: "a <amount-of-money>".to_string(),
            pattern: vec![regex(r"an?"), predicate(is_simple_money)],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        // intersect (and X cents): "$20 and 43 cents"
        Rule {
            name: "intersect (and X cents)".to_string(),
            pattern: vec![
                predicate(is_without_cents),
                regex(r"and"),
                predicate(is_cents),
            ],
            production: Box::new(|nodes| {
                let fd = money_data(&nodes[0].token_data)?;
                let cents = money_data(&nodes[2].token_data)?;
                let c = cents.value?;
                Some(TokenData::AmountOfMoney(fd.clone().with_cents(c)))
            }),
        },
        // intersect: "$20 43" (no connector, natural number as cents)
        Rule {
            name: "intersect".to_string(),
            pattern: vec![
                predicate(is_without_cents),
                predicate(|td| {
                    is_natural(td) && matches!(td, TokenData::Numeral(d) if d.value < 100.0)
                }),
            ],
            production: Box::new(|nodes| {
                let fd = money_data(&nodes[0].token_data)?;
                let num = numeral_data(&nodes[1].token_data)?;
                Some(TokenData::AmountOfMoney(fd.clone().with_cents(num.value)))
            }),
        },
        // intersect (and number): "$20 and 43"
        Rule {
            name: "intersect (and number)".to_string(),
            pattern: vec![
                predicate(is_without_cents),
                regex(r"and"),
                predicate(|td| {
                    is_natural(td) && matches!(td, TokenData::Numeral(d) if d.value < 100.0)
                }),
            ],
            production: Box::new(|nodes| {
                let fd = money_data(&nodes[0].token_data)?;
                let num = numeral_data(&nodes[2].token_data)?;
                Some(TokenData::AmountOfMoney(fd.clone().with_cents(num.value)))
            }),
        },
        // intersect (X cents): "20 dollars 43 cents"
        Rule {
            name: "intersect (X cents)".to_string(),
            pattern: vec![predicate(is_without_cents), predicate(is_cents)],
            production: Box::new(|nodes| {
                let fd = money_data(&nodes[0].token_data)?;
                let cents = money_data(&nodes[1].token_data)?;
                let c = cents.value?;
                Some(TokenData::AmountOfMoney(fd.clone().with_cents(c)))
            }),
        },
        // about|exactly <amount-of-money>
        Rule {
            name: "about|exactly <amount-of-money>".to_string(),
            pattern: vec![
                regex(
                    r"exactly|precisely|about|approx(\.|imately)?|close to|near( to)?|around|almost",
                ),
                predicate(is_money_with_value),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        // between|from <numeral> to|and <amount-of-money>
        Rule {
            name: "between|from <numeral> to|and <amount-of-money>".to_string(),
            pattern: vec![
                regex(r"between|from"),
                predicate(is_positive),
                regex(r"to|and"),
                predicate(is_simple_money),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[1].token_data)?;
                let d = money_data(&nodes[3].token_data)?;
                let to = d.value?;
                if num.value >= to {
                    return None;
                }
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_interval(num.value, to),
                ))
            }),
        },
        // between|from <amount-of-money> to|and <amount-of-money>
        Rule {
            name: "between|from <amount> to|and <amount>".to_string(),
            pattern: vec![
                regex(r"between|from"),
                predicate(is_simple_money),
                regex(r"to|and"),
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
        // <numeral> - <amount-of-money>
        Rule {
            name: "<numeral> - <amount-of-money>".to_string(),
            pattern: vec![
                predicate(is_natural),
                regex(r"-"),
                predicate(is_simple_money),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let d = money_data(&nodes[2].token_data)?;
                let to = d.value?;
                if num.value >= to {
                    return None;
                }
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_interval(num.value, to),
                ))
            }),
        },
        // <amount-of-money> - <amount-of-money>
        Rule {
            name: "<amount-of-money> - <amount-of-money>".to_string(),
            pattern: vec![
                predicate(is_simple_money),
                regex(r"-"),
                predicate(is_simple_money),
            ],
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
        // under/less/lower/no more than <amount-of-money>
        Rule {
            name: "under|less than <amount-of-money>".to_string(),
            pattern: vec![
                regex(r"under|at most|(less|lower|no more) than"),
                predicate(is_simple_money),
            ],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[1].token_data)?;
                let to = d.value?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_max(to),
                ))
            }),
        },
        // over/above/at least/more than <amount-of-money>
        Rule {
            name: "over|more than <amount-of-money>".to_string(),
            pattern: vec![
                regex(r"over|above|at least|(more|no less) than"),
                predicate(is_simple_money),
            ],
            production: Box::new(|nodes| {
                let d = money_data(&nodes[1].token_data)?;
                let to = d.value?;
                Some(TokenData::AmountOfMoney(
                    AmountOfMoneyData::currency_only(d.currency).with_min(to),
                ))
            }),
        },
    ]
}

fn is_common_rule_name(name: &str) -> bool {
    matches!(
        name,
        "currencies" | "<amount> <unit>" | "<unit> <amount>" | "<amount> (latent)"
    )
}

pub fn common_rules() -> Vec<Rule> {
    rules()
        .into_iter()
        .filter(|r| is_common_rule_name(&r.name))
        .collect()
}

pub fn lang_rules() -> Vec<Rule> {
    rules()
        .into_iter()
        .filter(|r| !is_common_rule_name(&r.name))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::dimensions::numeral;
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::types::DimensionKind;

    #[test]
    fn test_money() {
        let mut rules = numeral::en::rules();
        rules.extend(super::rules());
        let options = Options { with_latent: false };
        let context = Context::default();

        for (text, expected_val, expected_unit) in &[
            ("$10", 10.0, "USD"),
            ("$3.50", 3.5, "USD"),
            ("20 euros", 20.0, "EUR"),
            ("ten pounds", 10.0, "GBP"),
            ("42 bucks", 42.0, "USD"),
        ] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::AmountOfMoney],
            );
            let found = entities.iter().any(|e| match &e.value {
                crate::types::DimensionValue::AmountOfMoney(
                    crate::types::MeasurementValue::Value { value, unit },
                ) => (*value - expected_val).abs() < 0.01 && unit == *expected_unit,
                _ => false,
            });
            assert!(
                found,
                "Expected {} {} for '{}', got: {:?}",
                expected_val, expected_unit, text, entities
            );
        }
    }
}
