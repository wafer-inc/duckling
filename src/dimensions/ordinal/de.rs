use crate::pattern::regex;
use crate::types::{Rule, TokenData};

use super::OrdinalData;

fn lookup_ordinal_word(s: &str) -> Option<i64> {
    match s.to_lowercase().as_str() {
        "erste" => Some(1),
        "zweite" => Some(2),
        "dritte" => Some(3),
        "vierte" => Some(4),
        "fünfte" => Some(5),
        "sechste" => Some(6),
        "siebte" => Some(7),
        "achte" => Some(8),
        "neunte" => Some(9),
        "zehnte" => Some(10),
        "elfte" => Some(11),
        "zwölfte" => Some(12),
        "dreizente" => Some(13),
        "vierzehnte" => Some(14),
        "fünfzehnte" => Some(15),
        "sechzente" => Some(16),
        "siebzehnte" => Some(17),
        "achtzehnte" => Some(18),
        "neunzehnte" => Some(19),
        "zwanzigste" => Some(20),
        "einundzwanzigste" => Some(21),
        "zweiundzwanzigste" => Some(22),
        "dreiundzwanzigste" => Some(23),
        "vierundzwanzigste" => Some(24),
        "fünfundzwanzigste" => Some(25),
        "sechsundzwanzigste" => Some(26),
        "siebenundzwanzigste" => Some(27),
        "achtundzwanzigste" => Some(28),
        "neunundzwanzigste" => Some(29),
        "dreissigste" | "dreißigste" => Some(30),
        "einunddreissigste" | "einunddreißigste" => Some(31),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "ordinal (1..31)".to_string(),
            pattern: vec![regex(
                "(erste|zweite|dritte|vierte|fünfte|sechste|siebte|achte|neunte|zehnte|elfte|zwölfte|dreizente|vierzehnte|fünfzehnte|sechzente|siebzehnte|achtzehnte|neunzehnte|zwanzigste|einundzwanzigste|zweiundzwanzigste|dreiundzwanzigste|vierundzwanzigste|fünfundzwanzigste|sechsundzwanzigste|siebenundzwanzigste|achtundzwanzigste|neunundzwanzigste|dreissigste|dreißigste|einunddreissigste|einunddreißigste)[rsn]?",
            )],
            production: Box::new(|nodes| {
                let mut s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                if s.ends_with('r') || s.ends_with('s') || s.ends_with('n') {
                    s.pop();
                }
                Some(TokenData::Ordinal(OrdinalData::new(lookup_ordinal_word(&s)?)))
            }),
        },
        Rule {
            name: "ordinal (digits)".to_string(),
            pattern: vec![regex("(\\d+)(\\.| ?(te(n|r|s)?)|(ste(n|r|s)?))")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: i64 = s.parse().ok()?;
                Some(TokenData::Ordinal(OrdinalData::new(v)))
            }),
        },
    ]
}
