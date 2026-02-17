use crate::dimensions::numeral::helpers::numeral_data;
use crate::dimensions::time_grain::Grain;
use crate::pattern::{dim, predicate, regex};
use crate::types::{DimensionKind, Rule, TokenData};

use super::DurationData;

fn is_natural(td: &TokenData) -> bool {
    matches!(td, TokenData::Numeral(d) if d.value > 0.0 && d.value.fract() == 0.0)
}

fn fixed(name: &str, pattern: &str, value: i64, grain: Grain) -> Rule {
    Rule {
        name: name.to_string(),
        pattern: vec![regex(pattern)],
        production: Box::new(move |_| Some(TokenData::Duration(DurationData::new(value, grain)))),
    }
}

pub fn rules() -> Vec<Rule> {
    vec![
        Rule {
            name: "single <unit-of-duration>".to_string(),
            pattern: vec![regex("ένα|μια|ένας"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::TimeGrain(g) => Some(TokenData::Duration(DurationData::new(1, *g))),
                _ => None,
            }),
        },
        Rule {
            name: "one second (genitive)".to_string(),
            pattern: vec![regex("ενός δευτερολέπτου|δευτερολέπτου")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(1, Grain::Second)))),
        },
        Rule {
            name: "thirty seconds".to_string(),
            pattern: vec![regex("τριάντα δευτερολέπτων")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(30, Grain::Second)))),
        },
        Rule {
            name: "half minute".to_string(),
            pattern: vec![regex("μισό λεπτό")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(30, Grain::Second)))),
        },
        Rule {
            name: "one minute (genitive)".to_string(),
            pattern: vec![regex("ενός λεπτού")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(1, Grain::Minute)))),
        },
        Rule {
            name: "two minutes (single word)".to_string(),
            pattern: vec![regex("δίλεπτο")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(2, Grain::Minute)))),
        },
        Rule {
            name: "two minutes".to_string(),
            pattern: vec![regex("δύο λεπτά|δυο λεπτά")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(2, Grain::Minute)))),
        },
        Rule {
            name: "half an hour".to_string(),
            pattern: vec![regex(r"(1/2\s?((της )?ώρας?|ω)|μισάωρου?)")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))),
        },
        Rule {
            name: "quarter of an hour".to_string(),
            pattern: vec![regex(r"(1/4|[εέ]ν(α|ός)\s+τ[εέ]τ[αά]ρτου?)(\s*ω|\s+(της\s+)?ώρας)?")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(15, Grain::Minute)))),
        },
        Rule {
            name: "<integer> and a half <grain>".to_string(),
            pattern: vec![predicate(is_natural), regex("και μισ[ήό]ς?"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let g = match &nodes[2].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = match g {
                    Grain::Minute => DurationData::new(60_i64.checked_mul(v)?.checked_add(30)?, Grain::Second),
                    Grain::Hour => DurationData::new(60_i64.checked_mul(v)?.checked_add(30)?, Grain::Minute),
                    Grain::Day => DurationData::new(24_i64.checked_mul(v)?.checked_add(12)?, Grain::Hour),
                    Grain::Month => DurationData::new(30_i64.checked_mul(v)?.checked_add(15)?, Grain::Day),
                    Grain::Year => DurationData::new(12_i64.checked_mul(v)?.checked_add(6)?, Grain::Month),
                    _ => return None,
                };
                Some(TokenData::Duration(dd))
            }),
        },
        Rule {
            name: "<integer> + '\"".to_string(),
            pattern: vec![
                predicate(is_natural),
                regex(r#"(['"])"#),
            ],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let q = match &nodes[1].token_data {
                    TokenData::RegexMatch(m) => m.group(1)?,
                    _ => return None,
                };
                match q {
                    "'" => Some(TokenData::Duration(DurationData::new(v, Grain::Minute))),
                    "\"" => Some(TokenData::Duration(DurationData::new(v, Grain::Second))),
                    _ => None,
                }
            }),
        },
        fixed("2 minutes (digits)", r"2 λεπτά", 2, Grain::Minute),
        fixed("15 minutes", r"δεκαπέντε λεπτά|δεκαπεντάλεπτο", 15, Grain::Minute),
        fixed("15 minutes quotes", r"15'", 15, Grain::Minute),
        fixed("30 seconds digits", r#"30 δευτερόλεπτα|30""#, 30, Grain::Second),
        fixed("30 minutes", r"τριάντα λεπτά|μισάωρο|μισή ώρα", 30, Grain::Minute),
        fixed("30 minutes quotes", r"30'", 30, Grain::Minute),
        fixed("45 minutes", r"τρία τέταρτα|σαρανταπεντάλεπτος", 45, Grain::Minute),
        fixed("45 minutes quotes", r"45'", 45, Grain::Minute),
        fixed("60 minutes", r"60 λεπτά|εξηντάλεπτο", 60, Grain::Minute),
        fixed(
            "90 minutes",
            r"μια και μισή ώρα|μιάμιση ώρα|1,5 ώρα|περίπου μια και μισή ώρα|ακριβώς μια και μισή ώρα",
            90,
            Grain::Minute,
        ),
        fixed("5 hours", r"πεντάωρο|5 ώρες", 5, Grain::Hour),
        fixed("60 hours", r"δυόμισι μέρες|60 ώρες|εξήντα ώρες", 60, Grain::Hour),
        fixed("15 days", r"15 μέρες|δεκαπενθήμερο", 15, Grain::Day),
        fixed("30 days", r"30 μέρες", 30, Grain::Day),
        fixed("7 weeks", r"εφτά εβδομάδες|7 βδομάδες", 7, Grain::Week),
        fixed("1 month", r"1 μήνας|ένα μήνα", 1, Grain::Month),
        fixed("3 quarters", r"3 τρίμηνα", 3, Grain::Quarter),
        fixed(
            "18 months",
            r"18 μήνες|ενάμισης χρόνος|ένας και μισός χρόνος|ενάμισι έτος|ένα και μισό έτος",
            18,
            Grain::Month,
        ),
        fixed("2 years", r"δυο χρόνια|δύο έτη|διετία|διετής|δίχρονο", 2, Grain::Year),
        fixed(
            "35 years",
            r"τριανταπενταετής|τριανταπεντάχρονος|τριανταπενταετία|35 χρόνια",
            35,
            Grain::Year,
        ),
    ]
}
