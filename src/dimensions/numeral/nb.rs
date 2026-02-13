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

fn number_between(low: f64, high: f64) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::Numeral(d) if d.value >= low && d.value < high)
}

fn zero_to_nineteen(s: &str) -> Option<f64> {
    match s {
        "null" | "ingen" | "intet" => Some(0.0),
        "en" | "ett" | "én" => Some(1.0),
        "to" => Some(2.0),
        "tre" => Some(3.0),
        "fire" => Some(4.0),
        "fem" => Some(5.0),
        "seks" => Some(6.0),
        "sju" | "syv" => Some(7.0),
        "åtte" => Some(8.0),
        "ni" => Some(9.0),
        "ti" => Some(10.0),
        "elleve" => Some(11.0),
        "tolv" => Some(12.0),
        "tretten" => Some(13.0),
        "fjorten" => Some(14.0),
        "femten" => Some(15.0),
        "seksten" => Some(16.0),
        "søtten" | "sytten" => Some(17.0),
        "atten" => Some(18.0),
        "nitten" => Some(19.0),
        _ => None,
    }
}

fn tens(s: &str) -> Option<f64> {
    match s {
        "tyve" | "tjue" => Some(20.0),
        "tredve" | "tretti" => Some(30.0),
        "førti" => Some(40.0),
        "femti" => Some(50.0),
        "seksti" => Some(60.0),
        "sytti" | "søtti" => Some(70.0),
        "åtti" => Some(80.0),
        "nitti" => Some(90.0),
        _ => None,
    }
}

fn twenty_to_hundred_compact(s: &str) -> Option<f64> {
    match s {
        "tjueen" | "tjueén" => Some(21.0),
        "tjueto" => Some(22.0),
        "tjuetre" => Some(23.0),
        "tjuefire" => Some(24.0),
        "tjuefem" => Some(25.0),
        "tjueseks" => Some(26.0),
        "tjuesju" | "tjuesyv" => Some(27.0),
        "tjueåtte" => Some(28.0),
        "tjueni" => Some(29.0),
        "trettien" | "trettién" => Some(31.0),
        "trettito" => Some(32.0),
        "trettitre" => Some(33.0),
        "trettifire" => Some(34.0),
        "trettifem" => Some(35.0),
        "trettiseks" => Some(36.0),
        "trettisju" | "trettisyv" => Some(37.0),
        "trettiåtte" => Some(38.0),
        "trettini" => Some(39.0),
        "førtien" | "førtién" => Some(41.0),
        "førtito" => Some(42.0),
        "førtitre" => Some(43.0),
        "førtifire" => Some(44.0),
        "førtifem" => Some(45.0),
        "førtiseks" => Some(46.0),
        "førtisju" | "førtisyv" => Some(47.0),
        "førtiåtte" => Some(48.0),
        "førtini" => Some(49.0),
        "femtien" | "femtién" => Some(51.0),
        "femtito" => Some(52.0),
        "femtitre" => Some(53.0),
        "femtifire" => Some(54.0),
        "femtifem" => Some(55.0),
        "femtiseks" => Some(56.0),
        "femtisju" | "femtisyv" => Some(57.0),
        "femtiåtte" => Some(58.0),
        "femtini" => Some(59.0),
        "sekstien" | "sekstién" => Some(61.0),
        "sekstito" => Some(62.0),
        "sekstitre" => Some(63.0),
        "sekstifire" => Some(64.0),
        "sekstifem" => Some(65.0),
        "sekstiseks" => Some(66.0),
        "sekstisju" | "sekstisyv" => Some(67.0),
        "sekstiåtte" => Some(68.0),
        "sekstini" => Some(69.0),
        "syttien" | "syttién" | "søttien" | "søttién" => Some(71.0),
        "syttito" | "søttito" => Some(72.0),
        "syttitre" | "søttitre" => Some(73.0),
        "syttifire" | "søttifire" => Some(74.0),
        "syttifem" | "søttifem" => Some(75.0),
        "syttiseks" | "søttiseks" => Some(76.0),
        "syttisju" | "syttisyv" | "søttisju" | "søttisyv" => Some(77.0),
        "syttiåtte" | "søttiåtte" => Some(78.0),
        "syttini" | "søttini" => Some(79.0),
        "åttien" | "åttién" => Some(81.0),
        "åttito" => Some(82.0),
        "åttitre" => Some(83.0),
        "åttifire" => Some(84.0),
        "åttifem" => Some(85.0),
        "åttiseks" => Some(86.0),
        "åttisju" | "åttisyv" => Some(87.0),
        "åttiåtte" => Some(88.0),
        "åttini" => Some(89.0),
        "nittien" | "nittién" => Some(91.0),
        "nittito" => Some(92.0),
        "nittitre" => Some(93.0),
        "nittifire" => Some(94.0),
        "nittifem" => Some(95.0),
        "nittiseks" => Some(96.0),
        "nittisju" | "nittisyv" => Some(97.0),
        "nittiåtte" => Some(98.0),
        "nittini" => Some(99.0),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "intersect (with and)".to_string(),
            pattern: vec![predicate(has_grain), regex("og"), predicate(|td| !is_multipliable(td) && is_positive(td))],
            production: Box::new(|nodes| {
                let n1 = numeral_data(&nodes[0].token_data)?;
                let n2 = numeral_data(&nodes[2].token_data)?;
                let g = n1.grain?;
                if 10f64.powi(g as i32) > n2.value {
                    Some(TokenData::Numeral(NumeralData::new(n1.value + n2.value)))
                } else {
                    None
                }
            }),
        },
        Rule {
            name: "numbers prefix with -, negative or minus".to_string(),
            pattern: vec![regex("-|minus|negativ"), predicate(is_positive)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(-v)))
            }),
        },
        Rule {
            name: "few".to_string(),
            pattern: vec![regex("(noen )?få")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(3.0)))),
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
        Rule {
            name: "decimal number".to_string(),
            pattern: vec![regex("(\\d*,\\d+)")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(t.replace(',', ".").parse().ok()?)))
            }),
        },
        Rule {
            name: "integer 21..99".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(n) if [70.0,20.0,60.0,50.0,40.0,90.0,30.0,80.0].contains(&n.value))),
                predicate(number_between(1.0, 10.0)),
            ],
            production: Box::new(|nodes| {
                let a = numeral_data(&nodes[0].token_data)?.value;
                let b = numeral_data(&nodes[1].token_data)?.value;
                Some(TokenData::Numeral(NumeralData::new(a + b)))
            }),
        },
        Rule {
            name: "single".to_string(),
            pattern: vec![regex("enkelt")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(1.0).with_grain(1)))),
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
            name: "powers of tens".to_string(),
            pattern: vec![regex("(hundre(de)?|tusen?|million(er)?|milliard(?:er)?|billion(?:er)?|billiard(?:er)?|trillion(?:er)?|trilliard(?:er)?|kvadrillion(?:er)?|kvadrilliard(?:er)?)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                let out = match s.as_str() {
                    "hundre" | "hundrede" => NumeralData::new(1e2).with_grain(2).with_multipliable(true),
                    "tuse" | "tusen" => NumeralData::new(1e3).with_grain(3).with_multipliable(true),
                    "million" | "millioner" => NumeralData::new(1e6).with_grain(6).with_multipliable(true),
                    "milliard" | "milliarder" => NumeralData::new(1e9).with_grain(9).with_multipliable(true),
                    "billion" | "billioner" => NumeralData::new(1e12).with_grain(12).with_multipliable(true),
                    "billiard" | "billiarder" => NumeralData::new(1e15).with_grain(15).with_multipliable(true),
                    "trillion" | "trillioner" => NumeralData::new(1e18).with_grain(18).with_multipliable(true),
                    "trilliard" | "trilliarder" => NumeralData::new(1e21).with_grain(21).with_multipliable(true),
                    "kvadrillion" | "kvadrillioner" => NumeralData::new(1e24).with_grain(24).with_multipliable(true),
                    "kvadrilliard" | "kvadrilliarder" => NumeralData::new(1e27).with_grain(27).with_multipliable(true),
                    _ => return None,
                };
                Some(TokenData::Numeral(out))
            }),
        },
        Rule {
            name: "a pair".to_string(),
            pattern: vec![regex("et par")],
            production: Box::new(|_| Some(TokenData::Numeral(NumeralData::new(2.0).with_grain(1)))),
        },
        Rule {
            name: "dozen".to_string(),
            pattern: vec![regex("dusin")],
            production: Box::new(|_| {
                Some(TokenData::Numeral(
                    NumeralData::new(12.0).with_grain(1).with_multipliable(true),
                ))
            }),
        },
        Rule {
            name: "integer (0..19)".to_string(),
            pattern: vec![regex("(intet|ingen|null|en|ett|én|to|tretten|tre|fire|femten|fem|seksten|seks|syv|sju|åtte|nitten|ni|ti|elleve|tolv|fjorten|sytten|søtten|atten)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(zero_to_nineteen(&s)?)))
            }),
        },
        Rule {
            name: "integer (21..99) compact".to_string(),
            pattern: vec![regex("(tjueen|tjueén|tjueto|tjuetre|tjuefire|tjuefem|tjueseks|tjuesju|tjuesyv|tjueåtte|tjueni|trettien|trettién|trettito|trettitre|trettifire|trettifem|trettiseks|trettisju|trettisyv|trettiåtte|trettini|førtien|førtién|førtito|førtitre|førtifire|førtifem|førtiseks|førtisju|førtisyv|førtiåtte|førtini|femtien|femtién|femtito|femtitre|femtifire|femtifem|femtiseks|femtisju|femtisyv|femtiåtte|femtini|sekstien|sekstién|sekstito|sekstitre|sekstifire|sekstifem|sekstiseks|sekstisju|sekstisyv|sekstiåtte|sekstini|syttien|syttién|syttito|syttitre|syttifire|syttifem|syttiseks|syttisju|syttisyv|syttiåtte|syttini|søttien|søttién|søttito|søttitre|søttifire|søttifem|søttiseks|søttisju|søttisyv|søttiåtte|søttini|åttien|åttién|åttito|åttitre|åttifire|åttifem|åttiseks|åttisju|åttisyv|åttiåtte|åttini|nittien|nittién|nittito|nittitre|nittifire|nittifem|nittiseks|nittisju|nittisyv|nittiåtte|nittini)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(twenty_to_hundred_compact(&s)?)))
            }),
        },
        Rule {
            name: "integer (20..90)".to_string(),
            pattern: vec![regex("(tyve|tjue|tredve|tretti|førti|femti|seksti|sytti|søtti|åtti|nitti)")],
            production: Box::new(|nodes| {
                let s = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?.to_lowercase(),
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(tens(&s)?)))
            }),
        },
        Rule {
            name: "number dot number".to_string(),
            pattern: vec![
                predicate(|td| matches!(td, TokenData::Numeral(_))),
                regex("komma"),
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
            pattern: vec![regex("(\\d{1,3}((?:\\.| )\\d\\d\\d){1,5})")],
            production: Box::new(|nodes| {
                let t = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                Some(TokenData::Numeral(NumeralData::new(
                    t.replace('.', "").replace(' ', "").parse().ok()?,
                )))
            }),
        },
    ]
}
