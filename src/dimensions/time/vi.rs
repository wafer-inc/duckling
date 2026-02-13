use crate::pattern::regex;
use crate::types::{Rule, TokenData};
use super::{TimeData, TimeForm};

pub fn rules() -> Vec<Rule> {
    let mut rules = super::en::rules();
    rules.extend(vec![
        Rule {
            name: "now (vi)".to_string(),
            pattern: vec![regex(r"b[âa]y\s+gi[ờo]|ngay lúc này")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Now)))),
        },
        Rule {
            name: "today (vi)".to_string(),
            pattern: vec![regex("h[oô]m nay|bữa nay")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Today)))),
        },
        Rule {
            name: "tomorrow (vi)".to_string(),
            pattern: vec![regex("ng[aà]y mai")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Tomorrow)))),
        },
        Rule {
            name: "yesterday (vi)".to_string(),
            pattern: vec![regex("h[oô]m qua")],
            production: Box::new(|_| Some(TokenData::Time(TimeData::new(TimeForm::Yesterday)))),
        },
        Rule {
            name: "day before yesterday (vi)".to_string(),
            pattern: vec![regex("hôm kia")],
            production: Box::new(|_| {
                Some(TokenData::Time(TimeData::new(TimeForm::DayBeforeYesterday)))
            }),
        },
        Rule {
            name: "day of week (vi)".to_string(),
            pattern: vec![regex("th(ứ) (2|hai)|th(ứ) (3|ba)|th(ứ) 4|th(ứ) b(ố)n|th(ứ) t(ư)|th(ứ) (5|n(ă)m)|th(ứ) 6|th(ứ) s(á)u|th(ứ) (7|b((ả)|(ẩ))y)|ch((ủ)|(ú)a) nh(ậ)t")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let dow = if s.contains("2") || s.contains("hai") {
                    0
                } else if s.contains("3") || s.contains("ba") {
                    1
                } else if s.contains("4") {
                    2
                } else if s.contains("5") {
                    3
                } else if s.contains("6") {
                    4
                } else if s.contains("7") {
                    5
                } else if s.contains("nhật") || s.contains("nhat") {
                    6
                } else {
                    return None;
                };
                Some(TokenData::Time(TimeData::new(TimeForm::DayOfWeek(dow))))
            }),
        },
    ]);
    rules
}
