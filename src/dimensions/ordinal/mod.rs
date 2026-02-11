pub mod en;

use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct OrdinalData {
    pub value: i64,
}

impl OrdinalData {
    pub fn new(value: i64) -> Self {
        OrdinalData { value }
    }
}

pub fn resolve(data: &OrdinalData) -> ResolvedValue {
    ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": data.value,
            "type": "value",
        }),
    }
}
