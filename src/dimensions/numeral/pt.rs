use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::helpers::*;
use super::NumeralData;

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some_and(|g| g > 1))
}

fn is_multipliable(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.multipliable)
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
            name: "a dozen of".to_string(),
            pattern: vec![regex("(uma )?d(u|ú)zias?( de)?")],
            production: Box::new(|_| {
                Some(TokenData::Numeral(
                    NumeralData::new(12.0)
                        .with_multipliable(true)
                        .with_quantifier(),
                ))
            }),
        },
        Rule {
            name: "integer (0..19)".to_string(),
            pattern: vec![regex(
                "(zero|d(oi|ua)s|(uma? )?par(es)?( de)?|tr(e|ê)s|(um )?pouco|uma?|(c|qu)atorze|quatro|quinze|cinco|dez[ea]sseis|seis|dez[ea]ssete|sete|dezoito|oito|dez[ea]nove|nove|dez|onze|doze|treze)",
            )],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match t.as_str() {
                    "zero" => 0.0,
                    "um" | "uma" => 1.0,
                    "dois" | "duas" => 2.0,
                    "tres" | "três" => 3.0,
                    "quatro" => 4.0,
                    "cinco" => 5.0,
                    "seis" => 6.0,
                    "sete" => 7.0,
                    "oito" => 8.0,
                    "nove" => 9.0,
                    "dez" => 10.0,
                    "onze" => 11.0,
                    "doze" => 12.0,
                    "treze" => 13.0,
                    "catorze" | "quatorze" => 14.0,
                    "quinze" => 15.0,
                    "dezesseis" | "dezasseis" => 16.0,
                    "dezessete" | "dezassete" => 17.0,
                    "dezoito" => 18.0,
                    "dezenove" | "dezanove" => 19.0,
                    "um par" | "um par de" | "par" | "pares" | "par de" | "pares de" => 2.0,
                    "um pouco" | "pouco" => 3.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "tens (20..90)".to_string(),
            pattern: vec![regex(
                "(vinte|trinta|quarenta|cin(co|q[uü])enta|sessenta|setenta|oitenta|noventa)",
            )],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match t.as_str() {
                    "vinte" => 20.0,
                    "trinta" => 30.0,
                    "quarenta" => 40.0,
                    "cincoenta" | "cinquenta" | "cinqüenta" => 50.0,
                    "sessenta" => 60.0,
                    "setenta" => 70.0,
                    "oitenta" => 80.0,
                    "noventa" => 90.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "hundreds (100..900)".to_string(),
            pattern: vec![regex(
                "(cem|cento|duzentos|trezentos|quatrocentos|quinhetos|seiscentos|setecentos|oitocentos|novecentos)",
            )],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let v = match t.as_str() {
                    "cem" | "cento" => 100.0,
                    "duzentos" => 200.0,
                    "trezentos" => 300.0,
                    "quatrocentos" => 400.0,
                    "quinhetos" => 500.0,
                    "seiscentos" => 600.0,
                    "setecentos" => 700.0,
                    "oitocentos" => 800.0,
                    "novecentos" => 900.0,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(milhao|milhão|milhões|milhoes|bilhao|bilhão|bilhões|bilhoes|mil)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let d = match t.as_str() {
                    "mil" => NumeralData::new(1e3).with_grain(3).with_multipliable(true),
                    "milhao" | "milhão" | "milhões" | "milhoes" => {
                        NumeralData::new(1e6).with_grain(6).with_multipliable(true)
                    }
                    "bilhao" | "bilhão" | "bilhões" | "bilhoes" => {
                        NumeralData::new(1e9).with_grain(9).with_multipliable(true)
                    }
                    _ => return None,
                };
                Some(TokenData::Numeral(d))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![
                predicate(one_of(&[20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0])),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let tens = numeral_data(&nodes[0].token_data)?.value;
                let units = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(tens + units)))
            }),
        },
        Rule {
            name: "number (21..29 31..39 .. 91..99)".to_string(),
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
        Rule {
            name: "integer 101..999".to_string(),
            pattern: vec![
                predicate(one_of(&[
                    100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0,
                ])),
                predicate(number_between(1.0, 100.0)),
            ],
            production: Box::new(|nodes| {
                let h = numeral_data(&nodes[0].token_data)?.value;
                let u = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(h + u)))
            }),
        },
        Rule {
            name: "number (101..199 201..299 .. 901..999)".to_string(),
            pattern: vec![
                predicate(one_of(&[
                    100.0, 200.0, 300.0, 400.0, 500.0, 600.0, 700.0, 800.0, 900.0,
                ])),
                regex("e"),
                predicate(number_between(1.0, 100.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "one twenty two".to_string(),
            pattern: vec![
                predicate(number_between(1.0, 10.0)),
                predicate(number_between(10.0, 100.0)),
            ],
            production: Box::new(|nodes| {
                let h = numeral_data(&nodes[0].token_data)?.value;
                let rest = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(h * 100.0 + rest)))
            }),
        },
        Rule {
            name: "one point 2".to_string(),
            pattern: vec![
                dim(DimensionKind::Numeral),
                regex("ponto"),
                predicate(|td| !has_grain(td)),
            ],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?.value;
                let n2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(n1 + decimals_to_double(n2))))
            }),
        },
        Rule {
            name: "point 77".to_string(),
            pattern: vec![regex("ponto"), predicate(|td| !has_grain(td))],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(decimals_to_double(n))))
            }),
        },
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*\\,\\d+)")],
            production: Box::new(|nodes| {
                let txt = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = txt.replace(',', ".").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "dot-separated numbers".to_string(),
            pattern: vec![regex("(\\d+(\\.\\d\\d\\d)+(\\,\\d+)?)")],
            production: Box::new(|nodes| {
                let txt = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let cleaned = txt.replace('.', "").replace(',', ".");
                let v: f64 = cleaned.parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "suffixes (K,M,G))".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), regex("(k|m|g)")],
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
            name: "negative numbers".to_string(),
            pattern: vec![regex("(-|menos|negativo)"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
        Rule {
            name: "intersect 2 numbers".to_string(),
            pattern: vec![
                predicate(has_grain),
                predicate(|td| !is_multipliable(td) && is_positive(td)),
            ],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?;
                let n2 = numeral_data(&nodes[1].token_data)?;
                let g = n1.grain? as i32;
                if 10f64.powi(g) > n2.value {
                    Some(TokenData::Numeral(NumeralData::new(n1.value + n2.value)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "intersect 2 numbers (with and)".to_string(),
            pattern: vec![
                predicate(has_grain),
                regex("e"),
                predicate(|td| !is_multipliable(td) && is_positive(td)),
            ],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?;
                let n2 = numeral_data(&nodes[2].token_data)?;
                let g = n1.grain? as i32;
                if 10f64.powi(g) > n2.value {
                    Some(TokenData::Numeral(NumeralData::new(n1.value + n2.value)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "compose by multiplication".to_string(),
            pattern: vec![dim(DimensionKind::Numeral), predicate(is_multipliable)],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?;
                let n2 = numeral_data(&nodes[1].token_data)?;
                match n2.grain {
                    None => Some(TokenData::Numeral(NumeralData::new(n1.value * n2.value))),
                    Some(g) if n2.value > n1.value => Some(TokenData::Numeral(
                        NumeralData::new(n1.value * n2.value).with_grain(g),
                    )),
                    _ => None,
                }
            }),
        },
    ]
}
