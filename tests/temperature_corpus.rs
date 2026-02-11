// Ported from Duckling/Temperature/EN/Corpus.hs
use duckling::{parse_en, DimensionKind};

fn check_temperature(text: &str, expected_val: f64, expected_unit: &str) {
    let entities = parse_en(text, &[DimensionKind::Temperature]);
    let found = entities.iter().any(|e| {
        if e.dim != "temperature" {
            return false;
        }
        // Check top-level value (simple temperature)
        let simple_match = e
            .value
            .value
            .get("value")
            .and_then(|v| v.as_f64())
            .map(|v| (v - expected_val).abs() < 0.01)
            .unwrap_or(false)
            && e.value.value.get("unit").and_then(|v| v.as_str()) == Some(expected_unit);
        if simple_match {
            return true;
        }
        // Check interval from value
        let interval_match = e
            .value
            .value
            .get("from")
            .and_then(|f| f.get("value"))
            .and_then(|v| v.as_f64())
            .map(|v| (v - expected_val).abs() < 0.01)
            .unwrap_or(false)
            && e.value
                .value
                .get("from")
                .and_then(|f| f.get("unit"))
                .and_then(|v| v.as_str())
                == Some(expected_unit);
        if interval_match {
            return true;
        }
        // Check open interval (min only)
        let min_match = e
            .value
            .value
            .get("type")
            .and_then(|v| v.as_str())
            == Some("interval")
            && e.value.value.get("to").is_none()
            && e.value
                .value
                .get("from")
                .and_then(|f| f.get("value"))
                .and_then(|v| v.as_f64())
                .map(|v| (v - expected_val).abs() < 0.01)
                .unwrap_or(false)
            && e.value
                .value
                .get("from")
                .and_then(|f| f.get("unit"))
                .and_then(|v| v.as_str())
                == Some(expected_unit);
        if min_match {
            return true;
        }
        // Check open interval (max only)
        let max_match = e
            .value
            .value
            .get("type")
            .and_then(|v| v.as_str())
            == Some("interval")
            && e.value.value.get("from").is_none()
            && e.value
                .value
                .get("to")
                .and_then(|f| f.get("value"))
                .and_then(|v| v.as_f64())
                .map(|v| (v - expected_val).abs() < 0.01)
                .unwrap_or(false)
            && e.value
                .value
                .get("to")
                .and_then(|f| f.get("unit"))
                .and_then(|v| v.as_str())
                == Some(expected_unit);
        max_match
    });
    assert!(
        found,
        "Expected temperature {} {} for '{}', got: {:?}",
        expected_val,
        expected_unit,
        text,
        entities
            .iter()
            .map(|e| format!("{}={:?}", e.dim, e.value))
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
    // These require range support which we haven't implemented
    // Including them so they show up as failures to track
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
