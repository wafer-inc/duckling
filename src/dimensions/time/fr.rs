use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (fr)".to_string(),
            pattern: vec![regex("maintenant|tout de suite|dans la journée|en ce moment")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (fr)".to_string(),
            pattern: vec![regex("aujourd'hui|ce jour")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (fr)".to_string(),
            pattern: vec![regex("(le len)?demain|jour suivant|le jour d'apr(e|é|è)s|un jour apr(e|é|è)s")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (fr)".to_string(),
            pattern: vec![regex("hier|le jour d'avant|le jour précédent|la veille")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
    ]);
    rules
}
