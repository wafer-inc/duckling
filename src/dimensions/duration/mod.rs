pub mod ar;
pub mod bg;
pub mod ca;
pub mod de;
pub mod el;
pub mod en;
pub mod es;
pub mod fr;
pub mod ga;
pub mod hi;
pub mod hu;
pub mod ja;
pub mod ka;
pub mod ko;
pub mod mn;
pub mod nb;
pub mod nl;
pub mod pl;
pub mod ro;
pub mod ru;
pub mod sv;
pub mod tr;
pub mod uk;
pub mod zh;

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
            let seconds = self.grain.in_seconds(self.value).unwrap_or(0) as f64;
            let v = (seconds / g.one_in_seconds_f64()).round() as i64;
            DurationData::new(v, g)
        }
    }

    /// Combine two durations (Haskell Semigroup `<>`).
    /// Converts both to the smaller grain, then adds values.
    pub fn combine(&self, other: &DurationData) -> Option<DurationData> {
        let g = std::cmp::min(self.grain, other.grain);
        let v1 = self.with_grain(g).value;
        let v2 = other.with_grain(g).value;
        Some(DurationData::new(v1.checked_add(v2)?, g))
    }
}

pub fn resolve(data: &DurationData) -> DimensionValue {
    DimensionValue::Duration {
        value: data.value,
        grain: data.grain,
        normalized_seconds: data.grain.in_seconds(data.value).unwrap_or(0),
    }
}
