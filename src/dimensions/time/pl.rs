use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (pl)".to_string(),
            pattern: vec![regex("teraz|w tej chwili|w tym momencie")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (pl)".to_string(),
            pattern: vec![regex("dzisiaj|dzi[śs]|obecnego dnia|tego dnia")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (pl)".to_string(),
            pattern: vec![regex("jutro")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "day after tomorrow (pl)".to_string(),
            pattern: vec![regex("pojutrze|po jutrze")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DayAfterTomorrow)))),
        },
        Rule {
            name: "yesterday (pl)".to_string(),
            pattern: vec![regex("wczoraj")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day of week (pl)".to_string(),
            pattern: vec![regex("poniedzia(l|ł)(ek|ku|kowi|kiem|kowy)|pon\\.?|wtorek|wtorku|wtorkowi|wtorkiem|wtr?\\.?|(Ś|ś|s)rod(a|ą|y|e|ę|zie|owy|o)|(s|ś|Ś)ro?\\.?|czwartek|czwartku|czwartkowi|czwartkiem|czwr?\\.?|piątek|piatek|piątku|piatku|piątkowi|piatkowi|piątkiem|piatkiem|pi(ą|a)tkowy|pia\\.?|sobota|soboty|sobocie|sobotę|sobote|sobotą|soboto|sob\\.?|niedziel(a|i|ę|e|ą|o)|n(ie)?dz?\\.?")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.starts_with("pon") {
                    0
                } else if s.starts_with("wt") {
                    1
                } else if s.starts_with("śr") || s.starts_with("sr") || s.starts_with("śro") || s.starts_with("sro") {
                    2
                } else if s.starts_with("czw") {
                    3
                } else if s.starts_with("pi") {
                    4
                } else if s.starts_with("sob") {
                    5
                } else if s.starts_with("n") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
        Rule {
            name: "pierwszy marca (pl)".to_string(),
            pattern: vec![regex("pierwszy\\s+marca")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::DateMDY { month: 3, day: 1, year: None })))),
        },
    ]);
    rules
}
