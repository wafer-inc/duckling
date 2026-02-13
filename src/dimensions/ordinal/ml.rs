use std::collections::HashMap;

use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn one_to_nineteen_map() -> HashMap<&'static str, i64> {
    HashMap::from([
        ("ഒന്നാം", 1),
        ("രണ്ടാം", 2),
        ("മൂന്നാം", 3),
        ("നാലാം", 4),
        ("അഞ്ചാം", 5),
        ("ആറാം", 6),
        ("ഏഴാം", 7),
        ("എട്ടാം", 8),
        ("ഒമ്പതാം", 9),
        ("പത്താം", 10),
        ("പതിനൊന്നാം", 11),
        ("പന്ത്രണ്ടാം", 12),
        ("പതിമൂന്നാം", 13),
        ("പതിനാലാം", 14),
        ("പതിനഞ്ചാം", 15),
        ("പതിനാറാം", 16),
        ("പതിനേഴാം", 17),
        ("പതിനെട്ടാം", 18),
        ("പത്തൊൻപതാം", 19),
    ])
}

fn tens_map() -> HashMap<&'static str, i64> {
    HashMap::from([
        ("ഇരുപതാം", 20),
        ("മുപ്പത്തഞ്ചാം", 30),
        ("നാല്പതാം", 40),
        ("അമ്പതാം", 50),
        ("അറുപതാം", 60),
        ("എഴുപത്താം", 70),
        ("എൺപത്താം", 80),
        ("തൊണ്ണൂറാം", 90),
    ])
}

fn tens_ordinal_map() -> HashMap<&'static str, i64> {
    HashMap::from([
        ("ഇരുപത്തി", 20),
        ("മുപ്പത്തി", 30),
        ("നാല്പത്തി", 40),
        ("അമ്പത്തി", 50),
        ("അറുപത്തി", 60),
        ("എഴുപത്തി", 70),
        ("എൺപത്തി", 80),
        ("തൊണ്ണൂറ്റി", 90),
    ])
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("0*(\\d+)\\.")],
            production: Box::new(|nodes| {
                let value: i64 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.parse().ok()?,
                    _ => return None,
                };
                Some(TokenData::Ordinal(OrdinalData::new(value)))
            }),
        },
        Rule {
            name: "integer (1..19)".to_string(),
            pattern: vec![regex("(ഒന്നാം|രണ്ടാം|മൂന്നാം|നാലാം|അഞ്ചാം|ആറാം|ഏഴാം|എട്ടാം|ഒമ്പതാം|പത്താം|പതിനൊന്നാം|പന്ത്രണ്ടാം|പതിമൂന്നാം|പതിനാലാം|പതിനഞ്ചാം|പതിനാറാം|പതിനേഴാം|പതിനെട്ടാം|പത്തൊൻപതാം)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let map = one_to_nineteen_map();
                Some(TokenData::Ordinal(OrdinalData::new(*map.get(text)?)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(ഇരുപതാം|മുപ്പത്തഞ്ചാം|നാല്പതാം|അമ്പതാം|അറുപതാം|എഴുപത്താം|എൺപത്താം|തൊണ്ണൂറാം)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let map = tens_map();
                Some(TokenData::Ordinal(OrdinalData::new(*map.get(text)?)))
            }),
        },
        Rule {
            name: "integer ([2-9][1-9])".to_string(),
            pattern: vec![regex("(ഇരുപത്തി|മുപ്പത്തി|നാല്പത്തി|അമ്പത്തി|അറുപത്തി|എഴുപത്തി|എൺപത്തി|തൊണ്ണൂറ്റി)(ആദ്യം|രണ്ടാം|മൂന്നാം|നാലാം|അഞ്ചാം|ആറാം|ഏഴാം|എട്ടാം|ഒമ്പത)")],
            production: Box::new(|nodes| {
                let m1 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let m2 = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(2)?,
                    _ => return None,
                };
                let tens = tens_ordinal_map();
                let units = one_to_nineteen_map();
                Some(TokenData::Ordinal(OrdinalData::new(
                    tens.get(m1)? + units.get(m2)?,
                )))
            }),
        },
    ]
}
