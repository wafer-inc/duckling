pub mod en;

use crate::types::ResolvedValue;

#[derive(Debug, Clone)]
pub struct VolumeData {
    pub value: f64,
    pub unit: Option<VolumeUnit>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeUnit {
    Gallon,
    Litre,
    Millilitre,
    Hectolitre,
    Cup,
    Pint,
    Quart,
    FluidOunce,
    Tablespoon,
    Teaspoon,
}

impl VolumeUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            VolumeUnit::Gallon => "gallon",
            VolumeUnit::Litre => "litre",
            VolumeUnit::Millilitre => "millilitre",
            VolumeUnit::Hectolitre => "hectolitre",
            VolumeUnit::Cup => "cup",
            VolumeUnit::Pint => "pint",
            VolumeUnit::Quart => "quart",
            VolumeUnit::FluidOunce => "fluid ounce",
            VolumeUnit::Tablespoon => "tablespoon",
            VolumeUnit::Teaspoon => "teaspoon",
        }
    }
}

impl VolumeData {
    pub fn new(value: f64, unit: VolumeUnit) -> Self {
        VolumeData {
            value,
            unit: Some(unit),
        }
    }
}

pub fn resolve(data: &VolumeData) -> ResolvedValue {
    ResolvedValue {
        kind: "value".to_string(),
        value: serde_json::json!({
            "value": data.value,
            "type": "value",
            "unit": data.unit.map(|u| u.as_str()).unwrap_or("unknown"),
        }),
    }
}
