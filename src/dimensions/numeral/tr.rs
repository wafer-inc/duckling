use crate::pattern::{predicate, regex};
use crate::types::{Rule, TokenData};

use super::NumeralData;

fn numeral_data(td: &TokenData) -> Option<&NumeralData> {
    match td {
        TokenData::Numeral(d) => Some(d),
        _ => None,
    }
}

fn is_positive(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value >= 0.0)
}

fn has_grain(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.grain.is_some())
}

fn is_multipliable(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.multipliable)
}

fn number_between(low: f64, high: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if d.value >= low && d.value < high)
}

fn norm(s: &str) -> String {
    s.to_lowercase()
}

fn units(s: &str) -> Option<f64> {
    match s {
        "sıfır" | "yok" | "hiç" => Some(0.0),
        "bir" | "bi" | "yek" | "tek" => Some(1.0),
        "iki" => Some(2.0),
        "üç" | "üçü" => Some(3.0),
        "dört" => Some(4.0),
        "beş" => Some(5.0),
        "altı" => Some(6.0),
        "yedi" => Some(7.0),
        "sekiz" => Some(8.0),
        "dokuz" => Some(9.0),
        _ => None,
    }
}

fn tens(s: &str) -> Option<f64> {
    match s {
        "on" => Some(10.0),
        "yirmi" => Some(20.0),
        "otuz" => Some(30.0),
        "kırk" => Some(40.0),
        "elli" => Some(50.0),
        "altmış" | "atmış" => Some(60.0),
        "yetmiş" => Some(70.0),
        "seksen" => Some(80.0),
        "doksan" => Some(90.0),
        _ => None,
    }
}

fn unit_word_to_digit(s: &str) -> Option<f64> {
    match s {
        "bir" | "bi" | "yek" | "tek" => Some(1.0),
        "iki" => Some(2.0),
        "üç" => Some(3.0),
        "dört" => Some(4.0),
        "beş" => Some(5.0),
        "altı" => Some(6.0),
        "yedi" => Some(7.0),
        "sekiz" => Some(8.0),
        "dokuz" => Some(9.0),
        _ => None,
    }
}

fn parse_tens_units_compact(s: &str) -> Option<f64> {
    let prefixes = [
        "yirmi", "otuz", "kırk", "elli", "altmış", "atmış", "yetmiş", "seksen", "doksan", "on",
    ];
    for p in prefixes {
        if let Some(rest) = s.strip_prefix(p) {
            let base = tens(p)?;
            if let Some(u) = units(rest) {
                return Some(base + u);
            }
        }
    }
    None
}

fn parse_hundreds_compact(s: &str) -> Option<f64> {
    if s == "yüz" {
        return Some(100.0);
    }
    if let Some(prefix) = s.strip_suffix("yüz") {
        let v = unit_word_to_digit(prefix)?;
        return Some(v * 100.0);
    }
    None
}

fn parse_thousands_compact(s: &str) -> Option<f64> {
    if s == "bin" {
        return Some(1000.0);
    }
    if let Some(prefix) = s.strip_suffix("bin") {
        let p = parse_hundreds_compact(prefix)
            .or_else(|| parse_tens_units_compact(prefix))
            .or_else(|| tens(prefix))
            .or_else(|| units(prefix))?;
        return Some(p * 1000.0);
    }
    None
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|eksi|negatif"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
        Rule {
            name: "a couple (of)".to_string(),
            pattern: vec![regex("(bir )?çift")],
            production: Box::new(|_| {
                Some(TokenData::Numeral(
                    NumeralData::new(2.0).with_grain(1),
                ))
            }),
        },
        Rule {
            name: "few".to_string(),
            pattern: vec![regex("(bir)?az")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(3.0)))),
        },
        Rule {
            name: "ten".to_string(),
            pattern: vec![regex("on")],
            production: Box::new(|_| {
                Some(TokenData::Numeral(
                    NumeralData::new(10.0)
                        .with_grain(1)
                        .with_multipliable(true),
                ))
            }),
        },
        Rule {
            name: "decimal with thousands separator".to_string(),
            pattern: vec![regex("(\\d+(\\.\\d\\d\\d)+,\\d+)")],
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
            name: "compose by multiplication".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(_))), predicate(is_multipliable)],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?;
                let n2 = numeral_data(&nodes[1].token_data)?;
                if n2.grain.is_none() || (n2.grain.is_some() && n2.value > n1.value) {
                    let mut out = NumeralData::new(n1.value * n2.value);
                    if let Some(g) = n2.grain {
                        out = out.with_grain(g);
                    }
                    return Some(TokenData::Numeral(out));
                }
                None
            }),
        },
        Rule {
            name: "integer (0..9)".to_string(),
            pattern: vec![regex("(yok|hiç|sıfır|bir|bi|yek|tek|iki|üç|üçü|dört|beş|altı|yedi|sekiz|dokuz)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(units(&norm(s))?)))
            }),
        },
        Rule {
            name: "integer (10..90)".to_string(),
            pattern: vec![regex("(on|yirmi|otuz|kırk|elli|atmış|altmış|yetmiş|seksen|doksan)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(tens(&norm(s))?)))
            }),
        },
        Rule {
            name: "integer 11..99 compact".to_string(),
            pattern: vec![regex("((on|yirmi|otuz|kırk|elli|atmış|altmış|yetmiş|seksen|doksan)(bir|bi|iki|üç|üçü|dört|beş|altı|yedi|sekiz|dokuz))")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(parse_tens_units_compact(&norm(s))?)))
            }),
        },
        Rule {
            name: "integer 100..900".to_string(),
            pattern: vec![regex("(yüz|ikiyüz|üçyüz|dörtyüz|beşyüz|altıyüz|yediyüz|sekizyüz|dokuzyüz)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(
                    NumeralData::new(parse_hundreds_compact(&norm(s))?).with_grain(2),
                ))
            }),
        },
        Rule {
            name: "integer 1000..9000".to_string(),
            pattern: vec![regex("(bin|ikibin|üçbin|dörtbin|beşbin|altıbin|yedibin|sekizbin|dokuzbin)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(
                    NumeralData::new(parse_thousands_compact(&norm(s))?).with_grain(3),
                ))
            }),
        },
        Rule {
            name: "integer 10000..90000".to_string(),
            pattern: vec![regex("(onbin|yirmibin|otuzbin|kırkbin|ellibin|atmışbin|altmışbin|yetmişbin|seksenbin|doksanbin)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(
                    NumeralData::new(parse_thousands_compact(&norm(s))?).with_grain(4),
                ))
            }),
        },
        Rule {
            name: "integer 100000..900000".to_string(),
            pattern: vec![regex("(yüzbin|ikiyüzbin|üçyüzbin|dörtyüzbin|beşyüzbin|altıyüzbin|yediyüzbin|sekizyüzbin|dokuzyüzbin)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(
                    NumeralData::new(parse_thousands_compact(&norm(s))?).with_grain(5),
                ))
            }),
        },
        Rule {
            name: "intersect".to_string(),
            pattern: vec![predicate(has_grain), predicate(|td| !is_multipliable(td) && is_positive(td))],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?;
                let n2 = numeral_data(&nodes[1].token_data)?;
                let g = n1.grain?;
                if 10f64.powi(g as i32) > n2.value {
                    Some(TokenData::Numeral(NumeralData::new(n1.value + n2.value)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "numbers suffixes (K, M, G)".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(_))), regex("([kmgb])")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                let s = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let out = match s.as_str() {
                    "k" | "b" => v * 1e3,
                    "m" => v * 1e6,
                    "g" => v * 1e9,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(out)))
            }),
        },
        Rule {
            name: "powers of tens".to_string(),
            pattern: vec![regex("(yüz|bin|milyon)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let out = match s.as_str() {
                    "yüz" => NumeralData::new(1e2).with_grain(2).with_multipliable(true),
                    "bin" => NumeralData::new(1e3).with_grain(3).with_multipliable(true),
                    "milyon" => NumeralData::new(1e6).with_grain(6).with_multipliable(true),
                    _ => return None,
                };
                Some(TokenData::Numeral(out))
            }),
        },
        Rule {
            name: "half".to_string(),
            pattern: vec![regex("(yarım)")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(0.5)))),
        },
        Rule {
            name: "dozen".to_string(),
            pattern: vec![regex("düzine")],
            production: Box::new(|_| {
                Some(TokenData::Numeral(
                    NumeralData::new(12.0)
                        .with_grain(1)
                        .with_multipliable(true),
                ))
            }),
        },
        Rule {
            name: "group of ten(s)".to_string(),
            pattern: vec![regex("deste")],
            production: Box::new(|_| {
                Some(TokenData::Numeral(
                    NumeralData::new(10.0)
                        .with_grain(1)
                        .with_multipliable(true),
                ))
            }),
        },
        Rule {
            name: "number suffixes (half-suffix)".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(_))), regex("buçuk")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(v + 0.5)))
            }),
        },
        Rule {
            name: "quarter".to_string(),
            pattern: vec![regex("(çeyrek)")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(0.25)))),
        },
        Rule {
            name: "number dot number".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(_))),
                regex("nokta|virgül"),
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
            name: "integer 11..99".to_string(),
            pattern: vec![predicate(|td| matches!(td, TokenData::Numeral(n) if [70.0,20.0,60.0,50.0,40.0,90.0,30.0,10.0,80.0].contains(&n.value))), predicate(number_between(1.0, 10.0))],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
    ]
}
