pub mod en;

use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct DistanceData {
    pub value: f64,
    pub unit: Option<DistanceUnit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceUnit {
    Mile,
    Yard,
    Foot,
    Inch,
    Kilometre,
    Metre,
    Centimetre,
    Millimetre,
}

impl DistanceUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            DistanceUnit::Mile => "mile",
            DistanceUnit::Yard => "yard",
            DistanceUnit::Foot => "foot",
            DistanceUnit::Inch => "inch",
            DistanceUnit::Kilometre => "kilometre",
            DistanceUnit::Metre => "metre",
            DistanceUnit::Centimetre => "centimetre",
            DistanceUnit::Millimetre => "millimetre",
        }
    }
}

impl DistanceData {
    pub fn new(value: f64, unit: DistanceUnit) -> Self {
        DistanceData {
            value,
            unit: Some(unit),
        }
    }
}

pub fn resolve(data: &DistanceData) -> ResolvedValue {
    ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": data.value,
            "type": "value",
            "unit": data.unit.map(|u| u.as_str()).unwrap_or("unknown"),
        }),
    }
}
