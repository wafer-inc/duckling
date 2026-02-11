pub mod en;

use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct TemperatureData {
    pub value: f64,
    pub unit: Option<TemperatureUnit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Degree,
}

impl TemperatureUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            TemperatureUnit::Celsius => "celsius",
            TemperatureUnit::Fahrenheit => "fahrenheit",
            TemperatureUnit::Degree => "degree",
        }
    }
}

impl TemperatureData {
    pub fn new(value: f64) -> Self {
        TemperatureData { value, unit: None }
    }

    pub fn with_unit(mut self, unit: TemperatureUnit) -> Self {
        self.unit = Some(unit);
        self
    }
}

pub fn resolve(data: &TemperatureData) -> ResolvedValue {
    ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": data.value,
            "type": "value",
            "unit": data.unit.map(|u| u.as_str()).unwrap_or("degree"),
        }),
    }
}
