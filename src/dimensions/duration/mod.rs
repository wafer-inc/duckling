pub mod en;

use crate::dimensions::time_grain::Grain;
use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct DurationData {
    pub value: i64,
    pub grain: Grain,
}

impl DurationData {
    pub fn new(value: i64, grain: Grain) -> Self {
        DurationData { value, grain }
    }
}

pub fn resolve(data: &DurationData) -> ResolvedValue {
    ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": data.value,
            "type": "value",
            "unit": data.grain.as_str(),
            "normalized": {
                "value": data.value,
                "unit": data.grain.as_str(),
            },
        }),
    }
}
