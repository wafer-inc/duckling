use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn zero_to_nine(s: &str) -> Option<f64> {
    match s {
        "പൂജ്യം" => Some(0.0),
        "ഒന്ന്" => Some(1.0),
        "രണ്ട്" => Some(2.0),
        "മുന്ന്" => Some(3.0),
        "നാല്" => Some(4.0),
        "അഞ്ച്" => Some(5.0),
        "ആറ്" => Some(6.0),
        "ഏഴ്" => Some(7.0),
        "എട്ട്" => Some(8.0),
        "ഒൻപത്" => Some(9.0),
        _ => None,
    }
}

fn ten_to_nineteen(s: &str) -> Option<f64> {
    match s {
        "പത്ത്" => Some(10.0),
        "പതിനൊന്ന്" => Some(11.0),
        "പന്ത്രണ്ട്" => Some(12.0),
        "പതിമൂന്ന്" => Some(13.0),
        "പതിനാല്" => Some(14.0),
        "പതിനഞ്ച്" => Some(15.0),
        "പതിനാറ്" => Some(16.0),
        "പതിനേഴ്" => Some(17.0),
        "പതിനെട്ട്" => Some(18.0),
        "പത്തൊമ്പത്" => Some(19.0),
        _ => None,
    }
}

fn tens(s: &str) -> Option<f64> {
    match s {
        "ഇരുപത്" | "ഇരുപത്തി" => Some(20.0),
        "മുപ്പത്" | "മുപ്പത്തി" => Some(30.0),
        "നാല്പത്" | "നാല്പത്തി" => Some(40.0),
        "അമ്പത്" | "അമ്പത്തി" => Some(50.0),
        "അറുപത്" | "അറുപത്തി" => Some(60.0),
        "എഴുപത്" | "എഴുപത്തി" => Some(70.0),
        "എൺപത്" | "എൺപത്തി" => Some(80.0),
        "തൊണ്ണൂറ്" | "തൊണ്ണൂറ്റി" => Some(90.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "integer (0..9)".to_string(),
            pattern: vec![regex("(പൂജ്യം|ഒന്ന്|രണ്ട്|മുന്ന്|നാല്|അഞ്ച്|ആറ്|ഏഴ്|എട്ട്|ഒൻപത്)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(zero_to_nine(s)?)))
            }),
        },
        Rule {
            name: "integer (10..19)".to_string(),
            pattern: vec![regex("(പത്ത്|പതിനൊന്ന്|പന്ത്രണ്ട്|പതിമൂന്ന്|പതിനാല്|പതിനഞ്ച്|പതിനാറ്|പതിനേഴ്|പതിനെട്ട്|പത്തൊമ്പത്)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(ten_to_nineteen(s)?)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(ഇരുപത്|മുപ്പത്|നാല്പത്|അമ്പത്|അറുപത്|എഴുപത്|എൺപത്|തൊണ്ണൂറ്)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(tens(s)?)))
            }),
        },
        Rule {
            name: "integer ([2-9][1-9])".to_string(),
            pattern: vec![regex("(ഇരുപത്തി|മുപ്പത്തി|നാല്പത്തി|അമ്പത്തി|അറുപത്തി|എഴുപത്തി|എൺപത്തി|തൊണ്ണൂറ്റി)(ഒന്ന്|രണ്ട്|മുന്ന്|നാല്|അഞ്ച്|ആറ്|ഏഴ്|എട്ട്|ഒൻപത്)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let t = m.group(1)?;
                let u = m.group(2)?;
                Some(TokenData::Numeral(NumeralData::new(
                    tens(t)? + zero_to_nine(u)?,
                )))
            }),
        },
    ]
}
