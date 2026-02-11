pub mod rules;

use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct PhoneNumberData {
    pub value: String,
}

impl PhoneNumberData {
    pub fn new(value: &str) -> Self {
        PhoneNumberData {
            value: value.to_string(),
        }
    }
}

pub fn resolve(data: &PhoneNumberData) -> ResolvedValue {
    ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": data.value,
            "type": "value",
        }),
    }
}
