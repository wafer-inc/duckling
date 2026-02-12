// Ported from Duckling/Temperature/EN/Corpus.hs
use duckling::{parse_en, DimensionKind, DimensionValue, MeasurementValue, MeasurementPoint};

fn check_temperature(text: &str, expected_val: f64, expected_unit: &str) {
    let entities = parse_en(text, &[DimensionKind::Temperature]);
    let found = entities.iter().any(|e| {
        match &e.value {
            DimensionValue::Temperature(mv) => match mv {
                MeasurementValue::Value { value, unit } => {
                    (*value - expected_val).abs() < 0.01 && unit == expected_unit
                }
                MeasurementValue::Interval { from, to } => {
                    if let Some(MeasurementPoint { value, unit }) = from {
                        if (*value - expected_val).abs() < 0.01 && unit == expected_unit {
                            return true;
                        }
                    }
                    if let Some(MeasurementPoint { value, unit }) = to {
                        if (*value - expected_val).abs() < 0.01 && unit == expected_unit {
                            return true;
                        }
                    }
                    false
                }
            }
            _ => false,
        }
    });
    assert!(
        found,
        "Expected temperature {} {} for '{}', got: {:?}",
        expected_val,
        expected_unit,
        text,
        entities
            .iter()
            .map(|e| format!("{:?}={:?}", e.value.dim_kind(), e.value))
            .collect::<Vec<_>>()
    );
}

// simple Celsius 37
#[test]
fn test_temp_37_celsius() {
    check_temperature("37°C", 37.0, "celsius");
    check_temperature("37 ° celsius", 37.0, "celsius");
    check_temperature("37 degrees Celsius", 37.0, "celsius");
    check_temperature("thirty seven celsius", 37.0, "celsius");
    check_temperature("37 degrees Celsius", 37.0, "celsius");
    check_temperature("thirty seven celsius", 37.0, "celsius");
}

// simple Fahrenheit 70
#[test]
fn test_temp_70_fahrenheit() {
    check_temperature("70°F", 70.0, "fahrenheit");
    check_temperature("70 ° Fahrenheit", 70.0, "fahrenheit");
    check_temperature("70 degrees F", 70.0, "fahrenheit");
    check_temperature("seventy Fahrenheit", 70.0, "fahrenheit");
}

// simple Fahrenheit 98.6
#[test]
fn test_temp_98_6_fahrenheit() {
    check_temperature("98.6°F", 98.6, "fahrenheit");
    check_temperature("98.6 ° Fahrenheit", 98.6, "fahrenheit");
    check_temperature("98.6 degrees F", 98.6, "fahrenheit");
}

// simple Degree 45
#[test]
fn test_temp_45_degree() {
    check_temperature("45°", 45.0, "degree");
    check_temperature("45 degrees", 45.0, "degree");
    check_temperature("45 deg.", 45.0, "degree");
}

// simple Degree (-2)
#[test]
fn test_temp_negative_2_degree() {
    check_temperature("-2°", -2.0, "degree");
    check_temperature("- 2 degrees", -2.0, "degree");
    check_temperature("2 degrees below zero", -2.0, "degree");
    check_temperature("2 below zero", -2.0, "degree");
}

// between Degree (30, 40) - range tests
#[test]
fn test_temp_between_30_40_degree() {
    check_temperature("between 30 and 40 degrees", 30.0, "degree");
    check_temperature("from 30 degrees to 40 degrees", 30.0, "degree");
}

// between Celsius (30, 40)
#[test]
fn test_temp_between_30_40_celsius() {
    check_temperature("between 30 and 40 celsius", 30.0, "celsius");
    check_temperature("from 30 celsius and 40 celsius", 30.0, "celsius");
    check_temperature("between 30 and 40 degrees celsius", 30.0, "celsius");
    check_temperature("from 30 degrees celsius to 40 degrees celsius", 30.0, "celsius");
    check_temperature("30-40 degrees celsius", 30.0, "celsius");
}

// above Degree 40
#[test]
fn test_temp_above_40_degree() {
    check_temperature("over 40 degrees", 40.0, "degree");
    check_temperature("at least 40 degrees", 40.0, "degree");
    check_temperature("more than 40 degrees", 40.0, "degree");
}

// under Degree 40
#[test]
fn test_temp_under_40_degree() {
    check_temperature("under 40 degrees", 40.0, "degree");
    check_temperature("less than 40 degrees", 40.0, "degree");
    check_temperature("lower than 40 degrees", 40.0, "degree");
}
