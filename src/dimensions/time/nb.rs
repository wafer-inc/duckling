use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (nb)".to_string(),
            pattern: vec![regex("n[åa]")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (nb)".to_string(),
            pattern: vec![regex("i dag|idag")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (nb)".to_string(),
            pattern: vec![regex("i morgen|imorgen|i morra")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (nb)".to_string(),
            pattern: vec![regex("i g[åa]r|ig[åa]r")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (nb)".to_string(),
            pattern: vec![regex("mandag|man\\.?|tirsdag|onsdag|torsdag|tors\\.?|fredag|fre\\.?|l[øo]rdag|l[øo]r\\.?|s[øo]ndag|s[øo]n\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("mandag") || s == "man." || s == "man" {
                    0
                } else if s.contains("tirsdag") {
                    1
                } else if s.contains("onsdag") {
                    2
                } else if s.contains("torsdag") || s == "tors." || s == "tors" {
                    3
                } else if s.contains("fredag") || s == "fre" || s == "fre." {
                    4
                } else if (s.contains("rdag") && (s.contains("lø") || s.contains("lo")))
                    || s == "lør"
                    || s == "lør."
                    || s == "lor"
                    || s == "lor."
                {
                    5
                } else if s.contains("ndag") || s == "søn" || s == "søn." || s == "son" || s == "son." {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "den første mars (nb)".to_string(),
            pattern: vec![regex("den\\s+f[øo]rste\\s+mars|1\\.\\s+mars")],
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
