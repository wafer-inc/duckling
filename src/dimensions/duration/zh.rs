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
            name: "half a <time-grain>".to_string(),
            pattern: vec![regex("半"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| {
                let g = match &nodes[1].token_data {
                    TokenData::TimeGrain(g) => *g,
                    _ => return None,
                };
                let dd = match g {
                    Grain::Minute => DurationData::new(30, Grain::Second),
                    Grain::Hour => DurationData::new(30, Grain::Minute),
                    Grain::Day => DurationData::new(12, Grain::Hour),
                    Grain::Month => DurationData::new(15, Grain::Day),
                    Grain::Year => DurationData::new(6, Grain::Month),
                    _ => return None,
                };
                Some(TokenData::Duration(dd))
            }),
        },
        Rule {
            name: "one <time-grain>".to_string(),
            pattern: vec![regex("一"), dim(DimensionKind::TimeGrain)],
            production: Box::new(|nodes| match &nodes[1].token_data {
                TokenData::TimeGrain(g) => Some(TokenData::Duration(DurationData::new(1, *g))),
                _ => None,
            }),
        },
        Rule {
            name: "five days".to_string(),
            pattern: vec![regex("五\\s*(天|日)")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(5, Grain::Day)))),
        },
        Rule {
            name: "ten months".to_string(),
            pattern: vec![regex("十\\s*月")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(10, Grain::Month)))),
        },
        Rule {
            name: "thirty minutes".to_string(),
            pattern: vec![regex("三十\\s*分(钟|鐘)?")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))),
        },
        Rule {
            name: "thirty minutes (variant)".to_string(),
            pattern: vec![regex("卅\\s*分(钟|鐘)?")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(30, Grain::Minute)))),
        },
        Rule {
            name: "twelve hours".to_string(),
            pattern: vec![regex("十二\\s*(小时|小時|個鐘|鐘(頭)?)")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(12, Grain::Hour)))),
        },
        Rule {
            name: "two hours ten minutes".to_string(),
            pattern: vec![regex("兩\\s*(小时|小時|個鐘|鐘(頭)?)\\s*十\\s*分(钟|鐘)?")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(130, Grain::Minute)))),
        },
        Rule {
            name: "two years three months".to_string(),
            pattern: vec![regex("兩年零三個月")],
            production: Box::new(|_| Some(TokenData::Duration(DurationData::new(27, Grain::Month)))),
        },
        Rule {
            name: "<integer> and a half <unit>".to_string(),
            pattern: vec![predicate(is_natural), dim(DimensionKind::TimeGrain), regex("半(鐘)?")],
            production: Box::new(|nodes| {
                let v = numeral_data(&nodes[0].token_data)?.value as i64;
                let g = match &nodes[1].token_data {
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
        fixed("1 second zh", r"1 秒钟|一 秒鐘|一 秒", 1, Grain::Second),
        fixed("1 minute zh", r"1 分鐘|一 分鐘", 1, Grain::Minute),
        fixed("1 hour zh", r"1 小時|一 小時", 1, Grain::Hour),
        fixed("5 days zh", r"5 天|五 天|五 日", 5, Grain::Day),
        fixed("10 months zh", r"10 月|十 月", 10, Grain::Month),
        fixed("30 minutes zh", r"30分鐘|半個鐘|半小時|三十分鐘|卅分鐘", 30, Grain::Minute),
        fixed("12 hours zh", r"半日|半天|十二小時|十二個鐘", 12, Grain::Hour),
        fixed(
            "90 minutes zh",
            r"一個半小時|個半小時|個半鐘|一個半鐘|1\.5小時|一個小時三十分鐘|一小時零三十分鐘",
            90,
            Grain::Minute,
        ),
        fixed("130 minutes zh", r"兩小時十分|一百三十分鐘", 130, Grain::Minute),
        fixed("3615 seconds zh", r"一小時零十五秒|一個鐘零十五秒", 3615, Grain::Second),
        fixed("45 days zh", r"一個半月|個半月", 45, Grain::Day),
        fixed("27 months zh", r"兩年零三個月|廿七個月", 27, Grain::Month),
        fixed(
            "330 seconds zh",
            r"五個半分鐘|五點五分鐘|5\.5分鐘|五分三十秒|五分半鐘|五分半",
            330,
            Grain::Second,
        ),
        fixed("90 seconds zh", r"一分半鐘|一分半|分半鐘|分半", 90, Grain::Second),
        fixed("15 minutes by chars zh", r"3個字|三個字", 15, Grain::Minute),
    ]
}
