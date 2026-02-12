pub mod en;

use crate::types::{DimensionValue, MeasurementPoint, MeasurementValue};

#[derive(Debug, Clone)]
pub struct VolumeData {
    pub value: Option<f64>,
    pub unit: Option<VolumeUnit>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
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
    /// Volume with a unit and value.
    pub fn new(value: f64, unit: VolumeUnit) -> Self {
        VolumeData {
            value: Some(value),
            unit: Some(unit),
            min_value: None,
            max_value: None,
        }
    }

    /// Value only (latent, no unit). Used by ruleNumeralAsVolume.
    pub fn value_only(value: f64) -> Self {
        VolumeData {
            value: Some(value),
            unit: None,
            min_value: None,
            max_value: None,
        }
    }

    /// Unit only (no value). Used by unit-only rules.
    pub fn unit_only(unit: VolumeUnit) -> Self {
        VolumeData {
            value: None,
            unit: Some(unit),
            min_value: None,
            max_value: None,
        }
    }

    pub fn with_unit(mut self, unit: VolumeUnit) -> Self {
        self.unit = Some(unit);
        self
    }

    pub fn with_interval(mut self, from: f64, to: f64) -> Self {
        self.value = None;
        self.min_value = Some(from);
        self.max_value = Some(to);
        self
    }

    pub fn with_min(mut self, v: f64) -> Self {
        self.value = None;
        self.min_value = Some(v);
        self.max_value = None;
        self
    }

    pub fn with_max(mut self, v: f64) -> Self {
        self.value = None;
        self.min_value = None;
        self.max_value = Some(v);
        self
    }
}

pub fn resolve(data: &VolumeData) -> Option<DimensionValue> {
    let unit = data.unit.as_ref()?;
    let unit_str = unit.as_str().to_string();

    let mv = match (data.value, data.min_value, data.max_value) {
        (Some(v), _, _) => MeasurementValue::Value {
            value: v,
            unit: unit_str,
        },
        (None, Some(from), Some(to)) => MeasurementValue::Interval {
            from: Some(MeasurementPoint {
                value: from,
                unit: unit_str.clone(),
            }),
            to: Some(MeasurementPoint {
                value: to,
                unit: unit_str,
            }),
        },
        (None, Some(from), None) => MeasurementValue::Interval {
            from: Some(MeasurementPoint {
                value: from,
                unit: unit_str,
            }),
            to: None,
        },
        (None, None, Some(to)) => MeasurementValue::Interval {
            from: None,
            to: Some(MeasurementPoint {
                value: to,
                unit: unit_str,
            }),
        },
        _ => return None,
    };
    Some(DimensionValue::Volume(mv))
}
