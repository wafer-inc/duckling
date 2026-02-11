pub mod rules;

use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct UrlData {
    pub value: String,
    pub domain: String,
}

impl UrlData {
    pub fn new(value: &str, domain: &str) -> Self {
        UrlData {
            value: value.to_string(),
            domain: domain.to_string(),
        }
    }
}

pub fn resolve(data: &UrlData) -> ResolvedValue {
    ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": data.value,
            "type": "value",
            "domain": data.domain,
        }),
    }
}
