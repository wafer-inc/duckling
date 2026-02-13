use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::helpers::*;
use super::NumeralData;

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some_and(|g| g > 1))
}

fn number_between(low: f64, up: f64) -> impl Fn(&TokenData) -> bool {
    move |td| {
        matches!(
            td,
            TokenData::Numeral(d) if !d.multipliable && d.value >= low && d.value < up
        )
    }
}

fn one_of(values: &'static [f64]) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if values.contains(&d.value))
}

fn is_multiple(v: f64, divisor: f64) -> bool {
    (v % divisor).abs() < f64::EPSILON
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "number (0..15)".to_string(),
            pattern: vec![regex(
                r"((c|z)ero|un(o|a)?|dos|tr(é|e)s|cuatro|cinco|s(e|é)is|siete|ocho|nueve|die(z|s)|once|doce|trece|catorce|quince)",
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let value = match text.to_lowercase().as_str() {
                    "zero" | "cero" => 0.0,
                    "un" | "una" | "uno" => 1.0,
                    "dos" => 2.0,
                    "trés" | "tres" => 3.0,
                    "cuatro" => 4.0,
                    "cinco" => 5.0,
                    "seis" | "séis" => 6.0,
                    "siete" => 7.0,
                    "ocho" => 8.0,
                    "nueve" => 9.0,
                    "diez" | "dies" => 10.0,
                    "once" => 11.0,
                    "doce" => 12.0,
                    "trece" => 13.0,
                    "catorce" => 14.0,
                    "quince" => 15.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(value)))
            }),
        },
        Rule {
            name: "integer (0-9) with two digits".to_string(),
            pattern: vec![regex(r"((c|z)ero)|0"), predicate(number_between(1.0, 10.0))],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "number (16..19 21..29)".to_string(),
            pattern: vec![regex(
                r"(die(c|s)is(é|e)is|diecisiete|dieciocho|diecinueve|veintiun(o|a)|veintid(o|ó)s|veintitr(é|e)s|veinticuatro|veinticinco|veintis(é|e)is|veintisiete|veintiocho|veintinueve)",
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let value = match text.to_lowercase().as_str() {
                    "dieciseis" | "diesiséis" | "diesiseis" | "dieciséis" => 16.0,
                    "diecisiete" => 17.0,
                    "dieciocho" => 18.0,
                    "diecinueve" => 19.0,
                    "veintiuno" | "veintiuna" => 21.0,
                    "veintidos" | "veintidós" => 22.0,
                    "veintitrés" | "veintitres" => 23.0,
                    "veinticuatro" => 24.0,
                    "veinticinco" => 25.0,
                    "veintiséis" | "veintiseis" => 26.0,
                    "veintisiete" => 27.0,
                    "veintiocho" => 28.0,
                    "veintinueve" => 29.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(value)))
            }),
        },
        Rule {
            name: "number (16..19, two words)".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value == 10.0)),
                regex("y"),
                predicate(number_between(6.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(10.0 + v)))
            }),
        },
        Rule {
            name: "number (20..90)".to_string(),
            pattern: vec![regex(r"(veinte|treinta|cuarenta|cincuenta|sesenta|setenta|ochenta|noventa)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let value = match text.to_lowercase().as_str() {
                    "veinte" => 20.0,
                    "treinta" => 30.0,
                    "cuarenta" => 40.0,
                    "cincuenta" => 50.0,
                    "sesenta" => 60.0,
                    "setenta" => 70.0,
                    "ochenta" => 80.0,
                    "noventa" => 90.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(value)))
            }),
        },
        Rule {
            name: "number (21..29 31..39 41..49 51..59 61..69 71..79 81..89 91..99)".to_string(),
            pattern: vec![
                predicate(one_of(&[20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0])),
                regex("y"),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "big number 100 to 1K".to_string(),
            pattern: vec![regex(
                r"(cien(to|tos)?|doscientos|trescientos|cuatrocientos|quinientos|seiscientos|setecientos|ochocientos|novecientos|(un )?mill(o|ó)n)",
            )],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let value = match text.to_lowercase().as_str() {
                    "cien" | "cientos" | "ciento" => 100.0,
                    "doscientos" => 200.0,
                    "trescientos" => 300.0,
                    "cuatrocientos" => 400.0,
                    "quinientos" => 500.0,
                    "seiscientos" => 600.0,
                    "setecientos" => 700.0,
                    "ochocientos" => 800.0,
                    "novecientos" => 900.0,
                    "millon" | "millón" | "un millon" | "un millón" => 1_000_000.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(value)))
            }),
        },
        Rule {
            name: "1K or 1M in multipliable form".to_string(),
            pattern: vec![regex(r"(mil(lones)?)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let value = match text.to_lowercase().as_str() {
                    "mil" => 1_000.0,
                    "millones" => 1_000_000.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(
                    NumeralData::new(value).with_multipliable(true),
                ))
            }),
        },
        Rule {
            name: "2..999 <multipliable>".to_string(),
            pattern: vec![predicate(number_between(2.0, 1000.0)), predicate(is_multipliable)],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 * v2)))
            }),
        },
        Rule {
            name: "<thousands> 0..999".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value > 0.0 && d.value < 1_000_000.0 && is_multiple(d.value, 1000.0))),
                predicate(number_between(0.0, 999.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "<millions> 0..999999".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value > 0.0 && is_multiple(d.value, 1_000_000.0))),
                predicate(number_between(0.0, 999_999.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "2..9 cientos".to_string(),
            pattern: vec![predicate(number_between(2.0, 10.0)), regex("cientos")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(100.0 * v)))
            }),
        },
        Rule {
            name: "<hundreds> 0..99".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(d) if d.value > 0.0 && d.value < 1000.0 && is_multiple(d.value, 100.0))),
                predicate(number_between(0.0, 100.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "number dot number".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex("(co(n|ma)|punto)"),
                predicate(|td| !has_grain(td)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + decimals_to_double(v2))))
            }),
        },
        Rule {
            name: "dot number".to_string(),
            pattern: vec![regex("coma|punto"), predicate(|td| !has_grain(td))],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(decimals_to_double(v))))
            }),
        },
        Rule {
            name: "numbers suffixes (K, M, G)".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("([kmg])")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let suffix = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let mult = match suffix.as_str() {
                    "k" => 1_000.0,
                    "m" => 1_000_000.0,
                    "g" => 1_000_000_000.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v * mult)))
            }),
        },
        Rule {
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|menos|negativ(o|a)"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
        Rule {
            name: "decimal number with comma".to_string(),
            pattern: vec![regex("(\\d*,\\d+)")],
            production: Box::new(|nodes| {
                let text = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let normalized = if text.starts_with(',') {
                    format!("0{}", text)
                } else {
                    text.to_string()
                };
                let value: f64 = normalized.replace(',', ".").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(value)))
            }),
        },
    ]
}
