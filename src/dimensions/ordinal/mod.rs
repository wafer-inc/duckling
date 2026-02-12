pub mod en;

use crate::types::DimensionValue;

#[derive(Debug, Clone)]
pub struct OrdinalData {
    pub value: i64,
}

impl OrdinalData {
    pub fn new(value: i64) -> Self {
        OrdinalData { value }
    }
}

pub fn resolve(data: &OrdinalData) -> DimensionValue {
    DimensionValue::Ordinal(data.value)
}
