use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "ordinals".to_string(),
        pattern: vec![regex("(đầu tiên|thứ nhất|thứ 1)")],
        production: Box::new(|_| Some(TokenData::Ordinal(OrdinalData::new(1)))),
    }]
}
