pub mod en;

use crate::types::DimensionValue;

/// Time grain, ordered from smallest to largest (Second < Minute < ... < Year).
/// Ordering matches Haskell Duckling's derived Ord.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize)]
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

    pub fn from_str(s: &str) -> Grain {
        match s {
            "second" => Grain::Second,
            "minute" => Grain::Minute,
            "hour" => Grain::Hour,
            "day" => Grain::Day,
            "week" => Grain::Week,
            "month" => Grain::Month,
            "quarter" => Grain::Quarter,
            "year" => Grain::Year,
            _ => Grain::Second,
        }
    }

    fn order(&self) -> u8 {
        match self {
            Grain::Second => 0,
            Grain::Minute => 1,
            Grain::Hour => 2,
            Grain::Day => 3,
            Grain::Week => 4,
            Grain::Month => 5,
            Grain::Quarter => 6,
            Grain::Year => 7,
        }
    }

    /// Returns the next finer grain level, matching Haskell's TG.lower.
    pub fn lower(&self) -> Grain {
        match self {
            Grain::Year => Grain::Month,
            Grain::Quarter => Grain::Month,
            Grain::Month => Grain::Day,
            Grain::Week => Grain::Day,
            Grain::Day => Grain::Hour,
            Grain::Hour => Grain::Minute,
            Grain::Minute => Grain::Second,
            Grain::Second => Grain::Second,
        }
    }

    /// Number of seconds in `n` units of this grain.
    /// Matches Haskell Duckling's `inSeconds`.
    pub fn in_seconds(&self, n: i64) -> i64 {
        match self {
            Grain::Second => n,
            Grain::Minute => n * 60,
            Grain::Hour => n * 3600,
            Grain::Day => n * 86400,
            Grain::Week => n * 604800,
            Grain::Month => n * 2592000,    // 30 days
            Grain::Quarter => n * 7776000,  // 90 days
            Grain::Year => n * 31536000,    // 365 days
        }
    }

    /// Number of seconds in one unit of this grain, as f64.
    pub fn one_in_seconds_f64(&self) -> f64 {
        self.in_seconds(1) as f64
    }
}

impl PartialOrd for Grain {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Grain {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.order().cmp(&other.order())
    }
}

pub fn resolve(grain: &Grain) -> DimensionValue {
    DimensionValue::TimeGrain(*grain)
}
