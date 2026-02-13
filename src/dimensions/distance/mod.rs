pub mod ca;
pub mod cs;
pub mod de;
pub mod en;
pub mod es;
pub mod bg;
pub mod ga;
pub mod fr;
pub mod hr;
pub mod it;
pub mod km;
pub mod ko;
pub mod mn;
pub mod nl;
pub mod pt;
pub mod ro;
pub mod ru;
pub mod sv;
pub mod tr;
pub mod zh;

use crate::types::{DimensionValue, MeasurementPoint, MeasurementValue};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistanceUnit {
    Millimetre,
    Centimetre,
    Metre,
    Kilometre,
    Inch,
    Foot,
    Yard,
    Mile,
    M, // ambiguous: miles or metres
}

impl DistanceUnit {
    pub fn as_str(&self) -> &'static str {
        match self {
            DistanceUnit::Millimetre => "millimetre",
            DistanceUnit::Centimetre => "centimetre",
            DistanceUnit::Metre => "metre",
            DistanceUnit::Kilometre => "kilometre",
            DistanceUnit::Inch => "inch",
            DistanceUnit::Foot => "foot",
            DistanceUnit::Yard => "yard",
            DistanceUnit::Mile => "mile",
            DistanceUnit::M => "m",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DistanceData {
    pub value: Option<f64>,
    pub unit: Option<DistanceUnit>,
    pub min_value: Option<f64>,
    pub max_value: Option<f64>,
}

impl DistanceData {
    pub fn new(value: f64, unit: DistanceUnit) -> Self {
        DistanceData {
            value: Some(value),
            unit: Some(unit),
            min_value: None,
            max_value: None,
        }
    }

    pub fn value_only(value: f64) -> Self {
        DistanceData {
            value: Some(value),
            unit: None,
            min_value: None,
            max_value: None,
        }
    }

    pub fn unit_only(unit: DistanceUnit) -> Self {
        DistanceData {
            value: None,
            unit: Some(unit),
            min_value: None,
            max_value: None,
        }
    }

    pub fn with_unit(mut self, unit: DistanceUnit) -> Self {
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

// === Unit conversion system (ported from DistanceUnits/Types.hs) ===

fn is_metric(u: DistanceUnit) -> bool {
    matches!(
        u,
        DistanceUnit::Millimetre
            | DistanceUnit::Centimetre
            | DistanceUnit::Metre
            | DistanceUnit::Kilometre
    )
}

fn is_imperial(u: DistanceUnit) -> bool {
    matches!(
        u,
        DistanceUnit::Inch | DistanceUnit::Foot | DistanceUnit::Yard | DistanceUnit::Mile
    )
}

/// Ordering: smaller physical unit = lower order.
/// Metric units always ordered before Imperial (so cross-system prefers metric).
fn unit_order(u: DistanceUnit) -> u8 {
    match u {
        DistanceUnit::Millimetre => 0,
        DistanceUnit::Centimetre => 1,
        DistanceUnit::Metre => 2,
        DistanceUnit::Kilometre => 3,
        DistanceUnit::Inch => 4,
        DistanceUnit::Foot => 5,
        DistanceUnit::Yard => 6,
        DistanceUnit::Mile => 7,
        DistanceUnit::M => 2, // defaults to Metre order
    }
}

/// Convert a value from the given unit to metres (SI base).
fn to_metres(v: f64, u: DistanceUnit) -> f64 {
    const METRES_PER_INCH: f64 = 0.0254;
    match u {
        DistanceUnit::Millimetre => v / 1000.0,
        DistanceUnit::Centimetre => v / 100.0,
        DistanceUnit::Metre => v,
        DistanceUnit::Kilometre => v * 1000.0,
        DistanceUnit::Inch => v * METRES_PER_INCH,
        DistanceUnit::Foot => v * 12.0 * METRES_PER_INCH,
        DistanceUnit::Yard => v * 36.0 * METRES_PER_INCH,
        DistanceUnit::Mile => v * 63360.0 * METRES_PER_INCH,
        DistanceUnit::M => v, // treated as metres when no context
    }
}

/// Convert metres to the target unit.
fn from_metres(metres: f64, target: DistanceUnit) -> f64 {
    const METRES_PER_INCH: f64 = 0.0254;
    match target {
        DistanceUnit::Millimetre => metres * 1000.0,
        DistanceUnit::Centimetre => metres * 100.0,
        DistanceUnit::Metre => metres,
        DistanceUnit::Kilometre => metres / 1000.0,
        DistanceUnit::Inch => metres / METRES_PER_INCH,
        DistanceUnit::Foot => metres / (12.0 * METRES_PER_INCH),
        DistanceUnit::Yard => metres / (36.0 * METRES_PER_INCH),
        DistanceUnit::Mile => metres / (63360.0 * METRES_PER_INCH),
        DistanceUnit::M => metres,
    }
}

/// Sum two distances with different units, converting to the appropriate result unit.
/// Handles the ambiguous M unit by resolving based on context.
/// Returns None if the combination is invalid.
pub fn distance_sum(
    v1: f64,
    u1: DistanceUnit,
    v2: f64,
    u2: DistanceUnit,
) -> Option<(f64, DistanceUnit)> {
    // Resolve ambiguous M based on context
    let (r1, r2) = match (u1, u2) {
        (DistanceUnit::M, other) if is_metric(other) => (DistanceUnit::Metre, other),
        (DistanceUnit::M, other) if is_imperial(other) => (DistanceUnit::Mile, other),
        (other, DistanceUnit::M) if is_metric(other) => (other, DistanceUnit::Metre),
        (other, DistanceUnit::M) if is_imperial(other) => (other, DistanceUnit::Mile),
        (DistanceUnit::M, DistanceUnit::M) => return None, // can't combine two ambiguous
        _ => (u1, u2),
    };

    // Pick the smallest (most precise) unit as the target
    let target = if unit_order(r1) <= unit_order(r2) {
        r1
    } else {
        r2
    };

    let m1 = to_metres(v1, r1);
    let m2 = to_metres(v2, r2);
    let result = from_metres(m1 + m2, target);
    Some((result, target))
}

pub fn resolve(data: &DistanceData) -> Option<DimensionValue> {
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
    Some(DimensionValue::Distance(mv))
}
