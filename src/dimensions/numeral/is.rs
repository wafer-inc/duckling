use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn is_number_value(s: &str) -> Option<f64> {
    match s {
        "núll" | "null" => Some(0.0),
        "einn" => Some(1.0),
        "tveir" => Some(2.0),
        "þrír" => Some(3.0),
        "fjórir" => Some(4.0),
        "fimm" => Some(5.0),
        "sex" => Some(6.0),
        "sjö" => Some(7.0),
        "átta" => Some(8.0),
        "níu" => Some(9.0),
        "tíu" => Some(10.0),
        "ellefu" => Some(11.0),
        "tólf" => Some(12.0),
        "þrettán" => Some(13.0),
        "fjórtán" => Some(14.0),
        "fimmtán" => Some(15.0),
        "sextán" => Some(16.0),
        "sautján" => Some(17.0),
        "átján" => Some(18.0),
        "nítján" => Some(19.0),
        "tuttugu" => Some(20.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![Rule {
        name: "number (0..20)".to_string(),
        pattern: vec![regex(r"(n[úu]ll|einn|tveir|þrír|fjórir|fimm(tán)?|sex(tán)?|sjö|átta|níu|tíu|ellefu|tólf|þrettán|fjórtán|sautján|átján|nítján|tuttugu)")],
        production: Box::new(|nodes| {
            let s = match &nodes[0].token_data {
                TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                _ => return None,
            };
            Some(TokenData::Numeral(NumeralData::new(is_number_value(&s)?)))
        }),
    }]
}
