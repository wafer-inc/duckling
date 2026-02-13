pub mod ar;
pub mod ca;
pub mod da;
pub mod de;
pub mod et;
pub mod en;
pub mod es;
pub mod el;
pub mod bg;
pub mod ga;
pub mod hi;
pub mod he;
pub mod fr;
pub mod hr;
pub mod hu;
pub mod id;
pub mod it;
pub mod ja;
pub mod ka;
pub mod km;
pub mod ko;
pub mod mn;
pub mod ml;
pub mod nb;
pub mod nl;
pub mod pl;
pub mod pt;
pub mod ro;
pub mod ru;
pub mod sv;
pub mod ta;
pub mod tr;
pub mod uk;
pub mod vi;
pub mod zh;

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
