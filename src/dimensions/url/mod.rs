pub mod rules;

use crate::types::DimensionValue;

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

pub fn resolve(data: &UrlData) -> DimensionValue {
    DimensionValue::Url {
        value: data.value.clone(),
        domain: data.domain.clone(),
    }
}
