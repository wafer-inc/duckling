// Ported from Duckling/Distance/EN/Corpus.hs
use duckling::{parse_en, DimensionKind, DimensionValue, MeasurementPoint, MeasurementValue};

fn check_distance(text: &str, expected_val: f64, expected_unit: &str) {
    let entities = parse_en(text, &[DimensionKind::Distance]);
    let found = entities.iter().any(|e| match &e.value {
        DimensionValue::Distance(mv) => match mv {
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
        },
        _ => false,
    });
    assert!(
        found,
        "Expected distance {} {} for '{}', got: {:?}",
        expected_val,
        expected_unit,
        text,
        entities
            .iter()
            .map(|e| format!("{:?}={:?}", e.value.dim_kind(), e.value))
            .collect::<Vec<_>>()
    );
}

// simple Kilometre 3
#[test]
fn test_distance_3_km() {
    check_distance("3 kilometers", 3.0, "kilometre");
    check_distance("3 km", 3.0, "kilometre");
    check_distance("3km", 3.0, "kilometre");
    check_distance("3k", 3.0, "kilometre");
    check_distance("3.0 km", 3.0, "kilometre");
}

// simple Mile 8
#[test]
fn test_distance_8_miles() {
    check_distance("8 miles", 8.0, "mile");
    check_distance("eight mile", 8.0, "mile");
    check_distance("8 mi", 8.0, "mile");
}

// simple M 9 - ambiguous "m" unit
#[test]
fn test_distance_9m() {
    check_distance("9m", 9.0, "m");
}

// simple Centimetre 2
#[test]
fn test_distance_2_cm() {
    check_distance("2cm", 2.0, "centimetre");
    check_distance("2 centimeters", 2.0, "centimetre");
}

// simple Inch 5
#[test]
fn test_distance_5_inches() {
    check_distance("5 in", 5.0, "inch");
    check_distance("5''", 5.0, "inch");
    check_distance("five inches", 5.0, "inch");
    check_distance("5\"", 5.0, "inch");
}

// simple Metre 1.87
#[test]
fn test_distance_1_87_metres() {
    check_distance("1.87 meters", 1.87, "metre");
}

// Composite values: 7 feet and 10 inches = 94 inches
#[test]
fn test_distance_composite_feet_inches() {
    check_distance("7 feet and 10 inches", 94.0, "inch");
    check_distance("7 feet, 10 inches", 94.0, "inch");
    check_distance("7 feet 10 inches", 94.0, "inch");
}

// 2 km and 1 meter = 2001 metres
#[test]
fn test_distance_composite_km_m() {
    check_distance("2 km and 1 meter", 2001.0, "metre");
    check_distance("2 kilometer, 1 metre", 2001.0, "metre");
    check_distance("2 kilometer 1 metre", 2001.0, "metre");
}

// 2 yards 7 ft 10 inches = 166 inches
#[test]
fn test_distance_composite_yards_feet_inches() {
    check_distance("2 yards 7 ft 10 inches", 166.0, "inch");
    check_distance("2 yds, 7 feet and 10 inches", 166.0, "inch");
    check_distance("2 yards, 7 feet, 10 in", 166.0, "inch");
}

// 2 yards and 7 feet = 13 feet
#[test]
fn test_distance_composite_yards_feet() {
    check_distance("2 yards and 7 feet", 13.0, "foot");
    check_distance("2 yards, 7 feet", 13.0, "foot");
    check_distance("2 yd 7'", 13.0, "foot");
}

// 10 kms 8 metres 6 cm = 1000806 centimetres
#[test]
fn test_distance_composite_km_m_cm() {
    check_distance("10 kms 8 metres 6 cm", 1000806.0, "centimetre");
    check_distance("10 kms, 8 meters, 6 cm", 1000806.0, "centimetre");
    check_distance(
        "10 kms, 8 meters and 6 centimeters",
        1000806.0,
        "centimetre",
    );
}

// 1 meter and 1 foot = 1.3048 metres
#[test]
fn test_distance_composite_m_ft() {
    check_distance("1 meter and 1 foot", 1.3048, "metre");
}

// 1 kilometer and 1 mile = 2.609344 kilometres
#[test]
fn test_distance_composite_km_mi() {
    check_distance("1 kilometer and 1 mile", 2.609344, "kilometre");
}

// 3m is ambiguous
#[test]
fn test_distance_3m_ambiguous() {
    check_distance("3m", 3.0, "m");
}

// 3m and 5cm = 305 centimetres (m inferred as metres)
#[test]
fn test_distance_3m_5cm() {
    check_distance("3m and 5cm", 305.0, "centimetre");
}

// 1m and 1ft = 5281 feet (m inferred as miles)
#[test]
fn test_distance_1m_1ft() {
    check_distance("1m and 1ft", 5281.0, "foot");
}

// Ranges: between 3 and 5 kilometres
#[test]
fn test_distance_range_3_5_km() {
    check_distance("between 3 and 5 kilometers", 3.0, "kilometre");
    check_distance("from 3km to 5km", 3.0, "kilometre");
    check_distance("around 3-5 kilometers", 3.0, "kilometre");
    check_distance("about 3km-5km", 3.0, "kilometre");
    check_distance("3-5 kilometers", 3.0, "kilometre");
}

// under 3.5 miles
#[test]
fn test_distance_under_3_5_miles() {
    check_distance("under 3.5 miles", 3.5, "mile");
    check_distance("less than 3.5mi", 3.5, "mile");
    check_distance("lower than three point five miles", 3.5, "mile");
}

// above 5 inches
#[test]
fn test_distance_above_5_inches() {
    check_distance("more than five inches", 5.0, "inch");
    check_distance("at least 5''", 5.0, "inch");
    check_distance("over 5\"", 5.0, "inch");
    check_distance("above 5 in", 5.0, "inch");
}

// between 5 and 6 millimetres
#[test]
fn test_distance_between_5_6_mm() {
    check_distance("between 5 and six millimeters", 5.0, "millimetre");
    check_distance("between 5 and six millimetres", 5.0, "millimetre");
    check_distance("5-6 mm", 5.0, "millimetre");
}
