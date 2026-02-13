pub mod de;
pub mod fr;
pub mod it;
pub mod rules;

use crate::types::DimensionValue;

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

pub fn resolve(data: &EmailData) -> DimensionValue {
    DimensionValue::Email(data.value.clone())
}
