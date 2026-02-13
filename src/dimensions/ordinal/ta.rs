use std::collections::HashMap;

use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn one_to_nine_map() -> HashMap<&'static str, i64> {
    HashMap::from([
        ("முதல்", 1),
        ("இரண்டாம்", 2),
        ("மூன்றாம்", 3),
        ("நான்காம்", 4),
        ("ஐந்தாம்", 5),
        ("ஆறாம்", 6),
        ("ஏழாம்", 7),
        ("எட்டாம்", 8),
        ("ஒன்பதாம்", 9),
    ])
}

fn ten_to_nineteen_map() -> HashMap<&'static str, i64> {
    HashMap::from([
        ("பத்தாம்", 10),
        ("பதினொன்றாம்", 11),
        ("பன்னிரண்டாம்", 12),
        ("பதின்மூன்றாம்", 13),
        ("பதினான்காம்", 14),
        ("பதினைந்தாம்", 15),
        ("பதினாறாம்", 16),
        ("பதினேழாம்", 17),
        ("பதினெட்டாம்", 18),
        ("பத்தொன்பதாம்", 19),
    ])
}

fn tens_map() -> HashMap<&'static str, i64> {
    HashMap::from([
        ("இருபதாம்", 20),
        ("முப்பதாம்", 30),
        ("நாற்பதாம்", 40),
        ("ஐம்பதாம்", 50),
        ("அறுபதாம்", 60),
        ("எழுபதாம்", 70),
        ("எண்பதாம்", 80),
        ("தொண்ணூறாம்", 90),
    ])
}

fn tens_ordinal_map() -> HashMap<&'static str, i64> {
    HashMap::from([
        ("இருபத்தி", 20),
        ("முப்பத்து", 30),
        ("நாற்பத்து", 40),
        ("ஐம்பத்தி", 50),
        ("அறுபத்", 60),
        ("எழுபத்தி", 70),
        ("எண்பத்தி", 80),
        ("தொண்ணுற்று", 90),
    ])
}

fn one_to_nine_ordinal_map() -> HashMap<&'static str, i64> {
    HashMap::from([
        ("ஒன்றாம்", 1),
        ("இரண்டாம்", 2),
        ("மூன்றாம்", 3),
        ("நான்காம்", 4),
        ("ஐந்தாம்", 5),
        ("ஆறாம்", 6),
        ("ஏழாம்", 7),
        ("எட்டாம்", 8),
        ("ஒன்பதாம்", 9),
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
            name: "integer (1..9)".to_string(),
            pattern: vec![regex("(முதல்|இரண்டாம்|மூன்றாம்|நான்காம்|ஐந்தாம்|ஆறாம்|ஏழாம்|எட்டாம்|ஒன்பதாம்)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let map = one_to_nine_map();
                Some(TokenData::Ordinal(OrdinalData::new(*map.get(text)?)))
            }),
        },
        Rule {
            name: "integer (10..19)".to_string(),
            pattern: vec![regex("(பத்தாம்|பதினொன்றாம்|பன்னிரண்டாம்|பதின்மூன்றாம்|பதினான்காம்|பதினைந்தாம்|பதினாறாம்|பதினேழாம்|பதினெட்டாம்|பத்தொன்பதாம்)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let map = ten_to_nineteen_map();
                Some(TokenData::Ordinal(OrdinalData::new(*map.get(text)?)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(இருபதாம்|முப்பதாம்|நாற்பதாம்|ஐம்பதாம்|அறுபதாம்|எழுபதாம்|எண்பதாம்|தொண்ணூறாம்)")],
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
            pattern: vec![regex("(இருபத்தி|முப்பத்து|நாற்பத்து|ஐம்பத்தி|அறுபத்|எழுபத்தி|எண்பத்தி|தொண்ணுற்று)(ஒன்றாம்|இரண்டாம்|மூன்றாம்|நான்காம்|ஐந்தாம்|ஆறாம்|ஏழாம்|எட்டாம்|ஒன்பதாம்)")],
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
                let units = one_to_nine_ordinal_map();
                Some(TokenData::Ordinal(OrdinalData::new(
                    tens.get(m1)? + units.get(m2)?,
                )))
            }),
        },
    ]
}
