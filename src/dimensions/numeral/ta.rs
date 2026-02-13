use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn zero_to_nine(s: &str) -> Option<f64> {
    match s {
        "பூஜ்ஜியம்" => Some(0.0),
        "ஒன்று" => Some(1.0),
        "இரண்டு" => Some(2.0),
        "மூன்று" => Some(3.0),
        "நான்கு" => Some(4.0),
        "ஐந்து" => Some(5.0),
        "ஆறு" => Some(6.0),
        "ஏழு" => Some(7.0),
        "எட்டு" => Some(8.0),
        "ஒன்பது" => Some(9.0),
        _ => None,
    }
}

fn ten_to_nineteen(s: &str) -> Option<f64> {
    match s {
        "பத்து" => Some(10.0),
        "பதினொன்று" => Some(11.0),
        "பன்னிரண்டு" => Some(12.0),
        "பதின்மூன்று" => Some(13.0),
        "பதினான்கு" => Some(14.0),
        "பதினைந்து" => Some(15.0),
        "பதினாறு" => Some(16.0),
        "பதினேழு" => Some(17.0),
        "பதினெட்டு" => Some(18.0),
        "பத்தொன்பது" => Some(19.0),
        _ => None,
    }
}

fn tens(s: &str) -> Option<f64> {
    match s {
        "இருபது" | "இருபத்தி" => Some(20.0),
        "முப்பது" | "முப்பத்து" => Some(30.0),
        "நாற்பது" | "நாற்பத்து" => Some(40.0),
        "ஐம்பது" | "ஐம்பத்தி" => Some(50.0),
        "அறுபது" | "அறுபத்" => Some(60.0),
        "எழுபது" | "எழுபத்தி" => Some(70.0),
        "எண்பது" | "எண்பத்" => Some(80.0),
        "தொண்ணூறு" | "தொண்ணுற்று" => Some(90.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "integer (0..9)".to_string(),
            pattern: vec![regex("(பூஜ்ஜியம்|ஒன்று|இரண்டு|மூன்று|நான்கு|ஐந்து|ஆறு|ஏழு|எட்டு|ஒன்பது)")],
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
            pattern: vec![regex("(பத்து|பதினொன்று|பன்னிரண்டு|பதின்மூன்று|பதினான்கு|பதினைந்து|பதினாறு|பதினேழு|பதினெட்டு|பத்தொன்பது)")],
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
            pattern: vec![regex("(இருபது|முப்பது|நாற்பது|ஐம்பது|அறுபது|எழுபது|எண்பது|தொண்ணூறு)")],
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
            pattern: vec![regex("(இருபத்தி|முப்பத்து|நாற்பத்து|ஐம்பத்தி|அறுபத்|எழுபத்தி|எண்பத்|தொண்ணுற்று)(ஒன்று|இரண்டு|மூன்று|நான்கு|ஐந்து|ஆறு|ஏழு|எட்டு|ஒன்பது)")],
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
