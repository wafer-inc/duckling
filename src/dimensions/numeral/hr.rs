use crate::dimensions::numeral::helpers::numeral_data;
use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some())
}

fn is_multipliable(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.multipliable)
}

fn number_between(low: f64, up: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if d.value >= low && d.value < up)
}

fn lookup_integer_0_19(s: &str) -> Option<f64> {
    match s {
        "ništa" | "nista" | "ništica" | "nistica" | "nula" => Some(0.0),
        "jednoga" | "jedna" | "jednog" | "jednu" | "jedan" | "sam" => Some(1.0),
        "dvoma" | "dvije" | "dvje" | "dva" | "dvama" | "par" => Some(2.0),
        "trima" | "tri" => Some(3.0),
        "četiri" | "cetiri" => Some(4.0),
        "pet" => Some(5.0),
        "šest" | "sest" => Some(6.0),
        "sedam" => Some(7.0),
        "osam" => Some(8.0),
        "devet" => Some(9.0),
        "jedanaest" => Some(11.0),
        "dvanaest" => Some(12.0),
        "trinaest" => Some(13.0),
        "četrnaest" | "cetrnaest" => Some(14.0),
        "petnaest" => Some(15.0),
        "šesnaest" | "sesnaest" => Some(16.0),
        "sedamnaest" => Some(17.0),
        "osamnaest" => Some(18.0),
        "devetnaest" => Some(19.0),
        _ => None,
    }
}

fn lookup_tens(s: &str) -> Option<f64> {
    match s {
        "dvadeset" => Some(20.0),
        "trideset" => Some(30.0),
        "četrdeset" | "cetrdeset" => Some(40.0),
        "pedeset" => Some(50.0),
        "šesdeset" | "sesdeset" => Some(60.0),
        "sedamdeset" => Some(70.0),
        "osamdeset" => Some(80.0),
        "devedeset" => Some(90.0),
        _ => None,
    }
}

fn lookup_hundreds(s: &str) -> Option<f64> {
    match s {
        "sto" => Some(100.0),
        "dvjesta" | "dvjesto" => Some(200.0),
        "tristo" => Some(300.0),
        "četiristo" | "cetiristo" => Some(400.0),
        "petsto" => Some(500.0),
        "šesto" | "sesto" => Some(600.0),
        "sedamsto" => Some(700.0),
        "osamsto" => Some(800.0),
        "devetsto" => Some(900.0),
        _ => None,
    }
}

fn lookup_powers(s: &str) -> Option<(f64, u8)> {
    match s {
        "stotinu" | "stotina" | "stotine" => Some((1e2, 2)),
        "tisuća" | "tisuca" | "tisuću" | "tisucu" | "tisuće" | "tisuce" => Some((1e3, 3)),
        "milijun" | "milijuna" | "milijon" | "milijona" => Some((1e6, 6)),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|minus|negativ"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let n = numeral_data(&nodes[1].token_data)?;
                Some(TokenData::Numeral(NumeralData::new(-n.value)))
            }),
        },
        Rule {
            name: "few".to_string(),
            pattern: vec![regex("nekoliko")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(3.0)))),
        },
        Rule {
            name: "ten".to_string(),
            pattern: vec![regex("deset|cener")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(10.0).with_grain(1)))),
        },
        Rule {
            name: "decimal with thousands separator".to_string(),
            pattern: vec![regex("(\\d+(\\.\\d\\d\\d)+\\,\\d+)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = t.replace('.', "").replace(',', ".").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*,\\d+)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let v: f64 = t.replace(',', ".").parse().ok()?;
                Some(TokenData::Numeral(NumeralData::new(v)))
            }),
        },
        Rule {
            name: "integer (100..900)".to_string(),
            pattern: vec![regex("(sto|dvjest(o|a)|tristo|(c|č)etiristo|petsto|(š|s)esto|sedamsto|osamsto|devetsto)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(lookup_hundreds(&s.to_lowercase())?)))
            }),
        },
        Rule {
            name: "single".to_string(),
            pattern: vec![regex("sam")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(1.0).with_grain(1)))),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(stotin(u|a|e)|tisu(c|ć)(a|u|e)|milij(u|o)na?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let (v, g) = lookup_powers(&s)?;
                Some(TokenData::Numeral(
                    NumeralData::new(v).with_grain(g).with_multipliable(true),
                ))
            }),
        },
        Rule {
            name: "numbers i".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(n) if [70.0,20.0,60.0,50.0,40.0,90.0,30.0,80.0].contains(&n.value))),
                regex("i"),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "intersect".to_string(),
            pattern: vec![predicate(has_grain), predicate(|td| !is_multipliable(td) && is_positive(td))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[1].token_data)?;
                let g = a.grain?;
                if 10f64.powi(g as i32) > b.value {
                    Some(TokenData::Numeral(NumeralData::new(a.value + b.value)))
                } else {
                    None
                }
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
            name: "a pair".to_string(),
            pattern: vec![regex("par")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(2.0).with_grain(1)))),
        },
        Rule {
            name: "dozen".to_string(),
            pattern: vec![regex("tucet?")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(12.0).with_grain(1)))),
        },
        Rule {
            name: "integer (0..19)".to_string(),
            pattern: vec![regex("(ni(s|š)ta|ni(s|š)tica|nula|jedanaest|dvanaest|trinaest|jeda?n(a|u|o(ga?)?)?|dv(i?je)?(a|o)?(ma)?|tri(ma)?|(č|c)etiri|(č|c)etrnaest|petnaest|pet|(s|š)esnaest|(š|s)est|sedamnaest|sedam|osamnaest|osam|devetnaest|devet)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(lookup_integer_0_19(&s.to_lowercase())?)))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(n) if [70.0,20.0,60.0,50.0,40.0,90.0,30.0,80.0].contains(&n.value))),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v1 + v2)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(dvadeset|trideset|(c|č)etrdeset|pedeset|(š|s)esdeset|sedamdeset|osamdeset|devedeset)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(lookup_tens(&s.to_lowercase())?)))
            }),
        },
        Rule {
            name: "numbers 100..999".to_string(),
            pattern: vec![
                predicate(number_between(1.0, 10.0)),
                predicate(|td| matches!(td, TokenData::Numeral(n) if (n.value - 100.0).abs() < 1e-9)),
                predicate(number_between(0.0, 100.0)),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(100.0 * v1 + v2)))
            }),
        },
        Rule {
            name: "number dot number".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(_))),
                regex("cijela|to(c|č)ka|zarez"),
                predicate(|td| matches!(td, TokenData::Numeral(n) if n.grain.is_none())),
            ],
            production: Box::new(|nodes| {
                let v1 = numeral_data(&nodes[0].token_data)?.value;
                let v2 = numeral_data(&nodes[2].token_data)?.value;
                let mut m = 1.0;
                while v2 >= m {
                    m *= 10.0;
                }
                Some(TokenData::Numeral(NumeralData::new(v1 + (v2 / m))))
            }),
        },
        Rule {
            name: "integer with thousands separator .".to_string(),
            pattern: vec![regex("(\\d{1,3}(\\.\\d\\d\\d){1,5})")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(
                    t.replace('.', "").parse().ok()?,
                )))
            }),
        },
        Rule {
            name: "compose by multiplication".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(_))), predicate(is_multipliable)],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?;
                let b = numeral_data(&nodes[1].token_data)?;
                if b.grain.is_none() || (b.grain.is_some() && b.value > a.value) {
                    let mut out = NumeralData::new(a.value * b.value);
                    if let Some(g) = b.grain {
                        out = out.with_grain(g);
                    }
                    Some(TokenData::Numeral(out))
                } else {
                    None
                }
            }),
        },
    ]
}
