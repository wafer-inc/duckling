use crate::dimensions::numeral::helpers::{is_natural, numeral_data};
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, PatternItem, Rule, TokenData};

use super::DurationData;

/// Helper: predicate that matches a specific TimeGrain.
fn is_grain(g: Grain) -> PatternItem {
    predicate(move |td| matches!(td, TokenData::TimeGrain(grain) if *grain == g))
}

/// Helper: extract DurationData from a token.
fn duration_data(td: &TokenData) -> Option<&DurationData> {
    match td {
        TokenData::Duration(d) => Some(d),
        _ => None,
    }
}

/// n-and-a-half: e.g., 1.5 hours = 90 minutes.
/// Matches Haskell's `nPlusOneHalf`.
fn n_plus_one_half(grain: Grain, n: i64) -> Option<DurationData> {
    match grain {
        Grain::Minute => Some(DurationData::new(
            60_i64.checked_mul(n)?.checked_add(30)?,
            Grain::Second,
        )),
        Grain::Hour => Some(DurationData::new(
            60_i64.checked_mul(n)?.checked_add(30)?,
            Grain::Minute,
        )),
        Grain::Day => Some(DurationData::new(
            24_i64.checked_mul(n)?.checked_add(12)?,
            Grain::Hour,
        )),
        Grain::Month => Some(DurationData::new(
            30_i64.checked_mul(n)?.checked_add(15)?,
            Grain::Day,
        )),
        Grain::Year => Some(DurationData::new(
            12_i64.checked_mul(n)?.checked_add(6)?,
            Grain::Month,
        )),
        _ => None,
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        // === Base rule (from Duration/Rules.hs) ===
        // <integer> <unit-of-duration>: "3 days", "2 hours"
        Rule {
            name: "<integer> <unit-of-duration>".to_string(),
            pattern: vec![predicate(is_natural), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(
                    num.value as i64,
                    grain,
                )))
            }),
        },
        // === EN-specific rules (from Duration/EN/Rules.hs) ===
        // quarter of an hour: "1/4 h", "a quarter of an hour"
        Rule {
            name: "quarter of an hour".to_string(),
            pattern: vec![regex(r"(1/4\s?h(our)?|(a\s)?quarter of an hour)")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(15, Grain::Minute)))
            }),
        },
        // half an hour (abbrev): "1/2 h", "1/2h"
        Rule {
            name: "half an hour (abbrev)".to_string(),
            pattern: vec![regex(r"1/2\s?h")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))
            }),
        },
        // three-quarters of an hour: "3/4 h", "three-quarters of an hour"
        Rule {
            name: "three-quarters of an hour".to_string(),
            pattern: vec![regex(r"(3/4\s?h(our)?|three(\s|-)quarters of an hour)")],
            production: Box::new(|_| {
                Some(TokenData::Duration(DurationData::new(45, Grain::Minute)))
            }),
        },
        // fortnight
        Rule {
            name: "fortnight".to_string(),
            pattern: vec![regex(r"(a|one)?\s*fortnight")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(14, Grain::Day)))),
        },
        // <integer> + '/" : "2'" = 2 minutes, "1"" = 1 second
        Rule {
            name: "<integer> + '\"".to_string(),
            pattern: vec![predicate(is_natural), regex(r#"(['"])"#)],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let quote = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                let grain = match quote {
                    "'" => Grain::Minute,
                    "\"" => Grain::Second,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(
                    num.value as i64,
                    grain,
                )))
            }),
        },
        // <integer> more/additional/extra/less/fewer <unit-of-duration>
        Rule {
            name: "<integer> more <unit-of-duration>".to_string(),
            pattern: vec![
                predicate(is_natural),
                regex(r"more|additional|extra|less|fewer"),
                dim(DimensionKind::TimeGrain),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let grain = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(
                    num.value as i64,
                    grain,
                )))
            }),
        },
        // number.number hours: "1.5 hours" → 90 minutes
        Rule {
            name: "number.number hours".to_string(),
            pattern: vec![regex(r"(\d+)\.(\d+)"), is_grain(Grain::Hour)],
            production: Box::new(|nodes| {
                let m = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let h: i64 = m.group(1)?.parse().ok()?;
                let m_str = m.group(2)?;
                let m_num: i64 = m_str.parse().ok()?;
                let d: i64 = 10_i64.pow(m_str.len() as u32);
                let total_minutes = 60_i64
                    .checked_mul(h)?
                    .checked_add(m_num.checked_mul(60)?.checked_div(d)?)?;
                Some(TokenData::Duration(DurationData::new(
                    total_minutes,
                    Grain::Minute,
                )))
            }),
        },
        // number.number minutes: "15.5 minutes" → 930 seconds
        Rule {
            name: "number.number minutes".to_string(),
            pattern: vec![regex(r"(\d+)\.(\d+)"), is_grain(Grain::Minute)],
            production: Box::new(|nodes| {
                let rm = match &nodes[0].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let mins: i64 = rm.group(1)?.parse().ok()?;
                let s_str = rm.group(2)?;
                let s_num: i64 = s_str.parse().ok()?;
                let d: i64 = 10_i64.pow(s_str.len() as u32);
                let total_seconds = 60_i64
                    .checked_mul(mins)?
                    .checked_add(s_num.checked_mul(60)?.checked_div(d)?)?;
                Some(TokenData::Duration(DurationData::new(
                    total_seconds,
                    Grain::Second,
                )))
            }),
        },
        // <integer> and a half hour(s): "5 and a half hours"
        Rule {
            name: "<integer> and a half hour".to_string(),
            pattern: vec![predicate(is_natural), regex(r"and (an? )?half hours?")],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let v = num.value as i64;
                Some(TokenData::Duration(DurationData::new(
                    60_i64.checked_mul(v)?.checked_add(30)?,
                    Grain::Minute,
                )))
            }),
        },
        // <integer> and a half minute(s): "5 and a half minutes"
        Rule {
            name: "<integer> and a half minute".to_string(),
            pattern: vec![predicate(is_natural), regex(r"and (an? )?half min(ute)?s?")],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let v = num.value as i64;
                Some(TokenData::Duration(DurationData::new(
                    60_i64.checked_mul(v)?.checked_add(30)?,
                    Grain::Second,
                )))
            }),
        },
        // a <unit-of-duration>: "a day", "an hour"
        Rule {
            name: "a <unit-of-duration>".to_string(),
            pattern: vec![regex(r"an?"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                Some(TokenData::Duration(DurationData::new(1, grain)))
            }),
        },
        // half a <time-grain>: "half an hour", "1/2 day", "half hour"
        Rule {
            name: "half a <time-grain>".to_string(),
            pattern: vec![regex(r"(1/2|half)( an?)?"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = n_plus_one_half(grain, 0)?;
                Some(TokenData::Duration(dd))
            }),
        },
        // a <unit-of-duration> and a half: "an hour and a half", "a month and a half"
        Rule {
            name: "a <unit-of-duration> and a half".to_string(),
            pattern: vec![
                regex(r"an?|one"),
                dim(DimensionKind::TimeGrain),
                regex(r"and (a )?half"),
            ],
            production: Box::new(|nodes| {
                let grain = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = n_plus_one_half(grain, 1)?;
                Some(TokenData::Duration(dd))
            }),
        },
        // <integer> hour(s) and <integer> (minutes implied): "1 hour thirty", "2 hours ten"
        Rule {
            name: "<integer> hour and <integer>".to_string(),
            pattern: vec![
                predicate(is_natural),
                regex(r"hours?( and)?"),
                predicate(|td| {
                    is_natural(td)
                        && matches!(td, TokenData::Numeral(d) if d.value >= 1.0 && d.value < 60.0)
                }),
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
        // about|exactly <duration>: "about 3 hours", "approximately 2 days"
        Rule {
            name: "about|exactly <duration>".to_string(),
            pattern: vec![
                regex(r"(about|around|approximately|exactly)"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| Some(nodes[1].token_data.clone())),
        },
        // <Integer> and <Integer> quarter(s) (of) hour(s):
        // "one and two quarter hour" → 90min, "two and a quarter hour" → 135min
        Rule {
            name: "<integer> and <integer> quarter of hour".to_string(),
            pattern: vec![
                predicate(is_natural),
                regex(r"and (a |an |one |two |three )?quarters?( of)?( an)?"),
                is_grain(Grain::Hour),
            ],
            production: Box::new(|nodes| {
                let h = numeral_data(&nodes[0].token_data)?.value as i64;
                let regex_data = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m,
                    _ => return None,
                };
                let q_match = regex_data.group(1).unwrap_or("");
                let q = match q_match.trim().to_lowercase().as_str() {
                    "a" | "an" | "one" => 1,
                    "two" => 2,
                    "three" => 3,
                    _ => 1, // default (empty or unrecognized)
                };
                Some(TokenData::Duration(DurationData::new(
                    15_i64.checked_mul(q)?.checked_add(60_i64.checked_mul(h)?)?,
                    Grain::Minute,
                )))
            }),
        },
        // composite <duration> (with ,/and): "2 years, 3 months", "1 hour and 30 seconds"
        Rule {
            name: "composite <duration> (with ,/and)".to_string(),
            pattern: vec![
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
                regex(r",|and"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = duration_data(&nodes[3].token_data)?;
                if g <= dd.grain {
                    return None; // larger grain must come first
                }
                let d1 = DurationData::new(num.value as i64, g);
                Some(TokenData::Duration(d1.combine(dd)?))
            }),
        },
        // composite <duration>: "2 years 3 months" (no separator)
        Rule {
            name: "composite <duration>".to_string(),
            pattern: vec![
                predicate(is_natural),
                dim(DimensionKind::TimeGrain),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let num = numeral_data(&nodes[0].token_data)?;
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = duration_data(&nodes[2].token_data)?;
                if g <= dd.grain {
                    return None;
                }
                let d1 = DurationData::new(num.value as i64, g);
                Some(TokenData::Duration(d1.combine(dd)?))
            }),
        },
        // composite <duration> and <duration>: "an hour and 30 seconds"
        Rule {
            name: "composite <duration> and <duration>".to_string(),
            pattern: vec![
                dim(DimensionKind::Duration),
                regex(r",|and"),
                dim(DimensionKind::Duration),
            ],
            production: Box::new(|nodes| {
                let d1 = duration_data(&nodes[0].token_data)?;
                let d2 = duration_data(&nodes[2].token_data)?;
                if d1.grain <= d2.grain {
                    return None; // larger grain must come first
                }
                Some(TokenData::Duration(d1.combine(d2)?))
            }),
        },
    ]
}

fn is_common_rule_name(name: &str) -> bool {
    name == "<integer> <unit-of-duration>"
}

pub fn common_rules() -> Vec<Rule> {
    rules()
        .into_iter()
        .filter(|r| is_common_rule_name(&r.name))
        .collect()
}

pub fn lang_rules() -> Vec<Rule> {
    rules()
        .into_iter()
        .filter(|r| !is_common_rule_name(&r.name))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dimensions::{numeral, time_grain};
    use crate::engine;
    use crate::resolve::{Context, Options};
    use crate::types::DimensionKind;

    fn all_rules() -> Vec<Rule> {
        let mut r = numeral::en::rules();
        r.extend(time_grain::en::rules());
        r.extend(rules());
        r
    }

    #[test]
    fn test_duration() {
        let rules = all_rules();
        let options = Options { with_latent: false };
        let context = Context::default();

        for (text, expected_val, expected_unit) in &[
            ("3 days", 3, "day"),
            ("2 hours", 2, "hour"),
            ("1 week", 1, "week"),
            ("5 minutes", 5, "minute"),
        ] {
            let entities = engine::parse_and_resolve(
                text,
                &rules,
                &context,
                &options,
                &[DimensionKind::Duration],
            );
            let found = entities.iter().any(|e| {
                matches!(&e.value, crate::types::DimensionValue::Duration { value, grain, .. } if *value == *expected_val as i64 && grain.as_str() == *expected_unit)
            });
            assert!(
                found,
                "Expected {} {} for '{}', got: {:?}",
                expected_val, expected_unit, text, entities
            );
        }
    }
}
