pub mod en;

use crate::types::ResolvedValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Grain {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Quarter,
    Year,
}

impl Grain {
    pub fn as_str(&self) -> &'static str {
        match self {
            Grain::Second => "second",
            Grain::Minute => "minute",
            Grain::Hour => "hour",
            Grain::Day => "day",
            Grain::Week => "week",
            Grain::Month => "month",
            Grain::Quarter => "quarter",
            Grain::Year => "year",
        }
    }
}

pub fn resolve(grain: &Grain) -> ResolvedValue {
    ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": grain.as_str(),
            "type": "value",
        }),
    }
}
