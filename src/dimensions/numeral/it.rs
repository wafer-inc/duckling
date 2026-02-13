use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::helpers::*;
use super::NumeralData;

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn number_between(low: f64, up: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if !d.multipliable && d.value >= low && d.value < up)
}

fn one_of(values: &'static [f64]) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if values.contains(&d.value))
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*,\\d+)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = text.replace(',', ".").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "decimal with thousands separator".to_string(),
            pattern: vec![regex("(\\d+(([\\. ])\\d\\d\\d)+,\\d+)")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let text = m.group(1)?;
                let sep = m.group(3)?;
                let cleaned = text.replace(sep, "").replace(',', ".");
                let v: f64 = cleaned.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer with thousands separator .".to_string(),
            pattern: vec![regex("(\\d{1,3}(([\\. ])\\d\\d\\d){1,5})")],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let text = m.group(1)?;
                let sep = m.group(3)?;
                let cleaned = text.replace(sep, "");
                let v: f64 = cleaned.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number (0..19)".to_string(),
            pattern: vec![regex(
                "(zero|nulla|niente|uno|due|tredici|tre|quattro|cinque|sei|sette|otto|nove|dieci|undici|dodici|quattordici|quindici|sedici|diciassette|diciotto|diciannove|un)",
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = match text.to_lowercase().as_str() {
                    "zero" | "niente" | "nulla" => 0.0,
                    "un" | "uno" => 1.0,
                    "due" => 2.0,
                    "tre" => 3.0,
                    "quattro" => 4.0,
                    "cinque" => 5.0,
                    "sei" => 6.0,
                    "sette" => 7.0,
                    "otto" => 8.0,
                    "nove" => 9.0,
                    "dieci" => 10.0,
                    "undici" => 11.0,
                    "dodici" => 12.0,
                    "tredici" => 13.0,
                    "quattordici" => 14.0,
                    "quindici" => 15.0,
                    "sedici" => 16.0,
                    "diciassette" => 17.0,
                    "diciotto" => 18.0,
                    "diciannove" => 19.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number (20..90)".to_string(),
            pattern: vec![regex(
                "(venti|trenta|quaranta|cinquanta|sessanta|settanta|ottanta|novanta)",
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v = match text.to_lowercase().as_str() {
                    "venti" => 20.0,
                    "trenta" => 30.0,
                    "quaranta" => 40.0,
                    "cinquanta" => 50.0,
                    "sessanta" => 60.0,
                    "settanta" => 70.0,
                    "ottanta" => 80.0,
                    "novanta" => 90.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number 100..1000 ".to_string(),
            pattern: vec![regex("(due|tre|quattro|cinque|sei|sette|otto|nove)?cento|mil(a|le)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let v = match text.as_str() {
                    "cento" => 100.0,
                    "duecento" => 200.0,
                    "trecento" => 300.0,
                    "quattrocento" => 400.0,
                    "cinquecento" => 500.0,
                    "seicento" => 600.0,
                    "settecento" => 700.0,
                    "ottocento" => 800.0,
                    "novecento" => 900.0,
                    "mila" | "mille" => 1000.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number (21..29 31..39 41..49 51..59 61..69 71..79 81..89 91..99)"
                .to_string(),
            pattern: vec![regex(
                "((venti|trenta|quaranta|cinquanta|sessanta|settanta|ottanta|novanta)(due|tre|tré|quattro|cinque|sei|sette|nove))|((vent|trent|quarant|cinquant|sessant|settant|ottant|novant)(uno|otto))",
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(0)?.to_lowercase(),
                    _ => return None,
                };
                let v = match text.as_str() {
                    "ventuno" => 21.0,
                    "ventidue" => 22.0,
                    "ventitre" | "ventitré" => 23.0,
                    "ventiquattro" => 24.0,
                    "venticinque" => 25.0,
                    "ventisei" => 26.0,
                    "ventisette" => 27.0,
                    "ventotto" => 28.0,
                    "ventinove" => 29.0,
                    "trentuno" => 31.0,
                    "trentadue" => 32.0,
                    "trentatre" | "trentatré" => 33.0,
                    "trentaquattro" => 34.0,
                    "trentacinque" => 35.0,
                    "trentasei" => 36.0,
                    "trentasette" => 37.0,
                    "trentotto" => 38.0,
                    "trentanove" => 39.0,
                    "quarantuno" => 41.0,
                    "quarantadue" => 42.0,
                    "quarantatre" | "quarantatré" => 43.0,
                    "quarantaquattro" => 44.0,
                    "quarantacinque" => 45.0,
                    "quarantasei" => 46.0,
                    "quarantasette" => 47.0,
                    "quarantotto" => 48.0,
                    "quarantanove" => 49.0,
                    "cinquantuno" => 51.0,
                    "cinquantadue" => 52.0,
                    "cinquantatre" | "cinquantatré" => 53.0,
                    "cinquantaquattro" => 54.0,
                    "cinquantacinque" => 55.0,
                    "cinquantasei" => 56.0,
                    "cinquantasette" => 57.0,
                    "cinquantotto" => 58.0,
                    "cinquantanove" => 59.0,
                    "sessantuno" => 61.0,
                    "sessantadue" => 62.0,
                    "sessantatre" | "sessantatré" => 63.0,
                    "sessantaquattro" => 64.0,
                    "sessantacinque" => 65.0,
                    "sessantasei" => 66.0,
                    "sessantasette" => 67.0,
                    "sessantotto" => 68.0,
                    "sessantanove" => 69.0,
                    "settantuno" => 71.0,
                    "settantadue" => 72.0,
                    "settantatre" | "settantatré" => 73.0,
                    "settantaquattro" => 74.0,
                    "settantacinque" => 75.0,
                    "settantasei" => 76.0,
                    "settantasette" => 77.0,
                    "settantotto" => 78.0,
                    "settantanove" => 79.0,
                    "ottantuno" => 81.0,
                    "ottantadue" => 82.0,
                    "ottantatre" | "ottantatré" => 83.0,
                    "ottantaquattro" => 84.0,
                    "ottantacinque" => 85.0,
                    "ottantasei" => 86.0,
                    "ottantasette" => 87.0,
                    "ottantotto" => 88.0,
                    "ottantanove" => 89.0,
                    "novantuno" => 91.0,
                    "novantadue" => 92.0,
                    "novantatre" | "novantatré" => 93.0,
                    "novantaquattro" => 94.0,
                    "novantacinque" => 95.0,
                    "novantasei" => 96.0,
                    "novantasette" => 97.0,
                    "novantotto" => 98.0,
                    "novantanove" => 99.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "numbers 200..999".to_string(),
            pattern: vec![
                predicate(number_between(2.0, 10.0)),
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value == 100.0)),
                predicate(number_between(0.0, 100.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[1].token_data)?.value;
                let v3 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 * v2 + v3)))
            }),
        },
        Rule {
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|meno|negativo"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
        Rule {
            name: "numbers suffixes (K, M, G)".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("([kmg])")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let s = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let mult = match s.as_str() {
                    "k" => 1_000.0,
                    "m" => 1_000_000.0,
                    "g" => 1_000_000_000.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v * mult)))
            }),
        },
        Rule {
            name: "numbers compose (tens + units) explicit".to_string(),
            pattern: vec![
                predicate(one_of(&[20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0])),
                regex("e"),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
    ]
}
