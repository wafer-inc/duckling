use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn lookup_one_to_ten(s: &str) -> Option<f64> {
    match s {
        "aon" => Some(1.0),
        "dha" | "dhá" => Some(2.0),
        "trí" | "tri" => Some(3.0),
        "ceithre" => Some(4.0),
        "cuig" | "cúig" => Some(5.0),
        "sé" | "se" => Some(6.0),
        "seacht" => Some(7.0),
        "ocht" => Some(8.0),
        "naoi" => Some(9.0),
        "deich" => Some(10.0),
        _ => None,
    }
}

fn lookup_twenty_to_ninety(s: &str) -> Option<f64> {
    match s {
        "fiche" => Some(20.0),
        "triocha" | "tríocha" => Some(30.0),
        "daichead" => Some(40.0),
        "caoga" => Some(50.0),
        "seasca" => Some(60.0),
        "seachto" | "seachtó" => Some(70.0),
        "ochto" | "ochtó" => Some(80.0),
        "nócha" | "nocha" => Some(90.0),
        _ => None,
    }
}

fn lookup_count_numeral(s: &str) -> Option<f64> {
    match s {
        "naid" | "náid" => Some(0.0),
        "haon" => Some(1.0),
        "dó" | "do" => Some(2.0),
        "trí" | "tri" => Some(3.0),
        "ceathair" => Some(4.0),
        "cuig" | "cúig" => Some(5.0),
        "sé" | "se" => Some(6.0),
        "seacht" => Some(7.0),
        "hocht" => Some(8.0),
        "naoi" => Some(9.0),
        "deich" => Some(10.0),
        _ => None,
    }
}

fn lookup_old_vigesimal(s: &str) -> Option<f64> {
    match s {
        "dá fhichead" | "da fhichead" | "dhá fhichead" | "dha fhichead" => Some(40.0),
        "trí fichid" | "tri fichid" => Some(60.0),
        "ceithre fichid" => Some(80.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "amháin".to_string(),
            pattern: vec![regex("amh(á|a)in")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(1.0)))),
        },
        Rule {
            name: "count numbers".to_string(),
            pattern: vec![regex("a (n(á|a)id|haon|d(ó|o)|tr(í|i)|ceathair|c(ú|u)ig|s(é|e)|seacht|hocht|naoi|deich)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = lookup_count_numeral(&s.to_lowercase())?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "déag".to_string(),
            pattern: vec![regex("d(é|e)ag")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(10.0)))),
        },
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*\\.\\d+)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(t.parse().ok()?)))
            }),
        },
        Rule {
            name: "decimal with thousands separator".to_string(),
            pattern: vec![regex("(\\d+(,\\d\\d\\d)+\\.\\d+)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = t.replace(',', "").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer with thousands separator ,".to_string(),
            pattern: vec![regex("(\\d{1,3}(,\\d\\d\\d){1,5})")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = t.replace(',', "").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "numbers, 20-90".to_string(),
            pattern: vec![regex("(fiche|tr(í|i)ocha|daichead|caoga|seasca|seacht(ó|o)|ocht(ó|o)|n(ó|o)cha)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = lookup_twenty_to_ninety(&s.to_lowercase())?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "numbers, 1-10".to_string(),
            pattern: vec![regex("(aon|dh(á|a)|tr(í|i)|ceithre|c(ú|u)ig|seacht|s(é|e)|ocht|naoi|deich)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = lookup_one_to_ten(&s.to_lowercase())?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|m(í|i)neas(\\sa)?"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
        Rule {
            name: "numbers suffixes (K, M, G)".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(_))), regex("([kmg])")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let s = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let out = match s.as_str() {
                    "k" => v * 1e3,
                    "m" => v * 1e6,
                    "g" => v * 1e9,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(out)))
            }),
        },
        Rule {
            name: "old vigesimal numbers, 20s".to_string(),
            pattern: vec![regex("(d[ée]ag )?is (dh?(á|a) fhichead|tr(í|i) fichid|ceithre fichid)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let ten = m.group(1).unwrap_or_default();
                let phrase = m.group(2)?;
                let base = lookup_old_vigesimal(&phrase.to_lowercase())?;
                let v = if ten.is_empty() { base } else { base + 10.0 };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "old vigesimal 20 + 10".to_string(),
            pattern: vec![regex("d[ée]ag is fiche")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(30.0)))),
        },
    ]
}
