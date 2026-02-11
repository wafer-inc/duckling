pub mod rules;

use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct EmailData {
    pub value: String,
}

impl EmailData {
    pub fn new(value: &str) -> Self {
        EmailData {
            value: value.to_string(),
        }
    }
}

pub fn resolve(data: &EmailData) -> ResolvedValue {
    ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": data.value,
            "type": "value",
        }),
    }
}
