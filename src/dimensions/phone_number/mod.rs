pub mod rules;

use crate::types::DimensionValue;

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

pub fn resolve(data: &PhoneNumberData) -> DimensionValue {
    DimensionValue::PhoneNumber(data.value.clone())
}
