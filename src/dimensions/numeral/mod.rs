pub mod en;
pub mod helpers;

use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct NumeralData {
    pub value: f64,
    pub grain: Option<u8>,
    pub multipliable: bool,
}

impl NumeralData {
    pub fn new(value: f64) -> Self {
        NumeralData {
            value,
            grain: None,
            multipliable: false,
        }
    }

    pub fn with_grain(mut self, grain: u8) -> Self {
        self.grain = Some(grain);
        self
    }

    pub fn with_multipliable(mut self, multipliable: bool) -> Self {
        self.multipliable = multipliable;
        self
    }
}

pub fn resolve(data: &NumeralData) -> ResolvedValue {
    ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": data.value,
            "type": "value",
        }),
    }
}
