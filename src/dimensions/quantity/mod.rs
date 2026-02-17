pub mod ar;
pub mod en;
pub mod es;
pub mod fr;
pub mod hr;
pub mod km;
pub mod ko;
pub mod mn;
pub mod nl;
pub mod pt;
pub mod ro;
pub mod ru;
pub mod zh;

use crate::types::{DimensionValue, MeasurementPoint, MeasurementValue};

#[derive(Debug, Clone)]
pub struct QuantityData {
    pub value: Option<f64>,
    pub unit: Option<QuantityUnit>,
    pub product: Option<String>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuantityUnit {
    Cup,
    Gram,
    Ounce,
    Pound,
    Tablespoon,
}

impl QuantityUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            QuantityUnit::Cup => "cup",
            QuantityUnit::Gram => "gram",
            QuantityUnit::Ounce => "ounce",
            QuantityUnit::Pound => "pound",
            QuantityUnit::Tablespoon => "tablespoon",
        }
    }
}

impl QuantityData {
    pub fn new(value: f64, unit: QuantityUnit) -> Self {
        QuantityData {
            value: Some(value),
            unit: Some(unit),
            product: None,
            min_value: None,
            max_value: None,
        }
    }

    pub fn unit_only(unit: QuantityUnit) -> Self {
        QuantityData {
            value: None,
            unit: Some(unit),
            product: None,
            min_value: None,
            max_value: None,
        }
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

pub fn resolve(data: &QuantityData) -> Option<DimensionValue> {
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
    Some(DimensionValue::Quantity {
        measurement: mv,
        product: data.product.clone(),
    })
}
