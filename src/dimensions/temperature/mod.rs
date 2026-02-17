pub mod ar;
pub mod ca;
pub mod en;
pub mod es;
pub mod fr;
pub mod ga;
pub mod hi;
pub mod hr;
pub mod it;
pub mod ja;
pub mod km;
pub mod ko;
pub mod mn;
pub mod pt;
pub mod ro;
pub mod tr;
pub mod zh;

use crate::types::{DimensionValue, MeasurementPoint, MeasurementValue};

#[derive(Debug, Clone)]
pub struct TemperatureData {
    pub value: Option<f64>,
    pub unit: Option<TemperatureUnit>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
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
        TemperatureData {
            value: Some(value),
            unit: None,
            min_value: None,
            max_value: None,
        }
    }

    pub fn unit_only(unit: TemperatureUnit) -> Self {
        TemperatureData {
            value: None,
            unit: Some(unit),
            min_value: None,
            max_value: None,
        }
    }

    pub fn with_unit(mut self, unit: TemperatureUnit) -> Self {
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

pub fn units_are_compatible(u1: Option<TemperatureUnit>, u2: TemperatureUnit) -> bool {
    match u1 {
        Some(u) => u == u2,
        None => true,
    }
}

pub fn resolve(data: &TemperatureData) -> Option<DimensionValue> {
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
    Some(DimensionValue::Temperature(mv))
}
