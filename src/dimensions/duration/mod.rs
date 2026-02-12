pub mod en;

use crate::dimensions::time_grain::Grain;
use crate::types::DimensionValue;

#[derive(Debug, Clone)]
pub struct DurationData {
    pub value: i64,
    pub grain: Grain,
}

impl DurationData {
    pub fn new(value: i64, grain: Grain) -> Self {
        DurationData { value, grain }
    }

    /// Convert this duration to a different grain, rounding to nearest integer.
    /// Matches Haskell Duckling's `withGrain`.
    pub fn with_grain(&self, g: Grain) -> DurationData {
        if self.grain == g {
            self.clone()
        } else {
            let seconds = self.grain.in_seconds(self.value) as f64;
            let v = (seconds / g.one_in_seconds_f64()).round() as i64;
            DurationData::new(v, g)
        }
    }

    /// Combine two durations (Haskell Semigroup `<>`).
    /// Converts both to the smaller grain, then adds values.
    pub fn combine(&self, other: &DurationData) -> DurationData {
        let g = std::cmp::min(self.grain, other.grain);
        let v1 = self.with_grain(g).value;
        let v2 = other.with_grain(g).value;
        DurationData::new(v1 + v2, g)
    }
}

pub fn resolve(data: &DurationData) -> DimensionValue {
    DimensionValue::Duration {
        value: data.value,
        grain: data.grain,
        normalized_seconds: data.grain.in_seconds(data.value),
    }
}
