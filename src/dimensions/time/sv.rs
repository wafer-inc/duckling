use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (sv)".to_string(),
            pattern: vec![regex("nu")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (sv)".to_string(),
            pattern: vec![regex("idag|i dag")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (sv)".to_string(),
            pattern: vec![regex("imorgon|i morgon")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (sv)".to_string(),
            pattern: vec![regex("ig[åa]r|i g[åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (sv)".to_string(),
            pattern: vec![regex("måndag(en)?s?|mån\\.?|tisdag(en)?s?|tis?\\.?|onsdag(en)?s?|ons\\.?|torsdag(en)?s?|tors?\\.?|fredag(en)?s?|fre\\.?|lördag(en)?s?|lör\\.?|söndag(en)?s?|sön\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("måndag")
                    || s.contains("mandag")
                    || s == "mån"
                    || s == "man"
                    || s == "mån."
                    || s == "man."
                {
                    0
                } else if s.contains("tisdag") {
                    1
                } else if s.contains("onsdag") {
                    2
                } else if s.contains("torsdag") || s == "tors" || s == "tors." {
                    3
                } else if s.contains("fredag") || s == "fre" || s == "fre." {
                    4
                } else if s.contains("lördag")
                    || s.contains("lordag")
                    || s == "lör"
                    || s == "lör."
                    || s == "lor"
                    || s == "lor."
                {
                    5
                } else if s.contains("söndag")
                    || s.contains("sondag")
                    || s == "sön"
                    || s == "sön."
                    || s == "son"
                    || s == "son."
                {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "den förste mars (sv)".to_string(),
            pattern: vec![regex("den\\s+f[öo]rste\\s+mars|den\\s+f[öo]rsta\\s+mars")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DateMDY {
                    month: 3,
                    day: 1,
                    year: None,
                })))
            }),
        },
    ]);
    rules
}
