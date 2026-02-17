use crate::dimensions::numeral::helpers::{is_natural, numeral_data};
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

fn duration_data(td: &TokenData) -> Option<&DurationData> {
    match td {
        TokenData::Duration(d) => Some(d),
        _ => None,
    }
}

fn is_grain(expected: Grain) -> impl Fn(&TokenData) -> bool {
    move |td| matches!(td, TokenData::TimeGrain(g) if *g == expected)
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "quarter of an hour".to_string(),
            pattern: vec![regex("(einer? )?Viertelstunde")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(15, Grain::Minute)))
            }),
        },
        Rule {
            name: "half an hour".to_string(),
            pattern: vec![regex("(1/2\\s?|(eine )?halbe |(einer )?halben )stunde")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))
            }),
        },
        Rule {
            name: "three-quarters of an hour".to_string(),
            pattern: vec![regex(
                "3/4\\s?stunde|(einer? )?dreiviertel stunde|drei viertelstunden",
            )],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(45, Grain::Minute)))
            }),
        },
        Rule {
            name: "fortnight".to_string(),
            pattern: vec![regex("zwei Wochen")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(14, Grain::Day)))),
        },
        Rule {
            name: "about|exactly <duration>".to_string(),
            pattern: vec![
                regex("ungef(ä|a)hr|zirka|genau|exakt"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        Rule {
            name: "number,number hours".to_string(),
            pattern: vec![regex("(\\d+),(\\d+)"), predicate(is_grain(Grain::Hour))],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let hh: i64 = m.group(1)?.parse().ok()?;
                let frac = m.group(2)?;
                let num: i64 = frac.parse().ok()?;
                let den: i64 = 10_i64.pow(frac.len() as u32);
                let minutes = 60_i64
                    .checked_mul(hh)?
                    .checked_add(num.checked_mul(60)?.checked_div(den)?)?;
                Some(TokenData::Duration(DurationData::new(
                    minutes,
                    Grain::Minute,
                )))
            }),
        },
        Rule {
            name: "number,number minutes".to_string(),
            pattern: vec![regex("(\\d+),(\\d+)"), predicate(is_grain(Grain::Minute))],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let mm: i64 = m.group(1)?.parse().ok()?;
                let frac = m.group(2)?;
                let num: i64 = frac.parse().ok()?;
                let den: i64 = 10_i64.pow(frac.len() as u32);
                let seconds = 60_i64
                    .checked_mul(mm)?
                    .checked_add(num.checked_mul(60)?.checked_div(den)?)?;
                Some(TokenData::Duration(DurationData::new(
                    seconds,
                    Grain::Second,
                )))
            }),
        },
        Rule {
            name: "<integer> and a half hour".to_string(),
            pattern: vec![
                predicate(is_natural),
                regex("(und )?(ein(en?)? )?halb(en?)?"),
                predicate(is_grain(Grain::Hour)),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                Some(TokenData::Duration(DurationData::new(
                    60_i64.checked_mul(v)?.checked_add(30)?,
                    Grain::Minute,
                )))
            }),
        },
        Rule {
            name: "<integer> and a half minutes".to_string(),
            pattern: vec![
                predicate(is_natural),
                regex("(und )?(ein(en?)? ?)?halb(en?)?"),
                predicate(is_grain(Grain::Minute)),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                Some(TokenData::Duration(DurationData::new(
                    60_i64.checked_mul(v)?.checked_add(30)?,
                    Grain::Second,
                )))
            }),
        },
        Rule {
            name: "a <unit-of-duration>".to_string(),
            pattern: vec![regex("ein(en?)?"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(1, g)))
            }),
        },
        Rule {
            name: "half a <time-grain>".to_string(),
            pattern: vec![
                regex("(ein(en)?)?(1/2|halbe?)"),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                match g {
                    Grain::Minute => {
                        Some(TokenData::Duration(DurationData::new(30, Grain::Second)))
                    }
                    Grain::Hour => Some(TokenData::Duration(DurationData::new(30, Grain::Minute))),
                    Grain::Day => Some(TokenData::Duration(DurationData::new(12, Grain::Hour))),
                    Grain::Month => Some(TokenData::Duration(DurationData::new(15, Grain::Day))),
                    Grain::Year => Some(TokenData::Duration(DurationData::new(6, Grain::Month))),
                    _ => None,
                }
            }),
        },
        Rule {
            name: "composite <duration> (with ,/and)".to_string(),
            pattern: vec![
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
                regex(",|und"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let d2 = duration_data(&nodes[3].token_data)?;
                if g <= d2.grain {
                    return None;
                }
                Some(TokenData::Duration(DurationData::new(v, g).combine(d2)?))
            }),
        },
        Rule {
            name: "composite <duration>".to_string(),
            pattern: vec![
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let d2 = duration_data(&nodes[2].token_data)?;
                if g <= d2.grain {
                    return None;
                }
                Some(TokenData::Duration(DurationData::new(v, g).combine(d2)?))
            }),
        },
        Rule {
            name: "composite <duration> and <duration>".to_string(),
            pattern: vec![
                dim(DimensionKind::Duration),
                regex(",|und"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let d1 = duration_data(&nodes[0].token_data)?;
                let d2 = duration_data(&nodes[2].token_data)?;
                if d1.grain <= d2.grain {
                    return None;
                }
                Some(TokenData::Duration(d1.combine(d2)?))
            }),
        },
        Rule {
            name: "<integer> hour and <integer>".to_string(),
            pattern: vec![
                predicate(is_natural),
                regex("(ein(en?) )?stunden?( und)?"),
                predicate(|td| {
                    is_natural(td)
                        && matches!(td, TokenData::Numeral(n) if n.value >= 1.0 && n.value < 60.0)
                }),
                predicate(is_grain(Grain::Minute)),
            ],
            production: Box::new(|nodes| {
                let h = numeral_data(&nodes[0].token_data)?.value as i64;
                let m = numeral_data(&nodes[2].token_data)?.value as i64;
                Some(TokenData::Duration(DurationData::new(
                    60_i64.checked_mul(h)?.checked_add(m)?,
                    Grain::Minute,
                )))
            }),
        },
    ]
}
