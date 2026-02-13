use crate::dimensions::time_grain::Grain;
use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (hu)".to_string(),
            pattern: vec![regex("most|azonnal|hónap vége|hó vége|hó végi|hó végén")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (hu)".to_string(),
            pattern: vec![regex("ma")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (hu)".to_string(),
            pattern: vec![regex("holnap")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (hu)".to_string(),
            pattern: vec![regex("tegnap")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (hu)".to_string(),
            pattern: vec![regex("hétf(ő(n|t|i(t)?)?|\\.)|kedd(en|et|i(t)?)?|szerda(i(t)?)?|szerdá(n|t)|szer\\.?|csütörtök(ö(n|t)|i(t)?)?|csüt\\.?|péntek(e(n|t)|i(t)?)?|pén\\.?|szombat(o(n|t)|i(t)?)?|szom\\.?|vasárnap(ot|i(t)?)?|vas\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("hét") || s.starts_with("het") {
                    0
                } else if s.starts_with("ked") {
                    1
                } else if s.starts_with("szer") {
                    2
                } else if s.starts_with("csü") || s.starts_with("csu") {
                    3
                } else if s.starts_with("pé") || s.starts_with("pen") {
                    4
                } else if s.starts_with("szo") {
                    5
                } else if s.starts_with("vas") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "year end (hu)".to_string(),
            pattern: vec![regex("év vége|év végi|év végén")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::BeginEnd {
                    begin: false,
                    target: Box::new(TimeForm::AllGrain(Grain::Year)),
                })))
            }),
        },
    ]);
    rules
}
